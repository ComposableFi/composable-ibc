use crate::{
	client::ClientError,
	config::{ContractName, EthereumClientConfig},
	contract::UnwrapContractError,
	ibc_provider::{DIAMONDABI_ABI, ICS20TRANSFERBANKABI_ABI, TENDERMINTCLIENTABI_ABI},
};
use ethers::{
	abi::{AbiError, Address, Detokenize, EventExt, Function, Token, Tokenize},
	contract::{ContractFactory, ContractInstance, FunctionCall},
	core::types::Bytes,
	prelude::{
		EthEvent, Event, Filter, Http, LocalWallet, Middleware, Provider, TransactionReceipt, H256,
	},
};
use ethers_solc::{
	artifacts::{
		output_selection::OutputSelection, Libraries, Optimizer, OptimizerDetails, Settings,
		StorageLayout,
	},
	Artifact, ConfigurableContractArtifact, EvmVersion, Project, ProjectCompileOutput,
	ProjectPathsConfig, SolcConfig,
};
use ibc::core::{ics02_client::client_state::ClientType, ics04_channel::packet::Packet};
use std::{
	borrow::Borrow,
	collections::HashMap,
	fs::File,
	iter::once,
	path::{Path, PathBuf},
	sync::Arc,
};

pub type ProviderImpl = ethers::prelude::SignerMiddleware<Provider<Http>, LocalWallet>;

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
			Token::Uint((FacetCutAction::Add as u32).into()),
			Token::Array(
				self.selectors.into_iter().map(|(_, x)| Token::FixedBytes(x.to_vec())).collect(),
			),
		])
	}
}

#[derive(Debug)]
pub struct Facet<B, M> {
	contract: ContractInstance<B, M>,
	abi_name: ContractName,
}

impl<B: Clone + Borrow<M>, M> Clone for Facet<B, M> {
	fn clone(&self) -> Self {
		Self { contract: self.contract.clone(), abi_name: self.abi_name }
	}
}

impl<B, M> Facet<B, M>
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
	pub deployed_facets: Vec<Facet<B, M>>,
	pub diamond: ContractInstance<B, M>,
	// pub storage_layout: StorageLayout,
	pub tendermint: Option<ContractInstance<B, M>>,
	pub bank: Option<ContractInstance<B, M>>,
}

impl<B, M> DeployYuiIbc<B, M>
where
	B: Borrow<M> + Clone,
	M: Middleware,
{
	pub fn from_addresses(
		client: B,
		diamond_address: Address,
		tendermint_address: Option<Address>,
		bank_address: Option<Address>,
		diamond_facets: Vec<(ContractName, Address)>,
	) -> Result<Self, ClientError> {
		Ok(Self {
			diamond: ContractInstance::<B, M>::new(
				diamond_address,
				DIAMONDABI_ABI.clone(),
				client.clone(),
			),
			tendermint: tendermint_address.map(|addr| {
				ContractInstance::<B, M>::new(addr, TENDERMINTCLIENTABI_ABI.clone(), client.clone())
			}),
			bank: bank_address.map(|addr| {
				ContractInstance::<B, M>::new(
					addr,
					ICS20TRANSFERBANKABI_ABI.clone(),
					client.clone(),
				)
			}),
			deployed_facets: diamond_facets
				.into_iter()
				.map(|(abi, addr)| Facet::from_address(addr, abi, client.clone()))
				.collect(),
		})
	}
}

impl<B, M> DeployYuiIbc<B, M>
where
	B: Clone + Borrow<M>,
	M: Middleware,
{
	pub async fn bind_port(&self, port_id: &str, address: Address) {
		let bind_port = self
			.method::<_, ()>("bindPort", (Token::String(port_id.into()), Token::Address(address)))
			.unwrap();
		let () = bind_port.call().await.unwrap_contract_error();
		let tx_recp = bind_port.send().await.unwrap_contract_error().await.unwrap().unwrap();
		assert_eq!(tx_recp.status, Some(1.into()));
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
		for faucet in self.deployed_facets.iter().map(|x| x.contract()).chain(once(&self.diamond)) {
			if let Ok(f) = faucet.abi().function(name) {
				if func.is_some() {
					log::error!(target: "hyperspace_ethereum", "ambiguous function name: {}", name);
					//panic!("ambiguous function name: {}", name);
					//d5a2448100000000000000000000000000000000000000000000000000000000
					//d5a2448100000000000000000000000000000000000000000000000000000000
					// d5a244810000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000f30372d74656e6465726d696e742d30000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
					// d5a244810000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000002c0000000000000000000000000000000000000000000000000000000000000000d30372d74656e6465726d696e74000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000fa000000000000000000000000000000000000000000000000000000000003d0900000000000000000000000000000000000000000000000000000000000001baf800000000000000000000000000000000000000000000000000006722feb7b0000000000000000000000000000000000000000000000000000000000000000000f000000000000000000000000000000000000000000000000000000037e11d60000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000cdc00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000001263656e74617572692d746573746e65742d310000000000000000000000000000000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000064f8fc0400000000000000000000000000000000000000000000000017826f89135e5218000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020008892c98d506a154ef7e5ad89b345e395cdafc8eb77857276f27ff5cef791da0000000000000000000000000000000000000000000000000000000000000020602fa35acfd377900d7fe3459730d96415eef369bd033c0923b2d2e2796a97d9
				}
				func = Some(f);
			}
		}
		func.ok_or_else(|| ethers::abi::Error::InvalidName(name.into()))
		// self.deployed_facets
		// 	.iter()
		// 	.find_map(|x| x.abi().function(name).ok())
		// 	.ok_or_else(|| ethers::abi::Error::InvalidName(name.into()))
	}

	pub fn method<T: Tokenize, D: Detokenize>(
		&self,
		name: &str,
		args: T,
	) -> Result<FunctionCall<B, M, D>, AbiError> {
		let mut contract: Option<&ContractInstance<B, M>> = None;

		let lookup_contracts =
			self.deployed_facets.iter().map(|x| x.contract()).chain(once(&self.diamond));

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
			f.tx.set_to(self.diamond.address());
		}

		f
	}

	pub fn event_for_name<D: EthEvent>(&self, name: &str) -> Result<Event<B, M, D>, AbiError> {
		let mut contract: Option<&ContractInstance<B, M>> = None;
		let lookup_contracts =
			self.deployed_facets.iter().map(|x| x.contract()).chain(once(&self.diamond));

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
			.address(self.diamond.address().into());

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
		assert_eq!(receipt.status, Some(1.into()));
	}

	// pub fn find_storage(&self, name: &str) -> &Storage {
	// 	self.storage_layout.storage.iter().find(|x| x.contract == name).unwrap()
	// }
}

impl<B: Clone, M: Clone> Clone for DeployYuiIbc<B, M>
where
	B: Clone + std::borrow::Borrow<M>,
{
	fn clone(&self) -> Self {
		Self {
			deployed_facets: self.deployed_facets.clone(),
			diamond: self.diamond.clone(),
			// storage_layout: self.storage_layout.clone(),
			tendermint: self.tendermint.clone(),
			bank: self.bank.clone(),
		}
	}
}

pub async fn deploy_contract<M, T>(
	contract: &ConfigurableContractArtifact,
	constructor_args: T,
	client: Arc<M>,
) -> ContractInstance<Arc<M>, M>
where
	M: Middleware,
	T: Tokenize,
{
	let (abi, bytecode, _) = contract.clone().into_parts();
	let mut factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client);
	factory.deploy(constructor_args).unwrap().send().await.unwrap()
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
				enabled: Some(false),
				runs: Some(5),
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

pub async fn deploy_yui_ibc<M>(
	project_output: &ProjectCompileOutput,
	diamond_project_output: &ProjectCompileOutput,
	client: Arc<M>,
) -> DeployYuiIbc<Arc<M>, M>
where
	M: Middleware,
{
	use ContractName::*;
	let facet_names = [
		IBCClient,
		IBCConnection,
		IBCChannelHandshake,
		IBCPacket,
		IBCQuerier,
		DiamondCutFacet,
		DiamondLoupeFacet,
		OwnershipFacet,
	];

	project_output.artifacts().for_each(|(name, artifact)| {
		let size = artifact.bytecode.as_ref().unwrap().object.as_bytes().unwrap().len();
		let max = 24 * 1024;
		if size > max {
			panic!("{} size is too big: {}/{}", name, size, max);
		}
		log::info!("{} size: {}/{}", name, size, max);
	});
	diamond_project_output.artifacts().for_each(|(name, artifact)| {
		let size = artifact.bytecode.as_ref().unwrap().object.as_bytes().unwrap().len();
		let max = 24 * 1024;
		if size > max {
			panic!("{} size is too big: {}/{}", name, size, max);
		}
		log::info!("{} size: {}/{}", name, size, max);
	});

	let acc = client.default_sender().unwrap();

	println!("Sender account: {acc:?}");

	let contract = diamond_project_output.find_first("DiamondInit").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let diamond_init = factory.deploy(()).unwrap().send().await.unwrap();
	println!("Diamond init address: {:?}", diamond_init.address());

	let mut sigs = HashMap::<[u8; 4], (ContractName, String)>::new();
	let mut facet_cuts = vec![];
	let mut deployed_facets = vec![];
	for facet_name in facet_names {
		let facet_name_str = facet_name.to_string();
		dbg!(&facet_name_str);
		let contract = project_output
			.find_first(&facet_name_str)
			.or_else(|| diamond_project_output.find_first(&facet_name_str))
			.unwrap();
		let (abi, bytecode, _) = contract.clone().into_parts();
		let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
		let facet = factory.deploy(()).unwrap().send().await.unwrap();
		let facet_address = facet.address();
		println!("Deployed {facet_name} on {facet_address:?}");
		let selectors = get_selectors(&facet);
		deployed_facets.push(Facet::new(facet, facet_name));

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

	let contract = diamond_project_output.find_first("Diamond").unwrap();
	let (abi, bytecode, _) = contract.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let diamond = factory
		.deploy(Token::Tuple(vec![
			Token::Array(facet_cuts.clone().into_iter().map(|x| x.into_token()).collect()),
			Token::Tuple(vec![
				Token::Address(acc),
				Token::Address(diamond_init.address()),
				Token::Bytes(init_calldata.0.into()),
			]),
		]))
		.unwrap()
		.send()
		.await
		.unwrap();

	let predefined_layout = serde_json::from_reader::<_, StorageLayout>(
		File::open("/Users/vmark/work/centauri-private/hyperspace/ethereum/src/storage_layout/ibc_storage.json").unwrap(),
	)
	.expect("failed to read predefined storage layout");

	let _storage_layout = project_output
		.compiled_artifacts()
		.iter()
		.chain(diamond_project_output.compiled_artifacts())
		.flat_map(|(_, artifact)| {
			artifact.into_iter().flat_map(|(an, artifact)| {
				println!("artifact name {an}");
				artifact
			})
		})
		.filter_map(|ar| ar.artifact.storage_layout.clone())
		.chain(once(predefined_layout))
		.fold(StorageLayout { storage: vec![], types: Default::default() }, |mut acc, layout| {
			acc.storage.extend(layout.storage);

			let len0 = acc.types.len();
			let len1 = layout.types.len();
			acc.types.extend(layout.types);
			assert_eq!(acc.types.len(), len0 + len1, "duplicated type");
			acc
		});

	DeployYuiIbc {
		diamond,
		deployed_facets,
		// storage_layout,
		tendermint: None,
		bank: None,
	}
}

pub async fn deploy_client<M: Middleware>(
	yui_solidity_path: &PathBuf,
	yui_ibc: DeployYuiIbc<Arc<M>, M>,
	client_type: ClientType,
	delegate_update_name: &str,
	client_name: &str,
	client: Arc<M>,
) -> Result<ContractInstance<Arc<M>, M>, ClientError> {
	let project_output1 = compile_yui(yui_solidity_path, "contracts/clients");
	let upd = project_output1.find_first(delegate_update_name).unwrap();

	let (abi, bytecode, _) = upd.clone().into_parts();
	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
	let update_client_delegate_contract = factory.deploy(()).unwrap().send().await.unwrap();

	let contract = project_output1.find_first(client_name).unwrap();
	let r = contract.clone();
	let (abi, bytecode, _) = r.into_parts();

	let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client);
	let tendermint_light_client = factory
		.deploy((
			Token::Address(yui_ibc.diamond.address()),
			Token::Address(update_client_delegate_contract.address()),
		))
		.unwrap()
		.send()
		.await
		.unwrap();

	let _ = yui_ibc.register_client(&client_type, tendermint_light_client.address()).await;
	Ok(tendermint_light_client)
}

pub async fn deploy_ibc<M: Middleware>(
	yui_solidity_path: &PathBuf,
	client: Arc<M>,
) -> Result<DeployYuiIbc<Arc<M>, M>, ClientError> {
	let project_output = compile_yui(&yui_solidity_path, "contracts/core");
	let diamond_project_output = compile_yui(&yui_solidity_path, "contracts/diamond");
	let yui_ibc = deploy_yui_ibc(&project_output, &diamond_project_output, client).await;
	Ok(yui_ibc)
}

pub async fn deploy_module<M: Middleware>(
	yui_solidity_path: &PathBuf,
	diamond_address: Address,
	client: Arc<M>,
) -> Result<ContractInstance<Arc<M>, M>, ClientError> {
	let project_output = compile_yui(&yui_solidity_path, "contracts/apps/20-transfer");

	let artifact = project_output.find_first("ICS20Bank").expect("no ICS20Bank in project output");
	let bank_contract = deploy_contract::<M, _>(artifact, (), client.clone()).await;
	println!("Bank module address: {:?}", bank_contract.address());
	let artifact = project_output
		.find_first("ICS20TransferBank")
		.expect("no ICS20TransferBank in project output");
	let constructor_args =
		(Token::Address(diamond_address), Token::Address(bank_contract.address()));
	let module_contract = deploy_contract(artifact, constructor_args, client.clone()).await;
	Ok(module_contract)
}
