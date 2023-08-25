use std::thread;
use std::{time::Duration, sync::Arc};

use ethers::abi::Token;
use ethers::prelude::EthAbiType;
use ethers::providers::Middleware;
use futures::{Stream, StreamExt};
use ibc::core::ics02_client::msgs::update_client::MsgUpdateAnyClient;
use ibc::core::ics02_client::{events::UpdateClient, msgs::create_client::MsgCreateAnyClient};
use ibc::Height;
use ibc::core::ics03_connection::msgs::conn_open_ack::MsgConnectionOpenAck;
use ibc::core::ics03_connection::msgs::conn_open_confirm::MsgConnectionOpenConfirm;
use ibc::core::ics03_connection::msgs::conn_open_init::MsgConnectionOpenInit;
use ibc::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry;
use ibc::protobuf::Protobuf;
use ibc::protobuf::google::protobuf::Timestamp;
use ics07_tendermint::client_message::{ClientMessage, Header};
use ics07_tendermint::client_state::ClientState;
use ics07_tendermint::consensus_state::ConsensusState;
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState, AnyConsensusState};
use primitives::mock::LocalClientTypes;
use primitives::{Chain, CommonClientState, LightClientSync, MisbehaviourHandler};
use serde::__private::de;

use ibc::core::ics04_channel::msgs as channel_msgs;
use channel_msgs::chan_close_confirm::MsgChannelCloseConfirm;
use channel_msgs::timeout_on_close::MsgTimeoutOnClose;
use channel_msgs::timeout::MsgTimeout;
use channel_msgs::acknowledgement::MsgAcknowledgement;
use channel_msgs::recv_packet::MsgRecvPacket;
use channel_msgs::chan_close_init::MsgChannelCloseInit;
use channel_msgs::chan_open_confirm::MsgChannelOpenConfirm;
use channel_msgs::chan_open_ack::MsgChannelOpenAck;
use channel_msgs::chan_open_try::MsgChannelOpenTry;
use channel_msgs::chan_open_init::MsgChannelOpenInit;


use crate::contract::IbcHandler;
use crate::yui_types::IntoToken;
use crate::yui_types::ics03_connection::conn_open_try::YuiMsgConnectionOpenTry;
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

fn msg_connection_open_init_token(x: MsgConnectionOpenInit) -> Token{
	use ethers::abi::encode as ethers_encode;
	use ethers::abi::Token as EthersToken;

	let consensus_state_data = EthersToken::Tuple(
		[
			// client id
			EthersToken::String(x.client_id.as_str().to_owned()),
			// counterparty
			EthersToken::Tuple(
				[
					// client id
					EthersToken::String(x.counterparty.client_id().as_str().to_owned()),
					// connection id
					EthersToken::String(x.counterparty.connection_id()
					.map(|connection_id| connection_id.as_str().to_owned())
					.unwrap_or(String::new())),
					// prefix
					EthersToken::Tuple(
						[
							// key prefix
							EthersToken::Bytes(x.counterparty.prefix().as_bytes().into()),
						].to_vec()),
				].to_vec()),
			// delay_period
			EthersToken::Uint(x.delay_period.as_secs().into()),
		].to_vec());
	consensus_state_data
}


fn msg_connection_open_ack_token<H>(msg: MsgConnectionOpenAck::<LocalClientTypes>, client_state: ClientState<H>) -> Token{
	use ethers::abi::encode as ethers_encode;
	use ethers::abi::Token as EthersToken;

	let client_state = client_state_abi_token(client_state);
	let client_state_data_vec = ethers_encode(&[client_state]);

	let consensus_state_data = EthersToken::Tuple(
		[
			// connectionId
			EthersToken::String(msg.connection_id.as_str().to_owned()),
			//clientStateBytes
			EthersToken::Bytes(client_state_data_vec),
			// //version
			EthersToken::Tuple(
				[
					//identifier
					EthersToken::String(msg.version.identifier().clone()),
					//features
					EthersToken::Array(
							msg.version.features().clone()
							.iter()
							.map(|feature| EthersToken::String(feature.to_string()))
							.collect()
						),
				].to_vec()),
			//counterpartyConnectionID
			EthersToken::String(msg.counterparty_connection_id.as_str().to_owned()),
			//proofTry
			EthersToken::Bytes(msg.proofs.object_proof().clone().into()),
			//proofClient
			EthersToken::Bytes(msg.proofs.client_proof().clone().map_or_else(Vec::new, |v| v.into())),
			//proofConsensus
			EthersToken::Bytes(msg.proofs.consensus_proof().map_or_else(Vec::new, |v| v.proof().clone().into())),
			//proofHeight tuple
			EthersToken::Tuple(
				[
					//revisionNumber
					EthersToken::Uint(msg.proofs.height().revision_number.into()),
					//revisionHeight
					EthersToken::Uint(msg.proofs.height().revision_height.into()),
				].to_vec()),
			//consensusHeight
			EthersToken::Tuple(
				[
					//revisionNumber
					EthersToken::Uint(msg.proofs.consensus_proof().unwrap().height().revision_number.into()),
					//revisionHeight
					EthersToken::Uint(msg.proofs.consensus_proof().unwrap().height().revision_number.into()),
				].to_vec()),
		].to_vec());
	consensus_state_data
}

fn msg_connection_open_try_token<H>(msg: MsgConnectionOpenTry::<LocalClientTypes>, client_state: ClientState<H>) -> Token{
	use ethers::abi::encode as ethers_encode;
	use ethers::abi::Token as EthersToken;
	let client_state = client_state_abi_token(client_state);
	let client_state_data_vec = ethers_encode(&[client_state]);
	let conn_open_try = YuiMsgConnectionOpenTry{
		counterparty: msg.counterparty.into(),
		delay_period: msg.delay_period.as_secs(),
		client_id: msg.client_id.as_str().to_owned(),
		client_state_bytes: client_state_data_vec,
		counterparty_versions: msg.counterparty_versions.into_iter().map(|version| version.into()).collect(),
		proof_init: msg.proofs.object_proof().clone().into(),
		proof_client: msg.proofs.client_proof().clone().map_or_else(Vec::new, |v| v.into()),
		proof_consensus: msg.proofs.consensus_proof().map_or_else(Vec::new, |v| v.proof().clone().into()),
		proof_height: msg.proofs.height().into(),
		consensus_height: msg.proofs.consensus_proof().unwrap().height().into(),
	};
	let conn_open_try_token = conn_open_try.into_token();
	conn_open_try_token
}

fn msg_connection_open_confirm_token(msg: MsgConnectionOpenConfirm) -> Token{
	use ethers::abi::encode as ethers_encode;
	use ethers::abi::Token as EthersToken;

	let confirm = EthersToken::Tuple(
		[
			// connectionId
			EthersToken::String(msg.connection_id.as_str().to_owned()),
			// proofAck
			EthersToken::Bytes(msg.proofs.object_proof().clone().into()),
			//proofHeight tuple
			EthersToken::Tuple(
				[
					//revisionNumber
					EthersToken::Uint(msg.proofs.height().revision_number.into()),
					//revisionHeight
					EthersToken::Uint(msg.proofs.height().revision_height.into()),
				].to_vec()),
		].to_vec());
	confirm
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

		let contract = crate::contract::get_contract_from_name(
			self.config.ibc_handler_address.clone(),
			Arc::clone(&self.http_rpc),
			"contracts/core",
			"OwnableIBCHandler"
		);

		let ibc_handler = IbcHandler::new(contract);

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

				thread::sleep(Duration::from_secs(5));



				//update mutex
				let mut update_mutex = self.prev_state.lock().unwrap();	
				*update_mutex = (client_state_data_vec.clone(), consensus_state_data_vec.clone());

				return Ok(());
			}
			else if msg.type_url == ibc::core::ics02_client::msgs::update_client::TYPE_URL{
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

				thread::sleep(Duration::from_secs(5));

				return Ok(());
			}
			else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_init::TYPE_URL{

				let msg = MsgConnectionOpenInit::decode_vec(&msg.value).unwrap();
				let token = msg_connection_open_init_token(msg);
				let connection_id = ibc_handler.connection_open_init(token).await;
				dbg!(connection_id);

				return Ok(());

				//there is no ignore field for EthAbiType so it is hard to reuse old struct and to create a tons of new one for each msg type is not a good idea
				// #[derive(EthAbiType)]
				// pub struct X{
				// 	a: String,
				// 	b: R,
				// }

				// #[derive(EthAbiType)]
				// pub struct R{
				// 	a: String,
				// 	b: String,
				// };

				// use ethers::abi::Detokenize;
				// use ethers::abi::Tokenize;
				// let r = X::into_tokens(X{a: "hello".to_string(), b : R{ a: "hello".to_string(), b: "hello".to_string()}});
			}
			else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_ack::TYPE_URL{
				let msg = MsgConnectionOpenAck::<LocalClientTypes>::decode_vec(&msg.value).unwrap();

				let client_state = match msg.client_state.clone(){
					Some(m) => {
						let AnyClientState::Tendermint(client_state) = m else{
							//TODO return error support only tendermint client state
							panic!("only tendermint client state is supported for now");
						};
						client_state
					}
					None => {
						//TODO return error support only tendermint client state
						panic!("only tendermint client state is supported for now");
					}
				};
				
				let token = msg_connection_open_ack_token(msg, client_state);
				ibc_handler.connection_open_ack(token).await;

				return Ok(());
			}
			else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_try::TYPE_URL{
				let msg = MsgConnectionOpenTry::<LocalClientTypes>::decode_vec(&msg.value).unwrap();
				let client_state = match msg.client_state.clone(){
					Some(m) => {
						let AnyClientState::Tendermint(client_state) = m else{
							//TODO return error support only tendermint client state
							panic!("only tendermint client state is supported for now");
						};
						client_state
					}
					None => {
						//TODO return error support only tendermint client state
						panic!("only tendermint client state is supported for now");
					}
				};

				let token = msg_connection_open_try_token(msg, client_state);
				ibc_handler.connection_open_try(token).await;
				
				return Ok(());
			}
			else if msg.type_url == ibc::core::ics03_connection::msgs::conn_open_confirm::TYPE_URL{
				let msg = MsgConnectionOpenConfirm::decode_vec(&msg.value).unwrap();
				let token = msg_connection_open_confirm_token(msg);
				ibc_handler.connection_open_confirm(token).await;
				return Ok(());
			}
			
			else if msg.type_url == channel_msgs::chan_open_init::TYPE_URL{
				let msg = MsgChannelOpenInit::decode_vec(&msg.value).unwrap();
				let token = msg.into_token();
				let channel_id = ibc_handler.send::<String>(token, "channelOpenInit").await;
				return Ok(());
			}
			else if msg.type_url == channel_msgs::chan_open_try::TYPE_URL{
				let msg = MsgChannelOpenTry::decode_vec(&msg.value).unwrap();
			}
			else if msg.type_url == channel_msgs::chan_open_ack::TYPE_URL{	
				let msg = MsgChannelOpenAck::decode_vec(&msg.value).unwrap();
			}
			else if msg.type_url == channel_msgs::chan_open_confirm::TYPE_URL{
				let msg = MsgChannelOpenConfirm::decode_vec(&msg.value).unwrap();
			}
			else if msg.type_url == channel_msgs::chan_close_init::TYPE_URL{
				let msg = MsgChannelCloseInit::decode_vec(&msg.value).unwrap();
			}
			else if msg.type_url == channel_msgs::chan_close_confirm::TYPE_URL{
				let msg = MsgChannelCloseConfirm::decode_vec(&msg.value).unwrap();
			}
			else if msg.type_url == channel_msgs::timeout_on_close::TYPE_URL{
				let msg = MsgTimeoutOnClose::decode_vec(&msg.value).unwrap();
			}
			else if msg.type_url == channel_msgs::timeout::TYPE_URL{
				let msg = MsgTimeout::decode_vec(&msg.value).unwrap();
			}
			else if msg.type_url == channel_msgs::acknowledgement::TYPE_URL{
				let msg = MsgAcknowledgement::decode_vec(&msg.value).unwrap();
			}
			else if msg.type_url == channel_msgs::recv_packet::TYPE_URL{
				let msg = MsgRecvPacket::decode_vec(&msg.value).unwrap();
			}
			


			unimplemented!("does not support this msg type for now");
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
