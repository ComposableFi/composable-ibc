use self::parachain_subxt::api::{
	ibc::calls::{Deliver, Transfer},
	ibc_ping::calls::SendPing,
	runtime_types::{
		frame_system::{extensions::check_nonce::CheckNonce, EventRecord},
		pallet_ibc::{events::IbcEvent as MetadataIbcEvent, TransferParams as RawTransferParams},
		pallet_ibc_ping::SendPingParams as RawSendPingParams,
	},
	sudo::calls::Sudo,
};
use crate::{
	define_any_wrapper, define_beefy_authority_set, define_event_record, define_events,
	define_head_data, define_ibc_event_wrapper, define_id, define_para_lifecycle,
	define_runtime_call, define_runtime_event, define_runtime_storage, define_runtime_transactions,
	define_send_ping_params, define_transfer_params,
	substrate::default::relaychain::api::runtime_types::sp_beefy::mmr::BeefyAuthoritySet,
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
use parachain_subxt::api::runtime_types::{
	ibc_primitives::Timeout as RawTimeout, parachain_runtime::MemoMessage,
};
use relaychain::api::runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle;
use sp_core::{crypto::AccountId32, H256};
use subxt::{
	config::{
		extrinsic_params::Era,
		polkadot::{
			PlainTip as Tip, PolkadotExtrinsicParams as ParachainExtrinsicParams,
			PolkadotExtrinsicParamsBuilder as ParachainExtrinsicsParamsBuilder,
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
	#[cfg(feature = "build-metadata-from-ws")]
	include!(concat!(env!("OUT_DIR"), "/parachain.rs"));

	#[cfg(not(feature = "build-metadata-from-ws"))]
	pub use subxt_generated::default::parachain::*;
}

pub mod relaychain {
	#[cfg(feature = "build-metadata-from-ws")]
	include!(concat!(env!("OUT_DIR"), "/polkadot.rs"));

	#[cfg(not(feature = "build-metadata-from-ws"))]
	pub use subxt_generated::default::relaychain::*;
}

pub type Balance = u128;

#[derive(Debug, Clone)]
pub enum DefaultConfig {}

define_id!(DefaultId, relaychain::api::runtime_types::polkadot_parachain::primitives::Id);

define_head_data!(
	DefaultHeadData,
	relaychain::api::runtime_types::polkadot_parachain::primitives::HeadData,
);

define_para_lifecycle!(DefaultParaLifecycle, ParaLifecycle);

define_beefy_authority_set!(DefaultBeefyAuthoritySet, BeefyAuthoritySet<T>);

define_runtime_storage!(
	DefaultRuntimeStorage,
	DefaultHeadData,
	DefaultId,
	DefaultParaLifecycle,
	DefaultBeefyAuthoritySet<H256>,
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

define_send_ping_params!(SendPingParamsWrapper, SendPingParams, RawSendPingParams);

define_transfer_params!(
	TransferParamsWrapper,
	TransferParams<AccountId32>,
	RawTransferParams<T>,
	RawTimeout,
	parachain_subxt::api::runtime_types::pallet_ibc::MultiAddress<T>
);

define_any_wrapper!(AnyWrapper, parachain_subxt::api::runtime_types::pallet_ibc::Any);

define_runtime_transactions!(
	DefaultRuntimeTransactions,
	Deliver,
	Transfer,
	Sudo,
	SendPing,
	DefaultParaRuntimeCall,
	SendPingParams,
	TransferParams<AccountId32>,
	TransferParamsWrapper,
	SendPingParamsWrapper,
	parachain_subxt::api::runtime_types::pallet_ibc::Any,
	|x| parachain_subxt::api::tx().ibc().deliver(x),
	|x, y, z, w| parachain_subxt::api::tx().ibc().transfer(x, y, z, w),
	|x| parachain_subxt::api::tx().sudo().sudo(x),
	|x| parachain_subxt::api::tx().ibc_ping().send_ping(x)
);

define_ibc_event_wrapper!(
	IbcEventWrapper, MetadataIbcEvent,
	MetadataIbcEvent::PushWasmCode { wasm_code_id } => RawIbcEvent::PushWasmCode { wasm_code_id },
);

define_event_record!(
	DefaultEventRecord,
	EventRecord<<DefaultConfig as light_client_common::config::Config>::ParaRuntimeEvent, H256>,
	IbcEventWrapper,
	parachain_subxt::api::runtime_types::frame_system::Phase,
	parachain_subxt::api::runtime_types::pallet_ibc::pallet::Event,
	parachain_subxt::api::runtime_types::parachain_runtime::RuntimeEvent
);

define_events!(DefaultEvents, parachain_subxt::api::ibc::events::Events, IbcEventWrapper);

define_runtime_event!(
	DefaultParaRuntimeEvent,
	parachain_subxt::api::runtime_types::parachain_runtime::RuntimeEvent
);

define_runtime_call!(
	DefaultParaRuntimeCall,
	parachain_subxt::api::runtime_types::parachain_runtime::RuntimeCall,
	AnyWrapper,
	parachain_subxt::api::runtime_types::pallet_ibc::pallet::Call
);

#[async_trait]
impl light_client_common::config::Config for DefaultConfig {
	type AssetId = u128;
	type Signature = <Self as subxt::Config>::Signature;
	type Address = <Self as subxt::Config>::Address;
	type Tip = Tip;
	type ParaRuntimeCall = DefaultParaRuntimeCall;
	type ParaRuntimeEvent = DefaultParaRuntimeEvent;
	type Events = DefaultEvents;
	type EventRecord = DefaultEventRecord;
	type Storage = DefaultRuntimeStorage;
	type Tx = DefaultRuntimeTransactions;
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

impl subxt::Config for DefaultConfig {
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
