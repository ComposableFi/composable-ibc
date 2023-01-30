use std::{collections::BTreeMap, fmt::Display};

use finality_grandpa::BlockNumberOps;
use grandpa_light_client_primitives::{
	FinalityProof, ParachainHeaderProofs, ParachainHeadersWithFinalityProof,
};
use ibc_proto::google::protobuf::Any;
use sp_runtime::{
	traits::{Header as HeaderT, IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
#[cfg(feature = "dali")]
use subxt::tx::AssetTip as Tip;
use subxt::tx::{BaseExtrinsicParamsBuilder, ExtrinsicParams};

use grandpa_prover::GrandpaProver;
use ibc::core::ics02_client::msgs::update_client::MsgUpdateAnyClient;
#[cfg(not(feature = "dali"))]
use subxt::tx::PlainTip as Tip;
use tendermint_proto::Protobuf;

use ibc::{core::ics24_host::identifier::ClientId, signer::Signer, tx_msg::Msg};
use ics10_grandpa::client_message::{ClientMessage, Header as GrandpaHeader};
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState};

use primitives::{mock::LocalClientTypes, Chain, KeyProvider, LightClientSync};

use super::{error::Error, ParachainClient};
use crate::{config, finality_protocol::FinalityProtocol};

#[async_trait::async_trait]
impl<T: config::Config + Send + Sync> LightClientSync for ParachainClient<T>
where
	T: config::Config + Send + Sync,
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>
		+ From<<T as subxt::Config>::BlockNumber>,
	Self: Chain + KeyProvider,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256> + From<[u8; 32]>,
	sp_core::H256: From<T::Hash>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, Tip>> + Send + Sync,
{
	async fn is_synced<C: Chain>(&self, counterparty: &C) -> Result<bool, anyhow::Error> {
		let latest_height = counterparty.latest_height_and_timestamp().await?.0;
		let response = counterparty.query_client_state(latest_height, self.client_id()).await?;
		let client_state = response.client_state.ok_or_else(|| {
			Error::Custom("Received an empty client state from counterparty".to_string())
		})?;

		let client_state = AnyClientState::try_from(client_state)
			.map_err(|_| Error::Custom("Failed to decode client state".to_string()))?;
		match self.finality_protocol {
			FinalityProtocol::Grandpa => {
				let prover = self.grandpa_prover();
				let client_state = match client_state {
					AnyClientState::Grandpa(client_state) => client_state,
					c => Err(Error::Custom(format!(
						"Expected AnyClientState::Grandpa found: {:?}",
						c
					)))?,
				};
				let latest_hash = self.relay_client.rpc().finalized_head().await?;
				let finalized_head =
					self.relay_client.rpc().header(Some(latest_hash)).await?.ok_or_else(|| {
						Error::Custom(format!("Expected finalized header, found None"))
					})?;
				let previous_finalized_height = client_state.latest_relay_height;
				let session_length = prover.session_length().await?;
				let session_end_block =
					prover.session_end_for_block(previous_finalized_height).await?;
				let latest_finalized_height = u32::from(*finalized_head.number());
				let session_changes =
					(latest_finalized_height - session_end_block) / session_length;
				// If no session changes have occurred between the last update and the latest
				// finalized height then the light client is still in sync
				Ok(!(session_changes >= 1))
			},
			FinalityProtocol::Beefy => unimplemented!(),
		}
	}

	async fn fetch_mandatory_updates<C: Chain>(
		&self,
		counterparty: &C,
	) -> Result<Vec<Any>, anyhow::Error> {
		let latest_height = counterparty.latest_height_and_timestamp().await?.0;
		let response = counterparty.query_client_state(latest_height, self.client_id()).await?;
		let client_state = response.client_state.ok_or_else(|| {
			Error::Custom("Received an empty client state from counterparty".to_string())
		})?;

		let client_state = AnyClientState::try_from(client_state)
			.map_err(|_| Error::Custom("Failed to decode client state".to_string()))?;
		let messages = match self.finality_protocol {
			FinalityProtocol::Grandpa => {
				let prover = self.grandpa_prover();
				let client_state = match client_state {
					AnyClientState::Grandpa(client_state) => client_state,
					c => Err(Error::Custom(format!(
						"Expected AnyClientState::Grandpa found: {:?}",
						c
					)))?,
				};
				let latest_hash = self.relay_client.rpc().finalized_head().await?;
				let finalized_head =
					self.relay_client.rpc().header(Some(latest_hash)).await?.ok_or_else(|| {
						Error::Custom(format!("Expected finalized header, found None"))
					})?;
				let latest_finalized_height = u32::from(*finalized_head.number());
				let (mut messages, previous_finalized_height, ..) = self
					.query_missed_grandpa_updates(
						client_state.latest_relay_height,
						latest_finalized_height,
						self.client_id(),
						counterparty.account_id(),
					)
					.await?;
				let (latest_message, ..) = get_message(
					prover,
					previous_finalized_height,
					latest_finalized_height,
					self.client_id(),
					counterparty.account_id(),
				)
				.await?;
				messages.push(latest_message);
				messages
			},
			// Current implementation of Beefy needs to be revised
			FinalityProtocol::Beefy => unimplemented!(),
		};

		Ok(messages)
	}
}

impl<T: config::Config + Send + Sync> ParachainClient<T>
where
	T: config::Config + Send + Sync,
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>
		+ From<<T as subxt::Config>::BlockNumber>,
	Self: Chain + KeyProvider,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256> + From<[u8; 32]>,
	sp_core::H256: From<T::Hash>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, Tip>> + Send + Sync,
{
	/// Returns a tuple of the client update messages in the exclusive range
	/// `previous_finalized_height..latest_finalized_height`, relay chain block of the last message
	/// in the list and latest parachain block finalized by the last message in the list
	pub async fn query_missed_grandpa_updates(
		&self,
		mut previous_finalized_height: u32,
		latest_finalized_height: u32,
		client_id: ClientId,
		signer: Signer,
	) -> Result<(Vec<Any>, u32, Option<T::BlockNumber>), anyhow::Error> {
		let prover = self.grandpa_prover();
		let session_length = prover.session_length().await?;
		let mut session_end_block = {
			let mut session_block_end =
				prover.session_end_for_block(previous_finalized_height).await?;
			if session_block_end == previous_finalized_height {
				session_block_end += session_length;
			}
			session_block_end
		};
		// Get all session change blocks between previously finalized relaychain height and latest
		// finalized height
		let mut messages = vec![];
		let mut latest_para_block = None;
		while session_end_block < latest_finalized_height {
			let (msg, para_block) = get_message(
				self.grandpa_prover(),
				previous_finalized_height,
				session_end_block,
				client_id.clone(),
				signer.clone(),
			)
			.await?;
			messages.push(msg);
			latest_para_block = latest_para_block.max(Some(para_block));
			previous_finalized_height = session_end_block;
			session_end_block += session_length;
		}
		Ok((messages, previous_finalized_height, latest_para_block))
	}
}

/// Return a single client update message and the latest parachain block number finalized by that
/// update
async fn get_message<T: crate::config::Config>(
	prover: GrandpaProver<T>,
	previous_finalized_height: u32,
	latest_finalized_height: u32,
	client_id: ClientId,
	signer: Signer,
) -> Result<(Any, T::BlockNumber), anyhow::Error>
where
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>
		+ From<<T as subxt::Config>::BlockNumber>,
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
{
	let (
		ParachainHeadersWithFinalityProof { finality_proof, parachain_headers },
		latest_para_block,
	) = prover
		.query_finality_proof(previous_finalized_height, latest_finalized_height, vec![])
		.await?;

	let grandpa_header = GrandpaHeader {
		finality_proof: finality_proof.into(),
		parachain_headers: parachain_headers.into(),
	};

	let msg = MsgUpdateAnyClient::<LocalClientTypes> {
		client_id,
		client_message: AnyClientMessage::Grandpa(ClientMessage::Header(grandpa_header)),
		signer,
	};
	let value = msg.encode_vec();
	Result::<_, anyhow::Error>::Ok((Any { value, type_url: msg.type_url() }, latest_para_block))
}
