use elliptic_curve::PrimeField;
use ethers::{
	abi,
	abi::{
		encode, encode_packed, ethabi, Abi, AbiEncode, Detokenize, InvalidOutputType, ParamType,
		RawLog, Token, Tokenizable,
	},
	contract::{abigen, EthEvent},
	middleware::contract::Contract,
	prelude::{Block, Log},
	providers::Middleware,
	types::{
		BlockId, BlockNumber, EIP1186ProofResponse, Filter, StorageProof, Topic, ValueOrArray,
		H160, H256, U256,
	},
	utils::keccak256,
};
use ibc::{
	core::{
		ics02_client::client_state::ClientType,
		ics04_channel::packet::Sequence,
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::{
			identifier::{ChannelId, ClientId, ConnectionId, PortId},
			path::{
				AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath,
				CommitmentsPath, ConnectionsPath, ReceiptsPath,
			},
			Path,
		},
	},
	timestamp::Timestamp,
	Height,
};
use ibc_proto::ibc::core::{
	channel::v1::{
		Counterparty as ChannelCounterparty, QueryChannelResponse, QueryChannelsResponse,
		QueryNextSequenceReceiveResponse, QueryPacketCommitmentResponse,
		QueryPacketReceiptResponse,
	},
	client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
	connection::v1::{
		Counterparty as ConnectionCounterparty, IdentifiedConnection, QueryConnectionResponse,
	},
};
use itertools::Itertools;
use primitives::{Chain, IbcProvider, KeyProvider, UpdateType};
use prost::Message;
use std::{
	collections::{HashMap, HashSet},
	fs,
	future::Future,
	iter,
	pin::Pin,
	str::FromStr,
	sync::Arc,
	time::Duration,
};

use crate::{
	client::{ClientError, EthereumClient},
	events::TryFromEvent,
};
use futures::{FutureExt, Stream, StreamExt};
use log::info;
use ssz_rs::{Merkleized, Node};
use sync_committee_primitives::consensus_types::BeaconBlockHeader;

#[cfg(not(feature = "no_beacon"))]
use crate::prove::prove;
#[cfg(feature = "no_beacon")]
use crate::prove::prove_fast as prove;
use crate::{
	chain::{
		client_state_abi_token, client_state_from_abi_token, consensus_state_from_abi_token,
		tm_header_from_abi_token,
	},
	client::run_updates_fetcher,
	config::ContractName,
	utils::{create_intervals, SEQUENCES_PER_ITER},
};
use ibc::{
	applications::transfer::{Amount, BaseDenom, PrefixedCoin, PrefixedDenom, TracePath},
	core::{
		ics02_client::{events::UpdateClient, msgs::update_client::MsgUpdateAnyClient},
		ics04_channel::{
			channel::{Order, State},
			events::SendPacket,
		},
		ics23_commitment::commitment::CommitmentRoot,
		ics24_host::path::SeqRecvsPath,
	},
	events::IbcEvent,
	protobuf::Protobuf,
	tx_msg::Msg,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{Channel, IdentifiedChannel},
		commitment::v1::MerklePrefix,
		connection::v1::{ConnectionEnd, Version},
	},
};
use ibc_rpc::{IbcApiClient, PacketInfo};
use ics07_tendermint::consensus_state::ConsensusState as TmConsensusState;
use icsxx_ethereum::{
	client_message::{ClientMessage, Header},
	client_state::ClientState,
	consensus_state::ConsensusState,
};
use pallet_ibc::light_clients::{
	AnyClientMessage, AnyClientState, AnyConsensusState, HostFunctionsManager,
};
use primitives::mock::LocalClientTypes;
use sync_committee_primitives::{
	types::VerifierState as LightClientState,
	util::{compute_epoch_at_slot, compute_sync_committee_period_at_slot},
};
use tokio::time::sleep;

pub const INDEXER_DELAY_BLOCKS: u64 = 1;
pub const MAX_UPDATES_PER_ITER: usize = 10;

abigen!(
	IbcClientAbi,
	"hyperspace/ethereum/src/abi/ibc-client-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	IbcConnectionAbi,
	"hyperspace/ethereum/src/abi/ibc-connection-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	IbcChannelAbi,
	"hyperspace/ethereum/src/abi/ibc-channel-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	IbcPacketAbi,
	"hyperspace/ethereum/src/abi/ibc-packet-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	IbcQuerierAbi,
	"hyperspace/ethereum/src/abi/ibc-querier-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	Ics20TransferBankAbi,
	"hyperspace/ethereum/src/abi/ics20-transfer-bank-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	Ics20BankAbi,
	"hyperspace/ethereum/src/abi/ics20-bank-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	TendermintClientAbi,
	"hyperspace/ethereum/src/abi/tendermint-client-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	DiamondAbi,
	"hyperspace/ethereum/src/abi/diamond-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	DiamondCutFacetAbi,
	"hyperspace/ethereum/src/abi/diamond-cut-facet-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	DiamondLoupeFacetAbi,
	"hyperspace/ethereum/src/abi/diamond-loupe-facet-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	OwnershipFacetAbi,
	"hyperspace/ethereum/src/abi/ownership-facet-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	ERC20TokenAbi,
	"hyperspace/ethereum/src/abi/erc20-abi.json",
	event_derives (serde::Deserialize, serde::Serialize);

	EthereumClientAbi,
	"hyperspace/ethereum/src/abi/ethereum-client-abi.json";

	GovernanceFacetAbi,
	"hyperspace/ethereum/src/abi/governance-facet-abi.json";

	GovernanceProxyAbi,
	"hyperspace/ethereum/src/abi/governance-proxy-abi.json";

	RelayerWhitelistFacetAbi,
	"hyperspace/ethereum/src/abi/relayer-whitelist-facet-abi.json";

	CallBatchFacetAbi,
	"hyperspace/ethereum/src/abi/call-batch-facet-abi.json";
);

impl From<HeightData> for Height {
	fn from(value: HeightData) -> Self {
		Self {
			revision_number: value.revision_number.into(),
			revision_height: value.revision_height.into(),
		}
	}
}

impl From<HeightData> for ibc_proto::ibc::core::client::v1::Height {
	fn from(value: HeightData) -> Self {
		Self {
			revision_number: value.revision_number.into(),
			revision_height: value.revision_height.into(),
		}
	}
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlockHeight(pub(crate) BlockNumber);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FinalityEvent {
	Ethereum { hash: H256 },
}

async fn query_proof_then<Fut, F, T, Fut2>(query_proof: Fut, f: F) -> Result<T, ClientError>
where
	F: FnOnce(StorageProof) -> Fut2,
	Fut2: Future<Output = Result<T, ClientError>>,
	Fut: Future<Output = Result<EIP1186ProofResponse, ClientError>>,
{
	let proof = query_proof.await?;

	if let Some(storage_proof) = proof.storage_proof.last() {
		f(storage_proof.clone()).await
	} else {
		Err(ClientError::NoStorageProof)
	}
}

pub const NUMBER_OF_BLOCKS_TO_PROCESS_PER_ITER: u64 = 10000;

pub async fn parse_ethereum_event(
	client: &EthereumClient,
	log: Log,
) -> Result<Option<IbcEvent>, ClientError> {
	let raw_log = RawLog::from(log.clone());
	let height = Height::new(
		0,
		log.block_number
			.ok_or(ClientError::Other("block number not found".to_string()))?
			.as_u64(),
	);
	let topic0 = log.topics[0];

	macro_rules! handle_events {
		    ($topic0:ident, $events:ident, $log:ident, $raw_log:ident, $height:ident, $($ty:ty),+) => {
				$(if $topic0 == <$ty>::signature() {
					 let event = <$ty>::decode_log(&$raw_log).expect("decode event");
					 let ev = IbcEvent::try_from_event(client, event, $log, $height).await?;
					 log::debug!(target: "hyperspace_ethereum", "encountered event: {:?} at {}", ev.event_type(), ev.height());
						return Ok(Some(ev));
				} else )+ {
					 log::warn!(
						 target: "hyperspace_ethereum", "unknown event: {}",
						   log.log_type.unwrap_or(format!("{:?}", $topic0))
					 );
					 return Ok(None)
				}
			};
		}

	handle_events!(
		topic0,
		event,
		log,
		raw_log,
		height,
		OpenInitConnectionFilter,
		OpenTryConnectionFilter,
		OpenAckConnectionFilter,
		OpenConfirmConnectionFilter,
		OpenInitChannelFilter,
		OpenAckChannelFilter,
		OpenConfirmChannelFilter,
		SendPacketFilter,
		WriteAcknowledgementFilter,
		AcknowledgePacketFilter,
		TimeoutPacketFilter,
		TimeoutOnClosePacketFilter,
		CloseInitChannelFilter,
		CloseConfirmChannelFilter
	)
}

impl EthereumClient {
	pub async fn query_client_state_exact_token(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<Token, ClientError> {
		// First, we try to find an `UpdateClient` event at the given height...
		let mut client_state = None;
		let maybe_log = self
			.get_logs_for_event_name::<UpdateClientHeightFilter>(
				self.contract_creation_block(),
				at.revision_height,
				&format!("event_data->>'client_id' = '{client_id}'"),
				Some("ORDER BY (event_data->'client_height'->>'revision_height') :: bigint"),
			)
			.await?
			.pop();
		let batch_func = self.yui.function("callBatch")?;
		match maybe_log {
			Some((_, log)) => {
				info!("found update client event at {}", log.block_number.unwrap());
				let tx_hash = log
					.transaction_hash
					.ok_or(ClientError::Other("tx hash not found".to_string()))?;
				let func = self.yui.function("updateClient")?;
				let tx = self
					.client()
					.get_transaction(tx_hash)
					.await
					.map_err(|err| {
						ClientError::Other(format!("failed to get transaction: {}", err))
					})?
					.ok_or_else(|| {
						ClientError::Other(format!("transaction not found: {}", tx_hash))
					})?;
				let Token::Array(batch_calldata) =
					batch_func
						.decode_input(&tx.input[4..])?
						.pop()
						.ok_or(ClientError::Other("batch calldata not found".to_string()))?
				else {
					return Err(ClientError::Other("batch calldata not found".to_string()))
				};

				for input_tok in batch_calldata.into_iter().rev() {
					let Token::Bytes(input) = input_tok else {
						return Err(ClientError::Other("input token should be bytes".to_string()))
					};
					if input[..4] == func.short_signature() {
						let calldata = func
							.decode_input(&input[4..])?
							.pop()
							.ok_or(ClientError::Other("calldata not found".to_string()))?;
						let Token::Tuple(toks) = calldata else {
							return Err(ClientError::Other("calldata should be bytes".to_string()))
						};
						let header = tm_header_from_abi_token(toks[1].clone())?;
						let client_state_token = toks[2].clone();
						let mut cs =
							client_state_from_abi_token::<LocalClientTypes>(client_state_token)?;
						if header.signed_header.header.height.value() >
							cs.latest_height.revision_height
						{
							cs.latest_height = Height::new(
								cs.latest_height.revision_number,
								header.signed_header.header.height.into(),
							);
						}
						let tok_after = client_state_abi_token(&cs);
						client_state = Some(Token::Bytes(encode(&[tok_after])).into());
						// TODO: figure out how to distinguish between the same function calls
						break
					}
				}
				// TODO: handle frozen height
			},
			None => {
				log::trace!(target: "hyperspace_ethereum", "no update client event found for blocks ..{at}, looking for a create client event...");

				let (_, log) = self
					.get_logs_for_event_name::<CreateClientFilter>(
						self.contract_creation_block(),
						at.revision_height,
						&format!("event_data->>'client_id' = '{client_id}'"),
						None,
					)
					.await?
					.pop()
					.ok_or_else(|| ClientError::Other("no events found".to_string()))?;

				let tx_hash = log
					.transaction_hash
					.ok_or(ClientError::Other("tx hash not found".to_string()))?;
				let func = self.yui.function("createClient")?;
				let tx = self
					.client()
					.get_transaction(tx_hash)
					.await
					.map_err(|err| {
						ClientError::Other(format!("failed to get transaction: {}", err))
					})?
					.ok_or_else(|| {
						ClientError::Other(format!("transaction not found: {}", tx_hash))
					})?;

				let Token::Array(batch_calldata) =
					batch_func
						.decode_input(&tx.input[4..])?
						.pop()
						.ok_or(ClientError::Other("batch calldata not found".to_string()))?
				else {
					return Err(ClientError::Other("batch calldata not found".to_string()))
				};

				for input_tok in batch_calldata.into_iter().rev() {
					let Token::Bytes(input) = input_tok else {
						return Err(ClientError::Other("input token should be bytes".to_string()))
					};
					if input[..4] == func.short_signature() {
						let calldata = func
							.decode_input(&input[4..])?
							.pop()
							.ok_or(ClientError::Other("calldata not found".to_string()))?;
						let Token::Tuple(toks) = calldata else {
							return Err(ClientError::Other("calldata should be bytes".to_string()))
						};
						let client_state_token = toks[1].clone();

						client_state = Some(client_state_token);

						break
					}
				}
			},
		}

		Ok(client_state.ok_or(ClientError::Other("client state not found".to_string()))?)
	}

	async fn fetch_next_update<T>(
		&mut self,
		counterparty: &T,
		client_id: ClientId,
		client_state: &ClientState<HostFunctionsManager>,
		latest_revision: u64,
		header: Header,
	) -> Result<Option<(Header, (Any, Height, Vec<IbcEvent>, UpdateType))>, ClientError>
	where
		T: Chain,
	{
		let update = &header.inner;

		let update_height =
			Height::new(latest_revision, update.execution_payload.block_number.into());
		let mut events = vec![];

		let update_client_header = {
			info!(target: "hyperspace_ethereum", "update client header height: {}, finalized slot: {}",
				update.execution_payload.block_number,
				update.finalized_header.slot
			);
			let msg = MsgUpdateAnyClient::<LocalClientTypes> {
				client_id: client_id.clone(),
				client_message: AnyClientMessage::Ethereum(ClientMessage::Header(header.clone())),
				signer: counterparty.account_id(),
			};
			let value = msg.encode_vec().map_err(|e| {
				ClientError::from(format!("Failed to encode MsgUpdateClient {msg:?}: {e:?}"))
			})?;
			Any { value, type_url: msg.type_url() }
		};

		let update_type = if header.inner.sync_committee_update.is_some() {
			UpdateType::Mandatory
		} else {
			UpdateType::Optional
		};
		Ok(Some((header, (update_client_header, update_height, events, update_type))))
	}

	async fn fetch_next_updates<T>(
		&mut self,
		counterparty: &T,
		client_id: ClientId,
		client_state: &ClientState<HostFunctionsManager>,
		latest_revision: u64,
		max_updates: usize,
	) -> Result<Vec<(Any, Height, Vec<IbcEvent>, UpdateType)>, ClientError>
	where
		T: primitives::Chain,
	{
		let mut client_state = client_state.clone();
		let mut updates_with_events = vec![];

		let mut updates = self.updates.lock().await;
		let idx = updates.iter().enumerate().find_map(|(idx, h)| {
			if h.inner.finalized_header.slot > client_state.inner.finalized_header.slot {
				Some(idx)
			} else {
				None
			}
		});
		if let Some(remove_up_to) = idx {
			updates.drain(0..remove_up_to);
		}

		if updates.is_empty() {
			log::debug!(target: "hyperspace_ethereum", "No new updates");
			return Ok(vec![])
		}
		updates.sort_by_key(|h| h.inner.finalized_header.slot);
		let headers = updates.clone();
		drop(updates);

		let latest_cp_client_height = client_state.latest_height().revision_height;
		let from = latest_cp_client_height + 1;

		for header in headers.into_iter().take(max_updates) {
			let maybe_update = self
				.fetch_next_update(
					counterparty,
					client_id.clone(),
					&client_state,
					latest_revision,
					header,
				)
				.await?;
			match maybe_update {
				Some((header, update)) => {
					info!(target: "hyperspace_ethereum", "Got update at slot {}", header.inner.finalized_header.slot);
					updates_with_events.push(update);
				},
				None => {
					log::debug!(target: "hyperspace_ethereum", "No more updates");
					break
				},
			}
		}
		if !updates_with_events.is_empty() {
			let to = updates_with_events.last().map(|(_, h, ..)| h.revision_height).unwrap();
			let mut events = vec![];
			let logs = self.get_logs(from, to.into(), None).await?;

			for log in logs {
				if let Some(event) = parse_ethereum_event(&self, log).await? {
					events.push(event);
				}
			}

			events.iter().for_each(|x| {
				info!("ev height = {}", x.height());
			});
			updates_with_events.last_mut().unwrap().2.extend(events);
		}
		Ok(updates_with_events)
	}
}

#[async_trait::async_trait]
impl IbcProvider for EthereumClient {
	type FinalityEvent = Block<H256>;

	type TransactionId = (H256, H256);

	type AssetId = String;

	type Error = ClientError;

	async fn query_latest_ibc_events<T>(
		&mut self,
		_finality_event: Self::FinalityEvent,
		counterparty: &T,
	) -> Result<Vec<(Any, Height, Vec<IbcEvent>, UpdateType)>, anyhow::Error>
	where
		T: primitives::Chain,
	{
		let client_id = self.client_id();
		let latest_cp_height = counterparty.latest_height_and_timestamp().await.unwrap().0;
		let latest_cp_client_state = counterparty
			.query_client_state(latest_cp_height, client_id.clone())
			.await
			.unwrap();
		let client_state_response = latest_cp_client_state
			.client_state
			.ok_or_else(|| {
				ClientError::Other("counterparty returned empty client state".to_string())
			})
			.unwrap();
		let AnyClientState::Ethereum(client_state) =
			AnyClientState::decode_recursive(client_state_response, |c| {
				matches!(c, AnyClientState::Ethereum(_))
			})
			.unwrap()
		else {
			unreachable!()
		};

		let latest_height = self.latest_height_and_timestamp().await.unwrap().0;
		let latest_revision = latest_height.revision_number;

		let sync_committee_prover = self.prover();
		if self.updates_fetcher_handle.is_finished() {
			// let res = self.updates_fetcher_handle.await;
			// if let Err(e) = res {
			// 	log::error!(target: "hyperspace_ethereum", "updates fetcher failed: {}", e);
			// }
			self.updates_fetcher_handle = Arc::new(run_updates_fetcher(
				client_state.clone(),
				sync_committee_prover,
				self.updates.clone(),
			));
		}

		let updates = self
			.fetch_next_updates(counterparty, client_id, &client_state, latest_revision, 10)
			.await
			.unwrap();

		Ok(updates)
	}

	// TODO: this function is mostly used in tests and in 'fishing' mode.
	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + 'static>> {
		let ibc_address = self.yui.ibc_core_diamond.address();
		let client = self.clone();

		let creation_block = self.contract_creation_block();
		let ws = self.websocket_provider().await.unwrap();
		let latest_block = ws.get_block_number().await.unwrap();
		(async_stream::stream! {
			let mut events_stream = ws.subscribe_logs(
				 &Filter::new()
					 .from_block(latest_block)
					 .address(ibc_address),
			)
			.await
			.unwrap()
			.filter_map(|log| async {
				parse_ethereum_event(&client, log).await.ok()?
			}).boxed();

			while let Some(ev) = events_stream.next().await {
				yield ev
			}
		})
		.boxed()
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		// First, we try to find an `UpdateClient` event at the given height...
		let mut consensus_state = None;
		let maybe_log = self
			.get_logs_for_event_name::<UpdateClientHeightFilter>(
                self.contract_creation_block(),
				at.revision_height,
				&format!(
					"event_data ->> 'client_id' = '{client_id}' AND event_data->'client_height'->>'revision_number' = '{}' AND event_data->'client_height'->>'revision_height' = '{}'",
					consensus_height.revision_number, consensus_height.revision_height
				),
				None,
			)
			.await?.pop();
		let batch_func = self.yui.function("callBatch")?;
		match maybe_log {
			Some((_, log)) => {
				let tx_hash = log.transaction_hash.expect("tx hash should exist");
				let func = self.yui.function("updateClient")?;
				let tx =
					self.client().get_transaction(tx_hash).await.unwrap().ok_or_else(|| {
						ClientError::Other(format!("transaction not found: {}", tx_hash))
					})?;
				let Token::Array(batch_calldata) =
					batch_func.decode_input(&tx.input[4..])?.pop().unwrap()
				else {
					return Err(ClientError::Other("batch calldata not found".to_string()))
				};

				for input_tok in batch_calldata.into_iter().rev() {
					let Token::Bytes(input) = input_tok else { panic!() };
					if input[..4] == func.short_signature() {
						let calldata = func.decode_input(&input[4..])?.pop().unwrap();
						let Token::Tuple(toks) = calldata else { panic!() };
						let header = tm_header_from_abi_token(toks[1].clone())?;
						consensus_state = Some(TmConsensusState {
							timestamp: header.signed_header.header.time,
							root: CommitmentRoot {
								bytes: header.signed_header.header.app_hash.as_bytes().to_owned(),
							},
							next_validators_hash: header.signed_header.header.next_validators_hash,
						});
						// TODO: figure out how to distinguish between the same function calls

						let proof_height = Some(at.into());
						let proof = self
							.query_proof(
								at,
								vec![ClientConsensusStatePath {
									client_id: client_id.clone(),
									epoch: consensus_height.revision_number,
									height: consensus_height.revision_height,
								}
								.to_string()
								.into_bytes()],
							)
							.await?;

						return Ok(QueryConsensusStateResponse {
							consensus_state: Some(
								consensus_state.expect("should always be initialized").to_any(),
							),
							proof,
							proof_height,
						})
					}
				}
				// TODO: handle frozen height
			},
			None => {},
		}

		log::trace!(target: "hyperspace_ethereum", "no update client event found for blocks ..{at}, looking for a create client event...");

		// ...otherwise, try to get the `CreateClient` event
		let (_, log) = self
			.get_logs_for_event_name::<CreateClientFilter>(
				self.contract_creation_block(),
				at.revision_height,
				&format!("event_data->>'client_id' = '{client_id}'"),
				None,
			)
			.await?
			.pop()
			.ok_or_else(|| ClientError::Other("no events found".to_string()))?;

		let tx_hash = log.transaction_hash.expect("tx hash should exist");
		let func = self.yui.function("createClient")?;
		let tx = self
			.client()
			.get_transaction(tx_hash)
			.await
			.unwrap()
			.ok_or_else(|| ClientError::Other(format!("transaction not found: {}", tx_hash)))?;

		let Token::Array(batch_calldata) = batch_func.decode_input(&tx.input[4..])?.pop().unwrap()
		else {
			return Err(ClientError::Other("batch calldata not found".to_string()))
		};

		for input_tok in batch_calldata.into_iter().rev() {
			let Token::Bytes(input) = input_tok else { panic!() };
			info!("sig = {:?}", func.short_signature());
			if input[..4] == func.short_signature() {
				let calldata = func.decode_input(&input[4..])?.pop().unwrap();
				let Token::Tuple(toks) = calldata else { panic!() };
				let consensus_state_token = toks[2].clone();
				// TODO: check that the state satisfies `consensus_height`
				consensus_state = Some(consensus_state_from_abi_token(consensus_state_token)?);
				break
			}
		}

		let proof = self
			.query_proof(
				at,
				vec![ClientConsensusStatePath {
					client_id: client_id.clone(),
					epoch: consensus_height.revision_number,
					height: consensus_height.revision_height,
				}
				.to_string()
				.into_bytes()],
			)
			.await?;

		let proof_height = Some(at.into());
		let state = consensus_state.expect("should always be initialized");
		let any = state.to_any();

		Ok(QueryConsensusStateResponse { consensus_state: Some(any), proof, proof_height })
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		info!(target: "hyperspace_ethereum", "querying client state for client {} at {}", client_id, at);
		// First, we try to find an `UpdateClient` event at the given height...
		let mut client_state = None;
		let maybe_log = self
			.get_logs_for_event_name::<UpdateClientHeightFilter>(
				self.contract_creation_block(),
				at.revision_height,
				&format!("event_data->>'client_id' = '{client_id}'"),
				Some("ORDER BY (event_data->'client_height'->>'revision_height') :: bigint"),
			)
			.await?
			.pop();
		let batch_func = self.yui.function("callBatch")?;
		match maybe_log {
			Some((_, log)) => {
				info!("found update client event at {}", log.block_number.unwrap());
				let tx_hash = log
					.transaction_hash
					.ok_or(ClientError::Other("tx hash not found".to_string()))?;
				let func = self.yui.function("updateClient")?;
				let tx = self
					.client()
					.get_transaction(tx_hash)
					.await
					.map_err(|err| {
						ClientError::Other(format!("failed to get transaction: {}", err))
					})?
					.ok_or_else(|| {
						ClientError::Other(format!("transaction not found: {}", tx_hash))
					})?;
				let Token::Array(batch_calldata) =
					batch_func
						.decode_input(&tx.input[4..])?
						.pop()
						.ok_or(ClientError::Other("batch calldata not found".to_string()))?
				else {
					return Err(ClientError::Other("batch calldata not found".to_string()))
				};

				for input_tok in batch_calldata.into_iter().rev() {
					let Token::Bytes(input) = input_tok else {
						return Err(ClientError::Other("input token should be bytes".to_string()))
					};
					if input[..4] == func.short_signature() {
						let calldata = func
							.decode_input(&input[4..])?
							.pop()
							.ok_or(ClientError::Other("calldata not found".to_string()))?;
						let Token::Tuple(toks) = calldata else {
							return Err(ClientError::Other("calldata should be bytes".to_string()))
						};
						let header = tm_header_from_abi_token(toks[1].clone())?;
						let client_state_token = toks[2].clone();
						let mut cs =
							client_state_from_abi_token::<LocalClientTypes>(client_state_token)?;
						if header.signed_header.header.height.value() >
							cs.latest_height.revision_height
						{
							cs.latest_height = Height::new(
								cs.latest_height.revision_number,
								header.signed_header.header.height.into(),
							);
						}
						client_state = Some(cs);
						// TODO: figure out how to distinguish between the same function calls
						break
					}
				}
				// TODO: handle frozen height
			},
			None => {
				log::trace!(target: "hyperspace_ethereum", "no update client event found for blocks ..{at}, looking for a create client event...");

				// ...otherwise, try to get the `CreateClient` event
				let (_, log) = self
					.get_logs_for_event_name::<CreateClientFilter>(
						self.contract_creation_block(),
						at.revision_height,
						&format!("event_data->>'client_id' = '{client_id}'"),
						None,
					)
					.await?
					.pop()
					.ok_or_else(|| ClientError::Other("no events found".to_string()))?;

				let tx_hash = log
					.transaction_hash
					.ok_or(ClientError::Other("tx hash not found".to_string()))?;
				let func = self.yui.function("createClient")?;
				let tx = self
					.client()
					.get_transaction(tx_hash)
					.await
					.map_err(|err| {
						ClientError::Other(format!("failed to get transaction: {}", err))
					})?
					.ok_or_else(|| {
						ClientError::Other(format!("transaction not found: {}", tx_hash))
					})?;

				let Token::Array(batch_calldata) =
					batch_func
						.decode_input(&tx.input[4..])?
						.pop()
						.ok_or(ClientError::Other("batch calldata not found".to_string()))?
				else {
					return Err(ClientError::Other("batch calldata not found".to_string()))
				};

				for input_tok in batch_calldata.into_iter().rev() {
					let Token::Bytes(input) = input_tok else {
						return Err(ClientError::Other("input token should be bytes".to_string()))
					};
					if input[..4] == func.short_signature() {
						let calldata = func
							.decode_input(&input[4..])?
							.pop()
							.ok_or(ClientError::Other("calldata not found".to_string()))?;
						let Token::Tuple(toks) = calldata else {
							return Err(ClientError::Other("calldata should be bytes".to_string()))
						};
						let client_state_token = toks[1].clone();
						client_state = Some(client_state_from_abi_token::<LocalClientTypes>(
							client_state_token,
						)?);
						break
					}
				}
			},
		}

		let proof_height = Some(at.into());
		let proof = self
			.query_proof(at, vec![ClientStatePath(client_id.clone()).to_string().into_bytes()])
			.await?;

		Ok(QueryClientStateResponse {
			client_state: Some(
				client_state
					.ok_or(ClientError::Other("client state not found".to_string()))?
					.to_any(),
			),
			proof_height,
			proof,
		})
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		let (connection_end, exists): (ConnectionEndData, bool) = self
			.yui
			.method("getConnection", (connection_id.to_string(),))
			.map_err(|err| {
				ClientError::Other(format!("contract is missing getConnectionEnd {}", err))
			})?
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await?;

		let connection = if exists {
			let prefix = if connection_end.counterparty.prefix.key_prefix.0.is_empty() {
				None
			} else {
				Some(MerklePrefix {
					key_prefix: connection_end.counterparty.prefix.key_prefix.to_vec(),
				})
			};

			Some(ConnectionEnd {
				client_id: connection_end.client_id,
				versions: connection_end
					.versions
					.into_iter()
					.map(|v| Version { identifier: v.identifier, features: v.features })
					.collect(),
				state: connection_end.state as _,
				counterparty: Some(ConnectionCounterparty {
					client_id: connection_end.counterparty.client_id,
					connection_id: connection_end.counterparty.connection_id,
					prefix,
				}),
				delay_period: connection_end.delay_period,
			})
		} else {
			None
		};

		let proof = self
			.query_proof(at, vec![ConnectionsPath(connection_id.clone()).to_string().into_bytes()])
			.await?;

		Ok(QueryConnectionResponse { connection, proof, proof_height: Some(at.into()) })
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let binding = self
			.yui
			.method::<_, ChannelData>(
				"getChannel",
				(port_id.as_str().to_owned(), channel_id.to_string()),
			)
			.map_err(|err| ClientError::Other(format!("contract is missing getChannel {}", err)))?;

		let channel_data = binding
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.map_err(|err| ClientError::Other(format!("failed to query channel: {}", err)))?;

		let _state = State::from_i32(channel_data.state as _)
			.map_err(|err| ClientError::Other(format!("invalid channel state: {}", err)))?;

		let proof = self
			.query_proof(at, vec![ChannelEndsPath(port_id, channel_id).to_string().into_bytes()])
			.await?;

		Ok(QueryChannelResponse {
			channel: Some(Channel {
				state: channel_data.state as _,
				ordering: channel_data.ordering as _,
				counterparty: Some(ChannelCounterparty {
					port_id: channel_data.counterparty.port_id,
					channel_id: channel_data.counterparty.channel_id,
				}),
				connection_hops: channel_data.connection_hops,
				version: channel_data.version,
			}),
			proof,
			proof_height: Some(at.into()),
		})
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		assert_eq!(keys.len(), 1);
		let key = String::from_utf8(keys[0].clone()).unwrap();
		let (bytes, _) = self.query_proof_with_value(&key, at).await?;
		Ok(bytes)
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		let path = Path::Commitments(CommitmentsPath {
			port_id: port_id.clone(),
			channel_id: channel_id.clone(),
			sequence: Sequence::from(seq),
		})
		.to_string();

		let (proof, bytes) = self.query_proof_with_value(&path, at).await?;
		Ok(QueryPacketCommitmentResponse {
			commitment: bytes,
			proof,
			proof_height: Some(at.into()),
		})
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<ibc_proto::ibc::core::channel::v1::QueryPacketAcknowledgementResponse, Self::Error>
	{
		let path = Path::Acks(AcksPath {
			port_id: port_id.clone(),
			channel_id: channel_id.clone(),
			sequence: Sequence::from(seq),
		})
		.to_string();

		let (proof, bytes) = self.query_proof_with_value(&path, at).await?;

		Ok(ibc_proto::ibc::core::channel::v1::QueryPacketAcknowledgementResponse {
			acknowledgement: bytes,
			proof,
			proof_height: Some(at.into()),
		})
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let binding = self
			.yui
			.method::<_, u64>(
				"getNextSequenceRecv",
				(port_id.as_str().to_owned(), channel_id.to_string()),
			)
			.map_err(|err| {
				ClientError::Other(format!("contract is missing getNextSequenceRecv {}", err))
			})?;

		let path = Path::SeqRecvs(SeqRecvsPath(port_id.clone(), channel_id.clone())).to_string();
		let proof = self.query_proof(at, vec![path.into_bytes()]).await?;

		let seq = binding
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.map_err(|err| ClientError::Other(format!("failed to query channel_data: {}", err)))?;
		Ok(QueryNextSequenceReceiveResponse {
			next_sequence_receive: seq,
			proof,
			proof_height: None,
		})
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		sequence: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		let path = Path::Receipts(ReceiptsPath {
			port_id: port_id.clone(),
			channel_id: channel_id.clone(),
			sequence: Sequence::from(sequence),
		})
		.to_string();

		let proof = self.query_proof(at, vec![path.into_bytes()]).await?;
		let received = self
			.has_packet_receipt(at, port_id.as_str().to_owned(), format!("{channel_id}"), sequence)
			.await?;

		Ok(QueryPacketReceiptResponse { received, proof, proof_height: Some(at.into()) })
	}

	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
		// TODO: fix latest_height_and_timestamp in basic builds
		let number;
		#[cfg(not(feature = "no_beacon"))]
		{
			let prover = self.prover();
			let block = prover.fetch_block("finalized").await?;
			let block2 = prover.fetch_block("head").await?;
			info!(target: "hyperspace_ethereum", "latest_height_and_timestamp: finalized: {} ({}), head: {} ({})", block.slot, block.body.execution_payload.block_number, block2.slot, block2.body.execution_payload.block_number);
			number =
				block2.body.execution_payload.block_number.saturating_sub(INDEXER_DELAY_BLOCKS);
		}
		#[cfg(feature = "no_beacon")]
		{
			number = self
				.client()
				.get_block_number()
				.await
				.map_err(|err| ClientError::Other(format!("failed to get block number: {}", err)))?
				.as_u64();
		}

		let height = Height::new(0, number.into());
		let block = self
			.client()
			.get_block(BlockId::Number(number.into()))
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.ok_or_else(|| ClientError::Other("block not found".to_string()))?;

		let nanoseconds = Duration::from_secs(block.timestamp.as_u64()).as_nanos() as u64;
		let timestamp = Timestamp::from_nanoseconds(nanoseconds).map_err(|e| {
			ClientError::Other(format!("failed to parse timestamp : {nanoseconds}, err: {e}"))
		})?;

		log::info!(target: "hyperspace_ethereum", "latest_height_and_timestamp: {height:?}, {timestamp:?}");

		Ok((height, timestamp))
	}

	async fn query_packet_commitments(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let start_seq = 0u64;
		let end_seq = self
			.query_next_send_sequence(at, channel_id.clone(), port_id.clone())
			.await?
			.0
			.saturating_sub(1);

		let mut seqs = vec![];
		for (start, end) in create_intervals(start_seq, end_seq) {
			let binding = self
				.yui
				.method(
					"hasCommitments",
					(port_id.as_str().to_owned(), channel_id.to_string(), start, end),
				)
				.map_err(|err| {
					ClientError::Other(format!("contract is missing hasCommitments {}", err))
				})?;

			let bitmap: U256 = binding
				.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
				.call()
				.await
				.map_err(|err| {
					ClientError::Other(format!("failed to query_packet_commitments: {}", err))
				})?;
			for i in 0..SEQUENCES_PER_ITER {
				if bitmap.bit(i as _).into() {
					seqs.push(start + i);
				}
			}
		}

		// next_ack is the sequence number used when acknowledging packets.
		// the value of next_ack is the sequence number of the next packet to be acknowledged yet.
		// aka the last acknowledged packet was next_ack - 1.

		// this function is called to calculate which acknowledgements have not yet been
		// relayed from this chain to the counterparty chain.
		Ok(seqs)
	}

	async fn query_next_send_sequence(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Sequence, Self::Error> {
		let binding = self
			.yui
			.method("getNextSequenceSend", (port_id.as_str().to_owned(), channel_id.to_string()))
			.map_err(|err| {
				ClientError::Other(format!("contract is missing hasCommitments {}", err))
			})?;

		let next_sequence: U256 = binding
			.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
			.call()
			.await
			.map_err(|err| {
				ClientError::Other(format!("failed to query_packet_commitments: {}", err))
			})?;
		Ok(Sequence(next_sequence.as_u64()))
	}

	async fn query_packet_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		next_send_seq: Sequence,
	) -> Result<Vec<u64>, Self::Error> {
		let start_seq = 0u64;
		let end_seq = next_send_seq.0;
		let mut seqs = vec![];
		for (start, end) in create_intervals(start_seq, end_seq) {
			let binding = self
				.yui
				.method(
					"hasAcknowledgements",
					(port_id.as_str().to_owned(), channel_id.to_string(), start, end),
				)
				.map_err(|err| {
					ClientError::Other(format!("contract is missing hasAcknowledgements {}", err))
				})?;

			let bitmap: U256 = binding
				.block(BlockId::Number(BlockNumber::Number(at.revision_height.into())))
				.call()
				.await
				.map_err(|err| {
					ClientError::Other(format!("failed to query_packet_acknowledgements: {}", err))
				})?;
			for i in 0..SEQUENCES_PER_ITER {
				if bitmap.bit(i as _).into() {
					seqs.push(start + i);
				}
			}
		}

		// next_ack is the sequence number used when acknowledging packets.
		// the value of next_ack is the sequence number of the next packet to be acknowledged yet.
		// aka the last acknowledged packet was next_ack - 1.

		// this function is called to calculate which acknowledgements have not yet been
		// relayed from this chain to the counterparty chain.
		Ok(seqs)
	}

	async fn query_unreceived_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let mut pending = vec![];

		for seq in seqs {
			let received = self
				.has_packet_receipt(at, port_id.as_str().to_owned(), format!("{channel_id}"), seq)
				.await?;

			if !received {
				pending.push(seq);
			}
		}

		Ok(pending)
	}

	async fn query_unreceived_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let mut pending = vec![];

		for seq in seqs {
			let received = !self
				.has_commitment(at, port_id.as_str().to_owned(), format!("{channel_id}"), seq)
				.await?;

			if !received {
				pending.push(seq);
			}
		}

		Ok(pending)
	}

	fn channel_whitelist(&self) -> HashSet<(ChannelId, PortId)> {
		self.channel_whitelist.lock().unwrap().clone().into_iter().collect()
	}

	async fn query_send_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let source_port = port_id.to_string();
		let source_channel = channel_id.to_string();

		if seqs.is_empty() {
			return Ok(vec![])
		}

		let sequences = seqs.clone().into_iter().map(|seq| format!("'{seq}'")).join(",");
		let logs = self
			.get_logs_for_event_name::<SendPacketFilter>(
				self.contract_creation_block(),
				BlockNumber::Latest,
				&format!(
					"event_data->>'sequence' IN ({sequences}) AND event_data->>'source_port' = '{source_port}' AND event_data->>'source_channel' = '{source_channel}'",
				),
				None
			)
			.await?;

		if logs.is_empty() {
			info!("no logs found for query_send_packets");
			return Ok(vec![])
		}

		let channel = self.query_channel_end(at, channel_id, port_id).await?;

		let channel = channel.channel.ok_or(ClientError::Other("channel is none".to_string()))?;
		let counterparty = channel
			.counterparty
			.ok_or(ClientError::Other("counterparty is none".to_string()))?;

		let mut ret = vec![];
		for (_, log) in logs.into_iter() {
			let value = SendPacketFilter::decode_log(&log.clone().into())
				.map_err(|err| ClientError::Other(format!("failed to decode log: {}", err)))?;
			if !seqs.contains(&value.sequence) {
				continue
			}
			let packet_info = PacketInfo {
				height: Some(
					log.block_number
						.ok_or(ClientError::Other("block number not found".to_string()))?
						.as_u64()
						.into(),
				),
				source_port: source_port.clone(),
				source_channel: source_channel.clone(),
				destination_port: counterparty.port_id.clone(),
				destination_channel: counterparty.channel_id.clone(),
				sequence: value.sequence,
				timeout_height: value.timeout_height.into(),
				timeout_timestamp: value.timeout_timestamp,
				data: value.data.to_vec(),
				channel_order: Order::from_i32(channel.ordering)
					.map_err(|_| ClientError::Other("invalid channel order".to_string()))?
					.to_string(),
				ack: None,
			};
			ret.push(packet_info);
		}
		Ok(ret)
	}

	async fn query_received_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let destination_port = port_id.to_string();
		let destination_channel = channel_id.to_string();
		if seqs.is_empty() {
			return Ok(vec![])
		}

		let destination_port_indexed = H256::from_slice(&encode(&[Token::FixedBytes(
			keccak256(destination_port.clone().into_bytes()).to_vec(),
		)]));
		let destination_channel_indexed = H256::from_slice(&encode(&[Token::FixedBytes(
			keccak256(destination_channel.clone().into_bytes()).to_vec(),
		)]));

		let sequences = seqs.clone().into_iter().map(|seq| format!("'{seq}'")).join(",");
		let logs = self
			.get_logs_for_event_name::<RecvPacketFilter>(
				self.contract_creation_block(),
				BlockNumber::Latest,
				&format!(
					"event_data->>'sequence' IN ({sequences}) AND event_data->>'destination_port_indexed' = '{destination_port_indexed:?}' AND event_data->>'destination_channel_indexed' = '{destination_channel_indexed:?}'",
				),
				None,
			)
			.await?;

		let channel = self.query_channel_end(at, channel_id, port_id).await?;
		let channel = channel.channel.ok_or(ClientError::Other("channel is none".to_string()))?;

		let acks = self
			.get_logs_for_event_name::<WriteAcknowledgementFilter>(
				self.contract_creation_block(),
				BlockNumber::Latest,
				&format!(
					"event_data->>'sequence' IN ({sequences}) AND event_data->>'destination_port' = '{destination_port}' AND event_data->>'destination_channel' = '{destination_channel}'",
				),
				None,
			)
			.await?;

		let mut acks_map = acks
			.into_iter()
			.map(|(ack, _log)| (ack.sequence, ack.acknowledgement.to_vec()))
			.collect::<HashMap<_, _>>();

		let mut ret = vec![];

		for (_, log) in logs.into_iter() {
			let value = RecvPacketFilter::decode_log(&log.clone().into()).map_err(|err| {
				ClientError::Other(format!("failed to decode log in query_send_packets: {}", err))
			})?;
			if !seqs.contains(&value.sequence) {
				continue
			}
			let packet_info = PacketInfo {
				height: Some(
					log.block_number
						.ok_or(ClientError::Other("block number not found".to_string()))?
						.as_u64()
						.into(),
				),
				source_port: value.source_port.clone(),
				source_channel: value.source_channel.clone(),
				destination_port: destination_port.clone(),
				destination_channel: destination_channel.clone(),
				sequence: value.sequence,
				timeout_height: value.timeout_height.into(),
				timeout_timestamp: value.timeout_timestamp,
				data: value.data.to_vec(),
				channel_order: Order::from_i32(channel.ordering)
					.map_err(|_| {
						Self::Error::Other("invalid channel order in query_send_packets".to_owned())
					})?
					.to_string(),
				ack: acks_map.get(&value.sequence).cloned(),
			};
			ret.push(packet_info);
		}

		Ok(ret)
	}

	fn expected_block_time(&self) -> Duration {
		Duration::from_millis(12070)
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error> {
		info!(target: "hyperspace_ethereum", "query_client_update_time_and_height: {client_id:?}, {client_height:?}");

		let result = self
			.get_logs_for_event_name::<UpdateClientHeightFilter>(
				self.contract_creation_block(),
				BlockNumber::Latest,
				&format!(
					"event_data->>'client_id' = '{client_id}' AND event_data->'client_height'->>'revision_number' = '{}' AND event_data->'client_height'->>'revision_height' = '{}'",
					client_height.revision_number, client_height.revision_height
				),
				None,
			)
			.await?.pop().ok_or_else(|| Self::Error::Other("no logs found".to_owned())).map(|(_, log)| log.block_number);
		let block_number = match result {
			Ok(v) => v,
			Err(_) => {
				 self
					.get_logs_for_event_name::<CreateClientHeightFilter>(
						self.contract_creation_block(),
						BlockNumber::Latest,
						&format!(
							"event_data->>'client_id' = '{client_id}' AND event_data->'client_height'->>'revision_number' = '{}' AND event_data->'client_height'->>'revision_height' = '{}'",
							client_height.revision_number, client_height.revision_height
						),
						None,
					)
					.await?.pop().ok_or_else(|| Self::Error::Other("no logs found".to_owned()))?.1.block_number
			},
		};

		let height = Height::new(
			0,
			block_number
				.ok_or(ClientError::Other(
					"block number not found in query_client_update_time_and_height".to_string(),
				))?
				.as_u64(),
		);

		let timestamp =
			Timestamp::from_nanoseconds(self.query_timestamp_at(height.revision_height).await?)
				.map_err(|err| {
					ClientError::Other(format!(
						"failed to get timestamp in query_client_update_time_and_height: {}",
						err
					))
				})?;

		Ok((height, timestamp))
	}

	async fn query_host_consensus_state_proof(
		&self,
		_client_state: &AnyClientState,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		Ok(Some(vec![]))
	}

	async fn query_ibc_balance(
		&self,
		asset_id: Self::AssetId,
	) -> Result<Vec<PrefixedCoin>, Self::Error> {
		let balance = self
			.yui
			.method_diamond::<_, U256>(
				"balanceOf",
				(
					H160::from_str(&self.account_id().to_string()).map_err(|_| {
						ClientError::Other("failed get bytes from account id".to_string())
					})?,
					asset_id.clone(),
				),
				ContractName::IbcTransferDiamond,
			)?
			.call()
			.await?;
		Ok(vec![PrefixedCoin {
			denom: PrefixedDenom {
				trace_path: TracePath::default(),
				base_denom: BaseDenom::from_str(&asset_id)?,
			},
			amount: Amount::from(balance),
		}])
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		CommitmentPrefix::try_from(self.config.commitment_prefix()).unwrap()
	}

	#[track_caller]
	fn client_id(&self) -> ClientId {
		self.client_id
			.lock()
			.unwrap()
			.as_ref()
			.expect("Client Id should be defined")
			.clone()
	}

	fn set_client_id(&mut self, client_id: ClientId) {
		*self.client_id.lock().unwrap() = Some(client_id);
	}

	fn connection_id(&self) -> Option<ConnectionId> {
		self.connection_id.lock().unwrap().clone()
	}

	fn set_channel_whitelist(&mut self, channel_whitelist: HashSet<(ChannelId, PortId)>) {
		*self.channel_whitelist.lock().unwrap() = channel_whitelist;
	}

	fn add_channel_to_whitelist(&mut self, channel: (ChannelId, PortId)) {
		self.channel_whitelist.lock().unwrap().insert(channel);
	}

	fn set_connection_id(&mut self, connection_id: ConnectionId) {
		*self.connection_id.lock().unwrap() = Some(connection_id);
	}

	fn client_type(&self) -> ClientType {
		"xx-ethereum".to_string()
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		let block = self
			.client()
			.get_block(BlockId::Number(BlockNumber::Number(block_number.into())))
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.ok_or_else(|| ClientError::Other("block not found".to_string()))?;

		Ok(Duration::from_secs(block.timestamp.as_u64()).as_nanos() as u64)
	}

	async fn query_clients(&self, _client_type: &ClientType) -> Result<Vec<ClientId>, Self::Error> {
		let event_filter = self
			.yui
			.event_for_name::<GeneratedClientIdentifierFilter>("GeneratedClientIdentifier")
			.map_err(|err| ClientError::ContractAbiError(err))?
			.from_block(self.contract_creation_block())
			.address(ValueOrArray::Value(self.yui.ibc_core_diamond.address()))
			.to_block(BlockNumber::Latest);

		// TODO: filter by client type?

		let logs = self
			.yui
			.ibc_core_diamond
			.client()
			.get_logs(&event_filter.filter)
			.await
			.map_err(|err| {
				ClientError::Other(format!("failed to get logs in query_clients: {}", err))
			})?;

		let mut client_ids: Vec<ClientId> = vec![];

		for log in logs.into_iter() {
			let value = GeneratedClientIdentifierFilter::decode_log(&log.clone().into()).map_err(
				|err| ClientError::Other(format!("failed to decode log in query_clients: {}", err)),
			)?;
			client_ids.push(value.0.parse()?);
		}

		Ok(client_ids)
	}

	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		// let ids = self.generated_channel_identifiers(0.into()).await?;
		// dbg!(&ids);
		// ids.into_iter()
		// 	.map(|id| Ok((id.1.parse().unwrap(), id.0.parse().unwrap())))
		// 	.collect()
		Ok(vec![])
	}

	async fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error> {
		let event_filter = self
			.yui
			.event_for_name::<GeneratedConnectionIdentifierFilter>("GeneratedConnectionIdentifier")
			.map_err(|err| ClientError::ContractAbiError(err))?
			.address(ValueOrArray::Value(self.yui.ibc_core_diamond.address()))
			.from_block(self.contract_creation_block())
			.to_block(BlockNumber::Number(height.into()));

		let logs = self
			.yui
			.ibc_core_diamond
			.client()
			.get_logs(&event_filter.filter)
			.await
			.map_err(|err| {
				ClientError::Other(format!(
					"failed to get logs in query_connection_using_client: {}",
					err
				))
			})?;

		let mut connections: Vec<IdentifiedConnection> = vec![];

		for log in logs.into_iter() {
			let value = GeneratedConnectionIdentifierFilter::decode_log(&log.clone().into())
				.map_err(|err| {
					ClientError::Other(format!(
						"failed to decode log in query_connection_using_client: {}",
						err
					))
				})?;

			let connection_id: ConnectionId = value.0.parse()?;
			let connection_end = self
				.query_connection_end(Height::new(0, height.into()), connection_id.clone())
				.await?;

			let conn = connection_end.connection.unwrap();
			if conn.client_id != client_id {
				continue
			}

			connections.push(IdentifiedConnection {
				id: connection_id.to_string(),
				client_id: conn.client_id,
				versions: conn.versions,
				state: conn.state.into(),
				counterparty: conn.counterparty,
				delay_period: conn.delay_period.into(),
			});
		}

		Ok(connections)
	}

	async fn query_connection_channels(
		&self,
		height: Height,
		connection_id: &ConnectionId,
	) -> Result<QueryChannelsResponse, Self::Error> {
		let event_filter = self
			.yui
			.event_for_name::<GeneratedChannelIdentifierFilter>("GeneratedChannelIdentifier")
			.map_err(|err| ClientError::ContractAbiError(err))?
			.from_block(self.contract_creation_block())
			.address(ValueOrArray::Value(self.yui.ibc_core_diamond.address()))
			.to_block(BlockNumber::Number(height.revision_height.into()));

		let logs = self
			.yui
			.ibc_core_diamond
			.client()
			.get_logs(&event_filter.filter)
			.await
			.map_err(|err| {
				ClientError::Other(format!(
					"failed to get logs in query_connection_channels: {}",
					err
				))
			})?;

		let mut channels: Vec<IdentifiedChannel> = vec![];

		for log in logs.into_iter() {
			let value = GeneratedChannelIdentifierFilter::decode_log(&log.clone().into()).map_err(
				|err| {
					ClientError::Other(format!(
						"failed to decode log in query_connection_channels: {}",
						err
					))
				},
			)?;
			let port_id = PortId::transfer(); // FIXME: query port id from contract
			let channel_id = value.0.parse()?;
			let Some(channel_end) =
				self.query_channel_end(height, channel_id, port_id.clone()).await?.channel
			else {
				continue
			};

			if !channel_end.connection_hops.contains(&connection_id.to_string()) {
				info!(
					"connection id mismatch: {connection_id} !∈ {:?}",
					channel_end.connection_hops
				);
				continue
			}

			channels.push(IdentifiedChannel {
				state: channel_end.state,
				ordering: channel_end.ordering,
				counterparty: channel_end.counterparty,
				connection_hops: channel_end.connection_hops,
				channel_id: channel_id.to_string(),
				port_id: port_id.to_string(),
				version: channel_end.version,
			});
		}

		Ok(QueryChannelsResponse { channels, pagination: None, height: None })
	}

	async fn is_update_required(
		&self,
		latest_height: u64,
		latest_client_height_on_counterparty: u64,
	) -> Result<bool, Self::Error> {
		Ok(false)
	}

	#[cfg(not(feature = "no_beacon"))]
	async fn initialize_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Self::Error> {
		let sync_committee_prover = self.prover();
		let block_id = "head";
		let block_header = sync_committee_prover.fetch_header(&block_id).await.map_err(|err| {
			ClientError::Other(format!(
				"failed to fetch header in initialize_client_state: {}",
				err
			))
		})?;

		let state = sync_committee_prover
			.fetch_beacon_state(&block_header.slot.to_string())
			.await
			.map_err(|err| {
				ClientError::Other(format!(
					"failed to fetch beacon state in initialize_client_state: {}",
					err
				))
			})?;
		// TODO: query `at` block
		// let finality_checkpoint =
		// sync_committee_prover.fetch_finalized_checkpoint().await.unwrap();

		let epoch = state.current_justified_checkpoint.epoch;
		let period_at_slot = compute_sync_committee_period_at_slot(block_header.slot);
		info!(target: "hyperspace_ethereum", "Using init state period: {period_at_slot}");
		let client_state = LightClientState {
			finalized_header: block_header.clone(),
			latest_finalized_epoch: compute_epoch_at_slot(block_header.slot),
			current_sync_committee: state.current_sync_committee,
			next_sync_committee: state.next_sync_committee,
			state_period: period_at_slot,
		};

		let execution_header = state.latest_execution_payload_header;
		info!(target: "hyperspace_ethereum", "Using init epoch: {epoch}, and height: {}", execution_header.block_number);

		let client_state = AnyClientState::Ethereum(ClientState {
			inner: client_state,
			frozen_height: None,
			latest_height: execution_header.block_number as _,
			ibc_core_address: self.yui.ibc_core_diamond.address(),
			next_upgrade_id: 0,
			_phantom: Default::default(),
		});

		let consensus_state = AnyConsensusState::Ethereum(ConsensusState {
			timestamp: tendermint::time::Time::from_unix_timestamp(
				execution_header.timestamp as i64,
				0,
			)
			.map_err(|err| {
				ClientError::Other(format!(
					"failed to get timestamp in initialize_client_state: {}, timestamp{}",
					err, execution_header.timestamp
				))
			})?
			.into(),
			root: CommitmentRoot { bytes: execution_header.state_root.to_vec() },
		});

		Ok((client_state, consensus_state))
	}

	#[cfg(feature = "no_beacon")]
	async fn initialize_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Self::Error> {
		let client = self.client();
		let block_number = client
			.get_block_number()
			.await
			.map_err(|err| {
				ClientError::Other(format!(
					"failed to get block number in initialize_client_state: {}",
					err
				))
			})?
			.as_u64();
		let block = client
			.get_block(BlockId::Number(BlockNumber::Number(block_number.into())))
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.ok_or(ClientError::Other(format!("not able to find a block : {block_number}")))?;

		let client_state = LightClientState {
			finalized_header: BeaconBlockHeader {
				slot: block_number,
				proposer_index: 0,
				parent_root: Default::default(),
				state_root: Node(block.state_root.0),
				body_root: Node(block.state_root.0),
			},
			latest_finalized_epoch: 0,
			current_sync_committee: Default::default(),
			next_sync_committee: Default::default(),
			state_period: 0,
		};

		let client_state = AnyClientState::Ethereum(ClientState {
			inner: client_state,
			frozen_height: None,
			latest_height: block_number as _,
			ibc_core_address: self.yui.ibc_core_diamond.address(),
			next_upgrade_id: 0,
			_phantom: Default::default(),
		});

		let consensus_state = AnyConsensusState::Ethereum(ConsensusState {
			timestamp: tendermint::time::Time::from_unix_timestamp(
				block.timestamp.as_u64() as i64,
				0,
			)
			.map_err(|err| {
				ClientError::Other(format!(
					"failed to get timestamp in initialize_client_state: {}, timestamp{}",
					err, block.timestamp
				))
			})?
			.into(),
			root: CommitmentRoot { bytes: block.state_root.0.to_vec() },
		});

		Ok((client_state, consensus_state))
	}

	async fn query_client_id_from_tx_hash(
		&self,
		(block_hash, tx_hash): Self::TransactionId,
	) -> Result<ClientId, Self::Error> {
		let block_number = self
			.client()
			.get_block(BlockId::Hash(block_hash))
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.ok_or(ClientError::Other(format!(
				"not able to find a block : {}",
				block_hash.to_string()
			)))?
			.number
			.ok_or(ClientError::Other(format!(
				"not able to find a block number with hash: {}",
				block_hash.to_string()
			)))?;
		let event_filter = self
			.yui
			.event_for_name::<GeneratedClientIdentifierFilter>("GeneratedClientIdentifier")
			.map_err(|err| ClientError::ContractAbiError(err))?
			.address(ValueOrArray::Value(self.yui.ibc_core_diamond.address()))
			.from_block(block_number)
			.to_block(block_number);
		let log = self
			.yui
			.ibc_core_diamond
			.client()
			.get_logs(&event_filter.filter)
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.into_iter()
			.find(|log| log.transaction_hash == Some(tx_hash))
			.ok_or(ClientError::Other(format!(
				"not able to find a log with tx hash: {}",
				tx_hash.to_string()
			)))?;

		let decoded_log = GeneratedClientIdentifierFilter::decode_log(&log.clone().into())
			.map_err(|err| {
				ClientError::Other(format!(
					"failed to decode log in query_client_id_from_tx_hash: {}",
					err
				))
			})?;
		Ok(decoded_log.0.parse()?)
	}

	async fn query_connection_id_from_tx_hash(
		&self,
		(block_hash, tx_hash): Self::TransactionId,
	) -> Result<ConnectionId, Self::Error> {
		let block_number = self
			.client()
			.get_block(BlockId::Hash(block_hash))
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.ok_or(ClientError::Other(format!(
				"not able to find a block with hash: {}",
				block_hash.to_string()
			)))?
			.number
			.ok_or(ClientError::Other(format!(
				"not able to find a block number with hash: {}",
				block_hash.to_string()
			)))?;
		let event_filter = self
			.yui
			.event_for_name::<OpenInitConnectionFilter>("OpenInitConnection")
			.map_err(|err| ClientError::ContractAbiError(err))?
			.from_block(block_number)
			.address(ValueOrArray::Value(self.yui.ibc_core_diamond.address()))
			.to_block(block_number);
		let log = self
			.yui
			.ibc_core_diamond
			.client()
			.get_logs(&event_filter.filter)
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.into_iter()
			.find(|log| log.transaction_hash == Some(tx_hash))
			.ok_or(ClientError::Other(format!(
				"not able to find a log with tx hash: {}",
				tx_hash.to_string()
			)))?;

		let decoded_log = OpenInitConnectionFilter::decode_log(&log.clone().into()).map_err(
			|err| ClientError::Other(format!("failed to decode OpenInitConnectionFilter in query_connection_id_from_tx_hash: {}", err)),
		)?;
		Ok(decoded_log.connection_id.parse()?)
	}

	async fn query_channel_id_from_tx_hash(
		&self,
		(block_hash, tx_hash): Self::TransactionId,
	) -> Result<(ChannelId, PortId), Self::Error> {
		let block_number = self
			.client()
			.get_block(BlockId::Hash(block_hash))
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.ok_or(ClientError::Other(format!(
				"not able to find a block with hash: {}",
				block_hash.to_string()
			)))?
			.number
			.ok_or(ClientError::Other(format!(
				"not able to find a block number with hash: {}",
				block_hash.to_string()
			)))?;
		let event_filter = self
			.yui
			.event_for_name::<OpenInitChannelFilter>("OpenInitChannel")
			.map_err(|err| ClientError::ContractAbiError(err))?
			.from_block(block_number)
			.address(ValueOrArray::Value(self.yui.ibc_core_diamond.address()))
			.to_block(block_number);
		let log = self
			.yui
			.ibc_core_diamond
			.client()
			.get_logs(&event_filter.filter)
			.await
			.map_err(|err| ClientError::MiddlewareError(err))?
			.into_iter()
			.find(|log| {
				if let Some(transaction_hash) = &log.transaction_hash {
					*transaction_hash == tx_hash
				} else {
					log::error!(target: "hyperspace_ethereum", "tx hash should exist: {tx_hash}");
					return false
				}
			})
			.ok_or(ClientError::Other(format!(
				"not able to find a log with tx hash: {}",
				tx_hash.to_string()
			)))?;

		let decoded_log =
			OpenInitChannelFilter::decode_log(&log.clone().into()).map_err(|err| {
				ClientError::Other(format!(
					"failed to decode OpenInitChannelFilter in query_channel_id_from_tx_hash: {}",
					err
				))
			})?;
		Ok((decoded_log.channel_id.parse()?, decoded_log.port_id.parse()?))
	}

	async fn upload_wasm(&self, _wasm: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
		unimplemented!("upload_wasm")
	}
}

pub(crate) fn u256_to_bytes(n: &U256) -> Vec<u8> {
	let mut bytes = vec![0u8; 256 / 8];
	n.to_big_endian(&mut bytes);
	bytes
}
