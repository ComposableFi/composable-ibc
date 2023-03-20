use self::parachain_subxt::api::{
	ibc::calls::{Deliver, Transfer},
	runtime_types::{
		composable_runtime::ibc::MemoMessage,
		frame_system::{extensions::check_nonce::CheckNonce, EventRecord},
		pallet_ibc::{events::IbcEvent as MetadataIbcEvent, TransferParams as RawTransferParams},
	},
	sudo::calls::Sudo,
};
use crate::{
	define_any_wrapper, define_beefy_authority_set, define_event_record, define_events,
	define_head_data, define_ibc_event_wrapper, define_id, define_para_lifecycle,
	define_runtime_call, define_runtime_event, define_runtime_storage, define_runtime_transactions,
	define_transfer_params,
	substrate::composable::{
		parachain_subxt::api::runtime_types::primitives::currency::CurrencyId,
		relaychain::api::runtime_types::sp_beefy::mmr::BeefyAuthoritySet,
	},
};
use async_trait::async_trait;
use codec::{Compact, Decode, Encode};
use ibc_proto::google::protobuf::Any;
use light_client_common::config::{
	BeefyAuthoritySetT, EventRecordT, IbcEventsT, LocalStaticStorageAddress, ParaLifecycleT,
	RuntimeCall, RuntimeStorage, RuntimeTransactions,
};
use pallet_ibc::{events::IbcEvent as RawIbcEvent, MultiAddress, Timeout, TransferParams};
use pallet_ibc_ping::SendPingParams;
use parachain_subxt::api::runtime_types::ibc_primitives::Timeout as RawTimeout;
use relaychain::api::runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle;
use sp_core::{crypto::AccountId32, H256};
use subxt::{
	config::{
		extrinsic_params::Era,
		substrate::{
			AssetTip as Tip, SubstrateExtrinsicParams as ParachainExtrinsicParams,
			SubstrateExtrinsicParamsBuilder as ParachainExtrinsicsParamsBuilder,
		},
		ExtrinsicParams,
	},
	events::{Phase, StaticEvent},
	metadata::DecodeStaticType,
	storage::{address::Yes, StaticStorageAddress},
	tx::StaticTxPayload,
	Error, OnlineClient,
};

pub mod parachain_subxt {
	pub use subxt_generated::composable::parachain::*;
}

pub mod relaychain {
	pub use subxt_generated::composable::relaychain::*;
}

pub type Balance = u128;

#[derive(Encode)]
pub struct DummySendPingParamsWrapper<T>(T);
#[derive(Encode)]
pub struct FakeSendPingParams;

impl From<SendPingParams> for FakeSendPingParams {
	fn from(_: SendPingParams) -> Self {
		Self
	}
}

#[derive(Debug, Clone)]
pub enum ComposableConfig {}

define_id!(ComposableId, relaychain::api::runtime_types::polkadot_parachain::primitives::Id);

define_head_data!(
	ComposableHeadData,
	relaychain::api::runtime_types::polkadot_parachain::primitives::HeadData,
);

define_para_lifecycle!(ComposableParaLifecycle, ParaLifecycle);

define_beefy_authority_set!(ComposableBeefyAuthoritySet, BeefyAuthoritySet<T>);

define_runtime_storage!(
	ComposableRuntimeStorage,
	ComposableHeadData,
	ComposableId,
	ComposableParaLifecycle,
	ComposableBeefyAuthoritySet<H256>,
	parachain_subxt::api::storage().timestamp().now(),
	|x| relaychain::api::storage().paras().heads(x),
	|x| relaychain::api::storage().paras().para_lifecycles(x),
	relaychain::api::storage().paras().parachains(),
	relaychain::api::storage().grandpa().current_set_id(),
	relaychain::api::storage().beefy().validator_set_id(),
	relaychain::api::storage().beefy().authorities(),
	relaychain::api::storage().mmr_leaf().beefy_next_authorities(),
	relaychain::api::storage().babe().epoch_start()
);

define_transfer_params!(
	TransferParamsWrapper,
	TransferParams<AccountId32>,
	RawTransferParams<T>,
	RawTimeout,
	parachain_subxt::api::runtime_types::pallet_ibc::MultiAddress<T>
);

define_any_wrapper!(AnyWrapper, parachain_subxt::api::runtime_types::pallet_ibc::Any);

define_runtime_transactions!(
	ComposableRuntimeTransactions,
	Deliver,
	Transfer,
	Sudo,
	DummySendPingParamsWrapper<FakeSendPingParams>,
	ComposableParaRuntimeCall,
	FakeSendPingParams,
	TransferParams<AccountId32>,
	TransferParamsWrapper,
	DummySendPingParamsWrapper,
	parachain_subxt::api::runtime_types::pallet_ibc::Any,
	|x| parachain_subxt::api::tx().ibc().deliver(x),
	|x, y, z, w| parachain_subxt::api::tx().ibc().transfer(x, CurrencyId(y), z, w),
	|x| parachain_subxt::api::tx().sudo().sudo(x),
	|_: DummySendPingParamsWrapper<FakeSendPingParams>| unimplemented!("ping is not implemented")
);

define_ibc_event_wrapper!(IbcEventWrapper, MetadataIbcEvent,);

define_event_record!(
	ComposableEventRecord,
	EventRecord<<ComposableConfig as light_client_common::config::Config>::ParaRuntimeEvent, H256>,
	IbcEventWrapper,
	parachain_subxt::api::runtime_types::frame_system::Phase,
	parachain_subxt::api::runtime_types::pallet_ibc::pallet::Event,
	parachain_subxt::api::runtime_types::composable_runtime::RuntimeEvent
);

define_events!(ComposableEvents, parachain_subxt::api::ibc::events::Events, IbcEventWrapper);

define_runtime_event!(
	ComposableParaRuntimeEvent,
	parachain_subxt::api::runtime_types::composable_runtime::RuntimeEvent
);

define_runtime_call!(
	ComposableParaRuntimeCall,
	parachain_subxt::api::runtime_types::composable_runtime::RuntimeCall,
	AnyWrapper,
	parachain_subxt::api::runtime_types::pallet_ibc::pallet::Call
);

#[async_trait]
impl light_client_common::config::Config for ComposableConfig {
	type AssetId = u128;
	type Signature = <Self as subxt::Config>::Signature;
	type Address = <Self as subxt::Config>::Address;
	type Tip = Tip;
	type ParaRuntimeCall = ComposableParaRuntimeCall;
	type ParaRuntimeEvent = ComposableParaRuntimeEvent;
	type Events = ComposableEvents;
	type EventRecord = ComposableEventRecord;
	type Storage = ComposableRuntimeStorage;
	type Tx = ComposableRuntimeTransactions;
	type SignedExtra = (Era, CheckNonce, Compact<Balance>);

	async fn custom_extrinsic_params(
		client: &OnlineClient<Self>,
	) -> Result<
		<Self::ExtrinsicParams as ExtrinsicParams<Self::Index, Self::Hash>>::OtherParams,
		Error,
	> {
		let params =
			ParachainExtrinsicsParamsBuilder::new().era(Era::Immortal, client.genesis_hash());
		Ok(params.into())
	}
}

impl subxt::Config for ComposableConfig {
	type Index = u32;
	type BlockNumber = u32;
	type Hash = H256;
	type Hasher = subxt::config::substrate::BlakeTwo256;
	type AccountId = AccountId32;
	type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
	type Header = subxt::config::substrate::SubstrateHeader<
		Self::BlockNumber,
		subxt::config::substrate::BlakeTwo256,
	>;
	type Signature = sp_runtime::MultiSignature;
	type ExtrinsicParams = ParachainExtrinsicParams<Self>;
}
