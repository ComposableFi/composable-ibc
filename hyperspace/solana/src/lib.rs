use std::{pin::Pin, rc::Rc, str::FromStr, time::Duration};

use anchor_client::{
	anchor_lang::{prelude::Pubkey, system_program},
	solana_sdk::{
		commitment_config::{CommitmentConfig, CommitmentLevel},
		signature::{Keypair, Signature}, signer::Signer as AnchorSigner,
	},
	Client as AnchorClient, Cluster, solana_client::rpc_config::RpcSendTransactionConfig,
};
use error::Error;
use ibc::{
	core::{
		ics02_client::events::UpdateClient,
		ics04_channel::packet::Packet,
		ics24_host::identifier::{ClientId, ConnectionId},
	},
	events::IbcEvent,
	signer::Signer,
	Height,
};
use ibc_proto::google::protobuf::Any;
use instructions::AnyCheck;
use pallet_ibc::light_clients::AnyClientMessage;
use primitives::{
	Chain, CommonClientConfig, CommonClientState, IbcProvider, KeyProvider, LightClientSync,
	MisbehaviourHandler, UndeliveredType,
};
use tendermint_rpc::{endpoint::abci_query::AbciQuery, HttpClient, Url, WebSocketClient};
use tokio_stream::Stream;

mod accounts;
mod instructions;
mod error;

const SOLANA_IBC_STORAGE_SEED: &[u8] = b"solana_ibc_storage";
const TRIE_SEED: &[u8] = b"trie";

pub struct InnerAny {
	pub type_url: String,
	pub value: Vec<u8>,
}

/// Implements the [`crate::Chain`] trait for solana
#[derive(Clone)]
pub struct Client {
	/// Chain name
	pub name: String,
	/// rpc url for cosmos
	pub rpc_url: String,
	/// Solana chain Id
	pub chain_id: String,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Connection Id
	pub connection_id: Option<ConnectionId>,
	/// Account prefix
	pub account_prefix: String,
	pub fee_denom: String,
	/// The key that signs transactions
	pub keybase: KeyEntry,
	/// Maximun transaction size
	pub max_tx_size: usize,
	pub commitment_level: CommitmentLevel,
	pub program_id: Pubkey,
	pub common_state: CommonClientState,
}

pub struct ClientConfig {
	/// Chain name
	pub name: String,
	/// rpc url for cosmos
	pub rpc_url: Url,
	/// Solana chain Id
	pub chain_id: String,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Connection Id
	pub connection_id: Option<ConnectionId>,
	/// Account prefix
	pub account_prefix: String,
	/// Fee denom
	pub fee_denom: String,
	/// Fee amount
	pub fee_amount: String,
	/// Fee amount
	pub gas_limit: u64,
	/// Store prefix
	pub store_prefix: String,
	/// Maximun transaction size
	pub max_tx_size: usize,
	/// All the client states and headers will be wrapped in WASM ones using the WASM code ID.
	pub wasm_code_id: Option<String>,
	pub common_state_config: CommonClientConfig,
}

#[derive(Clone)]
pub struct KeyEntry {
	pub public_key: Pubkey,
	pub private_key: Vec<u8>,
}

impl Client {}

#[async_trait::async_trait]
impl IbcProvider for Client {
    type FinalityEvent = Vec<u8>;

    type TransactionId = String;

    type AssetId = String;

    type Error = Error;

    async fn query_latest_ibc_events<T>(
		    &mut self,
		    finality_event: Self::FinalityEvent,
		    counterparty: &T,
	    ) -> Result<Vec<(Any, Height, Vec<IbcEvent>, primitives::UpdateType)>, anyhow::Error>
	    where
		    T: Chain {
        todo!()
    }

    async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
        todo!()
    }

    async fn query_client_consensus(
		    &self,
		    at: Height,
		    client_id: ClientId,
		    consensus_height: Height,
	    ) -> Result<ibc_proto::ibc::core::client::v1::QueryConsensusStateResponse, Self::Error> {
        todo!()
    }

    async fn query_client_state(
		    &self,
		    at: Height,
		    client_id: ClientId,
	    ) -> Result<ibc_proto::ibc::core::client::v1::QueryClientStateResponse, Self::Error> {
        todo!()
    }

    async fn query_connection_end(
		    &self,
		    at: Height,
		    connection_id: ConnectionId,
	    ) -> Result<ibc_proto::ibc::core::connection::v1::QueryConnectionResponse, Self::Error> {
        todo!()
    }

    async fn query_channel_end(
		    &self,
		    at: Height,
		    channel_id: ibc::core::ics24_host::identifier::ChannelId,
		    port_id: ibc::core::ics24_host::identifier::PortId,
	    ) -> Result<ibc_proto::ibc::core::channel::v1::QueryChannelResponse, Self::Error> {
        todo!()
    }

    async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }

    async fn query_packet_commitment(
		    &self,
		    at: Height,
		    port_id: &ibc::core::ics24_host::identifier::PortId,
		    channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		    seq: u64,
	    ) -> Result<ibc_proto::ibc::core::channel::v1::QueryPacketCommitmentResponse, Self::Error> {
        todo!()
    }

    async fn query_packet_acknowledgement(
		    &self,
		    at: Height,
		    port_id: &ibc::core::ics24_host::identifier::PortId,
		    channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		    seq: u64,
	    ) -> Result<ibc_proto::ibc::core::channel::v1::QueryPacketAcknowledgementResponse, Self::Error> {
        todo!()
    }

    async fn query_next_sequence_recv(
		    &self,
		    at: Height,
		    port_id: &ibc::core::ics24_host::identifier::PortId,
		    channel_id: &ibc::core::ics24_host::identifier::ChannelId,
	    ) -> Result<ibc_proto::ibc::core::channel::v1::QueryNextSequenceReceiveResponse, Self::Error> {
        todo!()
    }

    async fn query_packet_receipt(
		    &self,
		    at: Height,
		    port_id: &ibc::core::ics24_host::identifier::PortId,
		    channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		    seq: u64,
	    ) -> Result<ibc_proto::ibc::core::channel::v1::QueryPacketReceiptResponse, Self::Error> {
        todo!()
    }

    async fn latest_height_and_timestamp(&self) -> Result<(Height, ibc::timestamp::Timestamp), Self::Error> {
        todo!()
    }

    async fn query_packet_commitments(
		    &self,
		    at: Height,
		    channel_id: ibc::core::ics24_host::identifier::ChannelId,
		    port_id: ibc::core::ics24_host::identifier::PortId,
	    ) -> Result<Vec<u64>, Self::Error> {
        todo!()
    }

    async fn query_packet_acknowledgements(
		    &self,
		    at: Height,
		    channel_id: ibc::core::ics24_host::identifier::ChannelId,
		    port_id: ibc::core::ics24_host::identifier::PortId,
	    ) -> Result<Vec<u64>, Self::Error> {
        todo!()
    }

    async fn query_unreceived_packets(
		    &self,
		    at: Height,
		    channel_id: ibc::core::ics24_host::identifier::ChannelId,
		    port_id: ibc::core::ics24_host::identifier::PortId,
		    seqs: Vec<u64>,
	    ) -> Result<Vec<u64>, Self::Error> {
        todo!()
    }

    async fn query_unreceived_acknowledgements(
		    &self,
		    at: Height,
		    channel_id: ibc::core::ics24_host::identifier::ChannelId,
		    port_id: ibc::core::ics24_host::identifier::PortId,
		    seqs: Vec<u64>,
	    ) -> Result<Vec<u64>, Self::Error> {
        todo!()
    }

    fn channel_whitelist(&self) -> std::collections::HashSet<(ibc::core::ics24_host::identifier::ChannelId, ibc::core::ics24_host::identifier::PortId)> {
        todo!()
    }

    async fn query_connection_channels(
		    &self,
		    at: Height,
		    connection_id: &ConnectionId,
	    ) -> Result<ibc_proto::ibc::core::channel::v1::QueryChannelsResponse, Self::Error> {
        todo!()
    }

    async fn query_send_packets(
		    &self,
		    channel_id: ibc::core::ics24_host::identifier::ChannelId,
		    port_id: ibc::core::ics24_host::identifier::PortId,
		    seqs: Vec<u64>,
	    ) -> Result<Vec<ibc_rpc::PacketInfo>, Self::Error> {
        todo!()
    }

    async fn query_received_packets(
		    &self,
		    channel_id: ibc::core::ics24_host::identifier::ChannelId,
		    port_id: ibc::core::ics24_host::identifier::PortId,
		    seqs: Vec<u64>,
	    ) -> Result<Vec<ibc_rpc::PacketInfo>, Self::Error> {
        todo!()
    }

    fn expected_block_time(&self) -> Duration {
        todo!()
    }

    async fn query_client_update_time_and_height(
		    &self,
		    client_id: ClientId,
		    client_height: Height,
	    ) -> Result<(Height, ibc::timestamp::Timestamp), Self::Error> {
        todo!()
    }

    async fn query_host_consensus_state_proof(
		    &self,
		    client_state: &pallet_ibc::light_clients::AnyClientState,
	    ) -> Result<Option<Vec<u8>>, Self::Error> {
        todo!()
    }

    async fn query_ibc_balance(
		    &self,
		    asset_id: Self::AssetId,
	    ) -> Result<Vec<ibc::applications::transfer::PrefixedCoin>, Self::Error> {
        todo!()
    }

    fn connection_prefix(&self) -> ibc::core::ics23_commitment::commitment::CommitmentPrefix {
        todo!()
    }

    fn client_id(&self) -> ClientId {
        todo!()
    }

    fn set_client_id(&mut self, client_id: ClientId) {
        todo!()
    }

    fn connection_id(&self) -> Option<ConnectionId> {
        todo!()
    }

    fn set_channel_whitelist(&mut self, channel_whitelist: std::collections::HashSet<(ibc::core::ics24_host::identifier::ChannelId, ibc::core::ics24_host::identifier::PortId)>) {
        todo!()
    }

    fn add_channel_to_whitelist(&mut self, channel: (ibc::core::ics24_host::identifier::ChannelId, ibc::core::ics24_host::identifier::PortId)) {
        todo!()
    }

    fn set_connection_id(&mut self, connection_id: ConnectionId) {
        todo!()
    }

    fn client_type(&self) -> ibc::core::ics02_client::client_state::ClientType {
        todo!()
    }

    async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
        todo!()
    }

    async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
        todo!()
    }

    async fn query_channels(&self) -> Result<Vec<(ibc::core::ics24_host::identifier::ChannelId, ibc::core::ics24_host::identifier::PortId)>, Self::Error> {
        todo!()
    }

    async fn query_connection_using_client(
		    &self,
		    height: u32,
		    client_id: String,
	    ) -> Result<Vec<ibc_proto::ibc::core::connection::v1::IdentifiedConnection>, Self::Error> {
        todo!()
    }

    async fn is_update_required(
		    &self,
		    latest_height: u64,
		    latest_client_height_on_counterparty: u64,
	    ) -> Result<bool, Self::Error> {
        todo!()
    }

    async fn initialize_client_state(
		    &self,
	    ) -> Result<(pallet_ibc::light_clients::AnyClientState, pallet_ibc::light_clients::AnyConsensusState), Self::Error> {
        todo!()
    }

    async fn query_client_id_from_tx_hash(
		    &self,
		    tx_id: Self::TransactionId,
	    ) -> Result<ClientId, Self::Error> {
        todo!()
    }

    async fn query_connection_id_from_tx_hash(
		    &self,
		    tx_id: Self::TransactionId,
	    ) -> Result<ConnectionId, Self::Error> {
        todo!()
    }

    async fn query_channel_id_from_tx_hash(
		    &self,
		    tx_id: Self::TransactionId,
	    ) -> Result<(ibc::core::ics24_host::identifier::ChannelId, ibc::core::ics24_host::identifier::PortId), Self::Error> {
        todo!()
    }

    async fn upload_wasm(&self, wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }
}

impl KeyProvider for Client {
	fn account_id(&self) -> Signer {
		let key_entry = &self.keybase;
		let public_key = key_entry.public_key;
		Signer::from_str(&public_key.to_string()).unwrap()
	}
}

#[async_trait::async_trait]
impl MisbehaviourHandler for Client {
	async fn check_for_misbehaviour<C: Chain>(
		&self,
		_counterparty: &C,
		_client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		Ok(())
	}
}

#[async_trait::async_trait]
impl LightClientSync for Client {
	async fn is_synced<C: Chain>(&self, _counterparty: &C) -> Result<bool, anyhow::Error> {
		Ok(true)
	}

	async fn fetch_mandatory_updates<C: Chain>(
		&self,
		_counterparty: &C,
	) -> Result<(Vec<Any>, Vec<IbcEvent>), anyhow::Error> {
		Ok((vec![], vec![]))
	}
}

#[async_trait::async_trait]
impl Chain for Client {
	fn name(&self) -> &str {
		&self.name
	}

	fn block_max_weight(&self) -> u64 {
		self.max_tx_size as u64
	}

	async fn estimate_weight(&self, msg: Vec<Any>) -> Result<u64, Self::Error> {
		todo!()
	}

	async fn finality_notifications(
		&self,
	) -> Result<
		Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>>,
		Error,
	> {
		todo!()
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<Self::TransactionId, Error> {
		let cluster = Cluster::from_str(&self.rpc_url).unwrap();
		let authority = Keypair::from_bytes(&self.keybase.private_key).unwrap();
		let client = AnchorClient::new_with_options(
			cluster,
			&authority,
			CommitmentConfig { commitment: self.commitment_level },
		);
		let program = client.program(self.program_id).unwrap();

		// Build, sign, and send program instruction
		let storage_seeds = &[SOLANA_IBC_STORAGE_SEED];
		let solana_ibc_storage = Pubkey::find_program_address(storage_seeds, &self.program_id).0;
		let trie_seeds = &[TRIE_SEED];
		let trie = Pubkey::find_program_address(trie_seeds, &self.program_id).0;

		let all_messages = messages.into_iter().map(|message| AnyCheck {
			type_url: message.type_url,
			value: message.value,
		}).collect();

		let sig: Signature = program
			.request()
			.accounts(accounts::LocalDeliver::new(
				authority.pubkey(),
				solana_ibc_storage,
				trie,
				system_program::ID,
			))
			.args(instructions::Deliver { messages: all_messages })
			.payer(&authority)
			.signer(&authority)
			.send_with_spinner_and_config(RpcSendTransactionConfig {
				skip_preflight: true,
				..RpcSendTransactionConfig::default()
			}).unwrap();
		Ok(sig.to_string())
	}

	async fn query_client_message(
		&self,
		update: UpdateClient,
	) -> Result<AnyClientMessage, Self::Error> {
		todo!()
	}

	async fn get_proof_height(&self, block_height: Height) -> Height {
		block_height.increment()
	}

	async fn handle_error(&mut self, error: &anyhow::Error) -> Result<(), anyhow::Error> {
		todo!()
	}

	fn common_state(&self) -> &CommonClientState {
		&self.common_state
	}

	fn common_state_mut(&mut self) -> &mut CommonClientState {
		&mut self.common_state
	}

	async fn reconnect(&mut self) -> anyhow::Result<()> {
		todo!()
	}

	async fn on_undelivered_sequences(&self, has: bool, kind: UndeliveredType) {
		let _ = Box::pin(async move {
			let __self = self;
			let has = has;
			let kind = kind;
			let () = { __self.common_state().on_undelivered_sequences(has, kind).await };
		});
	}

	fn has_undelivered_sequences(&self, kind: UndeliveredType) -> bool {
		self.common_state().has_undelivered_sequences(kind)
	}

	fn rpc_call_delay(&self) -> Duration {
		self.common_state().rpc_call_delay()
	}

	fn initial_rpc_call_delay(&self) -> Duration {
		self.common_state().initial_rpc_call_delay
	}

	fn set_rpc_call_delay(&mut self, delay: Duration) {
		self.common_state_mut().set_rpc_call_delay(delay)
	}
}
