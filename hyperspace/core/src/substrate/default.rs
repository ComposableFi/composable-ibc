pub mod parachain_subxt {
	#[cfg(feature = "build-metadata-from-ws")]
	include!(concat!(env!("OUT_DIR"), "/parachain.rs"));

	#[cfg(not(feature = "build-metadata-from-ws"))]
	pub use subxt_generated::parachain::*;
}

pub mod relaychain {
	#[cfg(feature = "build-metadata-from-ws")]
	include!(concat!(env!("OUT_DIR"), "/polkadot.rs"));

	#[cfg(not(feature = "build-metadata-from-ws"))]
	pub use subxt_generated::relaychain::*;
}

use self::parachain_subxt::api::{
	ibc::calls::{Deliver, Transfer},
	ibc_ping::calls::SendPing,
	runtime_types::{
		frame_system::{extensions::check_nonce::CheckNonce, EventRecord},
		pallet_ibc::{
			events::IbcEvent as MetadataIbcEvent, PalletParams as RawPalletParams,
			TransferParams as RawTransferParams,
		},
		pallet_ibc_ping::SendPingParams as RawSendPingParams,
	},
	sudo::calls::Sudo,
};
use crate::{
	define_any_wrapper, define_head_data, define_ibc_event_wrapper, define_id,
	define_pallet_params, define_send_ping_params, define_transfer_params,
};
use async_trait::async_trait;
use codec::{Compact, Decode, Encode};
use derive_more::From;
use futures::Stream;
#[cfg(any(test, feature = "testing"))]
use ibc::applications::transfer::msgs::transfer::MsgTransfer;
use ibc::{
	applications::transfer::PrefixedCoin,
	core::{
		ics02_client::{client_state::ClientType, events::UpdateClient},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	downcast,
	events::IbcEvent,
	signer::Signer,
	timestamp::Timestamp,
	Height,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			QueryChannelResponse, QueryChannelsResponse, QueryNextSequenceReceiveResponse,
			QueryPacketAcknowledgementResponse, QueryPacketCommitmentResponse,
			QueryPacketReceiptResponse,
		},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::{IdentifiedConnection, QueryConnectionResponse},
	},
};
use light_client_common::config::{
	EventRecordT, IbcEventsT, LocalStaticStorageAddress, RuntimeCall, RuntimeStorage,
	RuntimeTransactions,
};
use pallet_ibc::{
	errors::IbcError,
	events::IbcEvent as RawIbcEvent,
	light_clients::{AnyClientMessage, AnyClientState, AnyConsensusState},
	MultiAddress, PalletParams, Timeout, TransferParams,
};
use pallet_ibc_ping::SendPingParams;
use parachain_subxt::api::runtime_types::{
	ibc_primitives::Timeout as RawTimeout, pallet_ibc::Any as RawAny,
	parachain_runtime::MemoMessage,
};
use primitives::{
	Chain, IbcProvider, KeyProvider, LightClientSync, MisbehaviourHandler, UpdateType,
};
use serde::{Deserialize, Serialize};
use sp_core::{crypto::AccountId32, H256};
use sp_runtime::scale_info::MetaType;
use std::{borrow::Borrow, pin::Pin, time::Duration};
#[cfg(not(feature = "dali"))]
use subxt::config::polkadot::{
	PolkadotExtrinsicParams as ParachainExtrinsicParams,
	PolkadotExtrinsicParamsBuilder as ParachainExtrinsicsParamsBuilder,
};
#[cfg(feature = "dali")]
use subxt::config::substrate::{
	SubstrateExtrinsicParams as ParachainExtrinsicParams,
	SubstrateExtrinsicParamsBuilder as ParachainExtrinsicsParamsBuilder,
};
use subxt::{
	config::{extrinsic_params::Era, ExtrinsicParams},
	events::{Phase, StaticEvent},
	ext::frame_metadata::{
		ExtrinsicMetadata, OpaqueMetadata, RuntimeMetadata, RuntimeMetadataDeprecated,
		RuntimeMetadataPrefixed, RuntimeMetadataV14, META_RESERVED,
	},
	metadata::{DecodeStaticType, DecodeWithMetadata},
	storage,
	storage::{
		address::{StorageMapKey, Yes},
		StaticStorageAddress, StorageAddress,
	},
	tx::StaticTxPayload,
	Error, Metadata, OnlineClient,
};
use thiserror::Error;

// TODO: expose extrinsic param builder
#[derive(Debug, Clone)]
pub enum DefaultConfig {}

define_id!(DefaultId, relaychain::api::runtime_types::polkadot_parachain::primitives::Id);

define_head_data!(
	DefaultHeadData,
	relaychain::api::runtime_types::polkadot_parachain::primitives::HeadData,
);

pub struct DefaultRuntimeStorage;

impl RuntimeStorage for DefaultRuntimeStorage {
	type HeadData = DefaultHeadData;
	type Id = DefaultId;

	fn timestamp_now() -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, ()> {
		parachain_subxt::api::storage().timestamp().now()
	}

	fn paras_heads(
		x: u32,
	) -> LocalStaticStorageAddress<DecodeStaticType<Self::HeadData>, Yes, (), Yes> {
		let x = relaychain::api::storage().paras().heads(&Self::Id::from(x).0);
		let mut bytes = vec![];
		fn fake_metadata() -> Metadata {
			Metadata::try_from(RuntimeMetadataPrefixed(
				META_RESERVED,
				RuntimeMetadata::V14(RuntimeMetadataV14::new(
					Vec::new(),
					ExtrinsicMetadata {
						ty: MetaType::new::<()>(),
						version: 0,
						signed_extensions: vec![],
					},
					MetaType::new::<()>(),
				)),
			))
			.unwrap()
		}
		x.append_entry_bytes(&fake_metadata(), &mut bytes)
			.expect("should always succeed");
		LocalStaticStorageAddress {
			pallet_name: "Paras",
			entry_name: "Heads",
			storage_entry_keys: bytes,
			validation_hash: x.validation_hash(),
			_marker: Default::default(),
		}
	}

	fn grandpa_current_set_id() -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, ()> {
		relaychain::api::storage().grandpa().current_set_id()
	}

	fn babe_epoch_start() -> StaticStorageAddress<DecodeStaticType<(u32, u32)>, Yes, Yes, ()> {
		relaychain::api::storage().babe().epoch_start()
	}
}

pub type Balance = u128;

define_pallet_params!(PalletParamsWrapper, PalletParams, RawPalletParams);

define_send_ping_params!(SendPingParamsWrapper, SendPingParams, RawSendPingParams,);

define_transfer_params!(
	TransferParamsWrapper,
	TransferParams<AccountId32>,
	RawTransferParams<T>,
	RawTimeout,
	parachain_subxt::api::runtime_types::pallet_ibc::MultiAddress<T>,
);

define_any_wrapper!(AnyWrapper, parachain_subxt::api::runtime_types::pallet_ibc::Any);

pub struct DefaultRuntimeTransactions;

impl RuntimeTransactions for DefaultRuntimeTransactions {
	type Deliver = Deliver;
	type Transfer = Transfer;
	type Sudo = Sudo;
	type SendPing = SendPing;

	type ParaRuntimeCall = DefaultParaRuntimeCall;
	type SendPingParams = SendPingParams;
	type TransferParams = TransferParams<AccountId32>;

	fn ibc_deliver(messages: Vec<Any>) -> StaticTxPayload<Self::Deliver> {
		parachain_subxt::api::tx().ibc().deliver(
			messages
				.into_iter()
				.map(|x| parachain_subxt::api::runtime_types::pallet_ibc::Any {
					type_url: x.type_url.into_bytes(),
					value: x.value,
				})
				.collect(),
		)
	}

	fn ibc_transfer(
		params: Self::TransferParams,
		asset_id: u128,
		amount: u128,
		memo: Option<()>,
	) -> StaticTxPayload<Self::Transfer> {
		parachain_subxt::api::tx().ibc().transfer(
			TransferParamsWrapper(params).into(),
			asset_id,
			amount,
			memo.map(|_| MemoMessage),
		)
	}

	fn sudo_sudo(call: Self::ParaRuntimeCall) -> StaticTxPayload<Self::Sudo> {
		parachain_subxt::api::tx().sudo().sudo(call.0)
	}

	fn ibc_ping_send_ping(params: SendPingParams) -> StaticTxPayload<Self::SendPing> {
		parachain_subxt::api::tx()
			.ibc_ping()
			.send_ping(SendPingParamsWrapper(params).into())
	}
}

/// Allows to implement traits for the subxt generated code
define_ibc_event_wrapper!(IbcEventWrapper, MetadataIbcEvent);

#[derive(Decode)]
pub struct DefaultEventRecord(
	EventRecord<<DefaultConfig as light_client_common::config::Config>::ParaRuntimeEvent, H256>,
);

impl EventRecordT for DefaultEventRecord {
	type IbcEvent = pallet_ibc::events::IbcEvent;

	fn phase(&self) -> Phase {
		use parachain_subxt::api::runtime_types::frame_system::Phase as ParaPhase;
		match self.0.phase {
			ParaPhase::ApplyExtrinsic(i) => Phase::ApplyExtrinsic(i as u32),
			ParaPhase::Finalization => Phase::Finalization,
			ParaPhase::Initialization => Phase::Initialization,
		}
	}

	fn ibc_events(self) -> Option<Vec<pallet_ibc::events::IbcEvent>> {
		use parachain_subxt::api::runtime_types::{
			frame_system::EventRecord,
			pallet_ibc::pallet::{Call as IbcCall, Event as PalletEvent},
			parachain_runtime::{RuntimeCall, RuntimeEvent},
		};
		if let RuntimeEvent::Ibc(PalletEvent::Events { events }) = self.0.event.0 {
			let events = events
				.into_iter()
				.filter_map(|event| {
					let ev = event.ok()?;
					Some(pallet_ibc::events::IbcEvent::from(IbcEventWrapper(ev)))
				})
				.collect();
			Some(events)
		} else {
			None
		}
	}
}

#[derive(Decode)]
pub struct DefaultEvents(parachain_subxt::api::ibc::events::Events);

impl IbcEventsT for DefaultEvents {
	type IbcEvent = pallet_ibc::events::IbcEvent;

	fn events(self) -> Vec<Self::IbcEvent> {
		self.0
			.events
			.into_iter()
			.filter_map(|event| {
				let ev = event.ok()?;
				Some(Self::IbcEvent::from(IbcEventWrapper(ev)))
			})
			.collect()
	}
}

impl StaticEvent for DefaultEvents {
	const PALLET: &'static str = parachain_subxt::api::ibc::events::Events::PALLET;
	const EVENT: &'static str = parachain_subxt::api::ibc::events::Events::EVENT;

	fn is_event(pallet: &str, event: &str) -> bool {
		parachain_subxt::api::ibc::events::Events::is_event(pallet, event)
	}
}

#[derive(Decode)]
pub struct DefaultParaRuntimeCall(
	parachain_subxt::api::runtime_types::parachain_runtime::RuntimeCall,
);

#[derive(Decode)]
pub struct DefaultParaRuntimeEvent(
	parachain_subxt::api::runtime_types::parachain_runtime::RuntimeEvent,
);

impl RuntimeCall for DefaultParaRuntimeCall {
	type PalletParams = PalletParams;

	fn extract_ibc_deliver_messages(self) -> Option<Vec<Any>> {
		use parachain_subxt::api::runtime_types::{
			pallet_ibc::pallet::Call as IbcCall, parachain_runtime::RuntimeCall,
		};
		match self.0 {
			RuntimeCall::Ibc(IbcCall::deliver { messages }) =>
				Some(messages.into_iter().map(|m| AnyWrapper(m).into()).collect()),
			_ => None,
		}
	}

	fn pallet_ibc_set_params(params: PalletParams) -> Self {
		use parachain_subxt::api::runtime_types::{
			pallet_ibc::pallet::Call as IbcCall, parachain_runtime::RuntimeCall,
		};

		DefaultParaRuntimeCall(RuntimeCall::Ibc(IbcCall::set_params {
			params: PalletParamsWrapper(params).into(),
		}))
	}
}

#[async_trait]
impl light_client_common::config::Config for DefaultConfig {
	type AssetId = u128;
	type Signature = <Self as subxt::Config>::Signature;
	type Address = <Self as subxt::Config>::Address;
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
