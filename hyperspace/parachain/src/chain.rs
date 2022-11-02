use anyhow::anyhow;
use codec::{Decode, Encode};
use std::{
	collections::BTreeMap,
	fmt::Display,
	pin::Pin,
	time::{Duration, Instant},
};

use beefy_gadget_rpc::BeefyApiClient;
use finality_grandpa::BlockNumberOps;
use futures::{Stream, StreamExt, TryFutureExt};
use grandpa_light_client_primitives::{FinalityProof, ParachainHeaderProofs};
use ibc_proto::google::protobuf::Any;
use sp_runtime::{
	generic::Era,
	traits::{Header as HeaderT, IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
use subxt::tx::{BaseExtrinsicParamsBuilder, ExtrinsicParams};
use transaction_payment_rpc::TransactionPaymentApiClient;
use transaction_payment_runtime_api::RuntimeDispatchInfo;

use primitives::{Chain, IbcProvider, MisbehaviourHandler};

use super::{error::Error, signer::ExtrinsicSigner, ParachainClient};
use crate::{
	config,
	parachain::{api, api::runtime_types::pallet_ibc::Any as RawAny, UncheckedExtrinsic},
	FinalityProtocol,
};
use finality_grandpa_rpc::GrandpaApiClient;
use ibc::{
	core::{
		ics02_client::msgs::{update_client::MsgUpdateAnyClient, ClientMsg},
		ics26_routing::msgs::Ics26Envelope,
	},
	tx_msg::Msg,
};
use ics10_grandpa::client_message::{ClientMessage, Misbehaviour, RelayChainHeader};
use pallet_ibc::light_clients::AnyClientMessage;
use primitives::mock::LocalClientTypes;
use sp_core::H256;
use subxt::tx::{PlainTip, PolkadotExtrinsicParamsBuilder};
use tokio::time::sleep;

type GrandpaJustification = grandpa_light_client_primitives::justification::GrandpaJustification<
	polkadot_core_primitives::Header,
>;

type BeefyJustification =
	beefy_primitives::SignedCommitment<u32, beefy_primitives::crypto::Signature>;

/// An encoded justification proving that the given header has been finalized
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct JustificationNotification(sp_core::Bytes);

#[async_trait::async_trait]
impl<T: config::Config + Send + Sync> Chain for ParachainClient<T>
where
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>,
	u32: From<<T as subxt::Config>::BlockNumber>,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256> + From<[u8; 32]>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	sp_core::H256: From<T::Hash>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, PlainTip>> + Send + Sync,
	RelayChainHeader: From<T::Header>,
{
	fn name(&self) -> &str {
		&*self.name
	}

	fn block_max_weight(&self) -> u64 {
		self.max_extrinsic_weight
	}

	async fn estimate_weight(&self, messages: Vec<Any>) -> Result<u64, Self::Error> {
		let extrinsic = {
			// todo: put this in utils
			let signer = ExtrinsicSigner::<T, Self>::new(
				self.key_store.clone(),
				self.key_type_id.clone(),
				self.public_key.clone(),
			);

			let messages = messages
				.into_iter()
				.map(|msg| RawAny { type_url: msg.type_url.as_bytes().to_vec(), value: msg.value })
				.collect::<Vec<_>>();

			let tx_params = PolkadotExtrinsicParamsBuilder::new()
				.tip(PlainTip::new(100_000))
				.era(Era::Immortal, self.para_client.genesis_hash());
			let call = api::tx().ibc().deliver(messages);
			self.para_client.tx().create_signed(&call, &signer, tx_params.into()).await?
		};
		let dispatch_info =
			TransactionPaymentApiClient::<sp_core::H256, RuntimeDispatchInfo<u128>>::query_info(
				&*self.para_ws_client,
				extrinsic.encoded().to_vec().into(),
				None,
			)
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(dispatch_info.weight)
	}

	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>> {
		match self.finality_protocol {
			FinalityProtocol::Grandpa => {
				let subscription =
					GrandpaApiClient::<JustificationNotification, sp_core::H256, u32>::subscribe_justifications(
						&*self.relay_ws_client,
					)
						.await
						.expect("Failed to subscribe to grandpa justifications")
						.chunks(6)
						.map(|mut notifs| notifs.remove(notifs.len() - 1)); // skip every 4 finality notifications

				let stream = subscription.filter_map(|justification_notif| {
					let encoded_justification = match justification_notif {
						Ok(JustificationNotification(sp_core::Bytes(justification))) =>
							justification,
						Err(err) => {
							log::error!("Failed to fetch Justification: {}", err);
							return futures::future::ready(None)
						},
					};

					let justification =
						match GrandpaJustification::decode(&mut &*encoded_justification) {
							Ok(j) => j,
							Err(err) => {
								log::error!("Grandpa Justification scale decode error: {}", err);
								return futures::future::ready(None)
							},
						};
					futures::future::ready(Some(Self::FinalityEvent::Grandpa(justification)))
				});

				Box::pin(Box::new(stream))
			},
			FinalityProtocol::Beefy => {
				let subscription =
					BeefyApiClient::<JustificationNotification, sp_core::H256>::subscribe_justifications(
						&*self.relay_ws_client,
					)
						.await
						.expect("Failed to subscribe to beefy justifications");

				let stream = subscription.filter_map(|commitment_notification| {
					let encoded_commitment = match commitment_notification {
						Ok(JustificationNotification(sp_core::Bytes(commitment))) => commitment,
						Err(err) => {
							log::error!("Failed to fetch Commitment: {}", err);
							return futures::future::ready(None)
						},
					};

					let signed_commitment =
						match BeefyJustification::decode(&mut &*encoded_commitment) {
							Ok(c) => c,
							Err(err) => {
								log::error!("SignedCommitment scale decode error: {}", err);
								return futures::future::ready(None)
							},
						};
					futures::future::ready(Some(Self::FinalityEvent::Beefy(signed_commitment)))
				});

				Box::pin(Box::new(stream))
			},
		}
	}

	async fn submit(
		&self,
		messages: Vec<Any>,
	) -> Result<(sp_core::H256, Option<sp_core::H256>), Error> {
		let messages = messages
			.into_iter()
			.map(|msg| RawAny { type_url: msg.type_url.as_bytes().to_vec(), value: msg.value })
			.collect::<Vec<_>>();

		let call = api::tx().ibc().deliver(messages);
		let (ext_hash, block_hash) = self.submit_call(call).await?;

		Ok((ext_hash.into(), Some(block_hash.into())))
	}

	async fn query_client_message(
		&self,
		host_block_hash: [u8; 32],
		transaction_index: usize,
		event_index: usize,
	) -> Result<AnyClientMessage, Error> {
		use api::runtime_types::{
			pallet_ibc::pallet::Call as IbcCall, parachain_runtime::Call as RuntimeCall,
		};

		let hash = H256(host_block_hash);
		log::debug!("Querying extrinsic data at {:?} {}", hash, transaction_index);

		let now = Instant::now();
		// There is no way to query a specific extrinsic on substrate directly, so block
		// query used instead.
		let block = loop {
			let maybe_block = self.para_client.rpc().block(Some(hash.into())).await?;
			match maybe_block {
				Some(block) => break block,
				None => {
					if now.elapsed() > Duration::from_secs(20) {
						return Err(Error::from("Timeout while waiting for block".to_owned()))
					}
					sleep(Duration::from_millis(100)).await;
				},
			}
		};

		let extrinsic_opaque =
			block.block.extrinsics.get(transaction_index).expect("Extrinsic not found");

		let unchecked_extrinsic = UncheckedExtrinsic::<T>::decode(&mut &*extrinsic_opaque.encode())
			.map_err(|e| Error::from(format!("Extrinsic decode error: {}", e)))?;

		match unchecked_extrinsic.function {
			RuntimeCall::Ibc(IbcCall::deliver { messages }) => {
				let message = messages.get(event_index).ok_or_else(|| {
					Error::from(format!("Message index {} out of bounds", event_index))
				})?;
				let envelope = Ics26Envelope::<LocalClientTypes>::try_from(Any {
					type_url: String::from_utf8(message.type_url.clone()).map_err(|_| {
						Error::from("failed to create String from utf-8".to_string())
					})?,
					value: message.value.clone(),
				});
				match envelope {
					Ok(Ics26Envelope::Ics2Msg(ClientMsg::UpdateClient(update_msg))) =>
						return Ok(update_msg.client_message),
					_ => (),
				}
			},
			_ => (),
		}
		Err(Error::Custom("No ICS02 update message found".into()))
	}
}

#[async_trait::async_trait]
impl<T: config::Config + Send + Sync> MisbehaviourHandler for ParachainClient<T>
where
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>,
	u32: From<<T as subxt::Config>::BlockNumber>,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256> + From<[u8; 32]>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	sp_core::H256: From<T::Hash>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, PlainTip>> + Send + Sync,
	RelayChainHeader: From<T::Header>,
{
	async fn check_for_misbehaviour<C: Chain>(
		&self,
		counterparty: &C,
		client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		match client_message {
			AnyClientMessage::Grandpa(ClientMessage::Header(header)) => {
				let base_header = header
					.finality_proof
					.unknown_headers
					.iter()
					.min_by_key(|h| h.number)
					.expect("unknown_headers always contain at least one header; qed");

				let common_ancestor_header = self
					.relay_client
					.rpc()
					.header(Some(base_header.parent_hash.into()))
					.await?
					.ok_or_else(|| {
						anyhow!("No header found for hash: {:?}", base_header.parent_hash)
					})?;

				let common_ancestor_block_number =
					u32::from(*common_ancestor_header.number() + 0u32.into());
				let encoded =
					GrandpaApiClient::<JustificationNotification, H256, u32>::prove_finality(
						&*self.relay_ws_client,
						common_ancestor_block_number,
					)
					.await?
					.ok_or_else(|| {
						anyhow!(
							"No justification found for block: {:?}",
							header.finality_proof.block
						)
					})?
					.0;

				let mut trusted_finality_proof =
					FinalityProof::<RelayChainHeader>::decode(&mut &encoded[..])?;
				let trusted_justification =
					GrandpaJustification::decode(&mut &*trusted_finality_proof.justification)?;
				trusted_finality_proof.unknown_headers.clear();
				let to_block = trusted_justification.commit.target_number;
				let from_block = (common_ancestor_block_number + 1).min(to_block);
				for i in from_block..=to_block {
					let unknown_header_hash =
						self.relay_client.rpc().block_hash(Some(i.into())).await?.ok_or_else(
							|| {
								anyhow!(
									"No block hash found for block number: {:?}",
									common_ancestor_block_number
								)
							},
						)?;
					let unknown_header = self
						.relay_client
						.rpc()
						.header(Some(unknown_header_hash))
						.await?
						.ok_or_else(|| {
							anyhow!("No header found for hash: {:?}", unknown_header_hash)
						})?;
					trusted_finality_proof.unknown_headers.push(unknown_header.into());
				}

				let justification =
					GrandpaJustification::decode(&mut &*header.finality_proof.justification)?;
				if justification.commit.target_hash != trusted_justification.commit.target_hash {
					log::warn!(
						"Found misbehaviour on client {}: {:?} != {:?}",
						self.client_id
							.as_ref()
							.map(|x| x.as_str().to_owned())
							.unwrap_or_else(|| "{unknown}".to_owned()),
						header.finality_proof.block,
						trusted_finality_proof.block
					);

					let misbehaviour = ClientMessage::Misbehaviour(Misbehaviour {
						first_finality_proof: header.finality_proof,
						second_finality_proof: trusted_finality_proof,
					});

					counterparty
						.submit(vec![MsgUpdateAnyClient::<LocalClientTypes>::new(
							self.client_id(),
							AnyClientMessage::Grandpa(misbehaviour.clone()),
							counterparty.account_id(),
						)
						.to_any()])
						.map_err(|e| anyhow!("Failed to submit misbehaviour report: {:?}", e))
						.await?;
				}
			},
			_ => {},
		}
		Ok(())
	}
}
