use std::{time::Duration, sync::Arc};

use ethers::providers::Middleware;
use futures::{Stream, StreamExt};
use ibc::core::ics02_client::events::UpdateClient;
use ibc::Height;
use pallet_ibc::light_clients::AnyClientMessage;
use primitives::{Chain, CommonClientState, LightClientSync, MisbehaviourHandler};

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

		//get tendermint client via address and contract name in yui project
		let contract = crate::contract::get_contract_from_name(
			self.config.tendermint_client_address.clone(),
			Arc::clone(&self.http_rpc),
			"TendermintLightClientSimple"
		);

		use ethers::abi::encode as ethers_encode;
		use ethers::abi::Token as EthersToken;

		let client_state_data = EthersToken::Tuple(
			[
				//chain_id
				EthersToken::String("cosmos-chain-id".to_string()),
				//trust_level
				EthersToken::Tuple(
					[
						//numerator
						EthersToken::Uint(1.into()),
						//denominator
						EthersToken::Uint(2.into()),
					].to_vec()),
				//trusting_period
				EthersToken::Tuple(
				[
					EthersToken::Int(1.into()),
					EthersToken::Int(2.into()),
				].to_vec()),
				//unbonding_period
				EthersToken::Tuple(
					[
						EthersToken::Int(1.into()),
						EthersToken::Int(2.into()),
					].to_vec()),
				//max_clock_drift	
				EthersToken::Tuple(
					[
						EthersToken::Int(1.into()),
						EthersToken::Int(2.into()),
					].to_vec()),
				//frozen_height
				EthersToken::Int(10.into()),
				//latest_height
				EthersToken::Int(100.into()),
				//allow_update_after_expiry
				EthersToken::Bool(true),
				//allow_update_after_misbehaviour
				EthersToken::Bool(true),
			].to_vec());

		let consensus_state_data = EthersToken::Tuple(
			[
				//timestamp	
				EthersToken::Tuple(
					[
						EthersToken::Int(1.into()),
						EthersToken::Int(2.into()),
					].to_vec()),
				//root
				EthersToken::Tuple(
					[
						EthersToken::Bytes(vec![1,2,3,4,5,6,7,8,9,10]),
					].to_vec()),
				//next_validators_hash
				EthersToken::Bytes(vec![1,2,3,4,5,6,7,8,9,10]),
			].to_vec());
	
	
		let commit_sig_data = &[client_state_data];
		let commit_sig_data_vec = ethers_encode(commit_sig_data);
	
		let consensus_state_data = &[consensus_state_data];
		let consensus_state_data_vec = ethers_encode(consensus_state_data);

		let contract_call_final = contract.method::<_, bool>
		("createClientSimple2", (EthersToken::String("client-id-1".to_string()) ,EthersToken::Bytes(commit_sig_data_vec.clone()), EthersToken::Bytes(consensus_state_data_vec.clone()))).unwrap();
		let gas_estimate_abi_encode_no_storage_final = contract_call_final.estimate_gas().await.unwrap();
		dbg!(&gas_estimate_abi_encode_no_storage_final);
		let gas_estimate_abi_encode_no_storage_final = contract_call_final.send().await.unwrap();
		dbg!(&gas_estimate_abi_encode_no_storage_final);

		std::thread::sleep(std::time::Duration::from_secs(20));

		// println!("the address again: {:?}, {client_id}", self.config.ibc_handler_address);

		Ok(())
		// todo!("submit to ethereum")
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
