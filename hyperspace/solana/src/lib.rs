#![feature(more_qualified_paths)]
extern crate alloc;

use alloc::rc::Rc;
use anchor_spl::associated_token::get_associated_token_address;
use base64::Engine;
use client_state::convert_new_client_state_to_old;
use consensus_state::convert_new_consensus_state_to_old;
use core::{pin::Pin, str::FromStr, time::Duration};
use msgs::convert_old_msgs_to_new;
use solana_transaction_status::UiTransactionEncoding;
use tendermint::Hash;
use tokio::sync::mpsc::unbounded_channel;

use anchor_client::{
	solana_client::{
		nonblocking::rpc_client::RpcClient as AsyncRpcClient,
		pubsub_client::PubsubClient,
		rpc_client::RpcClient,
		rpc_config::{
			RpcSendTransactionConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter,
		},
	},
	solana_sdk::{
		commitment_config::{CommitmentConfig, CommitmentLevel},
		signature::{Keypair, Signature},
		signer::Signer as AnchorSigner,
	},
	Client as AnchorClient, Cluster, Program,
};
use anchor_lang::{prelude::*, system_program};
use error::Error;
use ibc::{
	applications::transfer::{Amount, BaseDenom, PrefixedCoin, PrefixedDenom, TracePath},
	core::{
		ics02_client::{client_state::ClientType, events::UpdateClient},
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentRoot},
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::msgs::Ics26Envelope,
	},
	events::IbcEvent,
	timestamp::Timestamp,
	Height,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			Channel, Counterparty as ChanCounterparty, QueryChannelResponse,
			QueryNextSequenceReceiveResponse, QueryPacketAcknowledgementResponse,
			QueryPacketCommitmentResponse, QueryPacketReceiptResponse,
		},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::{
			ConnectionEnd, Counterparty as ConnCounterparty, IdentifiedConnection,
			QueryConnectionResponse, Version,
		},
	},
};
use pallet_ibc::light_clients::{AnyClientMessage, AnyConsensusState};
use primitives::{
	mock::LocalClientTypes, Chain, CommonClientConfig, CommonClientState, IbcProvider, KeyProvider,
	LightClientSync, MisbehaviourHandler, UndeliveredType,
};
use std::{
	collections::{BTreeMap, HashSet},
	result::Result,
	sync::{Arc, Mutex},
};
use tendermint_rpc::Url;
use tokio_stream::Stream;

// use crate::ibc_storage::{AnyConsensusState, Serialised};
use solana_ibc::storage::{PrivateStorage, SequenceTripleIdx, Serialised};
use solana_trie::trie;
use trie_ids::{ClientIdx, ConnectionIdx, PortChannelPK, TrieKey};

use crate::events::convert_new_event_to_old;

// mod accounts;
mod client_state;
mod consensus_state;
mod error;
mod events;
mod msgs;
// mod ibc_storage;
// mod ids;
// mod instructions;
// mod trie;
// mod trie_key;

const SOLANA_IBC_STORAGE_SEED: &[u8] = b"private";
const TRIE_SEED: &[u8] = b"trie";
const PACKET_STORAGE_SEED: &[u8] = b"packet";
const CHAIN_SEED: &[u8] = b"chain";

// Random key added to implement `#[account]` macro for the storage
declare_id!("EnfDJsAK7BGgetnmKzBx86CsgC5kfSPcsktFCQ4YLC81");

pub struct InnerAny {
	pub type_url: String,
	pub value: Vec<u8>,
}

/// Implements the [`crate::Chain`] trait for solana
#[derive(Clone)]
pub struct Client {
	/// Chain name
	pub name: String,
	/// rpc url for solana
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
	pub client_type: ClientType,
	/// Reference to commitment
	pub commitment_prefix: CommitmentPrefix,
	/// Channels cleared for packet relay
	pub channel_whitelist: Arc<Mutex<HashSet<(ChannelId, PortId)>>>,
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
	/// Reference to commitment
	pub commitment_prefix: CommitmentPrefix,
}

#[derive(Clone)]
pub struct KeyEntry {
	pub public_key: Pubkey,
	pub private_key: Vec<u8>,
}

impl KeyEntry {
	fn keypair(&self) -> Keypair {
		Keypair::from_bytes(&self.private_key).unwrap()
	}
}

impl Client {
	pub fn get_trie_key(&self) -> Pubkey {
		let trie_seeds = &[TRIE_SEED];
		let trie = Pubkey::find_program_address(trie_seeds, &self.program_id).0;
		trie
	}

	pub fn get_ibc_storage_key(&self) -> Pubkey {
		let storage_seeds = &[SOLANA_IBC_STORAGE_SEED];
		let ibc_storage = Pubkey::find_program_address(storage_seeds, &self.program_id).0;
		ibc_storage
	}

	// pub fn get_packet_storage_key(&self) -> Pubkey {
	// 	let packet_storage_seeds = &[PACKET_STORAGE_SEED];
	// 	let packet_storage = Pubkey::find_program_address(packet_storage_seeds, &self.program_id).0;
	// 	packet_storage
	// }

	pub fn get_chain_key(&self) -> Pubkey {
		let chain_seeds = &[CHAIN_SEED];
		let chain = Pubkey::find_program_address(chain_seeds, &self.program_id).0;
		chain
	}

	pub async fn get_trie(&self) -> trie::AccountTrie<Vec<u8>> {
		let trie_key = self.get_trie_key();
		let rpc_client = self.rpc_client();
		let trie_account = rpc_client
			.get_account_with_commitment(&trie_key, CommitmentConfig::processed())
			.await
			.unwrap()
			.value
			.unwrap();
		let trie = trie::AccountTrie::new(trie_account.data).unwrap();
		trie
	}

	pub fn get_ibc_storage(&self) -> PrivateStorage {
		let program = self.program();
		let ibc_storage_key = self.get_ibc_storage_key();
		let storage = program.account(ibc_storage_key).unwrap();
		storage
	}

	// pub fn get_packet_storage(&self) -> IbcPackets {
	// 	let program = self.program();
	// 	let packet_storage_key = self.get_packet_storage_key();
	// 	let storage = program.account(packet_storage_key).unwrap();
	// 	storage
	// }

	pub fn rpc_client(&self) -> AsyncRpcClient {
		let program = self.program();
		program.async_rpc()
	}

	pub fn client(&self) -> AnchorClient<Rc<Keypair>> {
		let cluster = Cluster::from_str(&self.rpc_url).unwrap();
		let signer = self.keybase.keypair();
		let authority = Rc::new(signer);
		let client =
			AnchorClient::new_with_options(cluster, authority, CommitmentConfig::processed());
		client
	}

	pub fn program(&self) -> Program<Rc<Keypair>> {
		let anchor_client = self.client();
		anchor_client.program(self.program_id).unwrap()
	}
}

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
		T: Chain,
	{
		todo!()
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
		let (tx, rx) = unbounded_channel();
		let cluster = Cluster::from_str(&self.rpc_url).unwrap();
		tokio::task::spawn_blocking(move || {
			let (_logs_subscription, receiver) = PubsubClient::logs_subscribe(
				cluster.ws_url(),
				RpcTransactionLogsFilter::Mentions(vec![ID.to_string()]),
				RpcTransactionLogsConfig { commitment: Some(CommitmentConfig::processed()) },
			)
			.unwrap();

			loop {
				match receiver.recv() {
					Ok(logs) => {
						let events = get_events_from_logs(logs.value.logs);
						events.iter().for_each(|event| {
							tx.send(events::convert_new_event_to_old(event.clone())).unwrap()
						});
					},
					Err(err) => {
						panic!("{}", format!("Disconnected: {err}"));
					},
				}
			}
		});

		let streams = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);
		Box::pin(streams)
	}

	// WIP
	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let trie = self.get_trie().await;
		let storage = self.get_ibc_storage();
		let revision_height = consensus_height.revision_height;
		let revision_number = consensus_height.revision_number;
		let new_client_id =
			ibc_new::core::host::types::identifiers::ClientId::from_str(client_id.as_str())
				.unwrap();
		let consensus_state_trie_key = TrieKey::for_consensus_state(
			ClientIdx::try_from(new_client_id).unwrap(),
			ibc_new::core::client::types::Height::new(
				consensus_height.revision_number,
				consensus_height.revision_height,
			)
			.unwrap(),
		);
		let (_, consensus_state_proof) = trie
			.prove(&consensus_state_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let client_store = storage
			.clients
			.iter()
			.find(|&client| client.client_id.as_str() == client_id.as_str())
			.ok_or("Client not found with the given client id".to_owned())?;
		let serialized_consensus_state = client_store
			.consensus_states
			.get(
				&ibc_new::core::client::types::Height::new(revision_number, revision_height)
					.unwrap(),
			)
			.ok_or(Error::Custom("No value at given key".to_owned()))?;
		let consensus_state = serialized_consensus_state
			.get()
			.map_err(|_| {
				Error::Custom(
					"Could not
deserialize consensus state"
						.to_owned(),
				)
			})
			.unwrap();
		let cs_state = convert_new_consensus_state_to_old(consensus_state);
		Ok(QueryConsensusStateResponse {
			consensus_state: Some(cs_state.into()),
			proof: borsh::to_vec(&consensus_state_proof).unwrap(),
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	// WIP
	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		let trie = self.get_trie().await;
		let storage = self.get_ibc_storage();
		let new_client_id =
			ibc_new::core::host::types::identifiers::ClientId::from_str(client_id.as_str())
				.unwrap();
		let client_state_trie_key =
			TrieKey::for_client_state(ClientIdx::try_from(new_client_id).unwrap());
		let (_, client_state_proof) = trie
			.prove(&client_state_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let client_store = storage
			.clients
			.iter()
			.find(|&client| client.client_id.as_str() == client_id.as_str())
			.ok_or("Client not found with the given client id".to_owned())?;
		let serialized_client_state = &client_store.client_state;
		let client_state = serialized_client_state
			.get()
			.map_err(|_| {
				Error::Custom(
					"Could not
deserialize client state"
						.to_owned(),
				)
			})
			.unwrap();
		let any_client_state = convert_new_client_state_to_old(client_state);
		Ok(QueryClientStateResponse {
			client_state: Some(any_client_state.into()),
			proof: borsh::to_vec(&client_state_proof).unwrap(),
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		let trie = self.get_trie().await;
		let storage = self.get_ibc_storage();
		let connection_idx = ConnectionIdx::try_from(
			ibc_new::core::host::types::identifiers::ConnectionId::from_str(connection_id.as_str())
				.unwrap(),
		)
		.unwrap();
		let connection_end_trie_key = TrieKey::for_connection(connection_idx);
		let (_, connection_end_proof) = trie
			.prove(&connection_end_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let serialized_connection_end =
			storage.connections.get(usize::from(connection_idx)).ok_or(
				"Connection not found with the given
		client id"
					.to_owned(),
			)?;
		let inner_connection_end = Serialised::get(serialized_connection_end)
			.map_err(|_| {
				Error::Custom(
					"Could not
	deserialize connection end"
						.to_owned(),
				)
			})
			.unwrap();
		let inner_counterparty = inner_connection_end.counterparty();
		let connection_end = ConnectionEnd {
			client_id: inner_connection_end.client_id().to_string(),
			versions: inner_connection_end
				.versions()
				.to_vec()
				.iter()
				.map(|version| {
					let raw_version =
						ibc_proto_new::ibc::core::connection::v1::Version::from(version.clone());
					Version { identifier: raw_version.identifier, features: raw_version.features }
				})
				.collect(),
			state: inner_connection_end.state.into(),
			counterparty: Some(ConnCounterparty {
				client_id: inner_counterparty.client_id().to_string(),
				connection_id: inner_counterparty.connection_id.as_ref().unwrap().to_string(),
				prefix: Some(ibc_proto::ibc::core::commitment::v1::MerklePrefix {
					key_prefix: inner_counterparty.prefix().clone().into_vec(),
				}),
			}),
			delay_period: inner_connection_end.delay_period().as_secs(),
		};
		Ok(QueryConnectionResponse {
			connection: Some(connection_end),
			proof: borsh::to_vec(&connection_end_proof).unwrap(),
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let trie = self.get_trie().await;
		let storage = self.get_ibc_storage();
		let new_port_id =
			ibc_new::core::host::types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_new::core::host::types::identifiers::ChannelId::new(channel_id.sequence());
		let channel_end_path =
			PortChannelPK::try_from(new_port_id.clone(), new_channel_id.clone()).unwrap();
		let channel_end_trie_key = TrieKey::for_channel_end(&channel_end_path);
		let (_, channel_end_proof) = trie
			.prove(&channel_end_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let serialized_channel_end = storage
			.channel_ends
			.get(&channel_end_path)
			.ok_or(Error::Custom("No value at given key".to_owned()))?;
		let inner_channel_end = Serialised::get(serialized_channel_end)
			.map_err(|_| Error::Custom("Could not deserialize connection end".to_owned()))?;
		let inner_counterparty = inner_channel_end.counterparty();
		let state = match inner_channel_end.state {
			ibc_new::core::channel::types::channel::State::Uninitialized => 0,
			ibc_new::core::channel::types::channel::State::Init => 1,
			ibc_new::core::channel::types::channel::State::TryOpen => 2,
			ibc_new::core::channel::types::channel::State::Open => 3,
			ibc_new::core::channel::types::channel::State::Closed => 4,
		};
		let ordering = match inner_channel_end.ordering {
			ibc_new::core::channel::types::channel::Order::None => 0,
			ibc_new::core::channel::types::channel::Order::Unordered => 1,
			ibc_new::core::channel::types::channel::Order::Ordered => 2,
		};
		let channel_end = Channel {
			state,
			ordering,
			counterparty: Some(ChanCounterparty {
				port_id: inner_counterparty.port_id.to_string(),
				channel_id: inner_counterparty.channel_id.clone().unwrap().to_string(),
			}),
			connection_hops: inner_channel_end
				.connection_hops
				.iter()
				.map(|connection_id| connection_id.to_string())
				.collect(),
			version: inner_channel_end.version.to_string(),
		};
		Ok(QueryChannelResponse {
			channel: Some(channel_end),
			proof: borsh::to_vec(&channel_end_proof).unwrap(),
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_proof(&self, _at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		let trie = self.get_trie().await;
		let (_, proof) = trie
			.prove(&keys[0])
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		Ok(borsh::to_vec(&proof).unwrap())
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		let trie = self.get_trie().await;
		let new_port_id =
			ibc_new::core::host::types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_new::core::host::types::identifiers::ChannelId::new(channel_id.sequence());
		let new_seq = ibc_new::core::host::types::identifiers::Sequence::from(seq);
		let packet_commitment_path = ibc_new::core::host::types::path::CommitmentPath {
			port_id: new_port_id,
			channel_id: new_channel_id,
			sequence: new_seq,
		};
		let packet_commitment_trie_key = TrieKey::try_from(&packet_commitment_path).unwrap();
		let (packet_commitment, packet_commitment_proof) = trie
			.prove(&packet_commitment_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let commitment =
			packet_commitment.ok_or(Error::Custom("No value at given key".to_owned()))?;
		Ok(QueryPacketCommitmentResponse {
			commitment: commitment.0.to_vec(),
			proof: borsh::to_vec(&packet_commitment_proof).unwrap(),
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
		let trie = self.get_trie().await;
		let new_port_id =
			ibc_new::core::host::types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_new::core::host::types::identifiers::ChannelId::new(channel_id.sequence());
		let new_seq = ibc_new::core::host::types::identifiers::Sequence::from(seq);
		let packet_ack_path = ibc_new::core::host::types::path::AckPath {
			port_id: new_port_id,
			channel_id: new_channel_id,
			sequence: new_seq,
		};
		let packet_ack_trie_key = TrieKey::try_from(&packet_ack_path).unwrap();
		let (packet_ack, packet_ack_proof) = trie
			.prove(&packet_ack_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let ack = packet_ack.ok_or(Error::Custom("No value at given key".to_owned()))?;
		Ok(QueryPacketAcknowledgementResponse {
			acknowledgement: ack.0.to_vec(),
			proof: borsh::to_vec(&packet_ack_proof).unwrap(),
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let trie = self.get_trie().await;
		let storage = self.get_ibc_storage();
		let new_port_id =
			ibc_new::core::host::types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_new::core::host::types::identifiers::ChannelId::new(channel_id.sequence());
		let next_sequence_recv_path = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
		let next_sequence_recv_trie_key = TrieKey::for_next_sequence(&next_sequence_recv_path);
		let (_, next_sequence_recv_proof) = trie
			.prove(&next_sequence_recv_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let next_seq = storage
			.next_sequence
			.get(&next_sequence_recv_path)
			.ok_or(Error::Custom("No value at given key".to_owned()))?;
		let next_seq_recv = next_seq
			.get(SequenceTripleIdx::Recv)
			.ok_or(Error::Custom("No value set for the next sequence receive".to_owned()))?;
		Ok(QueryNextSequenceReceiveResponse {
			next_sequence_receive: next_seq_recv.into(),
			proof: borsh::to_vec(&next_sequence_recv_proof).unwrap(),
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		let trie = self.get_trie().await;
		let new_port_id =
			ibc_new::core::host::types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_new::core::host::types::identifiers::ChannelId::new(channel_id.sequence());
		let new_seq = ibc_new::core::host::types::identifiers::Sequence::from(seq);
		let packet_recv_path = ibc_new::core::host::types::path::ReceiptPath {
			port_id: new_port_id,
			channel_id: new_channel_id,
			sequence: new_seq,
		};
		let packet_recv_trie_key = TrieKey::try_from(&packet_recv_path).unwrap();
		let (packet_recv, packet_recv_proof) = trie
			.prove(&packet_recv_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		Ok(QueryPacketReceiptResponse {
			received: packet_recv.is_some(),
			proof: borsh::to_vec(&packet_recv_proof).unwrap(),
			proof_height: increment_proof_height(Some(at.into())),
		})
	}

	async fn latest_height_and_timestamp(
		&self,
	) -> Result<(Height, ibc::timestamp::Timestamp), Self::Error> {
		let rpc_client = self.rpc_client();
		let height = rpc_client.get_block_height().await.map_err(|e| Error::RpcError(serde_json::to_string(&e.kind.get_transaction_error().unwrap()).unwrap()))?;
		let epoch = rpc_client.get_epoch_info().await.map_err(|e| Error::RpcError(serde_json::to_string(&e.kind.get_transaction_error().unwrap()).unwrap()))?.epoch;
		let slot = rpc_client.get_slot().await.map_err(|e| Error::RpcError(serde_json::to_string(&e.kind.get_transaction_error().unwrap()).unwrap()))?;
		let timestamp = rpc_client.get_block_time(slot).await.map_err(|e| Error::RpcError(serde_json::to_string(&e.kind.get_transaction_error().unwrap()).unwrap()))?;
		Ok((Height::new(epoch, height), Timestamp::from_nanoseconds((timestamp * 10_i64.pow(9)).try_into().unwrap()).unwrap()))
	}

	async fn query_packet_commitments(
		&self,
		_at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> Result<Vec<u64>, Self::Error> {
		// let storage = self.get_ibc_storage();
		// let packet_commitment_sequence = storage
		// 	.packet_commitment_sequence_sets
		// 	.get(&(port_id.to_string(), channel_id.to_string()))
		// 	.ok_or("No value found at given key".to_owned())?;
		// Ok(packet_commitment_sequence.clone())
		todo!()
	}

	async fn query_packet_acknowledgements(
		&self,
		_at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> Result<Vec<u64>, Self::Error> {
		// let storage = self.get_ibc_storage();
		// let packet_acknowledgement_sequence = storage
		// 	.packet_acknowledgement_sequence_sets
		// 	.get(&(port_id.to_string(), channel_id.to_string()))
		// 	.ok_or("No value found at given key".to_owned())?;
		// Ok(packet_acknowledgement_sequence.clone())
		todo!()
	}

	async fn query_unreceived_packets(
		&self,
		at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		// let storage = self.get_ibc_storage();
		// let packet_receipt_sequences = storage
		// 	.packet_receipt_sequence_sets
		// 	.get(&(port_id.to_string(), channel_id.to_string()))
		// 	.ok_or("No value found at given key".to_owned())?;
		// Ok(seqs
		// 	.iter()
		// 	.flat_map(|&seq| {
		// 		match packet_receipt_sequences.iter().find(|&&receipt_seq| receipt_seq == seq) {
		// 			Some(_) => None,
		// 			None => Some(seq),
		// 		}
		// 	})
		// 	.collect())
		todo!()
	}

	async fn query_unreceived_acknowledgements(
		&self,
		at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		// let storage = self.get_ibc_storage();
		// let packet_ack_sequences = storage
		// 	.packet_acknowledgement_sequence_sets
		// 	.get(&(port_id.to_string(), channel_id.to_string()))
		// 	.ok_or("No value found at given key".to_owned())?;
		// Ok(seqs
		// 	.iter()
		// 	.flat_map(|&seq| match packet_ack_sequences.iter().find(|&&ack_seq| ack_seq == seq) {
		// 		Some(_) => None,
		// 		None => Some(seq),
		// 	})
		// 	.collect())
		todo!()
	}

	fn channel_whitelist(
		&self,
	) -> std::collections::HashSet<(
		ibc::core::ics24_host::identifier::ChannelId,
		ibc::core::ics24_host::identifier::PortId,
	)> {
		self.channel_whitelist.lock().unwrap().clone()
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
		// let packet_storage = self.get_packet_storage();
		// let packets = packet_storage.0;
		// let sent_packets: Vec<ibc_rpc::PacketInfo> = packets
		// 	.iter()
		// 	.filter_map(|packet| match packet {
		// 		ibc_new::core::ics04_channel::msgs::PacketMsg::Recv(recv_packet) => {
		// 			let packet = &recv_packet.packet;
		// 			let does_seq_exist = seqs.binary_search(&u64::from(packet.seq_on_a)).is_ok();
		// 			if packet.chan_id_on_a.to_string() != channel_id.to_string() ||
		// 				packet.port_id_on_a.to_string() != port_id.to_string() ||
		// 				!does_seq_exist
		// 			{
		// 				None
		// 			} else {
		// 				let timeout_height = match packet.timeout_height_on_b {
		// 					ibc_new::core::ics04_channel::timeout::TimeoutHeight::Never =>
		// 						ibc_proto::ibc::core::client::v1::Height {
		// 							revision_height: 0,
		// 							revision_number: 0,
		// 						},
		// 					ibc_new::core::ics04_channel::timeout::TimeoutHeight::At(height) =>
		// 						ibc_proto::ibc::core::client::v1::Height {
		// 							revision_height: height.revision_height(),
		// 							revision_number: height.revision_number(),
		// 						},
		// 				};
		// 				let packet_info = ibc_rpc::PacketInfo {
		// 					height: Some(recv_packet.proof_height_on_a.revision_height()),
		// 					sequence: u64::from(packet.seq_on_a),
		// 					source_port: packet.port_id_on_a.to_string(),
		// 					source_channel: packet.chan_id_on_a.to_string(),
		// 					destination_port: packet.port_id_on_b.to_string(),
		// 					destination_channel: packet.chan_id_on_b.to_string(),
		// 					channel_order: String::from("IDK"),
		// 					data: packet.data.clone(),
		// 					timeout_height,
		// 					timeout_timestamp: packet.timeout_timestamp_on_b.nanoseconds(),
		// 					ack: None,
		// 				};
		// 				Some(packet_info)
		// 			}
		// 		},
		// 		ibc_new::core::ics04_channel::msgs::PacketMsg::Ack(ack_packet) => {
		// 			let packet = &ack_packet.packet;
		// 			let does_seq_exist = seqs.binary_search(&u64::from(packet.seq_on_a)).is_ok();
		// 			if packet.chan_id_on_a.to_string() != channel_id.to_string() ||
		// 				packet.port_id_on_a.to_string() != port_id.to_string() ||
		// 				!does_seq_exist
		// 			{
		// 				None
		// 			} else {
		// 				let timeout_height = match packet.timeout_height_on_b {
		// 					ibc_new::core::ics04_channel::timeout::TimeoutHeight::Never =>
		// 						ibc_proto::ibc::core::client::v1::Height {
		// 							revision_height: 0,
		// 							revision_number: 0,
		// 						},
		// 					ibc_new::core::ics04_channel::timeout::TimeoutHeight::At(height) =>
		// 						ibc_proto::ibc::core::client::v1::Height {
		// 							revision_height: height.revision_height(),
		// 							revision_number: height.revision_number(),
		// 						},
		// 				};
		// 				let packet_info = ibc_rpc::PacketInfo {
		// 					height: Some(ack_packet.proof_height_on_b.revision_height()),
		// 					sequence: u64::from(packet.seq_on_a),
		// 					source_port: packet.port_id_on_a.to_string(),
		// 					source_channel: packet.chan_id_on_a.to_string(),
		// 					destination_port: packet.port_id_on_b.to_string(),
		// 					destination_channel: packet.chan_id_on_b.to_string(),
		// 					channel_order: String::from("IDK"),
		// 					data: packet.data.clone(),
		// 					timeout_height,
		// 					timeout_timestamp: packet.timeout_timestamp_on_b.nanoseconds(),
		// 					ack: Some(ack_packet.acknowledgement.as_ref().to_vec()),
		// 				};
		// 				Some(packet_info)
		// 			}
		// 		},
		// 		_ => None,
		// 	})
		// 	.collect();
		// Ok(sent_packets)
	}

	fn expected_block_time(&self) -> Duration {
		// solana block time is roughly 400 milliseconds
		Duration::from_millis(400)
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, ibc::timestamp::Timestamp), Self::Error> {
		let storage = self.get_ibc_storage();
		let client_store = storage
			.clients
			.iter()
			.find(|&client| client.client_id.as_str() == client_id.as_str())
			.ok_or("Client not found with the given client id".to_owned())?;
		let inner_client_height = ibc_new::core::client::types::Height::new(
			client_height.revision_number,
			client_height.revision_height,
		)
		.unwrap();
		let height = client_store
			.processed_heights
			.get(&inner_client_height)
			.ok_or("No host height found with the given height".to_owned())?;
		let timestamp = client_store
			.processed_times
			.get(&inner_client_height)
			.ok_or("No timestamp found with the given height".to_owned())?;
		Ok((
			Height {
				revision_height: height.revision_height(),
				revision_number: height.revision_number(),
			},
			Timestamp::from_nanoseconds(*timestamp).unwrap(),
		))
	}

	async fn query_host_consensus_state_proof(
		&self,
		client_state: &pallet_ibc::light_clients::AnyClientState,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		let trie = self.get_trie().await;
		let height = client_state.latest_height();
		let client_id = self.client_id();
		let client_type = self.client_type();
		let new_client_id =
			ibc_new::core::host::types::identifiers::ClientId::from_str(client_id.as_str())
				.unwrap();
		let client_idx = ClientIdx::try_from(new_client_id).unwrap();
		let consensus_state_trie_key = TrieKey::for_consensus_state(
			client_idx,
			ibc_new::core::client::types::Height::new(
				height.revision_number,
				height.revision_height,
			)
			.unwrap(),
		);
		let (_, host_consensus_state_proof) = trie
			.prove(&consensus_state_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		Ok(Some(borsh::to_vec(&host_consensus_state_proof).unwrap()))
	}

	async fn query_ibc_balance(
		&self,
		asset_id: Self::AssetId,
	) -> Result<Vec<ibc::applications::transfer::PrefixedCoin>, Self::Error> {
		let denom = &asset_id;
		let (token_mint_key, _bump) =
			Pubkey::find_program_address(&[denom.as_ref()], &solana_ibc::ID);
		let user_token_address =
			get_associated_token_address(&self.keybase.public_key, &token_mint_key);
		let sol_rpc_client = self.rpc_client();
		let balance = sol_rpc_client.get_token_account_balance(&user_token_address).await.unwrap();
		Ok(vec![PrefixedCoin {
			denom: PrefixedDenom {
				trace_path: TracePath::default(),
				base_denom: BaseDenom::from_str(denom).unwrap(),
			},
			amount: Amount::from_str(&balance.ui_amount_string).unwrap(),
		}])
	}

	fn connection_prefix(&self) -> ibc::core::ics23_commitment::commitment::CommitmentPrefix {
		self.commitment_prefix.clone()
	}

	fn client_id(&self) -> ClientId {
		self.client_id.clone().expect("No client ID found")
	}

	fn set_client_id(&mut self, client_id: ClientId) {
		self.client_id = Some(client_id);
	}

	fn connection_id(&self) -> Option<ConnectionId> {
		self.connection_id.clone()
	}

	fn set_channel_whitelist(
		&mut self,
		channel_whitelist: std::collections::HashSet<(
			ibc::core::ics24_host::identifier::ChannelId,
			ibc::core::ics24_host::identifier::PortId,
		)>,
	) {
		*self.channel_whitelist.lock().unwrap() = channel_whitelist;
	}

	fn add_channel_to_whitelist(
		&mut self,
		channel: (
			ibc::core::ics24_host::identifier::ChannelId,
			ibc::core::ics24_host::identifier::PortId,
		),
	) {
		self.channel_whitelist.lock().unwrap().insert(channel);
	}

	fn set_connection_id(&mut self, connection_id: ConnectionId) {
		self.connection_id = Some(connection_id)
	}

	fn client_type(&self) -> ibc::core::ics02_client::client_state::ClientType {
		self.client_type.clone()
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		let rpc_client = self.rpc_client();
		let timestamp = rpc_client.get_block_time(block_number.into()).await.map_err(|e| Error::RpcError(serde_json::to_string(&e.kind.get_transaction_error().unwrap()).unwrap()))?;
		Ok(timestamp as u64)
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		let storage = self.get_ibc_storage();
		let client_ids: Vec<ClientId> = storage
			.clients
			.iter()
			.map(|client| ClientId::from_str(&client.client_id.to_string()).unwrap())
			.collect();
		Ok(client_ids)
	}

	async fn query_channels(
		&self,
	) -> Result<
		Vec<(
			ibc::core::ics24_host::identifier::ChannelId,
			ibc::core::ics24_host::identifier::PortId,
		)>,
		Self::Error,
	> {
		let storage = self.get_ibc_storage();
		let channels: Vec<(ChannelId, PortId)> = BTreeMap::keys(&storage.channel_ends)
			.map(|channel_end| {
				(
					ChannelId::from_str(&channel_end.channel_id().as_str()).unwrap(),
					PortId::from_str(&channel_end.port_id().as_str()).unwrap(),
				)
			})
			.collect();
		Ok(channels)
	}

	async fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error> {
		let storage = self.get_ibc_storage();
		let client_id_key =
			ibc_new::core::host::types::identifiers::ClientId::from_str(&client_id).unwrap();
		let mut index = -1;
		let connections: Vec<IdentifiedConnection> = storage
			.connections
			.iter()
			.filter_map(|serialized_connection| {
				index += 1;
				let connection = serialized_connection.get().unwrap();
				if connection.client_id_matches(&client_id_key) {
					let versions: Vec<Version> = connection
						.versions()
						.iter()
						.map(|version| {
							let proto_version =
								ibc_proto_new::ibc::core::connection::v1::Version::from(
									version.clone(),
								);
							Version {
								identifier: proto_version.identifier,
								features: proto_version.features,
							}
						})
						.collect();
					let counterparty = connection.counterparty();
					Some(IdentifiedConnection {
						id: format!("{}-{}", ConnectionId::prefix(), index),
						client_id: client_id.clone(),
						versions,
						state: i32::from(connection.state().clone()),
						counterparty: Some(ConnCounterparty {
							client_id: counterparty.client_id.to_string(),
							connection_id: counterparty
								.connection_id()
								.map_or_else(|| "".to_string(), |v| v.as_str().to_string()),
							prefix: Some(ibc_proto::ibc::core::commitment::v1::MerklePrefix {
								key_prefix: counterparty.prefix.clone().into_vec(),
							}),
						}),
						delay_period: connection.delay_period().as_secs(),
					});
				}
				None
			})
			.collect();
		Ok(connections)
	}

	async fn is_update_required(
		&self,
		latest_height: u64,
		latest_client_height_on_counterparty: u64,
	) -> Result<bool, Self::Error> {
		// we never need to use LightClientSync trait in this case, because
		// all the events will be eventually submitted via `finality_notifications`
		Ok(false)
	}

	async fn initialize_client_state(
		&self,
	) -> Result<
		(pallet_ibc::light_clients::AnyClientState, pallet_ibc::light_clients::AnyConsensusState),
		Self::Error,
	> {
		todo!()
	}

	async fn query_client_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ClientId, Self::Error> {
		let program = self.program();
		let signature = Signature::from_str(&tx_id).unwrap();
		let sol_rpc_client: RpcClient = program.rpc();
		let tx = sol_rpc_client.get_transaction(&signature, UiTransactionEncoding::Json).unwrap();
		let logs = match tx.transaction.meta.unwrap().log_messages {
			solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
			solana_transaction_status::option_serializer::OptionSerializer::None =>
				return Err(Error::Custom(String::from("No logs found"))),
			solana_transaction_status::option_serializer::OptionSerializer::Skip =>
				return Err(Error::Custom(String::from("Logs were skipped, so not available"))),
		};
		let events = get_events_from_logs(logs);
		let result: Vec<&ibc_new::core::client::types::events::CreateClient> = events
			.iter()
			.filter_map(|event| match event {
				ibc_new::core::handler::types::events::IbcEvent::CreateClient(e) => Some(e),
				_ => None,
			})
			.collect();
		if result.len() != 1 {
			return Err(Error::Custom(format!(
				"Expected exactly one CreateClient event, found {}",
				result.len()
			)))
		}
		let client_id = result[0].client_id();
		Ok(ClientId::from_str(client_id.as_str()).unwrap())
	}

	async fn query_connection_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ConnectionId, Self::Error> {
		let program = self.program();
		let signature = Signature::from_str(&tx_id).unwrap();
		let sol_rpc_client: RpcClient = program.rpc();
		let tx = sol_rpc_client.get_transaction(&signature, UiTransactionEncoding::Json).unwrap();
		let logs = match tx.transaction.meta.unwrap().log_messages {
			solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
			solana_transaction_status::option_serializer::OptionSerializer::None =>
				return Err(Error::Custom(String::from("No logs found"))),
			solana_transaction_status::option_serializer::OptionSerializer::Skip =>
				return Err(Error::Custom(String::from("Logs were skipped, so not available"))),
		};
		let events = get_events_from_logs(logs);
		let result: Vec<&ibc_new::core::connection::types::events::OpenInit> = events
			.iter()
			.filter_map(|event| match event {
				ibc_new::core::handler::types::events::IbcEvent::OpenInitConnection(e) => Some(e),
				_ => None,
			})
			.collect();
		if result.len() != 1 {
			return Err(Error::Custom(format!(
				"Expected exactly one OpenInitConnection event, found {}",
				result.len()
			)))
		}
		let connection_id = result[0].conn_id_on_a();
		Ok(ConnectionId::from_str(connection_id.as_str()).unwrap())
	}

	async fn query_channel_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<
		(ibc::core::ics24_host::identifier::ChannelId, ibc::core::ics24_host::identifier::PortId),
		Self::Error,
	> {
		let program = self.program();
		let signature = Signature::from_str(&tx_id).unwrap();
		let sol_rpc_client: RpcClient = program.rpc();
		let tx = sol_rpc_client.get_transaction(&signature, UiTransactionEncoding::Json).unwrap();
		let logs = match tx.transaction.meta.unwrap().log_messages {
			solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
			solana_transaction_status::option_serializer::OptionSerializer::None =>
				return Err(Error::Custom(String::from("No logs found"))),
			solana_transaction_status::option_serializer::OptionSerializer::Skip =>
				return Err(Error::Custom(String::from("Logs were skipped, so not available"))),
		};
		let events = get_events_from_logs(logs);
		let result: Vec<&ibc_new::core::channel::types::events::OpenInit> = events
			.iter()
			.filter_map(|event| match event {
				ibc_new::core::handler::types::events::IbcEvent::OpenInitChannel(e) => Some(e),
				_ => None,
			})
			.collect();
		if result.len() != 1 {
			return Err(Error::Custom(format!(
				"Expected exactly one OpenInitChannel event, found {}",
				result.len()
			)))
		}
		let channel_id = result[0].chan_id_on_a();
		let port_id = result[0].port_id_on_a();
		Ok((
			ChannelId::from_str(channel_id.as_str()).unwrap(),
			PortId::from_str(port_id.as_str()).unwrap(),
		))
	}

	async fn upload_wasm(&self, wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
		todo!()
	}
}

impl KeyProvider for Client {
	fn account_id(&self) -> ibc::signer::Signer {
		let key_entry = &self.keybase;
		let public_key = key_entry.public_key;
		ibc::signer::Signer::from_str(&public_key.to_string()).unwrap()
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
		let keypair = self.keybase.keypair();
		let authority = Rc::new(keypair);
		let program = self.program();

		// Build, sign, and send program instruction
		let solana_ibc_storage_key = self.get_ibc_storage_key();
		let trie_key = self.get_trie_key();
		let chain_key = self.get_chain_key();

		let my_message = Ics26Envelope::<LocalClientTypes>::try_from(messages[0].clone()).unwrap();
		let messages = convert_old_msgs_to_new(vec![my_message]);

		// let all_messages = MsgEnvelope::try_from(messages[0].clone()).unwrap();
		// .into_iter()

		let sig: Signature = program
			.request()
			.accounts(solana_ibc::accounts::Deliver {
				sender: authority.pubkey(),
				storage: solana_ibc_storage_key,
				trie: trie_key,
				chain: chain_key,
				system_program: system_program::ID,
			})
			.args(solana_ibc::instruction::Deliver { message: messages[0].clone() })
			.payer(authority.clone())
			.signer(&*authority)
			.send_with_spinner_and_config(RpcSendTransactionConfig {
				skip_preflight: true,
				..RpcSendTransactionConfig::default()
			})
			.unwrap();
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
		let err_str = if let Some(rpc_err) = error.downcast_ref::<Error>() {
			match rpc_err {
				Error::RpcError(s) => s.clone(),
				_ => "".to_string(),
			}
		} else {
			error.to_string()
		};
		log::debug!(target: "hyperspace_solana", "Handling error: {err_str}");
		if err_str.contains("dispatch task is gone") ||
			err_str.contains("failed to send message to internal channel")
		{
			self.reconnect().await?;
			self.common_state.rpc_call_delay *= 2;
		}

		Ok(())
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

fn increment_proof_height(
	height: Option<ibc_proto::ibc::core::client::v1::Height>,
) -> Option<ibc_proto::ibc::core::client::v1::Height> {
	height.map(|height| ibc_proto::ibc::core::client::v1::Height {
		revision_height: height.revision_height + 1,
		..height
	})
}

fn get_events_from_logs(logs: Vec<String>) -> Vec<ibc_new::core::handler::types::events::IbcEvent> {
	let serialized_events: Vec<&str> = logs
		.iter()
		.filter_map(|log| {
			if log.starts_with("Program data: ") {
				Some(log.strip_prefix("Program data: ").unwrap())
			} else {
				None
			}
		})
		.collect();
	let events: Vec<ibc_new::core::handler::types::events::IbcEvent> = serialized_events
		.iter()
		.filter_map(|event| {
			let decoded_event = base64::prelude::BASE64_STANDARD.decode(event).unwrap();
			let decoded_event: solana_ibc::events::Event =
				borsh::BorshDeserialize::try_from_slice(&decoded_event).unwrap();
			match decoded_event {
				solana_ibc::events::Event::IbcEvent(e) => Some(e),
				_ => None,
			}
		})
		.collect();
	events
}

#[test]
pub fn test_storage_deserialization() {
	println!("How is this test, do you like it?");
	let authority = Rc::new(Keypair::new());
	let client = AnchorClient::new_with_options(
		Cluster::Localnet,
		authority.clone(),
		CommitmentConfig::processed(),
	);
	let program = client.program(ID).unwrap();

	let storage = Pubkey::find_program_address(&[SOLANA_IBC_STORAGE_SEED], &ID).0;
	// println!("THis is the sotrage key {} {}", storage, ID);
	let solana_ibc_storage_account: PrivateStorage = program.account(storage).unwrap();
	// println!("This is the storage account {:?} {}", solana_ibc_storage_account, ID);
	let serialized_consensus_state = solana_ibc_storage_account.clients[0]
		.consensus_states
		.get(&ibc_new::core::client::types::Height::new(0, 1).unwrap())
		.ok_or(Error::Custom("No value at given key".to_owned()))
		.unwrap();
	let serialized_connection_end = &solana_ibc_storage_account.connections[0];
	let connection_end = Serialised::get(serialized_connection_end).unwrap();
	let in_vec = serialized_consensus_state.try_to_vec().unwrap();
	// println!("This is invec {:?}", in_vec);

	let rpc_client = program.rpc();

	let signature = Signature::from_str("3dAyQEVTz7RpousUXWGfdunb8vxJgeehXLwQRB3gX7ngovQswhFZJuvjq49YLPpg53k5tLHG44vgK32BRBhesJJh").unwrap();
	let tx = rpc_client.get_transaction(&signature, UiTransactionEncoding::Json).unwrap();
	let logs = match tx.transaction.meta.unwrap().log_messages {
    solana_transaction_status::option_serializer::OptionSerializer::Some(e) => e,
		_ => panic!(),
	};
	let events = get_events_from_logs(logs);
	println!("THis is new 1 event {:?}", events[1].clone());
	let old_event = convert_new_event_to_old(events[1].clone());
	println!("THis is old 1 event {:?}", old_event);

}
