use crate::{
	client::ClientError,
	config::{
		ContractName,
		ContractName::{BankDiamond, GovernanceFacet},
	},
	contract::UnwrapContractError,
	ibc_provider::{
		DIAMONDABI_ABI, GOVERNANCEPROXYABI_ABI, ICS20BANKABI_ABI, ICS20TRANSFERBANKABI_ABI,
		TENDERMINTCLIENTABI_ABI,
	},
};
use cast::revm::primitives::hex_literal::hex;
use ethers::{
	abi::{AbiError, Address, Detokenize, EventExt, Function, Token, Tokenizable, Tokenize},
	contract::{ContractFactory, ContractInstance, FunctionCall},
	core::types::Bytes,
	middleware::SignerMiddleware,
	prelude::{
		builders::Deployer, Block, ContractError, EthEvent, Event, Filter, Http, LocalWallet,
		Middleware, Provider, Signer, TransactionReceipt, TransactionRequest, H256, U256,
	},
	types::{BlockNumber, Bloom, H160, H64, U64},
	utils::{rlp, rlp::RlpStream},
};
use ethers_solc::{
	artifacts::{
		output_selection::OutputSelection, Libraries, Optimizer, OptimizerDetails, Settings,
		StorageLayout,
	},
	report::{BasicStdoutReporter, Report},
	Artifact, ArtifactOutput, ConfigurableContractArtifact, EvmVersion, Project,
	ProjectCompileOutput, ProjectPathsConfig, SolcConfig,
};
use ibc::core::{
	ics02_client::client_state::ClientType,
	ics04_channel::packet::Packet,
	ics23_commitment::{commitment::CommitmentProofBytes, merkle::MerkleProof},
};
use ics23::commitment_proof::Proof;
use icsxx_ethereum::utils::keccak256;
use log::info;
use pallet_ibc::light_clients::HostFunctionsManager;
use std::{
	borrow::Borrow,
	collections::{HashMap, HashSet},
	iter::once,
	ops::Mul,
	path::{Path, PathBuf},
	str::FromStr,
	sync::{Arc, Mutex},
	time::Duration,
};
use tokio::time::sleep;
use ContractName::*;

pub type ProviderImpl = ethers::prelude::SignerMiddleware<Provider<Http>, LocalWallet>;

pub const SEQUENCES_PER_ITER: u64 = 256;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum FacetCutAction {
	Add = 0,
	Replace = 1,
	Remove = 2,
}

#[derive(Clone, Debug)]
pub struct FacetCut {
	pub address: Address,
	pub action: FacetCutAction,
	pub selectors: Vec<(String, [u8; 4])>,
}

impl FacetCut {
	pub fn into_token(self) -> Token {
		Token::Tuple(vec![
			Token::Address(self.address),
			Token::Uint((self.action as u32).into()),
			Token::Array(
				self.selectors.into_iter().map(|(_, x)| Token::FixedBytes(x.to_vec())).collect(),
			),
		])
	}
}

#[derive(Debug)]
pub struct NamedContract<B, M> {
	contract: ContractInstance<B, M>,
	abi_name: ContractName,
}

impl<B: Clone + Borrow<M>, M> Clone for NamedContract<B, M> {
	fn clone(&self) -> Self {
		Self { contract: self.contract.clone(), abi_name: self.abi_name }
	}
}

impl<B, M> NamedContract<B, M>
where
	B: Borrow<M> + Clone,
	M: Middleware,
{
	pub fn new(contract: ContractInstance<B, M>, abi_name: ContractName) -> Self {
		Self { contract, abi_name }
	}

	pub fn from_address(address: Address, abi_name: ContractName, client: B) -> Self {
		Self {
			contract: ContractInstance::<B, M>::new(address, abi_name.to_abi(), client),
			abi_name,
		}
	}

	pub fn contract(&self) -> &ContractInstance<B, M> {
		&self.contract
	}

	pub fn abi_name(&self) -> ContractName {
		self.abi_name
	}
}

#[derive(Debug)]
pub struct DeployYuiIbc<B, M> {
	pub ibc_core_diamond: ContractInstance<B, M>,
	pub ibc_core_facets: Vec<NamedContract<B, M>>,

	pub ibc_transfer_diamond: Option<ContractInstance<B, M>>,
	pub ibc_transfer_facets: Vec<NamedContract<B, M>>,

	pub bank_diamond: Option<ContractInstance<B, M>>,
	pub bank_facets: Vec<NamedContract<B, M>>,

	pub tendermint_diamond: Option<ContractInstance<B, M>>,
	pub tendermint_facets: Vec<NamedContract<B, M>>,

	pub gov_proxy: Option<ContractInstance<B, M>>,
	pub contract_creation_block: Arc<Mutex<Option<BlockNumber>>>,
	// pub storage_layout: StorageLayout,
}

impl<B, M> DeployYuiIbc<B, M>
where
	B: Borrow<M> + Clone,
	M: Middleware,
{
	pub async fn new(
		ibc_core_diamond: ContractInstance<B, M>,
		ibc_core_facets: Vec<NamedContract<B, M>>,
		tendermint_diamond: Option<ContractInstance<B, M>>,
		tendermint_facets: Vec<NamedContract<B, M>>,
		ibc_transfer_diamond_diamond: Option<ContractInstance<B, M>>,
		ibc_transfer_diamond_facets: Vec<NamedContract<B, M>>,
		ics20_bank_diamond: Option<ContractInstance<B, M>>,
		ics20_bank_facets: Vec<NamedContract<B, M>>,
		gov_proxy: Option<ContractInstance<B, M>>,
	) -> Result<Self, ClientError> {
		let ibc = Self {
			ibc_core_diamond,
			tendermint_diamond,
			tendermint_facets,
			gov_proxy,
			ibc_transfer_diamond: ibc_transfer_diamond_diamond,
			ibc_transfer_facets: ibc_transfer_diamond_facets,
			bank_diamond: ics20_bank_diamond,
			ibc_core_facets,
			contract_creation_block: Arc::new(Mutex::new(None)),
			bank_facets: ics20_bank_facets,
		};
		let creation_block: U256 = ibc
			.method("getContractCreationBlock", ())?
			.call()
			.await
			.map_err(|e| {
				ClientError::Other(format!("Error getting contract creation block: {}", e))
			})
			.unwrap_or(U256::zero());
		ibc.set_contract_creation_block(creation_block);
		Ok(ibc)
	}

	pub async fn from_addresses(
		client: B,
		ibc_core_diamond_address: Address,
		ibc_core_facets: Vec<(ContractName, Address)>,
		tendermint_diamond_address: Option<Address>,
		tendermint_facets: Vec<(ContractName, Address)>,
		ibc_transfer_diamond_address: Option<Address>,
		ibc_transfer_facets: Vec<(ContractName, Address)>,
		ics20_bank_diamond_address: Option<Address>,
		ics20_bank_facets: Vec<(ContractName, Address)>,
		gov_proxy_address: Option<Address>,
	) -> Result<Self, ClientError> {
		let ibc_core_diamond = ContractInstance::<B, M>::new(
			ibc_core_diamond_address,
			DIAMONDABI_ABI.clone(),
			client.clone(),
		);
		let tendermint_diamond = tendermint_diamond_address.map(|addr| {
			ContractInstance::<B, M>::new(addr, DIAMONDABI_ABI.clone(), client.clone())
		});
		let gov_proxy = gov_proxy_address.map(|addr| {
			ContractInstance::<B, M>::new(addr, GOVERNANCEPROXYABI_ABI.clone(), client.clone())
		});
		let ibc_transfer_diamond = ibc_transfer_diamond_address.map(|addr| {
			ContractInstance::<B, M>::new(addr, DIAMONDABI_ABI.clone(), client.clone())
		});
		let ics20_bank_diamond = ics20_bank_diamond_address.map(|addr| {
			ContractInstance::<B, M>::new(addr, DIAMONDABI_ABI.clone(), client.clone())
		});
		let deployed_facets = ibc_core_facets
			.into_iter()
			.map(|(abi, addr)| NamedContract::from_address(addr, abi, client.clone()))
			.collect();
		Ok(Self::new(
			ibc_core_diamond,
			deployed_facets,
			tendermint_diamond,
			tendermint_facets
				.into_iter()
				.map(|(abi, addr)| NamedContract::from_address(addr, abi, client.clone()))
				.collect(),
			ibc_transfer_diamond,
			ibc_transfer_facets
				.into_iter()
				.map(|(abi, addr)| NamedContract::from_address(addr, abi, client.clone()))
				.collect(),
			ics20_bank_diamond,
			ics20_bank_facets
				.into_iter()
				.map(|(abi, addr)| NamedContract::from_address(addr, abi, client.clone()))
				.collect(),
			gov_proxy,
		)
		.await?)
	}

	pub fn set_contract_creation_block(&self, number: U256) {
		*self.contract_creation_block.lock().unwrap() =
			Some(BlockNumber::Number(number.as_u64().into()));
	}

	pub fn contract_creation_block(&self) -> BlockNumber {
		self.contract_creation_block
			.lock()
			.unwrap()
			.clone()
			.unwrap_or(BlockNumber::Earliest)
	}

	pub fn contract_address_by_name(&self, contract_name: ContractName) -> Option<Address> {
		use ContractName::*;
		match contract_name {
			IbcCoreDiamond => Some(self.ibc_core_diamond.address()),
			BankDiamond => self.bank_diamond.as_ref().map(|x| x.address()),
			IbcTransferDiamond => self.ibc_transfer_diamond.as_ref().map(|x| x.address()),
			TendermintDiamond => self.tendermint_diamond.as_ref().map(|x| x.address()),
			DiamondLoupeFacet |
			DiamondCutFacet |
			CallBatchFacet |
			IBCChannelHandshake |
			IBCClient |
			IBCConnection |
			IBCPacket |
			IBCQuerier |
			OwnershipFacet |
			GovernanceFacet |
			RelayerWhitelistFacet => self
				.ibc_core_facets
				.iter()
				.find(|x| x.abi_name == contract_name)
				.map(|x| x.contract.address()),
			ICS20Bank => self
				.bank_facets
				.iter()
				.find(|x| x.abi_name == contract_name)
				.map(|x| x.contract.address()),
			ICS20TransferBank => self
				.ibc_transfer_facets
				.iter()
				.find(|x| x.abi_name == contract_name)
				.map(|x| x.contract.address()),
			TendermintLightClientZK => self
				.tendermint_facets
				.iter()
				.find(|x| x.abi_name == contract_name)
				.map(|x| x.contract.address()),
			ERC20Token => None,
		}
	}
}

impl<B, M> DeployYuiIbc<B, M>
where
	B: Clone + Borrow<M>,
	M: Middleware,
{
	pub async fn bind_port(&self, port_id: &str, address: Address) {
		sleep(Duration::from_secs(12)).await;
		let bind_port = self
			.method::<_, ()>("bindPort", (Token::String(port_id.into()), Token::Address(address)))
			.unwrap();
		let () = bind_port.call().await.unwrap_contract_error();
		let tx_recp = bind_port.send().await.unwrap_contract_error().await.unwrap().unwrap();
		handle_gas_usage(&tx_recp);
		assert_eq!(tx_recp.status, Some(1.into()));
	}

	pub async fn add_relayer(&self, address: Address) {
		sleep(Duration::from_secs(12)).await;
		let method = self.method::<_, ()>("addRelayer", Token::Address(address)).unwrap();
		let () = method.call().await.unwrap_contract_error();
		let tx_recp = method.send().await.unwrap_contract_error().await.unwrap().unwrap();
		handle_gas_usage(&tx_recp);
		assert_eq!(tx_recp.status, Some(1.into()));
	}

	pub async fn set_gov_tendermint_client(&self, address: Address, diamond_name: ContractName) {
		sleep(Duration::from_secs(12)).await;
		let method = self
			.method_diamond::<_, ()>("setTendermintClient", Token::Address(address), diamond_name)
			.unwrap();
		let () = method.call().await.unwrap_contract_error();
		let tx_recp = method.send().await.unwrap_contract_error().await.unwrap().unwrap();
		handle_gas_usage(&tx_recp);
		assert_eq!(tx_recp.status, Some(1.into()));
	}

	pub async fn transfer_ownership(&self, address: Address, diamond_name: ContractName) {
		sleep(Duration::from_secs(12)).await;
		let method = self
			.method_diamond::<_, ()>("transferOwnership", Token::Address(address), diamond_name)
			.unwrap();
		let () = method.call().await.unwrap_contract_error();
		let tx_recp = method.send().await.unwrap_contract_error().await.unwrap().unwrap();
		handle_gas_usage(&tx_recp);
		assert_eq!(tx_recp.status, Some(1.into()));
	}

	pub async fn set_gov_proxy(&self, address: Address, diamond_name: ContractName) {
		sleep(Duration::from_secs(12)).await;
		let method = self
			.method_diamond::<_, ()>("setProxy", Token::Address(address), diamond_name)
			.unwrap();
		let () = method.call().await.unwrap_contract_error();
		let tx_recp = method.send().await.unwrap_contract_error().await.unwrap().unwrap();
		handle_gas_usage(&tx_recp);
		assert_eq!(tx_recp.status, Some(1.into()));
	}

	pub async fn add_gov_signatory(&self, address: Address, diamond_name: ContractName) {
		sleep(Duration::from_secs(12)).await;
		let method = self
			.method_diamond::<_, ()>("addSignatory", Token::Address(address), diamond_name)
			.unwrap();
		send_retrying(&method).await.unwrap();
	}

	pub async fn remove_gov_signatory(&self, address: Address, diamond_name: ContractName) {
		let method = self
			.method_diamond::<_, ()>("removeSignatory", Token::Address(address), diamond_name)
			.unwrap();
		send_retrying(&method).await.unwrap();
	}

	pub async fn connection_open_init_mock(&self, client_id: &str) -> String {
		let connection_open_init = self
			.method::<_, String>(
				"connectionOpenInit",
				(Token::Tuple(vec![
					Token::String(client_id.into()),
					Token::Tuple(vec![
						Token::String(client_id.into()),
						Token::String("port-0".into()),
						Token::Tuple(vec![Token::Bytes(vec![])]),
					]),
					Token::Uint(0.into()),
				]),),
			)
			.unwrap();
		let connection_id = connection_open_init.call().await.unwrap_contract_error();
		let tx_recp = connection_open_init
			.send()
			.await
			.unwrap_contract_error()
			.await
			.unwrap()
			.unwrap();
		assert_eq!(tx_recp.status, Some(1.into()));
		connection_id
	}

	pub async fn connection_open_ack_mock(&self, connection_id: &str, client_state_bytes: Vec<u8>) {
		let connection_open_ack = self
			.method::<_, ()>(
				"connectionOpenAck",
				(Token::Tuple(vec![
					Token::String(connection_id.to_string()),
					Token::Bytes(client_state_bytes), // clientStateBytes
					Token::Tuple(vec![
						Token::String("counterparty-version".into()),
						Token::Array(vec![]),
					]), // Version.Data
					Token::String("counterparty-connection-id".into()), // counterpartyConnectionID
					Token::Bytes(vec![]),             // proofTry
					Token::Bytes(vec![]),             // proofClient
					Token::Bytes(vec![]),             // proofConsensus
					Token::Tuple(vec![Token::Uint(0.into()), Token::Uint(1.into())]), // proofHeight
					Token::Tuple(vec![Token::Uint(0.into()), Token::Uint(1.into())]), // consesusHeight
				]),),
			)
			.unwrap();

		let () = connection_open_ack.call().await.unwrap_contract_error();
		let tx_recp =
			connection_open_ack.send().await.unwrap_contract_error().await.unwrap().unwrap();

		dbg!(&tx_recp.block_number);

		assert_eq!(tx_recp.status, Some(1.into()));
	}

	pub async fn channel_open_init_mock(&self, port_id: &str, connection_id: &str) -> String {
		let fut = self
			.method::<_, String>(
				"channelOpenInit",
				(Token::Tuple(vec![
					Token::String(port_id.into()),
					Token::Tuple(vec![
						// Channel.Data
						Token::Uint(1.into()), // State, Init
						Token::Uint(1.into()), // Ordering
						Token::Tuple(vec![
							Token::String("port-0".into()),
							Token::String("channel-0".into()),
						]), // counterparty
						Token::Array(vec![Token::String(connection_id.into())]), // connectionHops
						Token::String("1".into()), // version
					]),
				]),),
			)
			.unwrap();

		let channel_id = fut.call().await.unwrap_contract_error();
		let tx = fut.send().await.unwrap_contract_error().await.unwrap().unwrap();
		assert_eq!(tx.status, Some(1.into()));
		channel_id
	}

	pub async fn channel_open_ack(&self, channel_id: &str, port_id: &str) {
		let fut = self
			.method::<_, ()>(
				"channelOpenAck",
				(Token::Tuple(vec![
					Token::String(port_id.into()),     // port-id
					Token::String(channel_id.into()),  // channel-id
					Token::String("1".into()),         // counterparty-version
					Token::String("channel-0".into()), // counterparty-channel-id
					Token::Bytes(vec![]),              // proof-try
					Token::Tuple(vec![
						// proof-height
						Token::Uint(0.into()),
						Token::Uint(1.into()),
					]),
				]),),
			)
			.unwrap();

		let () = fut.call().await.unwrap_contract_error();
		let tx = fut.send().await.unwrap_contract_error().await.unwrap().unwrap();
		assert_eq!(tx.status, Some(1.into()));
	}

	pub async fn recv_packet(&self, packet: Packet) -> TransactionReceipt {
		let fut = self
			.method::<_, ()>(
				"recvPacket",
				(Token::Tuple(vec![
					Token::Tuple(vec![
						Token::Uint(packet.sequence.0.into()),              // sequence
						Token::String(packet.source_port.to_string()),      // port-id
						Token::String(packet.source_channel.to_string()),   // channel-id
						Token::String(packet.destination_port.to_string()), // port-id
						Token::String(packet.destination_channel.to_string()), // channel-id
						Token::Bytes(packet.data),                          // data
						Token::Tuple(vec![
							// timeout-height
							Token::Uint(packet.timeout_height.revision_number.into()),
							Token::Uint(packet.timeout_height.revision_height.into()),
						]),
						Token::Uint(
							packet
								.timeout_timestamp
								.into_tm_time()
								.map(|x| x.unix_timestamp_nanos() as u64)
								.unwrap_or(0u64)
								.into(),
						), /* timeout-timestamp */
					]),
					Token::Bytes(vec![]), /* proof */
					Token::Tuple(vec![
						// proof-height
						Token::Uint(0.into()),
						Token::Uint(1.into()),
					]),
				]),),
			)
			.unwrap();

		let () = fut.call().await.unwrap_contract_error();
		// let trace = self
		// 	.ibc_handler
		// 	.client()
		// 	.borrow()
		// 	.debug_trace_call(fut.tx.clone(), None, GethDebugTracingCallOptions::default())
		// 	.await
		// 	.unwrap();
		// std::fs::write("trace.txt", format!("{:#?}", trace)).unwrap();
		// println!("trace: {:?}", trace);
		let tx = fut.send().await.unwrap_contract_error().await.unwrap().unwrap();
		// dbg!(tx.logs);
		let status = tx.status.expect("status not found");

		if status == 0.into() {
			panic!("status is 0");
		}
		tx
	}

	pub async fn create_client(&self, msg: Token) -> (String, (H256, H256)) {
		let method = self.method::<_, String>("createClient", (msg,)).unwrap();

		let client_id = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));

		(client_id, (receipt.block_hash.unwrap(), receipt.transaction_hash))
	}

	pub async fn create_client_calldata(&self, msg: Token) -> Bytes {
		let method = self.method::<_, String>("createClient", (msg,)).unwrap();
		method.calldata().unwrap()
	}

	pub async fn update_client(&self, msg: Token) {
		let method = self.method::<_, ()>("updateClient", (msg,)).unwrap();

		let gas_estimate_update_client = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate_update_client);
		let client_id = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
	}

	pub async fn update_client_calldata(&self, msg: Token) -> Bytes {
		let method = self.method::<_, ()>("updateClient", (msg,)).unwrap();
		method.calldata().unwrap()
	}

	pub async fn connection_open_ack(&self, msg: Token) {
		let method = self.method::<_, ()>("connectionOpenAck", (msg,)).unwrap();

		let gas_estimate_connection_open = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate_connection_open);
		let _ = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
	}

	pub async fn connection_open_ack_calldata(&self, msg: Token) -> Bytes {
		let method = self.method::<_, ()>("connectionOpenAck", (msg,)).unwrap();
		method.calldata().unwrap()
	}

	pub async fn connection_open_try(&self, msg: Token) -> String {
		let method = self.method::<_, String>("connectionOpenTry", (msg,)).unwrap();

		let gas_estimate_connection_open_try = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate_connection_open_try);
		let id = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
		id
	}

	pub async fn connection_open_try_calldata(&self, msg: Token) -> Bytes {
		let method = self.method::<_, String>("connectionOpenTry", (msg,)).unwrap();
		method.calldata().unwrap()
	}

	pub async fn connection_open_init(&self, msg: Token) -> (String, (H256, H256)) {
		let method = self.method::<_, String>("connectionOpenInit", (msg,)).unwrap();

		let gas_estimate_connection_open_try = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate_connection_open_try);
		let id = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
		let tx_id = (receipt.block_hash.unwrap(), receipt.transaction_hash);
		(id, tx_id)
	}

	pub async fn connection_open_init_calldata(&self, msg: Token) -> Bytes {
		let method = self.method::<_, String>("connectionOpenInit", (msg,)).unwrap();
		method.calldata().unwrap()
	}

	pub async fn connection_open_confirm(&self, msg: Token) {
		let method = self.method::<_, ()>("connectionOpenConfirm", (msg,)).unwrap();

		let gas_estimate_connection_open_confirm = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate_connection_open_confirm);
		let _ = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
	}

	pub async fn connection_open_confirm_calldata(&self, msg: Token) -> Bytes {
		let method = self.method::<_, ()>("connectionOpenConfirm", (msg,)).unwrap();
		method.calldata().unwrap()
	}

	pub async fn channel_open_init(&self, msg: Token) -> (String, (H256, H256)) {
		let method = self.method::<_, String>("channelOpenInit", (msg,)).unwrap();

		let gas_estimate_connection_id = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate_connection_id);
		let connection_id = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));

		let tx_id = (receipt.block_hash.unwrap(), receipt.transaction_hash);
		(connection_id, tx_id)
	}

	pub async fn channel_open_init_calldata(&self, msg: Token) -> Bytes {
		let method = self.method::<_, String>("channelOpenInit", (msg,)).unwrap();
		method.calldata().unwrap()
	}

	pub async fn channel_open_try(&self, msg: Token) -> String {
		let method = self.method::<_, String>("channelOpenTry", (msg,)).unwrap();

		let gas_estimate_connection_id = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate_connection_id);
		let connection_id = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
		connection_id
	}

	pub async fn channel_open_try_calldata(&self, msg: Token) -> Bytes {
		let method = self.method::<_, String>("channelOpenTry", (msg,)).unwrap();
		method.calldata().unwrap()
	}

	pub async fn send_and_get_tuple(&self, msg: Token, method_name: impl AsRef<str>) -> () {
		let method = self.method::<_, ()>(method_name.as_ref(), (msg,)).unwrap();

		let gas_estimate = method.estimate_gas().await.unwrap();
		dbg!(gas_estimate);
		let ret = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		assert_eq!(receipt.status, Some(1.into()));
		ret
	}

	pub async fn send_and_get_tuple_calldata(
		&self,
		msg: Token,
		method_name: impl AsRef<str>,
	) -> Bytes {
		let method = self.method::<_, ()>(method_name.as_ref(), (msg,)).unwrap();
		method.calldata().unwrap()
	}

	pub fn function(&self, name: &str) -> ethers::abi::Result<&Function> {
		let mut func = None;
		for faucet in self
			.ibc_core_facets
			.iter()
			.map(|x| x.contract())
			.chain(once(&self.ibc_core_diamond))
		{
			if let Ok(f) = faucet.abi().function(name) {
				if func.is_some() {
					log::error!(target: "hyperspace_ethereum", "ambiguous function name: {}", name);
				}
				func = Some(f);
			}
		}
		func.ok_or_else(|| ethers::abi::Error::InvalidName(name.into()))
	}

	pub fn method<T: Tokenize, D: Detokenize>(
		&self,
		name: &str,
		args: T,
	) -> Result<FunctionCall<B, M, D>, AbiError> {
		self.method_diamond(name, args, IbcCoreDiamond)
	}

	pub fn method_diamond<T: Tokenize, D: Detokenize>(
		&self,
		name: &str,
		args: T,
		diamond_name: ContractName,
	) -> Result<FunctionCall<B, M, D>, AbiError> {
		let mut contract: Option<&ContractInstance<B, M>> = None;

		let (diamond_contract, facets) = match diamond_name {
			IbcCoreDiamond => (&self.ibc_core_diamond, self.ibc_core_facets.as_slice()),
			IbcTransferDiamond =>
				(self.ibc_transfer_diamond.as_ref().unwrap(), self.ibc_transfer_facets.as_slice()),
			BankDiamond => (self.bank_diamond.as_ref().unwrap(), self.bank_facets.as_slice()),
			TendermintDiamond =>
				(self.tendermint_diamond.as_ref().unwrap(), self.tendermint_facets.as_slice()),
			_ => panic!("wrong diamond name"),
		};

		let lookup_contracts = facets.iter().map(|x| x.contract()).chain(once(diamond_contract));

		for lookup_contract in lookup_contracts {
			if lookup_contract.abi().function(name).is_ok() {
				if contract.is_some() {
					panic!("ambiguous method name: {}", name);
				}
				contract = Some(lookup_contract);
			}
		}
		let contract = contract.take().ok_or_else(|| AbiError::WrongSelector)?;

		let mut f = contract.method(name, args);

		if let Ok(f) = &mut f {
			f.tx.set_to(diamond_contract.address());
		}

		f
	}

	pub fn event_for_name<D: EthEvent>(&self, name: &str) -> Result<Event<B, M, D>, AbiError> {
		let mut contract: Option<&ContractInstance<B, M>> = None;
		let lookup_contracts = self
			.ibc_core_facets
			.iter()
			.map(|x| x.contract())
			.chain(once(&self.ibc_core_diamond));

		for lookup_contract in lookup_contracts {
			if lookup_contract.abi().event(name).is_ok() {
				if contract.is_some() {
					panic!("ambiguous event name: {}", name);
				}
				contract = Some(lookup_contract);
			}
		}

		let contract = contract.take().ok_or_else(|| AbiError::WrongSelector)?;
		let mut event = contract.abi().event(name).expect("we've just found the event");
		let filter = contract
			.event_with_filter(Filter::new().event(&event.abi_signature()))
			.address(self.ibc_core_diamond.address().into());

		Ok(filter)
	}

	pub async fn register_client(&self, kind: &str, address: Address) {
		let method = self
			.method::<_, ()>(
				"registerClient",
				(Token::String(kind.into()), Token::Address(address)),
			)
			.unwrap();

		let _ = method.call().await.unwrap_contract_error();

		let receipt = method.send().await.unwrap().await.unwrap().unwrap();
		handle_gas_usage(&receipt);

		assert_eq!(receipt.status, Some(1.into()));
	}

	// pub fn find_storage(&self, name: &str) -> &Storage {
	// 	self.storage_layout.storage.iter().find(|x| x.contract == name).unwrap()
	// }
}

impl<B: Clone, M: Clone> Clone for DeployYuiIbc<B, M>
where
	B: Clone + Borrow<M>,
{
	fn clone(&self) -> Self {
		Self {
			ibc_core_facets: self.ibc_core_facets.clone(),
			ibc_core_diamond: self.ibc_core_diamond.clone(),
			// storage_layout: self.storage_layout.clone(),
			tendermint_diamond: self.tendermint_diamond.clone(),
			tendermint_facets: self.tendermint_facets.clone(),
			gov_proxy: self.gov_proxy.clone(),
			bank_diamond: self.bank_diamond.clone(),
			ibc_transfer_diamond: self.ibc_transfer_diamond.clone(),
			contract_creation_block: self.contract_creation_block.clone(),
			ibc_transfer_facets: self.ibc_transfer_facets.clone(),
			bank_facets: self.bank_facets.clone(),
		}
	}
}

pub async fn deploy_contract<M, T>(
	name: &str,
	artifacts: &[&ProjectCompileOutput],
	constructor_args: T,
	client: Arc<M>,
) -> ContractInstance<Arc<M>, M>
where
	M: Middleware,
	T: Tokenize + std::fmt::Debug + Clone,
{
	info!("Deploying contract {} with args {:?}", name, constructor_args);
	let contract = artifacts.into_iter().filter_map(|x| x.find_first(name)).next().unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let mut factory = ContractFactory::new(
		abi.unwrap(),
		bytecode.expect(&format!("{name} bytecode not found")),
		client.clone(),
	);
	let deployer = factory.deploy(constructor_args).unwrap();
	let gas = client.estimate_gas(&deployer.tx, None).await.unwrap();
	let (contract, receipt) = deploy_retrying(deployer).await.unwrap();
	info!("Deployed contract {} ({:?}), estimated gas price: {}", name, contract.address(), gas);
	handle_gas_usage(&receipt);
	contract
}

#[track_caller]
pub fn compile_solc(project_paths: ProjectPathsConfig) -> ProjectCompileOutput {
	// custom solc config to solve Yul-relatated compilation errors
	let mut selection = OutputSelection::default_output_selection();
	// selection
	// 	.0
	// 	.get_mut("*")
	// 	.unwrap()
	// 	.get_mut("*")
	// 	.unwrap()
	// 	.push("storageLayout".to_string());
	let solc_config = SolcConfig {
		settings: Settings {
			stop_after: None,
			remappings: vec![],
			optimizer: Optimizer {
				enabled: Some(cfg!(feature = "mainnet")),
				runs: Some(256),
				details: Some(OptimizerDetails {
					peephole: Some(true),
					inliner: Some(true),
					jumpdest_remover: Some(true),
					order_literals: Some(true),
					deduplicate: Some(true),
					cse: Some(true),
					constant_optimizer: Some(true),
					yul: Some(false),
					yul_details: None,
				}),
			},
			model_checker: None,
			metadata: None,
			output_selection: selection,
			evm_version: Some(EvmVersion::Paris),
			via_ir: Some(true), // TODO: this flag is unstable
			// debug: Some(DebuggingSettings {
			// 	revert_strings: Some(RevertStrings::Debug),
			// 	debug_info: vec!["location".to_string()],
			// }),
			debug: None,
			libraries: Libraries { libs: Default::default() },
		},
	};

	let mut project = Project::builder()
		.paths(project_paths)
		.ephemeral()
		.no_artifacts()
		.solc_config(solc_config)
		.build()
		.expect("project build failed");
	// TODO: figure out how to enable it in the config
	// project.artifacts.additional_values.storage_layout = true;
	// project.artifacts.additional_files.abi = true;
	// project.solc.args.push("--storage-layout".to_string());

	let project_output = project.compile().expect("compilation failed");

	if project_output.has_compiler_errors() {
		for err in project_output.output().errors {
			eprintln!("error: {}", err);
		}
		panic!("compiler errors");
	}

	return project_output
}

/// Uses solc to compile the yui-ibc-solidity contracts.
///
/// first argument is the path to the yui-ibc-solidity repo.
/// the second argument is the path to the solidity sources, relative to the first argument.
///
/// so if you have the yui-ibc-solidity as the path to yui then sources should be "contracts/core"
/// for IBCHandler or "contracts/clients" for the clients.
#[track_caller]
pub fn compile_yui(path_to_yui: &Path, sources: &str) -> ProjectCompileOutput {
	assert!(
		path_to_yui.exists(),
		"path to yui-ibc-solidity does not exist: {}",
		path_to_yui.display()
	);

	let project_paths = ProjectPathsConfig::builder()
		.root(&path_to_yui)
		.sources(path_to_yui.join(sources))
		.build()
		.unwrap();

	compile_solc(project_paths)
}

fn get_selectors<M>(contract: &ContractInstance<Arc<M>, M>) -> Vec<(String, [u8; 4])>
where
	M: Middleware,
{
	let signatures = contract.abi().functions.keys().cloned().collect::<Vec<_>>();
	signatures
		.into_iter()
		.filter(|val| val != "init(bytes)")
		.map(|val| (val.clone(), contract.abi().function(&val).unwrap().short_signature()))
		.collect()
}

pub fn check_code_size<'a>(
	artifacts: impl Iterator<Item = (String, &'a ConfigurableContractArtifact)>,
) {
	let ignore_list = ["Verifier"].into_iter().collect::<HashSet<_>>();
	artifacts
		.filter_map(|(name, artifact)| {
			Some((name, artifact.bytecode.as_ref()?.object.as_bytes()?.len()))
		})
		.filter(|(name, _)| {
			let ignored = ignore_list.contains(name.as_str());
			if ignored {
				log::warn!("{} size is ignored", name);
			}
			!ignored
		})
		.for_each(|(name, size)| {
			let max = 24 * 1024;
			if size > max {
				panic!("{} size is too big: {}/{}", name, size, max);
			}
			log::info!("{} size: {}/{}", name, size, max);
		});
}

pub async fn deploy_yui_ibc<M>(
	project_output: &ProjectCompileOutput,
	diamond_project_output: &ProjectCompileOutput,
	client: Arc<M>,
) -> DeployYuiIbc<Arc<M>, M>
where
	M: Middleware,
{
	let facet_names = vec![
		DiamondCutFacet,
		DiamondLoupeFacet,
		OwnershipFacet,
		CallBatchFacet,
		IBCClient,
		IBCConnection,
		IBCChannelHandshake,
		IBCPacket,
		IBCQuerier,
		RelayerWhitelistFacet,
	];

	let (deployed_facets, diamond) = deploy_diamond_with_facets(
		client,
		project_output,
		diamond_project_output,
		facet_names,
		None,
	)
	.await;

	println!("Deployed IBC Core Diamond on {:?}", diamond.address());

	// let predefined_layout = serde_json::from_reader::<_, StorageLayout>(
	// 	File::open("ethereum/src/storage_layout/
	// ibc_storage.json").unwrap(), )
	// .expect("failed to read predefined storage layout");
	//
	// let _storage_layout = project_output
	// 	.compiled_artifacts()
	// 	.iter()
	// 	.chain(diamond_project_output.compiled_artifacts())
	// 	.flat_map(|(_, artifact)| artifact.into_iter().flat_map(|(an, artifact)| artifact))
	// 	.filter_map(|ar| ar.artifact.storage_layout.clone())
	// 	.chain(once(predefined_layout))
	// 	.fold(StorageLayout { storage: vec![], types: Default::default() }, |mut acc, layout| {
	// 		acc.storage.extend(layout.storage);
	//
	// 		let len0 = acc.types.len();
	// 		let len1 = layout.types.len();
	// 		acc.types.extend(layout.types);
	// 		assert_eq!(acc.types.len(), len0 + len1, "duplicated type");
	// 		acc
	// 	});

	DeployYuiIbc::<Arc<M>, M>::new(
		diamond,
		deployed_facets,
		None,
		vec![],
		None,
		vec![],
		None,
		vec![],
		None,
	)
	.await
	.unwrap()
}

pub async fn cut_diamond<M: Middleware>(
	client: Arc<M>,
	yui: &DeployYuiIbc<Arc<M>, M>,
	project_output: &ProjectCompileOutput,
	facet_names: Vec<ContractName>,
	action: FacetCutAction,
	diamonds: Vec<ContractName>,
) -> Vec<NamedContract<Arc<M>, M>> {
	check_code_size(project_output.artifacts());

	let mut sigs = HashMap::<[u8; 4], (ContractName, String)>::new();
	let mut facet_cuts = vec![];
	let mut deployed_facets = vec![];
	for facet_name in facet_names {
		let facet_name_str = facet_name.to_string();

		let facet = deploy_contract(&facet_name_str, &[project_output], (), client.clone()).await;
		let facet_address = facet.address();
		println!("Deployed {facet_name} on {facet_address:?}");
		let selectors = get_selectors(&facet);
		deployed_facets.push(NamedContract::new(facet, facet_name));

		for (name, selector) in &selectors {
			if sigs.contains_key(selector) {
				let (contract_name, fn_name) = &sigs[selector];
				panic!(
					"duplicate selector: {}:{} and {}:{}",
					contract_name, fn_name, facet_name_str, name
				);
			}
			sigs.insert(*selector, (facet_name, name.clone()));
		}

		let facet_cut = FacetCut { address: facet_address, action, selectors };
		facet_cuts.push(facet_cut);
	}

	let cuts = Token::Array(facet_cuts.clone().into_iter().map(|x| x.into_token()).collect());
	for diamond in diamonds {
		let f = yui
			.method_diamond::<_, ()>(
				"diamondCut",
				(cuts.clone(), Address::zero(), Token::Bytes(vec![].into())),
				diamond,
			)
			.unwrap();
		send_retrying(&f).await.unwrap();
	}
	deployed_facets
}

pub async fn deploy_diamond_with_facets<M>(
	client: Arc<M>,
	project_output: &ProjectCompileOutput,
	diamond_project_output: &ProjectCompileOutput,
	facet_names: Vec<ContractName>,
	maybe_yui: Option<&DeployYuiIbc<Arc<M>, M>>,
) -> (Vec<NamedContract<Arc<M>, M>>, ContractInstance<Arc<M>, M>)
where
	M: Middleware,
{
	check_code_size(project_output.artifacts());
	check_code_size(diamond_project_output.artifacts());

	let acc = client.default_sender().unwrap();

	println!("Sender account: {acc:?}");

	let diamond_init =
		deploy_contract("DiamondInit", &[&diamond_project_output], (), client.clone()).await;
	println!("Diamond init address: {:?}", diamond_init.address());

	let mut sigs = HashMap::<[u8; 4], (ContractName, String)>::new();
	let mut facet_cuts = vec![];
	let mut deployed_facets = vec![];
	for facet_name in facet_names {
		let facet_name_str = facet_name.to_string();

		let facet = if let Some(facet) =
			maybe_yui.and_then(|yui| yui.ibc_core_facets.iter().find(|x| x.abi_name == facet_name))
		{
			facet.contract().clone()
		} else {
			deploy_contract(
				&facet_name_str,
				&[&project_output, diamond_project_output],
				(),
				client.clone(),
			)
			.await
		};
		let facet_address = facet.address();
		println!("Deployed {facet_name} on {facet_address:?}");
		let selectors = get_selectors(&facet);
		deployed_facets.push(NamedContract::new(facet, facet_name));

		for (name, selector) in &selectors {
			if sigs.contains_key(selector) {
				let (contract_name, fn_name) = &sigs[selector];
				panic!(
					"duplicate selector: {}:{} and {}:{}",
					contract_name, fn_name, facet_name_str, name
				);
			}
			sigs.insert(*selector, (facet_name, name.clone()));
		}

		let facet_cut = FacetCut { address: facet_address, action: FacetCutAction::Add, selectors };
		facet_cuts.push(facet_cut);
	}
	let init_calldata = diamond_init.method::<_, ()>("init", ()).unwrap().calldata().unwrap();

	let diamond = deploy_contract(
		"Diamond",
		&[&diamond_project_output],
		Token::Tuple(vec![
			Token::Array(facet_cuts.clone().into_iter().map(|x| x.into_token()).collect()),
			Token::Tuple(vec![
				Token::Address(acc),
				Token::Address(diamond_init.address()),
				Token::Bytes(init_calldata.0.into()),
			]),
		]),
		client.clone(),
	)
	.await;
	(deployed_facets, diamond)
}

pub async fn deploy_client<M: Middleware>(
	yui_solidity_path: &PathBuf,
	mut yui_ibc: DeployYuiIbc<Arc<M>, M>,
	client_type: ClientType,
	client_name: &str,
	client: Arc<M>,
) -> Result<(ContractInstance<Arc<M>, M>, Vec<NamedContract<Arc<M>, M>>), ClientError> {
	let project_output1 = compile_yui(yui_solidity_path, "contracts/clients");
	let diamond_project_output = compile_yui(&yui_solidity_path, "contracts/diamond");

	let ics23_contract =
		deploy_contract("Ics23Contract", &[&project_output1], (), client.clone()).await;

	let tendermint_zk_contract =
		deploy_contract("TendermintZKContract", &[&project_output1], (), client.clone()).await;

	let plonk_verifier_contract =
		deploy_contract("PlonkVerifier", &[&project_output1], (), client.clone()).await;

	let contract_name = ContractName::from_str(client_name).map_err(|_| {
		ClientError::Other(format!("invalid contract name: {}", client_name.to_string()))
	})?;
	let facet_names = vec![
		DiamondCutFacet,
		DiamondLoupeFacet,
		OwnershipFacet,
		contract_name, // TendermintLightClientZK
	];

	let (deployed_facets, diamond) = deploy_diamond_with_facets(
		client,
		&project_output1,
		&diamond_project_output,
		facet_names,
		Some(&yui_ibc),
	)
	.await;

	println!("Deployed {client_name} light-client Diamond on {:?}", diamond.address());

	yui_ibc.tendermint_diamond = Some(diamond.clone());
	yui_ibc.tendermint_facets = deployed_facets.clone();
	let method = yui_ibc.method_diamond::<_, ()>(
		"init",
		(yui_ibc.ibc_core_diamond.address(), ics23_contract.address(), tendermint_zk_contract.address(), plonk_verifier_contract.address()),
		TendermintDiamond,
	)?;
	send_retrying(&method).await.unwrap();

	let _ = yui_ibc.register_client(&client_type, diamond.address()).await;

	Ok((diamond, deployed_facets))
}

pub async fn deploy_governance<M: Middleware>(
	yui_solidity_path: &PathBuf,
	client: Arc<M>,
	mut yui_ibc: DeployYuiIbc<Arc<M>, M>,
	additional_signatories: &[Address],
) -> Result<DeployYuiIbc<Arc<M>, M>, ClientError> {
	let utils_output = compile_yui(&yui_solidity_path, "contracts/utils");
	let facets_output = compile_yui(&yui_solidity_path, "contracts/diamond/facets");
	let gov_proxy = deploy_contract(
		"GovernanceProxy",
		&[&utils_output],
		vec![
			yui_ibc.ibc_core_diamond.address(),
			yui_ibc.bank_diamond.as_ref().unwrap().address(),
			yui_ibc.ibc_transfer_diamond.as_ref().unwrap().address(),
			yui_ibc.tendermint_diamond.as_ref().unwrap().address(),
		],
		client.clone(),
	)
	.await;

	yui_ibc.gov_proxy = Some(gov_proxy.clone());
	let diamonds = [IbcTransferDiamond, IbcCoreDiamond, BankDiamond, TendermintDiamond];
	let facets = cut_diamond(
		client.clone(),
		&yui_ibc,
		&facets_output,
		vec![GovernanceFacet],
		FacetCutAction::Add,
		diamonds.to_vec(),
	)
	.await;
	yui_ibc.ibc_core_facets.extend(facets.clone());
	yui_ibc.bank_facets.extend(facets.clone());
	yui_ibc.ibc_transfer_facets.extend(facets.clone());
	yui_ibc.tendermint_facets.extend(facets.clone());
	for name in diamonds {
		yui_ibc
			.set_gov_tendermint_client(yui_ibc.tendermint_diamond.as_ref().unwrap().address(), name)
			.await;
		yui_ibc.set_gov_proxy(gov_proxy.address(), name).await;
		for signatory in additional_signatories {
			yui_ibc.add_gov_signatory(*signatory, name).await;
		}
		yui_ibc
			.transfer_ownership(yui_ibc.gov_proxy.as_ref().unwrap().address(), name)
			.await;
	}

	Ok(yui_ibc)
}

pub async fn deploy_ibc<M: Middleware>(
	yui_solidity_path: &PathBuf,
	client: Arc<M>,
) -> Result<DeployYuiIbc<Arc<M>, M>, ClientError> {
	let project_output = compile_yui(&yui_solidity_path, "contracts/core");
	let diamond_project_output = compile_yui(&yui_solidity_path, "contracts/diamond");
	let yui_ibc = deploy_yui_ibc(&project_output, &diamond_project_output, client.clone()).await;

	Ok(yui_ibc)
}

pub async fn deploy_transfer_module<M: Middleware, S: Signer>(
	yui_solidity_path: &PathBuf,
	mut yui_ibc: DeployYuiIbc<Arc<SignerMiddleware<M, S>>, SignerMiddleware<M, S>>,
	ibc_core_diamond_address: Address,
	client: Arc<SignerMiddleware<M, S>>,
	min_timeout_timestamp: u64,
) -> Result<
	(
		ContractInstance<Arc<SignerMiddleware<M, S>>, SignerMiddleware<M, S>>,
		Vec<NamedContract<Arc<SignerMiddleware<M, S>>, SignerMiddleware<M, S>>>,
		ContractInstance<Arc<SignerMiddleware<M, S>>, SignerMiddleware<M, S>>,
		Vec<NamedContract<Arc<SignerMiddleware<M, S>>, SignerMiddleware<M, S>>>,
	),
	ClientError,
> {
	let project_output = compile_yui(&yui_solidity_path, "contracts/apps/20-transfer");
	let diamond_project_output = compile_yui(&yui_solidity_path, "contracts/diamond");

	let facet_names = vec![ICS20Bank, DiamondCutFacet, DiamondLoupeFacet, OwnershipFacet];
	let (bank_facets, bank_diamond) = deploy_diamond_with_facets(
		client.clone(),
		&project_output,
		&diamond_project_output,
		facet_names,
		Some(&yui_ibc),
	)
	.await;

	info!("Deployed Bank Diamond address: {:?}", bank_diamond.address());

	let facet_names = vec![ICS20TransferBank, DiamondCutFacet, DiamondLoupeFacet, OwnershipFacet];
	let (ibc_transfer_facets, ibc_transfer_diamond) = deploy_diamond_with_facets(
		client.clone(),
		&project_output,
		&diamond_project_output,
		facet_names,
		Some(&yui_ibc),
	)
	.await;

	info!("Deployed ICS-20 Transfer Diamond address: {:?}", ibc_transfer_diamond.address());

	yui_ibc.bank_diamond = Some(bank_diamond.clone());
	yui_ibc.bank_facets = bank_facets.clone();
	let method =
		yui_ibc.method_diamond::<_, ()>("init", Token::String("ETH".into()), BankDiamond)?;
	send_retrying(&method).await.unwrap();

	let method = yui_ibc
		.method_diamond::<_, ()>(
			"transferRole",
			(ethers::utils::keccak256("OWNER_ROLE"), ibc_transfer_diamond.address()),
			BankDiamond,
		)
		.unwrap();
	send_retrying(&method).await.unwrap();

	yui_ibc.ibc_transfer_diamond = Some(ibc_transfer_diamond.clone());
	yui_ibc.ibc_transfer_facets = ibc_transfer_facets.clone();
	let method = yui_ibc.method_diamond::<_, ()>(
		"init",
		(
			Token::Address(ibc_core_diamond_address),
			Token::Address(bank_diamond.address()),
			min_timeout_timestamp,
		),
		IbcTransferDiamond,
	)?;
	send_retrying(&method).await.unwrap();

	yui_ibc.add_relayer(client.address()).await;
	yui_ibc.bind_port("transfer", ibc_transfer_diamond.address()).await;

	Ok((ibc_transfer_diamond, ibc_transfer_facets, bank_diamond, bank_facets))
}

pub fn handle_gas_usage(receipt: &TransactionReceipt) {
	if let Some(gas) = receipt.effective_gas_price {
		info!("GAS: {gas}");
	} else {
		info!("GAS: {} (2)", receipt.gas_used.unwrap());
	}
}

pub fn create_intervals(start: u64, end: u64) -> Vec<(u64, u64)> {
	let mut intervals = Vec::new();
	let mut current_start = start;

	while current_start <= end {
		let current_end = std::cmp::min(current_start + SEQUENCES_PER_ITER - 1, end);
		intervals.push((current_start, current_end));
		current_start = current_end + 1;
	}

	intervals
}

pub struct Header {
	pub parent_hash: H256,
	pub ommers_hash: H256,
	pub beneficiary: Address,
	pub state_root: H256,
	pub transactions_root: H256,
	pub receipts_root: H256,
	pub logs_bloom: Bloom,
	pub difficulty: U256,
	pub number: U256,
	pub gas_limit: U256,
	pub gas_used: U256,
	pub timestamp: u64,
	pub extra_data: Bytes,
	pub mix_hash: H256,
	pub nonce: H64,
	/// BaseFee was added by EIP-1559 and is ignored in legacy headers.
	pub base_fee_per_gas: Option<U256>,
	/// Ignored in legacy headers
	pub withdrawals_root: Option<H256>,
}

impl rlp::Encodable for Header {
	fn rlp_append(&self, s: &mut RlpStream) {
		let mut list_len = 15;
		if self.base_fee_per_gas.is_some() {
			list_len += 1;
		}
		if self.withdrawals_root.is_some() {
			list_len += 1;
		}
		s.begin_list(list_len);
		s.append(&self.parent_hash);
		s.append(&self.ommers_hash);
		s.append(&self.beneficiary);
		s.append(&self.state_root);
		s.append(&self.transactions_root);
		s.append(&self.receipts_root);
		s.append(&self.logs_bloom);
		s.append(&self.difficulty);
		s.append(&self.number);
		s.append(&self.gas_limit);
		s.append(&self.gas_used);
		s.append(&self.timestamp);
		s.append(&self.extra_data.as_ref());
		s.append(&self.mix_hash);
		s.append(&self.nonce);
		if let Some(ref base_fee) = self.base_fee_per_gas {
			s.append(base_fee);
		}
		if let Some(ref root) = self.withdrawals_root {
			s.append(root);
		}
	}
}

impl<T> From<Block<T>> for Header {
	fn from(value: Block<T>) -> Self {
		Header {
			parent_hash: value.parent_hash,
			ommers_hash: value.uncles_hash,
			beneficiary: value.author.expect("author not found"),
			state_root: value.state_root,
			transactions_root: value.transactions_root,
			receipts_root: value.receipts_root,
			logs_bloom: value.logs_bloom.unwrap_or_default(),
			difficulty: value.difficulty,
			number: U256::from(value.number.expect("block number should exist").as_u64()),
			gas_limit: value.gas_limit,
			gas_used: value.gas_used,
			timestamp: value.timestamp.as_u64(),
			extra_data: value.extra_data,
			mix_hash: value.mix_hash.expect("mix hash not found"),
			nonce: value.nonce.expect("nonce not found"),
			base_fee_per_gas: value.base_fee_per_gas,
			withdrawals_root: value.withdrawals_root,
		}
	}
}

#[test]
fn test_block_header_rlp_encoding() {
	let block = Block::<()> {
		hash: None,
		parent_hash: H256(hex!("1e77d8f1267348b516ebc4f4da1e2aa59f85f0cbd853949500ffac8bfc38ba14")),
		uncles_hash: H256(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347")),
		author: Some(H160(hex!("2a65Aca4D5fC5B5C859090a6c34d164135398226"))),
		state_root: H256(hex!("0b5e4386680f43c224c5c037efc0b645c8e1c3f6b30da0eec07272b4e6f8cd89")),
		transactions_root: H256(hex!(
			"56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
		)),
		receipts_root: H256(hex!(
			"56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
		)),
		logs_bloom: Some(Bloom::from_slice(&[0u8; 256])),
		difficulty: U256::from(6022643743806u64),
		total_difficulty: None,
		seal_fields: vec![],
		uncles: vec![],
		transactions: vec![],
		number: Some(U64::from(400000u32)),
		gas_limit: U256::from(3141592u64),
		gas_used: U256::from(0u64),
		timestamp: U256::from(1445130204u64),
		extra_data: Bytes::from(hex!("d583010202844765746885676f312e35856c696e7578").to_vec()),
		mix_hash: Some(H256(hex!(
			"3fbea7af642a4e20cd93a945a1f5e23bd72fc5261153e09102cf718980aeff38"
		))),
		nonce: Some(H64(hex!("6af23caae95692ef"))),
		base_fee_per_gas: None,
		withdrawals_root: None,
		withdrawals: None,
		size: None,
		other: Default::default(),
	};
	let header: Header = block.clone().into();
	let rlp_encoded_header = rlp::encode(&header).to_vec();
	let hash = keccak256(rlp_encoded_header);
	assert_eq!(
		H256(hash),
		H256(hex!("5d15649e25d8f3e2c0374946078539d200710afc977cdfc6a977bd23f20fa8e8"))
	);

	let block = serde_json::from_str::<Block<()>>(r#"
	{
	  "baseFeePerGas": "0x7",
	  "difficulty": "0x0",
	  "extraData": "0xd883010d01846765746888676f312e32312e31856c696e7578",
	  "gasLimit": "0x1c9c380",
	  "gasUsed": "0x570c1",
	  "hash": "0x89f2e55516b1ec33275f67fb08864b269f29d682c4294692bd7885ff98022376",
	  "logsBloom": "0x00000000000000002000000000000000000000000800000000800000000000000000000100000000000200000000000000000000000000000000000000000000000000080000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
	  "miner": "0x123463a4b065722e99115d6c222f267d9cabb524",
	  "mixHash": "0x9465af5f5db63c8abc700d61e60baae7f386479c78d8cfd1013ce98663aa2399",
	  "nonce": "0x0000000000000000",
	  "number": "0x3c28",
	  "parentHash": "0xdc31160f48f2a7338b2943077e639019ba7478f2ba00c96d59e9aa1f27e24cba",
	  "receiptsRoot": "0xd80423deccefededa1392413952297320135f4414ddca1850cfea1ae3527d3c1",
	  "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
	  "size": "0x53c",
	  "stateRoot": "0x51fa3910be4db6c196677c750ee9b3126e334f71e795ac17475a345da8b9ad6a",
	  "timestamp": "0x6537f3ce",
	  "totalDifficulty": "0x1",
	  "transactions": [],
	  "transactionsRoot": "0x931a3d5aca9f9ea1a61e9b6642f69f9943dcde5ebb92030edcd03a959d33e968",
	  "uncles": [],
	  "withdrawals": [],
	  "withdrawalsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
	}"#).unwrap();
	let header: Header = block.clone().into();
	let rlp_encoded_header = rlp::encode(&header).to_vec();
	let hash = keccak256(rlp_encoded_header);
	assert_eq!(H256(hash), block.hash.unwrap());
}

pub fn clear_proof_value(
	commitment_proof: &CommitmentProofBytes,
) -> Result<CommitmentProofBytes, ClientError> {
	let cs_proof = commitment_proof.as_bytes();
	use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
	let mut merkle_proof: MerkleProof<HostFunctionsManager> =
		RawMerkleProof::try_from(CommitmentProofBytes::try_from(cs_proof.to_vec()).unwrap())
			.unwrap()
			.into();
	for (i, proof) in merkle_proof.proofs.iter_mut().enumerate() {
		if let Some(proof) = proof.proof.as_mut() {
			match proof {
				Proof::Exist(p) =>
					if i == 0 {
						p.value.clear();
					},
				p => return Err(ClientError::Other(format!("unexpected proof type: {:?}", p))),
			}
		}
	}
	let new_raw_proof = RawMerkleProof::from(merkle_proof.clone());
	Ok(CommitmentProofBytes::try_from(new_raw_proof).unwrap())
}

pub async fn send_retrying<B, M, D>(
	method: &FunctionCall<B, M, D>,
) -> Result<TransactionReceipt, ContractError<M>>
where
	B: Clone + Borrow<M>,
	M: Middleware,
	D: Detokenize,
{
	loop {
		let _ = method.call().await.unwrap_contract_error();
		let result = method.send().await;
		match result {
			Ok(v) => {
				let receipt = v.await.unwrap().unwrap();
				handle_gas_usage(&receipt);
				assert_eq!(receipt.status, Some(1.into()));
				return Ok(receipt);
			},
			Err(e) =>
				if e.to_string().contains("replacement transaction underpriced") {
					sleep(Duration::from_secs(1)).await;
					continue;
				} else {
					return Err(e);
				},
		}
	}
}

pub async fn deploy_retrying<B, M>(
	deployer: Deployer<B, M>,
) -> Result<(ContractInstance<B, M>, TransactionReceipt), ContractError<M>>
where
	B: Clone + Borrow<M>,
	M: Middleware,
{
	loop {
		let result = deployer.clone().send_with_receipt().await;
		match result {
			Ok((c, receipt)) => {
				handle_gas_usage(&receipt);
				assert_eq!(receipt.status, Some(1.into()));
				return Ok((c, receipt));
			},
			Err(e) =>
				if e.to_string().contains("replacement transaction underpriced") {
					sleep(Duration::from_secs(1)).await;
					continue;
				} else {
					return Err(e);
				},
		}
	}
}
