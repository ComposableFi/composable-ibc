// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
	signer::ExtrinsicSigner, utils::unsafe_cast_to_jsonrpsee_client, Error, ParachainClient,
};
use finality_grandpa::BlockNumberOps;
use futures::{Stream, StreamExt};
use grandpa_light_client_primitives::ParachainHeaderProofs;
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, PrefixedCoin},
	core::ics24_host::identifier::{ChannelId, ClientId, PortId},
};
use ibc_proto::google::protobuf::Any;
use ibc_rpc::IbcApiClient;
use jsonrpsee::{core::client::SubscriptionClientT, rpc_params};
use light_client_common::config::RuntimeTransactions;
use pallet_ibc::{MultiAddress, Timeout, TransferParams};
use pallet_ibc_ping::SendPingParams;
use primitives::{KeyProvider, TestProvider};
use sp_core::{
	crypto::{AccountId32, Ss58Codec},
	H256,
};
use sp_runtime::{
	traits::{IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
use std::{collections::BTreeMap, fmt::Display, pin::Pin, str::FromStr};
use subxt::config::{
	extrinsic_params::BaseExtrinsicParamsBuilder, ExtrinsicParams, Header as HeaderT,
};
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

	T::BlockNumber: Ord + sp_runtime::traits::Zero + One,
	T::Header: HeaderT,
	<<T::Header as HeaderT>::Hasher as subxt::config::Hasher>::Output: From<T::Hash>,
	T::BlockNumber: From<u32>,
	BTreeMap<H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	T::BlockNumber: Ord + sp_runtime::traits::Zero,
	<T as subxt::Config>::AccountId: Send + Sync,
	<T as subxt::Config>::Address: Send + Sync,
	<<T as light_client_common::config::Config>::Tx as RuntimeTransactions>::TransferParams:
		From<TransferParams<AccountId32>>,
{
	pub fn set_client_id(&mut self, client_id: ClientId) {
		self.client_id = Some(client_id)
	}

	pub async fn submit_create_client_msg(&self, msg: Any) -> Result<ClientId, Error> {
		let call = T::Tx::ibc_deliver(vec![msg]);
		let (ext_hash, block_hash) = self.submit_call(call).await?;

		// Query newly created client Id
		let identified_client_state = IbcApiClient::<
			u32,
			H256,
			<T as light_client_common::config::Config>::AssetId,
		>::query_newly_created_client(
			&*self.para_ws_client, block_hash.into(), ext_hash.into()
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;

		let client_id = ClientId::from_str(&identified_client_state.client_id)
			.expect("Should have a valid client id");
		Ok(client_id)
	}

	pub async fn transfer_tokens(
		&self,
		params: TransferParams<AccountId32>,
		asset_id: u128,
		amount: u128,
	) -> Result<(), Error> {
		// Submit extrinsic to parachain node
		let call = T::Tx::ibc_transfer(params.into(), asset_id, amount, None);
		self.submit_call(call).await?;
		Ok(())
	}

	pub async fn submit_sudo_call(&self, call: T::ParaRuntimeCall) -> Result<(), Error> {
		let signer = ExtrinsicSigner::<T, Self>::new(
			self.key_store.clone(),
			self.key_type_id.clone(),
			self.public_key.clone(),
		);

		let ext = T::Tx::sudo_sudo(call);
		// Submit extrinsic to parachain node

		let other_params = T::custom_extrinsic_params(&self.para_client).await?;

		let _progress = self
			.para_client
			.tx()
			.sign_and_submit_then_watch(&ext, &signer, other_params)
			.await?
			.wait_for_in_block()
			.await?
			.wait_for_success()
			.await?;

		Ok(())
	}
}

#[async_trait::async_trait]
impl<T> TestProvider for ParachainClient<T>
where
	T: light_client_common::config::Config + Send + Sync + Clone,
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>,
	u32: From<<T as subxt::Config>::BlockNumber>,
	Self: KeyProvider,
	<<T as light_client_common::config::Config>::Signature as Verify>::Signer:
		From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	<T as subxt::Config>::Signature: From<MultiSignature> + Send + Sync,
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256> + From<[u8; 32]>,
	H256: From<T::Hash>,
	BTreeMap<H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, T::Tip>> + Send + Sync,
	<T as subxt::Config>::AccountId: Send + Sync,
	<T as subxt::Config>::Address: Send + Sync,
	<T as light_client_common::config::Config>::AssetId: Clone,
	<<T as light_client_common::config::Config>::Tx as RuntimeTransactions>::TransferParams:
		From<TransferParams<AccountId32>>,
	<<T as light_client_common::config::Config>::Tx as RuntimeTransactions>::SendPingParams:
		From<SendPingParams>,
{
	async fn send_transfer(&self, transfer: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error> {
		let account_id = AccountId32::from_ss58check(transfer.receiver.as_ref())
			.map(MultiAddress::Id)
			.unwrap_or_else(|_| MultiAddress::Raw(transfer.receiver.to_string().into_bytes()));
		let params = TransferParams {
			to: account_id,
			source_channel: transfer.source_channel.sequence(),
			timeout: Timeout::Absolute {
				timestamp: Some(transfer.timeout_timestamp.nanoseconds()),
				height: Some(transfer.timeout_height.revision_height),
			},
		};
		let amount = str::parse::<u128>(&transfer.token.amount.to_string()).expect("Infallible!");
		// TODO: get asset_id by denom
		let string = transfer.token.denom.to_string();
		let asset_id = if string == *r#""UNIT""# || string == "UNIT" { 1 } else { 2 };
		log::info!(
			"Sending transfer: {:?}, asset id: {asset_id}, amount: {amount}",
			transfer.token.denom
		);
		self.transfer_tokens(params, asset_id, amount).await?;

		Ok(())
	}

	async fn send_ordered_packet(
		&self,
		channel_id: ChannelId,
		timeout: Timeout,
	) -> Result<(), Self::Error> {
		let (timeout_height, timestamp) = match timeout {
			Timeout::Offset { timestamp, height } => (height.unwrap(), timestamp.unwrap()),
			_ => panic!("Only offset timeouts allowed"),
		};

		let params = SendPingParams {
			data: "ping".as_bytes().to_vec(),
			timeout_height_offset: timeout_height,
			timeout_timestamp_offset: timestamp,
			channel_id: channel_id.sequence(),
		};

		let call = T::Tx::ibc_ping_send_ping(params.into());

		self.submit_call(call).await.map(|_| ())
	}

	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>> {
		let para_client = unsafe { unsafe_cast_to_jsonrpsee_client(&self.para_ws_client) };
		let stream = para_client
			.subscribe::<T::Header, _>(
				"chain_subscribeNewHeads",
				rpc_params![],
				"chain_unsubscribeNewHeads",
			)
			.await
			.unwrap()
			.map(|header| {
				let header = header.unwrap();
				let block_number: u64 = (header.number()).into();
				block_number
			});

		Box::pin(Box::new(stream))
	}

	fn set_channel_whitelist(&mut self, channel_whitelist: Vec<(ChannelId, PortId)>) {
		self.channel_whitelist = channel_whitelist;
	}
}
