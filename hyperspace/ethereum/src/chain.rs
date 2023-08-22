use std::thread;
use std::{time::Duration, sync::Arc};

use ethers::abi::Token;
use ethers::prelude::EthAbiType;
use ethers::providers::Middleware;
use futures::{Stream, StreamExt};
use ibc::core::ics02_client::msgs::update_client::MsgUpdateAnyClient;
use ibc::core::ics02_client::{events::UpdateClient, msgs::create_client::MsgCreateAnyClient};
use ibc::Height;
use ibc::protobuf::Protobuf;
use ibc::protobuf::google::protobuf::Timestamp;
use ics07_tendermint::client_message::{ClientMessage, Header};
use ics07_tendermint::client_state::ClientState;
use ics07_tendermint::consensus_state::ConsensusState;
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState, AnyConsensusState};
use primitives::mock::LocalClientTypes;
use primitives::{Chain, CommonClientState, LightClientSync, MisbehaviourHandler};

use crate::contract::IbcHandler;
use crate::{client::EthereumClient, ibc_provider::BlockHeight};

#[async_trait::async_trait]
impl MisbehaviourHandler for EthereumClient {
	async fn check_for_misbehaviour<C>(
		&self,
		counterparty: &C,
		client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		// assume no misbehaviour occurs for now.
		Ok(())
	}
}

fn client_state_abi_token<H>(client: ClientState<H>) -> Token{
	use ethers::abi::encode as ethers_encode;
	use ethers::abi::Token as EthersToken;

	let client_state_data = EthersToken::Tuple(
		[
			//chain_id
			EthersToken::String(client.chain_id.as_str().to_string()),
			//trust_level
			EthersToken::Tuple(
				[
					//numerator
					EthersToken::Uint(client.trust_level.numerator().into()),
					//denominator
					EthersToken::Uint(client.trust_level.denominator().into()),
				].to_vec()),
			//trusting_period
			EthersToken::Tuple(
			[
				EthersToken::Int(client.trusting_period.as_secs().into()),
				EthersToken::Int(client.trusting_period.as_millis().into()),
			].to_vec()),
			//unbonding_period
			EthersToken::Tuple(
				[
					EthersToken::Int(client.unbonding_period.as_secs().into()),
					EthersToken::Int(client.unbonding_period.as_nanos().into()),
				].to_vec()),
			//max_clock_drift	
			EthersToken::Tuple(
				[
					EthersToken::Int(client.max_clock_drift.as_secs().into()),
					EthersToken::Int(client.max_clock_drift.as_nanos().into()),
				].to_vec()),
			//frozen_height
			EthersToken::Int(client.frozen_height.unwrap_or(Height::default()).revision_height.into()),
			//latest_height
			EthersToken::Int(client.latest_height.revision_height.into()),
			//allow_update_after_expiry
			EthersToken::Bool(true), //TODO check if this is correct
			//allow_update_after_misbehaviour
			EthersToken::Bool(true), //TODO check if this is correct
		].to_vec());
		client_state_data
}

fn consensus_state_abi_token(consensus_state : ConsensusState) -> Token{
	use ethers::abi::encode as ethers_encode;
	use ethers::abi::Token as EthersToken;
	let time = consensus_state.timestamp.duration_since(Timestamp {
		seconds: 0,
		nanos: 0,
	}.try_into().unwrap()).unwrap();
	let consensus_state_data = EthersToken::Tuple(
		[
			//timestamp	
			EthersToken::Tuple(
				[
					EthersToken::Int(time.as_secs().into()),
					EthersToken::Int(time.as_nanos().into()),
				].to_vec()),
			//root
			EthersToken::Tuple(
				[
					EthersToken::Bytes(consensus_state.root.bytes),
				].to_vec()),
			//next_validators_hash
			EthersToken::Bytes(consensus_state.next_validators_hash.as_ref().into()),
		].to_vec());
	consensus_state_data
}

fn tm_header_abi_token(header: Header) -> Token{
	use ethers::abi::encode as ethers_encode;
	use ethers::abi::Token as EthersToken;
	let block_header = header.signed_header.header;
	let last_commit = header.signed_header.commit;


	// block_header.time.;
	let time = block_header.time.duration_since(Timestamp {
		seconds: 0,
		nanos: 0,
	}.try_into().unwrap()).unwrap();

	let _signedheader_header = 
		EthersToken::Tuple([
			//version
			EthersToken::Tuple(
			[
				EthersToken::Uint(block_header.version.block.into()),
				EthersToken::Uint(block_header.version.app.into()),
			].to_vec()),
			//chain_id
			EthersToken::String(block_header.chain_id.into()),
			//height
			EthersToken::Int((block_header.height.value() as i64).into()),
			//time
			EthersToken::Tuple(
			[
				EthersToken::Int(time.as_secs().into()),
				EthersToken::Int(time.as_nanos().into()),
			].to_vec()),
			// //last_block_id
			EthersToken::Tuple(
			[
				EthersToken::Bytes(block_header.last_block_id.unwrap().hash.into()),
				//part_set_header
				EthersToken::Tuple(
					[
						EthersToken::Uint(block_header.last_block_id.unwrap().part_set_header.total.into()),
						EthersToken::Bytes(block_header.last_block_id.unwrap().part_set_header.hash.into()),
					].to_vec()),
			].to_vec()),
			//last_commit_hash
			EthersToken::Bytes(block_header.last_commit_hash.unwrap().into()),
			//data_hash
			EthersToken::Bytes(block_header.data_hash.unwrap().into()),
			//validators_hash
			EthersToken::Bytes(block_header.validators_hash.into()),
			//next_validators_hash
			EthersToken::Bytes(block_header.next_validators_hash.into()),
			//consensus_hash
			EthersToken::Bytes(block_header.consensus_hash.into()),
			//app_hash
			EthersToken::Bytes(block_header.app_hash.into()),
			//last_results_hash
			EthersToken::Bytes(block_header.last_results_hash.unwrap().into()),
			//evidence_hash
			EthersToken::Bytes(block_header.evidence_hash.unwrap().into()),
			//proposer_address
			EthersToken::Bytes(block_header.proposer_address.into()),
		].to_vec());
	

	let _signedheader_comit = 
		EthersToken::Tuple([
			//height
			EthersToken::Int((last_commit.height.value()).into()),
			//round
			EthersToken::Int((last_commit.round.value()).into()),
			//block_id
			EthersToken::Tuple(
			[
				EthersToken::Bytes(last_commit.block_id.hash.into()),
				//part_set_header
				EthersToken::Tuple(
					[
						//total
						EthersToken::Uint(last_commit.block_id.part_set_header.total.into()),
						//hash
						EthersToken::Bytes(last_commit.block_id.part_set_header.hash.into()),
					].to_vec()),
			].to_vec()),
		].to_vec());

	EthersToken::Tuple([
		EthersToken::Tuple([
			_signedheader_header,
			_signedheader_comit
		].to_vec()),
		EthersToken::Int(header.trusted_height.revision_height.into()),
	].to_vec())
	
}

#[async_trait::async_trait]
impl Chain for EthereumClient {
	#[inline]
	fn name(&self) -> &str {
		&self.config.name
	}

	#[inline]
	fn block_max_weight(&self) -> u64 {
		self.config.max_block_weight
	}

	async fn estimate_weight(
		&self,
		msg: Vec<ibc_proto::google::protobuf::Any>,
	) -> Result<u64, Self::Error> {
		// TODO: estimate gas for the tx. Convert any to another type (see `wrap_any_msg_into_wasm` for an example)
		Ok(1)
	}

	async fn finality_notifications(
		&self,
	) -> Result<std::pin::Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send>>, Self::Error> {
		let ws = crate::client::WsEth::connect(self.ws_uri.to_string())
			.await
			.map_err(|err| crate::client::ClientError::ProviderError(self.ws_uri.clone(), err))?;

		let stream = async_stream::stream! {
			// TODO: is it really finalized blocks stream?
			// - PoW: probabilistic finality (wait for ~30 blocks)
			// - PoS: finality is deterministic
			let mut stream = ws.subscribe_blocks().await.expect("fuck");

			while let Some(block) = stream.next().await {
				if let Some(hash) = block.hash.clone() {
					yield Self::FinalityEvent::Ethereum { hash };
				}
			}
		};

		Ok(stream.boxed())
	}

	async fn submit(
		&self,
		messages: Vec<ibc_proto::google::protobuf::Any>,
	) -> Result<Self::TransactionId, Self::Error> {

		use ethers::abi::encode as ethers_encode;
		use ethers::abi::Token as EthersToken;

		//get tendermint client via address and contract name in yui project
		let contract = crate::contract::get_contract_from_name(
			self.config.tendermint_client_address.clone(),
			Arc::clone(&self.http_rpc),
			"contracts/clients",
			"TendermintLightClientSimple"
		);

		let msg = messages.iter().next();
		if let Some(msg) = msg {
			if msg.type_url == ibc::core::ics02_client::msgs::create_client::TYPE_URL{
				dbg!(&msg.value.len());
				let msg = MsgCreateAnyClient::<LocalClientTypes>::decode_vec(&msg.value).unwrap();
				let AnyClientState::Tendermint(client_state) = msg.client_state else{
					//TODO return error support only tendermint client state
					return Ok(());
				};
				let AnyConsensusState::Tendermint(client_consensus) = msg.consensus_state else{
					//TODO return error support only tendermint consensus state
					return Ok(());
				};
				let client_state_abi_token = client_state_abi_token(client_state);
				let consensus_state_abi_token = consensus_state_abi_token(client_consensus);

				let client_state_data_vec = ethers_encode(&[client_state_abi_token]);
				let consensus_state_data_vec = ethers_encode(&[consensus_state_abi_token]);
				
				dbg!(&client_state_data_vec.len());
				dbg!(&consensus_state_data_vec.len());

				let contract = crate::contract::get_contract_from_name(
					self.config.ibc_handler_address.clone(),
					Arc::clone(&self.http_rpc),
					"contracts/core",
					"OwnableIBCHandler"
				);

				let ibc_handler = IbcHandler::new(contract);

				let token = EthersToken::Tuple(
					vec![
						//should be the same that we use to register client 
						//client type
						EthersToken::String(self.config.client_type.clone()),
						//clientStateBytes
						EthersToken::Bytes(client_state_data_vec.clone()),
						//consensusStateBytes
						EthersToken::Bytes(consensus_state_data_vec.clone()),
					]
				);

				let client_id = ibc_handler.create_client(token).await;
				dbg!(&client_id);

				/*
					.confirmations(6) is used to wait for 6 blocks to be mined
					but with anvil it is just wait for infinte time
					just sleep for 10 seconds instead of waiting for 6 blocks
					let receipt = method.send().await.unwrap();
					let receipt = receipt.confirmations(6).await.unwrap().unwrap();
				 */

				thread::sleep(Duration::from_secs(20));



				//update mutex
				let mut update_mutex = self.prev_state.lock().unwrap();	
				*update_mutex = (client_state_data_vec.clone(), consensus_state_data_vec.clone());

				return Ok(());
			}

			if msg.type_url == ibc::core::ics02_client::msgs::update_client::TYPE_URL{
				let msg = MsgUpdateAnyClient::<LocalClientTypes>::decode_vec(&msg.value).unwrap();
				let AnyClientMessage::Tendermint(client_state) = msg.client_message else{
					//TODO return error support only tendermint client state
					return Ok(());
				};
				let ClientMessage::Header(header) = client_state else{
					return Ok(());
				};

				//get abi token to update client
				let tm_header_abi_token = tm_header_abi_token(header);
				let tm_header_bytes = ethers_encode(&[tm_header_abi_token]);

				//todo replace empty vec for prev state clint with an actual client state
				let client_state = self.prev_state.lock().unwrap().0.clone();


				let contract = crate::contract::get_contract_from_name(
					self.config.ibc_handler_address.clone(),
					Arc::clone(&self.http_rpc),
					"contracts/core",
					"OwnableIBCHandler"
				);

				let ibc_handler = IbcHandler::new(contract);
				//TODO replace client id. it was genereated when we created the client. use 0 for testing
				let client_id = format!("{}-0", self.config.client_type.clone());

				let token = EthersToken::Tuple(
					vec![
						//should be the same that we use to create client 
						//client id
						EthersToken::String(client_id),
						//tm header
						EthersToken::Bytes(tm_header_bytes),
						//tm header
						EthersToken::Bytes(client_state),
					]
				);
				let _ = ibc_handler.update_client(token).await;

				thread::sleep(Duration::from_secs(20));
			}
			return Ok(())
		};

		unimplemented!("client create and client update is implemented only for now");
	}

	async fn query_client_message(&self, update: UpdateClient) -> Result<AnyClientMessage, Self::Error> {
		todo!("used for misbehaviour; skip for now")
	}

	async fn get_proof_height(&self, block_height: Height) -> Height {
		block_height
	}


	async fn handle_error(&mut self, error: &anyhow::Error) -> Result<(), anyhow::Error> {
		tracing::error!(?error, "handle-error");
		Ok(())
	}

	fn common_state(&self) -> &CommonClientState {
		&self.common_state
	}

	fn common_state_mut(&mut self) -> &mut CommonClientState {
		&mut self.common_state
	}

	fn rpc_call_delay(&self) -> Duration {
		Duration::from_millis(100)
	}

	fn set_rpc_call_delay(&mut self, delay: Duration) {}

	async fn reconnect(&mut self) -> anyhow::Result<()> {
		// TODO: reconnection logic
		Ok(())
	}
}
