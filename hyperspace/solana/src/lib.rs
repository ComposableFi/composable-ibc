#![feature(more_qualified_paths)]
extern crate alloc;

use anchor_client::solana_sdk::transaction::Transaction;
use anchor_spl::associated_token::get_associated_token_address;
use client::FinalityEvent;
use client_state::convert_new_client_state_to_old;
use consensus_state::convert_new_consensus_state_to_old;
use core::{pin::Pin, str::FromStr, time::Duration};
use ibc_core_channel_types::msgs::PacketMsg;
use ibc_core_client_types::msgs::ClientMsg;
use ibc_core_handler_types::msgs::MsgEnvelope;
use ics07_tendermint::{
	client_message::ClientMessage, client_state::ClientState as TmClientState,
	consensus_state::ConsensusState as TmConsensusState,
};
use msgs::convert_old_msgs_to_new;
use solana_transaction_status::UiTransactionEncoding;
use std::ops::Deref;
use tendermint::{Hash, Time};
use tendermint_proto::Protobuf;
use tokio::sync::mpsc::unbounded_channel;


use anchor_client::{
	solana_client::{
		pubsub_client::PubsubClient,
		rpc_client::GetConfirmedSignaturesForAddress2Config,
		rpc_config::{
			RpcBlockSubscribeConfig, RpcBlockSubscribeFilter, RpcTransactionLogsConfig,
			RpcTransactionLogsFilter,
		},
	},
	solana_sdk::{
		commitment_config::CommitmentConfig, signature::Signature, signer::Signer as AnchorSigner,
	},
};
use anchor_lang::prelude::*;
use error::Error;
use ibc::{
	applications::transfer::{Amount, BaseDenom, PrefixedCoin, PrefixedDenom, TracePath},
	core::{
		ics02_client::{
			events::UpdateClient, msgs::update_client::MsgUpdateAnyClient,
			trust_threshold::TrustThreshold,
		},
		ics23_commitment::specs::ProofSpecs,
		ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::msgs::Ics26Envelope,
	},
	events::IbcEvent,
	timestamp::Timestamp,
	tx_msg::Msg,
	Height,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			Channel, Counterparty as ChanCounterparty, IdentifiedChannel, QueryChannelResponse,
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
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState, AnyConsensusState};
use primitives::{
	mock::LocalClientTypes, Chain, CommonClientState, IbcProvider, KeyProvider, LightClientSync,
	MisbehaviourHandler, UndeliveredType, UpdateType,
};
use std::{result::Result, sync::Arc};
use tokio_stream::Stream;

use solana_ibc::storage::{SequenceKind, Serialised};

use trie_ids::{ClientIdx, ConnectionIdx, PortChannelPK, Tag, TrieKey};

pub use crate::{
	client::{DeliverIxType, SolanaClient, SolanaClientConfig},
	events::convert_new_event_to_old,
};

pub mod client;
mod client_state;
mod consensus_state;
mod error;
mod events;
mod msgs;
#[cfg(feature = "testing")]
mod test_provider;
mod utils;

const SOLANA_IBC_STORAGE_SEED: &[u8] = b"private";
const TRIE_SEED: &[u8] = b"trie";
const CHAIN_SEED: &[u8] = b"chain";
pub const NUMBER_OF_BLOCKS_TO_PROCESS_PER_ITER: u64 = 250;
pub const WRITE_ACCOUNT_SEED: &[u8] = b"write";

pub struct InnerAny {
	pub type_url: String,
	pub value: Vec<u8>,
}

#[async_trait::async_trait]
impl IbcProvider for SolanaClient {
	type FinalityEvent = FinalityEvent;

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
		log::info!("Came into solana lts events");
		let (_finality_event_height, _finality_event_timestamp, _finality_event_slot) =
			match finality_event {
				FinalityEvent::Tendermint {
					previous_blockhash: _,
					blockhash: _,
					height,
					timestamp,
					slot,
				} => (height, timestamp, slot),
			};
		let client_id = self.client_id();
		let latest_cp_height = counterparty.latest_height_and_timestamp().await?.0;
		log::info!("this is the latest cp height {:?}", latest_cp_height);
		let latest_cp_client_state =
			counterparty.query_client_state(latest_cp_height, client_id.clone()).await?;
		let client_state_response = latest_cp_client_state
			.client_state
			.ok_or_else(|| Error::Custom("counterparty returned empty client state".to_string()))?;
		log::info!("This is the type url in solana {:?}", client_state_response.type_url);
		let AnyClientState::Tendermint(client_state) =
			AnyClientState::decode_recursive(client_state_response, |c| {
				matches!(c, AnyClientState::Tendermint(_))
			})
			.ok_or_else(|| Error::Custom(format!("Could not decode client state")))?
		else {
			unreachable!()
		};
		let latest_cp_client_height = client_state.latest_height().revision_height;
		println!("This is counterparty client height {:?}", latest_cp_client_height);
		let latest_height = self.latest_height_and_timestamp().await?.0;
		let mut block_events: Vec<(u64, Vec<IbcEvent>)> = Vec::new();
		block_events.push((0, Vec::new()));
		let rpc_client = self.rpc_client();
		let sigs = rpc_client
			.get_signatures_for_address(&solana_ibc::ID)
			.await
			.map_err(|e| Error::RpcError(format!("{:?}", e)))?;
		for sig in sigs {
			if sig.slot < latest_cp_client_height {
				break
			}
			let signature = Signature::from_str(&sig.signature).unwrap();
			let tx = rpc_client
				.get_transaction(&signature, UiTransactionEncoding::Json)
				.await
				.unwrap();
			let logs = match tx.transaction.meta.unwrap().log_messages {
				solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
				solana_transaction_status::option_serializer::OptionSerializer::None =>
					return Err(Error::Custom(String::from("No logs found")).into()),
				solana_transaction_status::option_serializer::OptionSerializer::Skip =>
					return Err(
						Error::Custom(String::from("Logs were skipped, so not available")).into()
					),
			};
			let events = events::get_events_from_logs(logs);
			let converted_events = events
				.iter()
				.filter_map(|event| {
					convert_new_event_to_old(event.clone(), client_state.latest_height())
				})
				.collect();
			block_events.push((sig.slot, converted_events));
		}

		let updates: Vec<_> = block_events
			.iter()
			.map(|event| {
				let mut header =
					ics07_tendermint::client_message::test_util::get_dummy_ics07_header();
				header.signed_header.header.height =
					tendermint::block::Height::try_from(latest_height.revision_height).unwrap();
				header.signed_header.commit.height =
					tendermint::block::Height::try_from(latest_height.revision_height).unwrap();
				header.trusted_height = Height::new(1, latest_height.revision_height);
				let msg = MsgUpdateAnyClient::<LocalClientTypes> {
					client_id: self.client_id(),
					client_message: AnyClientMessage::Tendermint(ClientMessage::Header(
						header.clone(),
					)),
					signer: counterparty.account_id(),
				};
				let value = msg
					.encode_vec()
					.map_err(|e| {
						Error::from(format!("Failed to encode MsgUpdateClient {msg:?}: {e:?}"))
					})
					.unwrap();
				(
					Any { type_url: msg.type_url(), value },
					Height::new(1, latest_height.revision_height),
					event.1.clone(),
					if event.1.len() > 0 { UpdateType::Mandatory } else { UpdateType::Optional },
				)
			})
			.collect();

		log::info!("These are latest event updates");
		Ok(updates)
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
		let (tx, rx) = unbounded_channel();
		let ws_url = self.ws_url.clone();
		tokio::task::spawn_blocking(move || {
			let (_logs_subscription, receiver) = PubsubClient::logs_subscribe(
				&ws_url,
				RpcTransactionLogsFilter::Mentions(vec![solana_ibc::ID.to_string()]),
				RpcTransactionLogsConfig { commitment: Some(CommitmentConfig::processed()) },
			)
			.unwrap();

			loop {
				match receiver.recv() {
					Ok(logs) => {
						let events = events::get_events_from_logs(logs.value.logs);
						log::info!("These are events {:?} ", events);
						log::info!("Total {:?} events", events.len());
						let mut broke = false;
						events.iter().for_each(|event| {
							log::info!("Came into ibc events");
							let height = Height::new(1, 100);
							let converted_event =
								events::convert_new_event_to_old(event.clone(), height);
							if let Some(event) = converted_event {
								log::info!("Sending message");
								tx.send(event.clone()).unwrap_or_else(|_| {
									log::info!("Broke");
									broke = true;
								});
							}
						});
						if broke {
							break
						}
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

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let trie = self.get_trie().await;
		let storage = self.get_ibc_storage().await;
		let revision_height = consensus_height.revision_height;
		let revision_number = consensus_height.revision_number;
		let new_client_id =
			ibc_core_host_types::identifiers::ClientId::from_str(client_id.as_str()).unwrap();
		let consensus_state_trie_key = TrieKey::for_consensus_state(
			ClientIdx::try_from(new_client_id).unwrap(),
			ibc_core_client_types::Height::new(
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
			.ok_or(
				"Client not found with the given client id while querying client consensus"
					.to_owned(),
			)?;
		let serialized_consensus_state = client_store
			.consensus_states
			.get(&ibc_core_client_types::Height::new(revision_number, revision_height).unwrap())
			.ok_or(Error::Custom("No value at given key".to_owned()))?;
		let consensus_state = serialized_consensus_state
			.state()
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
			proof_height: Some(at.into()),
		})
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		log::info!("Quering solana client state at height {:?} {:?}", at, client_id);
		let trie = self.get_trie().await;
		let storage = self.get_ibc_storage().await;
		let new_client_id =
			ibc_core_host_types::identifiers::ClientId::from_str(client_id.as_str()).unwrap();
		let client_state_trie_key =
			TrieKey::for_client_state(ClientIdx::try_from(new_client_id).unwrap());
		let (_, client_state_proof) = trie
			.prove(&client_state_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let client_store = storage
			.clients
			.iter()
			.find(|&client| client.client_id.as_str() == client_id.as_str())
			.ok_or(
				"Client not found with the given client id while querying client state".to_owned(),
			)?;
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
			proof_height: Some(at.into()),
		})
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		let trie = self.get_trie().await;
		let storage = self.get_ibc_storage().await;
		let connection_idx = ConnectionIdx::try_from(
			ibc_core_host_types::identifiers::ConnectionId::from_str(connection_id.as_str())
				.unwrap(),
		)
		.unwrap();
		log::info!(
			"This is connection ID {:?} and index {:?} while querying connection end",
			connection_id,
			connection_idx
		);
		let connection_end_trie_key = TrieKey::for_connection(connection_idx);
		let (_, connection_end_proof) = trie
			.prove(&connection_end_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		log::info!("This is serialized connection {:?}", storage.connections);
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
				connection_id: inner_counterparty
					.connection_id
					.as_ref()
					.map_or_else(|| "".to_string(), |v| v.as_str().to_string()),
				prefix: Some(ibc_proto::ibc::core::commitment::v1::MerklePrefix {
					key_prefix: inner_counterparty.prefix().clone().into_vec(),
				}),
			}),
			delay_period: inner_connection_end.delay_period().as_secs(),
		};
		Ok(QueryConnectionResponse {
			connection: Some(connection_end),
			proof: borsh::to_vec(&connection_end_proof).unwrap(),
			proof_height: Some(at.into()),
		})
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let trie = self.get_trie().await;
		let storage = self.get_ibc_storage().await;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let channel_end_path =
			PortChannelPK::try_from(new_port_id.clone(), new_channel_id.clone()).unwrap();
		let channel_end_trie_key = TrieKey::for_channel_end(&channel_end_path);
		let (_, channel_end_proof) = trie
			.prove(&channel_end_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let inner_channel_end = storage
			.port_channel
			.deref()
			.get(&channel_end_path)
			.ok_or(Error::Custom("No value at given key".to_owned()))?
			.channel_end()
			.unwrap()
			.ok_or(Error::Custom("Channel end not found".to_owned()))?;
		let inner_counterparty = inner_channel_end.counterparty();
		let state = match inner_channel_end.state {
			ibc_core_channel_types::channel::State::Uninitialized => 0,
			ibc_core_channel_types::channel::State::Init => 1,
			ibc_core_channel_types::channel::State::TryOpen => 2,
			ibc_core_channel_types::channel::State::Open => 3,
			ibc_core_channel_types::channel::State::Closed => 4,
		};
		let ordering = match inner_channel_end.ordering {
			ibc_core_channel_types::channel::Order::None => 0,
			ibc_core_channel_types::channel::Order::Unordered => 1,
			ibc_core_channel_types::channel::Order::Ordered => 2,
		};
		let channel_end = Channel {
			state,
			ordering,
			counterparty: Some(ChanCounterparty {
				port_id: inner_counterparty.port_id.to_string(),
				channel_id: if inner_counterparty.channel_id.is_none() {
					"".to_owned()
				} else {
					inner_counterparty.channel_id.clone().unwrap().to_string()
				},
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
			proof_height: Some(at.into()),
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
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let new_seq = ibc_core_host_types::identifiers::Sequence::from(seq);
		let packet_commitment_path = ibc_core_host_types::path::CommitmentPath {
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
			proof_height: Some(at.into()),
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
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let new_seq = ibc_core_host_types::identifiers::Sequence::from(seq);
		let packet_ack_path = ibc_core_host_types::path::AckPath {
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
			proof_height: Some(at.into()),
		})
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let trie = self.get_trie().await;
		let storage = self.get_ibc_storage().await;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let next_sequence_recv_path = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
		let next_sequence_recv_trie_key = TrieKey::for_next_sequence(&next_sequence_recv_path);
		let (_, next_sequence_recv_proof) = trie
			.prove(&next_sequence_recv_trie_key)
			.map_err(|_| Error::Custom("value is sealed and cannot be fetched".to_owned()))?;
		let next_seq = &storage
			.port_channel
			.deref()
			.get(&next_sequence_recv_path)
			.ok_or(Error::Custom("No value at given key".to_owned()))?
			.next_sequence;
		let next_seq_recv = next_seq
			.get(SequenceKind::Recv)
			.ok_or(Error::Custom("No value set for the next sequence receive".to_owned()))?;
		Ok(QueryNextSequenceReceiveResponse {
			next_sequence_receive: next_seq_recv.into(),
			proof: borsh::to_vec(&next_sequence_recv_proof).unwrap(),
			proof_height: Some(at.into()),
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
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let new_seq = ibc_core_host_types::identifiers::Sequence::from(seq);
		let packet_recv_path = ibc_core_host_types::path::ReceiptPath {
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
			proof_height: Some(at.into()),
		})
	}

	async fn latest_height_and_timestamp(
		&self,
	) -> Result<(Height, ibc::timestamp::Timestamp), Self::Error> {
		let rpc_client = self.rpc_client();
		let chain = self.get_chain_storage().await;
		let _height: u64 = chain.head().unwrap().block_height.into();
		let slot = rpc_client.get_slot().await.map_err(|e| {
			Error::RpcError(
				serde_json::to_string(&e.kind.get_transaction_error().unwrap()).unwrap(),
			)
		})?;
		let timestamp = rpc_client.get_block_time(slot).await.map_err(|e| {
			Error::RpcError(
				serde_json::to_string(&e.kind.get_transaction_error().unwrap()).unwrap(),
			)
		})?;
		Ok((
			Height::new(1, slot),
			Timestamp::from_nanoseconds((timestamp * 10_i64.pow(9)).try_into().unwrap()).unwrap(),
		))
	}

	async fn query_packet_commitments(
		&self,
		_at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let trie = self.get_trie().await;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let trie_comp = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
		let key = TrieKey::new(Tag::Commitment, trie_comp);
		let sequences: Vec<u64> = trie
			.get_subtrie(&key)
			.unwrap()
			.iter()
			.map(|c| {
				u64::from_be_bytes(
					Vec::<u8>::try_from(c.sub_key.clone()).unwrap().as_slice().try_into().unwrap(),
				)
			})
			.collect();
		Ok(sequences)
	}

	async fn query_packet_acknowledgements(
		&self,
		_at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let trie = self.get_trie().await;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let trie_comp = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
		let key = TrieKey::new(Tag::Ack, trie_comp);
		let sequences: Vec<u64> = trie
			.get_subtrie(&key)
			.unwrap()
			.iter()
			.map(|c| {
				u64::from_be_bytes(
					Vec::<u8>::try_from(c.sub_key.clone()).unwrap().as_slice().try_into().unwrap(),
				)
			})
			.collect();
		Ok(sequences)
	}

	async fn query_unreceived_packets(
		&self,
		_at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let trie = self.get_trie().await;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let trie_comp = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
		let key = TrieKey::new(Tag::Receipt, trie_comp);
		let packet_receipt_sequences: Vec<u64> = trie
			.get_subtrie(&key)
			.unwrap()
			.iter()
			.map(|c| {
				u64::from_be_bytes(
					Vec::<u8>::try_from(c.sub_key.clone()).unwrap().as_slice().try_into().unwrap(),
				)
			})
			.collect();
		Ok(seqs
			.iter()
			.flat_map(|&seq| {
				match packet_receipt_sequences.iter().find(|&&receipt_seq| receipt_seq == seq) {
					Some(_) => None,
					None => Some(seq),
				}
			})
			.collect())
	}

	async fn query_unreceived_acknowledgements(
		&self,
		_at: Height,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let trie = self.get_trie().await;
		let new_port_id =
			ibc_core_host_types::identifiers::PortId::from_str(port_id.as_str()).unwrap();
		let new_channel_id =
			ibc_core_host_types::identifiers::ChannelId::new(channel_id.sequence());
		let trie_comp = PortChannelPK::try_from(new_port_id, new_channel_id).unwrap();
		let key = TrieKey::new(Tag::Ack, trie_comp);
		let packet_receipt_sequences: Vec<u64> = trie
			.get_subtrie(&key)
			.unwrap()
			.iter()
			.map(|c| {
				u64::from_be_bytes(
					Vec::<u8>::try_from(c.sub_key.clone()).unwrap().as_slice().try_into().unwrap(),
				)
			})
			.collect();
		Ok(seqs
			.iter()
			.flat_map(|&seq| {
				match packet_receipt_sequences.iter().find(|&&receipt_seq| receipt_seq == seq) {
					Some(_) => None,
					None => Some(seq),
				}
			})
			.collect())
	}

	fn channel_whitelist(
		&self,
	) -> std::collections::HashSet<(
		ibc::core::ics24_host::identifier::ChannelId,
		ibc::core::ics24_host::identifier::PortId,
	)> {
		self.channel_whitelist.lock().unwrap().clone()
	}

	/// We just return all the channels since there doesnt seem to be any kind of relation between
	/// connection ID and channels.
	async fn query_connection_channels(
		&self,
		at: Height,
		_connection_id: &ConnectionId,
	) -> Result<ibc_proto::ibc::core::channel::v1::QueryChannelsResponse, Self::Error> {
		let storage = self.get_ibc_storage().await;
		let channels: Vec<IdentifiedChannel> = storage
			.port_channel
			.deref()
			.into_iter()
			.map(|(key, value)| {
				let channel = value.channel_end().unwrap().unwrap();
				let state = match channel.state {
					ibc_core_channel_types::channel::State::Uninitialized => 0,
					ibc_core_channel_types::channel::State::Init => 1,
					ibc_core_channel_types::channel::State::TryOpen => 2,
					ibc_core_channel_types::channel::State::Open => 3,
					ibc_core_channel_types::channel::State::Closed => 4,
				};
				let ordering = match channel.ordering {
					ibc_core_channel_types::channel::Order::None => 0,
					ibc_core_channel_types::channel::Order::Unordered => 1,
					ibc_core_channel_types::channel::Order::Ordered => 2,
				};
				IdentifiedChannel {
					state,
					ordering,
					counterparty: Some(ChanCounterparty {
						port_id: channel.counterparty().port_id.to_string(),
						channel_id: channel.counterparty().channel_id.clone().unwrap().to_string(),
					}),
					connection_hops: channel
						.connection_hops
						.iter()
						.map(|connection_id| connection_id.to_string())
						.collect(),
					version: channel.version.to_string(),
					port_id: key.port_id().to_string(),
					channel_id: key.channel_id().to_string(),
				}
			})
			.collect();
		Ok(ibc_proto::ibc::core::channel::v1::QueryChannelsResponse {
			channels,
			pagination: None,
			height: Some(at.into()),
		})
	}

	async fn query_send_packets(
		&self,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<ibc_rpc::PacketInfo>, Self::Error> {
		let rpc_client = self.rpc_client();
		let hash = None;
		// let mut last_sent_packet_hash = self.last_searched_sig_for_send_packets.lock().await;
		// let hash = if last_sent_packet_hash.is_empty() {
		// 	None
		// } else {
		// 	Some(Signature::from_str(&last_sent_packet_hash.as_str()).unwrap())
		// };
		let sigs = rpc_client
			.get_signatures_for_address_with_config(
				&solana_ibc::ID,
				GetConfirmedSignaturesForAddress2Config {
					until: hash,
					..GetConfirmedSignaturesForAddress2Config::default()
				},
			)
			.await
			.map_err(|e| Error::RpcError(format!("{:?}", e)))?;
		// if !sigs.is_empty() {
		// 	*last_sent_packet_hash = sigs[0].signature.clone();
		// }
		let mut transactions = Vec::new();
		for sig in sigs {
			let signature = Signature::from_str(&sig.signature).unwrap();
			let cloned_sig = signature.clone();
			let rpc_client = self.rpc_client();
			let tx = rpc_client
				.get_transaction(&cloned_sig, UiTransactionEncoding::Json)
				.await
				.unwrap();
			transactions.push(tx)
		}
		let send_packet_events: Vec<_> = transactions
			.iter()
			.filter_map(|tx| {
				let logs = match tx.transaction.meta.clone().unwrap().log_messages {
					solana_transaction_status::option_serializer::OptionSerializer::Some(e) => e,
					_ => Vec::new(),
				};
				let events = events::get_events_from_logs(logs);
				let send_packet_event = events.iter().find(|event| {
					matches!(event, ibc_core_handler_types::events::IbcEvent::SendPacket(_))
				});
				match send_packet_event {
					Some(e) => match e {
						ibc_core_handler_types::events::IbcEvent::SendPacket(packet) =>
							if packet.chan_id_on_a().as_str() == &channel_id.to_string() &&
								packet.port_id_on_a().as_str() == port_id.as_str() &&
								seqs.iter()
									.find(|&&seq| packet.seq_on_a().value() == seq)
									.is_some()
							{
								Some(packet.clone())
							} else {
								None
							},
						_ => None,
					},
					None => None,
				}
			})
			.collect();
		let height = self.latest_height_and_timestamp().await.unwrap().0;
		let packets: Vec<_> = send_packet_events
			.iter()
			.map(|packet| ibc_rpc::PacketInfo {
				height: Some(height.revision_height),
				sequence: packet.seq_on_a().value(),
				source_port: packet.port_id_on_a().to_string(),
				source_channel: packet.chan_id_on_a().to_string(),
				destination_port: packet.port_id_on_b().to_string(),
				destination_channel: packet.chan_id_on_b().to_string(),
				channel_order: packet.channel_ordering().to_string(),
				data: packet.packet_data().to_vec(),
				timeout_height: Height {
					revision_height: packet.timeout_height_on_b().commitment_revision_height(),
					revision_number: packet.timeout_height_on_b().commitment_revision_number(),
				}
				.into(),
				timeout_timestamp: packet.timeout_timestamp_on_b().nanoseconds(),
				ack: None,
			})
			.collect();
		Ok(packets)
	}

	async fn query_received_packets(
		&self,
		channel_id: ibc::core::ics24_host::identifier::ChannelId,
		port_id: ibc::core::ics24_host::identifier::PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<ibc_rpc::PacketInfo>, Self::Error> {
		let rpc_client = self.rpc_client();
		let mut last_recv_packet_hash = self.last_searched_sig_for_recv_packets.lock().await;
		let hash = if last_recv_packet_hash.is_empty() {
			None
		} else {
			Some(Signature::from_str(&last_recv_packet_hash.as_str()).unwrap())
		};
		let sigs = rpc_client
			.get_signatures_for_address_with_config(
				&solana_ibc::ID,
				GetConfirmedSignaturesForAddress2Config {
					until: hash,
					..GetConfirmedSignaturesForAddress2Config::default()
				},
			)
			.await
			.map_err(|e| Error::RpcError(format!("{:?}", e)))?;
		if !sigs.is_empty() {
			*last_recv_packet_hash = sigs[0].signature.clone();
		}
		let mut transactions = Vec::new();
		for sig in sigs {
			let signature = Signature::from_str(&sig.signature).unwrap();
			let cloned_sig = signature.clone();
			let rpc_client = self.rpc_client();
			let tx = rpc_client
				.get_transaction(&cloned_sig, UiTransactionEncoding::Json)
				.await
				.unwrap();
			transactions.push(tx)
		}
		let recv_packet_events: Vec<_> = transactions
			.iter()
			.filter_map(|tx| {
				let logs = match tx.transaction.meta.clone().unwrap().log_messages {
					solana_transaction_status::option_serializer::OptionSerializer::Some(e) => e,
					_ => Vec::new(),
				};
				let events = events::get_events_from_logs(logs);
				let send_packet_event = events.iter().find(|event| {
					matches!(event, ibc_core_handler_types::events::IbcEvent::ReceivePacket(_)) ||
						matches!(
							event,
							ibc_core_handler_types::events::IbcEvent::WriteAcknowledgement(_)
						)
				});
				match send_packet_event {
					Some(e) => match e {
						ibc_core_handler_types::events::IbcEvent::ReceivePacket(packet) =>
							if packet.chan_id_on_a().as_str() == &channel_id.to_string() &&
								packet.port_id_on_a().as_str() == port_id.as_str() &&
								seqs.iter()
									.find(|&&seq| packet.seq_on_b().value() == seq)
									.is_some()
							{
								Some(e.clone())
							} else {
								None
							},
						ibc_core_handler_types::events::IbcEvent::WriteAcknowledgement(packet) =>
							if packet.chan_id_on_a().as_str() == &channel_id.to_string() &&
								packet.port_id_on_a().as_str() == port_id.as_str() &&
								seqs.iter()
									.find(|&&seq| packet.seq_on_a().value() == seq)
									.is_some()
							{
								Some(e.clone())
							} else {
								None
							},
						_ => None,
					},
					None => None,
				}
			})
			.collect();
		let packets: Vec<_> = recv_packet_events
			.iter()
			.map(|recv_packet| match recv_packet {
				ibc_core_handler_types::events::IbcEvent::ReceivePacket(packet) =>
					ibc_rpc::PacketInfo {
						height: None,
						sequence: packet.seq_on_b().value(),
						source_port: packet.port_id_on_a().to_string(),
						source_channel: packet.chan_id_on_a().to_string(),
						destination_port: packet.port_id_on_b().to_string(),
						destination_channel: packet.chan_id_on_b().to_string(),
						channel_order: packet.channel_ordering().to_string(),
						data: packet.packet_data().to_vec(),
						timeout_height: Height {
							revision_height: packet
								.timeout_height_on_b()
								.commitment_revision_height(),
							revision_number: packet
								.timeout_height_on_b()
								.commitment_revision_number(),
						}
						.into(),
						timeout_timestamp: packet.timeout_timestamp_on_b().nanoseconds(),
						ack: None,
					},
				ibc_core_handler_types::events::IbcEvent::WriteAcknowledgement(packet) =>
					ibc_rpc::PacketInfo {
						height: None,
						sequence: packet.seq_on_a().value(),
						source_port: packet.port_id_on_a().to_string(),
						source_channel: packet.chan_id_on_a().to_string(),
						destination_port: packet.port_id_on_b().to_string(),
						destination_channel: packet.chan_id_on_b().to_string(),
						channel_order: String::from(""),
						data: packet.packet_data().to_vec(),
						timeout_height: Height {
							revision_height: packet
								.timeout_height_on_b()
								.commitment_revision_height(),
							revision_number: packet
								.timeout_height_on_b()
								.commitment_revision_number(),
						}
						.into(),
						timeout_timestamp: packet.timeout_timestamp_on_b().nanoseconds(),
						ack: Some(packet.acknowledgement().as_bytes().to_vec()),
					},
				_ => panic!("Infallible"),
			})
			.collect();
		println!("Length of packets {}", packets.len());
		Ok(packets)
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
		let storage = self.get_ibc_storage().await;
		let client_store = storage
			.clients
			.iter()
			.find(|&client| client.client_id.as_str() == client_id.as_str())
			.ok_or("Client not found with the given client id while querying client update time and height".to_owned())?;
		let inner_client_height = ibc_core_client_types::Height::new(
			client_height.revision_number,
			client_height.revision_height,
		)
		.unwrap();
		let height = client_store
			.consensus_states
			.deref()
			.get(&inner_client_height)
			.ok_or("No host height found with the given height".to_owned())?
			.processed_height()
			.ok_or("No height found".to_owned())?;
		let timestamp = client_store
			.consensus_states
			.deref()
			.get(&inner_client_height)
			.ok_or("No timestamp found with the given height".to_owned())?
			.processed_time()
			.ok_or("No timestamp found".to_owned())?;
		Ok((
			Height {
				revision_height: u64::from(height),
				// TODO: Use epoch
				revision_number: 1_u64,
			},
			Timestamp::from_nanoseconds(u64::from(timestamp)).unwrap(),
		))
	}

	async fn query_host_consensus_state_proof(
		&self,
		client_state: &pallet_ibc::light_clients::AnyClientState,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		let trie = self.get_trie().await;
		let height = client_state.latest_height();
		let client_id = self.client_id();
		let new_client_id =
			ibc_core_host_types::identifiers::ClientId::from_str(client_id.as_str()).unwrap();
		let client_idx = ClientIdx::try_from(new_client_id).unwrap();
		let consensus_state_trie_key = TrieKey::for_consensus_state(
			client_idx,
			ibc_core_client_types::Height::new(height.revision_number, height.revision_height)
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
		// let (token_mint_key, _bump) =
		// 	Pubkey::find_program_address(&[denom.as_ref()], &solana_ibc::ID);
		let token_mint_key = Pubkey::from_str(&asset_id).unwrap();
		let user_token_address =
			get_associated_token_address(&self.keybase.public_key, &token_mint_key);
		let sol_rpc_client = self.rpc_client();
		let balance = sol_rpc_client.get_token_account_balance(&user_token_address).await.unwrap();
		log::info!("IBC Balance on solana {}", balance.amount);
		Ok(vec![PrefixedCoin {
			denom: PrefixedDenom {
				trace_path: TracePath::default(),
				base_denom: BaseDenom::from_str(denom).unwrap(),
			},
			amount: Amount::from_str(&balance.amount).unwrap(),
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
		let timestamp = rpc_client.get_block_time(block_number.into()).await.map_err(|e| {
			Error::RpcError(
				serde_json::to_string(&e.kind.get_transaction_error().unwrap()).unwrap(),
			)
		})?;
		Ok(timestamp as u64)
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		let storage = self.get_ibc_storage().await;
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
		let storage = self.get_ibc_storage().await;
		let channels: Vec<(ChannelId, PortId)> = storage
			.port_channel
			.deref()
			.keys()
			.map(|channel_store| {
				(
					ChannelId::from_str(&channel_store.channel_id().as_str()).unwrap(),
					PortId::from_str(&channel_store.port_id().as_str()).unwrap(),
				)
			})
			.collect();
		Ok(channels)
	}

	async fn query_connection_using_client(
		&self,
		_height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error> {
		let storage = self.get_ibc_storage().await;
		let client_id_key =
			ibc_core_host_types::identifiers::ClientId::from_str(&client_id).unwrap();
		let mut index = -1;
		let connections: Vec<IdentifiedConnection> = storage
			.connections
			.iter()
			.filter_map(|serialized_connection| {
				index += 1;
				// let serialized_connection = &storage.connections[0];
				let connection = serialized_connection.get().unwrap();
				if connection.client_id_matches(&client_id_key) {
					let versions = connection
						.versions()
						.iter()
						.map(|version| {
							let proto_version: ibc_proto_new::ibc::core::connection::v1::Version =
								version.clone().try_into().unwrap();
							Version {
								identifier: proto_version.identifier,
								features: proto_version.features,
							}
						})
						.collect();
					let counterparty = connection.counterparty();
					let connection_id = format!("{}-{}", ConnectionId::prefix(), index);
					return Some(IdentifiedConnection {
						id: connection_id,
						client_id: client_id.clone(),
						versions,
						state: i32::from(connection.state),
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
					})
				};
				None
			})
			.collect();
		log::info!("querying connecting using client final");
		Ok(connections)
	}

	async fn is_update_required(
		&self,
		_latest_height: u64,
		_latest_client_height_on_counterparty: u64,
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
		let latest_height_timestamp = self.latest_height_and_timestamp().await?;
		println!("This is height on solana {:?}", latest_height_timestamp);
		let client_state = TmClientState::new(
			ChainId::from_string(&self.chain_id),
			TrustThreshold::default(),
			Duration::from_secs(64000),
			Duration::from_secs(1814400),
			Duration::new(15, 0),
			latest_height_timestamp.0,
			ProofSpecs::default(),
			vec!["upgrade".to_string(), "upgradedIBCState".to_string()],
		)
		.map_err(|e| Error::from(format!("Invalid client state {e}")))?;
		let timestamp_in_nano = latest_height_timestamp.1.nanoseconds();
		let secs = timestamp_in_nano / 10_u64.pow(9);
		let nano = timestamp_in_nano % 10_u64.pow(9);
		let time =
			Time::from_unix_timestamp(secs.try_into().unwrap(), nano.try_into().unwrap()).unwrap();
		let client_state_in_bytes = borsh::to_vec(&timestamp_in_nano).unwrap();
		let trie = self.get_trie().await;
		let sub_trie = trie.get_subtrie(&borsh::to_vec(&1).unwrap()).unwrap();
		println!("This is sub trie {:?}", sub_trie.len());
		let consensus_state = TmConsensusState::new(client_state_in_bytes.into(), time, Hash::None);
		// let mock_header = ibc::mock::header::MockHeader {
		// 	height: ibc::Height::new(1, 1),
		// 	timestamp: ibc::timestamp::Timestamp::from_nanoseconds(1).unwrap(),
		// };
		// let client_state = ibc::mock::client_state::MockClientState::new(mock_header.into());
		// let consensus_state = ibc::mock::client_state::MockConsensusState::new(mock_header);
		Ok((
			AnyClientState::Tendermint(client_state),
			AnyConsensusState::Tendermint(consensus_state),
		))
	}

	async fn query_client_id_from_tx_hash(
		&self,
		tx_id: Self::TransactionId,
	) -> Result<ClientId, Self::Error> {
		let program = self.program();
		let signature = Signature::from_str(&tx_id).unwrap();
		let sol_rpc_client = program.async_rpc();
		let tx = sol_rpc_client
			.get_transaction(&signature, UiTransactionEncoding::Base64)
			.await
			.unwrap();
		let logs = match tx.transaction.meta.unwrap().log_messages {
			solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
			solana_transaction_status::option_serializer::OptionSerializer::None =>
				return Err(Error::Custom(String::from("No logs found"))),
			solana_transaction_status::option_serializer::OptionSerializer::Skip =>
				return Err(Error::Custom(String::from("Logs were skipped, so not available"))),
		};
		let events = events::get_events_from_logs(logs);
		let result: Vec<&ibc_core_client_types::events::CreateClient> = events
			.iter()
			.filter_map(|event| match event {
				ibc_core_handler_types::events::IbcEvent::CreateClient(e) => Some(e),
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
		let sol_rpc_client = program.async_rpc();
		let tx = sol_rpc_client
			.get_transaction(&signature, UiTransactionEncoding::Json)
			.await
			.unwrap();
		let logs = match tx.transaction.meta.unwrap().log_messages {
			solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
			solana_transaction_status::option_serializer::OptionSerializer::None =>
				return Err(Error::Custom(String::from("No logs found"))),
			solana_transaction_status::option_serializer::OptionSerializer::Skip =>
				return Err(Error::Custom(String::from("Logs were skipped, so not available"))),
		};
		let events = events::get_events_from_logs(logs);
		let result: Vec<&ibc_core_connection_types::events::OpenInit> = events
			.iter()
			.filter_map(|event| match event {
				ibc_core_handler_types::events::IbcEvent::OpenInitConnection(e) => Some(e),
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
		let sol_rpc_client = program.async_rpc();
		let tx = sol_rpc_client
			.get_transaction(&signature, UiTransactionEncoding::Json)
			.await
			.unwrap();
		let logs = match tx.transaction.meta.unwrap().log_messages {
			solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
			solana_transaction_status::option_serializer::OptionSerializer::None =>
				return Err(Error::Custom(String::from("No logs found"))),
			solana_transaction_status::option_serializer::OptionSerializer::Skip =>
				return Err(Error::Custom(String::from("Logs were skipped, so not available"))),
		};
		let events = events::get_events_from_logs(logs);
		let result: Vec<&ibc_core_channel_types::events::OpenInit> = events
			.iter()
			.filter_map(|event| match event {
				ibc_core_handler_types::events::IbcEvent::OpenInitChannel(e) => Some(e),
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

	async fn upload_wasm(&self, _wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
		todo!()
	}
}

impl KeyProvider for SolanaClient {
	fn account_id(&self) -> ibc::signer::Signer {
		let key_entry = &self.keybase;
		let public_key = key_entry.public_key;
		ibc::signer::Signer::from_str(&public_key.to_string()).unwrap()
	}
}

#[async_trait::async_trait]
impl MisbehaviourHandler for SolanaClient {
	async fn check_for_misbehaviour<C: Chain>(
		&self,
		_counterparty: &C,
		_client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		Ok(())
	}
}

#[async_trait::async_trait]
impl LightClientSync for SolanaClient {
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
impl Chain for SolanaClient {
	fn name(&self) -> &str {
		&self.name
	}

	fn block_max_weight(&self) -> u64 {
		self.max_tx_size as u64
	}

	async fn estimate_weight(&self, _msg: Vec<Any>) -> Result<u64, Self::Error> {
		Ok(0)
	}

	async fn finality_notifications(
		&self,
	) -> Result<
		Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>>,
		Error,
	> {
		let (tx, rx) = unbounded_channel();
		let ws_url = self.ws_url.clone();
		tokio::task::spawn_blocking(move || {
			let (_logs_listener, receiver) = PubsubClient::block_subscribe(
				&ws_url, /* Quicknode rpc should be used for devnet/mainnet and
				          * incase of localnet,
				          * the flag `--rpc-pubsub-enable-block-subscription` has
				          * to be passed to
				          * local validator. */
				RpcBlockSubscribeFilter::All,
				Some(RpcBlockSubscribeConfig {
					commitment: Some(CommitmentConfig::finalized()),
					..Default::default()
				}),
			)
			.unwrap();

			loop {
				match receiver.recv() {
					Ok(logs) =>
						if logs.value.block.is_some() {
							let block_info = logs.value.block.clone().unwrap();
							let slot = logs.value.slot;
							let finality_event = FinalityEvent::Tendermint {
								previous_blockhash: block_info.previous_blockhash,
								blockhash: block_info.blockhash,
								height: block_info.block_height.unwrap(),
								timestamp: block_info.block_time.unwrap() as u64,
								slot,
							};
							tx.send(finality_event).unwrap();
						},
					Err(err) => {
						panic!("{}", format!("Disconnected: {err}"));
					},
				}
			}
		});

		let streams = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);
		Ok(Box::pin(streams))
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<Self::TransactionId, Error> {
		let keypair = self.keybase.keypair();
		println!("submitting tx now, {}", keypair.pubkey());
		let authority = Arc::new(keypair);
		let program = self.program();

		// Build, sign, and send program instruction
		let mut signature = String::new();
		let rpc = program.async_rpc();

		for message in messages {
			let my_message = Ics26Envelope::<LocalClientTypes>::try_from(message.clone()).unwrap();
			let new_messages = convert_old_msgs_to_new(vec![my_message]);
			let message = new_messages[0].clone();
			let mut instruction_data =
				anchor_lang::InstructionData::data(&solana_ibc::instruction::Deliver {
					message: message.clone(),
				});
			let instruction_len = instruction_data.len() as u32;
			instruction_data.splice(..0, instruction_len.to_le_bytes());

			let balance = program.async_rpc().get_balance(&authority.pubkey()).await.unwrap();
			println!("This is balance {}", balance);
			println!("This is start of payload ---------------------------------");
			println!("{:?}", message);
			println!("This is end of payload ----------------------------------");

			let max_tries = 5;

			let blockhash = rpc.get_latest_blockhash().await.unwrap();

			let write_account_program_id =
				Pubkey::from_str("BHgp5XwSmDpbVQXy5vFkExjEhKL86hBy1JBTHCYDtA4e").unwrap();

			let (mut chunks, chunk_account, _) = write::instruction::WriteIter::new(
				&write_account_program_id,
				authority.pubkey(),
				WRITE_ACCOUNT_SEED,
				instruction_data,
			)
			.unwrap();

			chunks.chunk_size = core::num::NonZeroU16::new(500).unwrap();
			for instruction in &mut chunks {
				let transaction = Transaction::new_signed_with_payer(
					&[instruction],
					Some(&authority.pubkey()),
					&[&*authority],
					blockhash,
				);
				let sig =
					rpc.send_and_confirm_transaction_with_spinner(&transaction).await.unwrap();
				println!("  Signature {sig}");
			}
			if let MsgEnvelope::Client(ClientMsg::UpdateClient(e)) = message {
				signature = self
					.send_deliver(
						DeliverIxType::UpdateClient {
							client_message: e.client_message,
							client_id: e.client_id,
						},
						chunk_account,
						max_tries,
					)
					.await?;
			} else if let MsgEnvelope::Packet(PacketMsg::Recv(e)) = message {
				let packet_data: ibc_app_transfer_types::packet::PacketData =
					serde_json::from_slice(&e.packet.data).unwrap();
				signature = self
					.send_deliver(
						DeliverIxType::PacketTransfer {
							token: packet_data.token,
							port_id: e.packet.port_id_on_a,
							channel_id: e.packet.chan_id_on_a,
						},
						chunk_account,
						max_tries,
					)
					.await?;
			} else {
				signature =
					self.send_deliver(DeliverIxType::Normal, chunk_account, max_tries).await?;
			}
		}
		Ok(signature)
	}

	async fn query_client_message(
		&self,
		_update: UpdateClient,
	) -> Result<AnyClientMessage, Self::Error> {
		todo!()
	}

	async fn get_proof_height(&self, block_height: Height) -> Height {
		block_height
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
