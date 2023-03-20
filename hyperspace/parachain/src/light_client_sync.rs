use codec::Encode;
use std::{
	collections::{BTreeMap, BTreeSet, HashMap},
	fmt::Display,
};

use finality_grandpa::BlockNumberOps;
use grandpa_light_client_primitives::{ParachainHeaderProofs, ParachainHeadersWithFinalityProof};
use ibc_proto::google::protobuf::Any;
use sp_core::H256;
use sp_runtime::{
	traits::{IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
use subxt::config::{
	extrinsic_params::{BaseExtrinsicParamsBuilder, ExtrinsicParams},
	Header as HeaderT,
};

use grandpa_prover::GrandpaProver;
use ibc::core::ics02_client::msgs::update_client::MsgUpdateAnyClient;
use tendermint_proto::Protobuf;

use ibc::{
	core::ics24_host::identifier::ClientId, events::IbcEvent, signer::Signer, tx_msg::Msg, Height,
};
use ibc_rpc::{BlockNumberOrHash, IbcApiClient};
use ics10_grandpa::client_message::{ClientMessage, Header as GrandpaHeader};
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState};

use primitives::{mock::LocalClientTypes, Chain, KeyProvider, LightClientSync};

use super::{error::Error, ParachainClient};
use crate::finality_protocol::FinalityProtocol;

#[async_trait::async_trait]
impl<T: light_client_common::config::Config + Send + Sync> LightClientSync for ParachainClient<T>
where
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>,
	u32: From<<T as subxt::Config>::BlockNumber>,
	Self: KeyProvider,
	<<T as light_client_common::config::Config>::Signature as Verify>::Signer:
		From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	<T as subxt::Config>::Signature: From<MultiSignature> + Send + Sync,
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256> + From<[u8; 32]>,
	sp_core::H256: From<T::Hash>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, T::Tip>> + Send + Sync,
	<T as subxt::Config>::AccountId: Send + Sync,
	<T as subxt::Config>::Address: Send + Sync,
{
	async fn is_synced<C: Chain>(&self, counterparty: &C) -> Result<bool, anyhow::Error> {
		let latest_height = counterparty.latest_height_and_timestamp().await?.0;
		let response = counterparty.query_client_state(latest_height, self.client_id()).await?;
		let any_client_state = response.client_state.ok_or_else(|| {
			Error::Custom("Received an empty client state from counterparty".to_string())
		})?;

		match self.finality_protocol {
			FinalityProtocol::Grandpa => {
				let prover = self.grandpa_prover();
				let AnyClientState::Grandpa(client_state) = AnyClientState::decode_recursive(any_client_state, |c| matches!(c, AnyClientState::Grandpa(_)))
					.ok_or_else(|| Error::Custom(format!("Could not decode client state")))? else { unreachable!() };

				let latest_hash = self.relay_client.rpc().finalized_head().await?;
				let finalized_head =
					self.relay_client.rpc().header(Some(latest_hash)).await?.ok_or_else(|| {
						Error::Custom(format!("Expected finalized header, found None"))
					})?;
				let previous_finalized_height = client_state.latest_relay_height;
				let session_length = prover.session_length().await?;
				let (.., session_end_block) =
					prover.session_start_and_end_for_block(previous_finalized_height).await?;
				let latest_finalized_height = u32::from(finalized_head.number());
				let session_changes =
					latest_finalized_height.saturating_sub(session_end_block) / session_length;
				// If no session changes have occurred between the last update and the latest
				// finalized height then the light client is still in sync
				Ok(session_changes == 0)
			},
			FinalityProtocol::Beefy => unimplemented!(),
		}
	}

	async fn fetch_mandatory_updates<C: Chain>(
		&self,
		counterparty: &C,
	) -> Result<(Vec<Any>, Vec<IbcEvent>), anyhow::Error> {
		let latest_height = counterparty.latest_height_and_timestamp().await?.0;
		let response = counterparty.query_client_state(latest_height, self.client_id()).await?;
		let any_client_state = response.client_state.ok_or_else(|| {
			Error::Custom("Received an empty client state from counterparty".to_string())
		})?;

		let (messages, events) = match self.finality_protocol {
			FinalityProtocol::Grandpa => {
				let prover = self.grandpa_prover();
				let AnyClientState::Grandpa(client_state) = AnyClientState::decode_recursive(any_client_state, |c| matches!(c, AnyClientState::Grandpa(_)))
					.ok_or_else(|| Error::Custom(format!("Could not decode client state")))? else { unreachable!() };
				let latest_hash = self.relay_client.rpc().finalized_head().await?;
				let finalized_head =
					self.relay_client.rpc().header(Some(latest_hash)).await?.ok_or_else(|| {
						Error::Custom(format!("Expected finalized header, found None"))
					})?;
				let latest_finalized_height = u32::from(finalized_head.number());
				let (mut messages, mut events, previous_para_height, previous_finalized_height) =
					self.query_missed_grandpa_updates(
						client_state.latest_para_height,
						client_state.latest_relay_height,
						latest_finalized_height,
						self.client_id(),
						counterparty.account_id(),
					)
					.await?;
				let (latest_message, evs, ..) = get_message(
					prover,
					previous_para_height,
					previous_finalized_height,
					latest_finalized_height,
					self.client_id(),
					counterparty.account_id(),
					&self.name,
					self.para_id,
				)
				.await?;
				messages.push(latest_message);
				events.extend(evs);
				(messages, events)
			},
			// Current implementation of Beefy needs to be revised
			FinalityProtocol::Beefy => unimplemented!(),
		};

		Ok((messages, events))
	}
}

impl<T: light_client_common::config::Config + Send + Sync> ParachainClient<T>
where
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>,
	Self: KeyProvider,
	<<T as light_client_common::config::Config>::Signature as Verify>::Signer:
		From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	<T as subxt::Config>::Signature: From<MultiSignature> + Send + Sync,
	H256: From<T::Hash>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	<T as subxt::Config>::AccountId: Send + Sync,
	<T as subxt::Config>::Address: Send + Sync,
{
	/// Returns a tuple of the client update messages in the exclusive range
	/// `previous_finalized_height..latest_finalized_height`, relay chain block of the last message
	/// in the list and latest parachain block finalized by the last message in the list
	pub async fn query_missed_grandpa_updates(
		&self,
		mut previous_finalized_para_height: u32,
		mut previous_finalized_height: u32,
		latest_finalized_height: u32,
		client_id: ClientId,
		signer: Signer,
	) -> Result<(Vec<Any>, Vec<IbcEvent>, u32, u32), anyhow::Error> {
		let prover = self.grandpa_prover();
		let session_length = prover.session_length().await?;
		let mut session_end_block = {
			let (.., mut session_block_end) =
				prover.session_start_and_end_for_block(previous_finalized_height).await?;
			if session_block_end == previous_finalized_height {
				session_block_end += session_length;
			}
			session_block_end
		};

		// Get all session change blocks between previously finalized relaychain height and latest
		// finalized height
		let mut messages = vec![];
		let mut events = vec![];
		while session_end_block < latest_finalized_height {
			let (msg, evs, previous_para_height, ..) = get_message(
				self.grandpa_prover(),
				previous_finalized_para_height,
				previous_finalized_height,
				session_end_block,
				client_id.clone(),
				signer.clone(),
				&self.name,
				self.para_id,
			)
			.await?;
			messages.push(msg);
			events.extend(evs);
			previous_finalized_height = session_end_block;
			previous_finalized_para_height = previous_para_height;
			session_end_block += session_length;
		}
		Ok((messages, events, previous_finalized_para_height, previous_finalized_height))
	}
}

/// Return a single client update message
async fn get_message<T: light_client_common::config::Config>(
	prover: GrandpaProver<T>,
	previous_finalized_para_height: u32,
	previous_finalized_height: u32,
	latest_finalized_height: u32,
	client_id: ClientId,
	signer: Signer,
	name: &str,
	para_id: u32,
) -> Result<(Any, Vec<IbcEvent>, u32, u32), anyhow::Error>
where
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>
		+ From<<T as subxt::Config>::BlockNumber>,
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	H256: From<T::Hash>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
{
	// fetch the latest finalized parachain header
	let finalized_para_header =
		prover.query_latest_finalized_parachain_header(latest_finalized_height).await?;
	let finalized_para_height = u32::from(finalized_para_header.number());
	let latest_finalized_para_height = finalized_para_height;
	let finalized_blocks =
		((previous_finalized_para_height + 1)..=latest_finalized_para_height).collect::<Vec<_>>();

	if !finalized_blocks.is_empty() {
		log::info!(
			"Fetching events from {} for blocks {}..{}",
			name,
			finalized_blocks[0],
			finalized_blocks.last().unwrap(),
		);
	}

	let finalized_block_numbers = finalized_blocks
		.iter()
		.map(|h| BlockNumberOrHash::Number(*h))
		.collect::<Vec<_>>();

	// block_number => events
	let events: HashMap<String, Vec<IbcEvent>> = IbcApiClient::<
		u32,
		H256,
		<T as light_client_common::config::Config>::AssetId,
	>::query_events(
		&*prover.para_ws_client, finalized_block_numbers
	)
	.await?;

	// header number is serialized to string
	let mut headers_with_events = events
		.iter()
		.filter_map(|(num, events)| {
			if events.is_empty() {
				None
			} else {
				str::parse::<u32>(&*num).ok().map(T::BlockNumber::from)
			}
		})
		.collect::<BTreeSet<_>>();

	// We ensure we advance the finalized latest parachain height
	if previous_finalized_para_height < finalized_para_height {
		headers_with_events.insert(finalized_para_header.number());
	}

	let events: Vec<IbcEvent> = events.into_values().flatten().collect();
	let ParachainHeadersWithFinalityProof { finality_proof, parachain_headers, .. } = prover
		.query_finalized_parachain_headers_with_proof::<T::Header>(
			previous_finalized_height,
			latest_finalized_height,
			None,
			headers_with_events.into_iter().collect(),
		)
		.await?;

	let grandpa_header = GrandpaHeader {
		finality_proof: codec::Decode::decode(&mut &*finality_proof.encode())
			.expect("Same struct from different crates,decode should not fail"),
		parachain_headers: parachain_headers.into(),
		height: Height::new(para_id as u64, finalized_para_height as u64),
	};

	let msg = MsgUpdateAnyClient::<LocalClientTypes> {
		client_id,
		client_message: AnyClientMessage::Grandpa(ClientMessage::Header(grandpa_header)),
		signer,
	};
	let value = msg.encode_vec()?;
	Result::<_, anyhow::Error>::Ok((
		Any { value, type_url: msg.type_url() },
		events,
		latest_finalized_para_height,
		latest_finalized_height,
	))
}
