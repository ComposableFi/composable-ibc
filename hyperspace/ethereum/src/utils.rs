use crate::contract::UnwrapContractError;
use ethers::{
	abi::{AbiError, Address, Detokenize, EventExt, Function, FunctionExt, Token, Tokenize},
	contract::{ContractInstance, FunctionCall},
	core::types::Bytes,
	prelude::{
		EthEvent, Event, Filter, Http, LocalWallet, Middleware, Provider, TransactionReceipt, H256,
	},
};
use ethers_solc::artifacts::{Storage, StorageLayout};
use ibc::core::ics04_channel::packet::Packet;
use std::iter;

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
pub struct DeployYuiIbc<B, M> {
	// pub ibc_client: ContractInstance<B, M>,
	// pub ibc_connection: ContractInstance<B, M>,
	// pub ibc_channel_handshake: ContractInstance<B, M>,
	// pub ibc_packet: ContractInstance<B, M>,
	// pub ibc_handler: ContractInstance<B, M>,
	pub facet_cuts: Vec<FacetCut>,
	pub deployed_facets: Vec<ContractInstance<B, M>>,
	pub diamond: ContractInstance<B, M>,
	pub storage_layout: StorageLayout,
	pub tendermint: ContractInstance<B, M>,
	pub bank: Option<ContractInstance<B, M>>,
}

impl<B, M> DeployYuiIbc<B, M>
where
	B: Clone + std::borrow::Borrow<M>,
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
		for faucet in self.deployed_facets.iter().chain(iter::once(&self.diamond)) {
			if let Ok(f) = faucet.abi().function(name) {
				log::info!(target: "hyperspace_ethereum", "found function: {name}, {}, {}, {}", f.signature(), f.abi_signature(), hex::encode(&f.short_signature()));
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

		let lookup_contracts = self.deployed_facets.iter().chain(std::iter::once(&self.diamond));

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
		let lookup_contracts = self.deployed_facets.iter().chain(std::iter::once(&self.diamond));

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

	pub fn find_storage(&self, name: &str) -> &Storage {
		self.storage_layout.storage.iter().find(|x| x.contract == name).unwrap()
	}
}

impl<B: Clone, M: Clone> Clone for DeployYuiIbc<B, M>
where
	B: Clone + std::borrow::Borrow<M>,
{
	fn clone(&self) -> Self {
		Self {
			facet_cuts: self.facet_cuts.clone(),
			deployed_facets: self.deployed_facets.clone(),
			diamond: self.diamond.clone(),
			storage_layout: self.storage_layout.clone(),
			tendermint: self.tendermint.clone(),
			bank: self.bank.clone(),
		}
	}
}
