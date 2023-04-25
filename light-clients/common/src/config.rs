use alloc::borrow::Cow;
use async_trait::async_trait;
use codec::{Codec, Decode, Encode};
use ibc::events::IbcEvent;
use ibc_proto::google::protobuf::Any;
use sp_core::H256;
use sp_runtime::scale_info::MetaType;
use subxt::{
	client::OnlineClient,
	config::ExtrinsicParams,
	error::Error,
	events::{Phase, StaticEvent},
	ext::{
		frame_metadata::{
			ExtrinsicMetadata, RuntimeMetadata, RuntimeMetadataPrefixed, RuntimeMetadataV14,
			META_RESERVED,
		},
		scale_decode::DecodeAsType,
		scale_encode::{EncodeAsFields, EncodeAsType},
	},
	metadata::{DecodeWithMetadata, Metadata},
	storage::{
		address::{StaticStorageMapKey, Yes},
		Address, StorageAddress,
	},
	tx::Payload,
	utils::{Encoded, Static},
};

/// This represents a statically generated storage lookup address.
pub struct LocalAddress<StorageKey, ReturnTy, Fetchable, Defaultable, Iterable> {
	pub pallet_name: Cow<'static, str>,
	pub entry_name: Cow<'static, str>,
	// How to access the specific value at that storage address.
	pub storage_entry_keys: Vec<StorageKey>,
	// Hash provided from static code for validation.
	pub validation_hash: Option<[u8; 32]>,
	pub _marker: std::marker::PhantomData<(ReturnTy, Fetchable, Defaultable, Iterable)>,
}

impl<
		// StorageKey: EncodeWithMetadata,
		ReturnTy: DecodeWithMetadata,
		Fetchable,
		Defaultable,
		Iterable,
	> LocalAddress<StaticStorageMapKey, ReturnTy, Fetchable, Defaultable, Iterable>
{
	pub fn new<NewReturnTy>(
		pallet_name: Cow<'static, str>,
		entry_name: Cow<'static, str>,
		storage: Address<StaticStorageMapKey, ReturnTy, Fetchable, Defaultable, Iterable>,
	) -> LocalAddress<StaticStorageMapKey, NewReturnTy, Fetchable, Defaultable, Iterable> {
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
		storage
			.append_entry_bytes(&fake_metadata(), &mut bytes)
			.expect("should always succeed");
		LocalAddress {
			pallet_name,
			entry_name,
			storage_entry_keys: vec![Static(Encoded(bytes))],
			validation_hash: storage.validation_hash(),
			_marker: Default::default(),
		}
	}
}

impl<ReturnTy, Fetchable, Defaultable, Iterable> StorageAddress
	for LocalAddress<StaticStorageMapKey, ReturnTy, Fetchable, Defaultable, Iterable>
where
	// StorageKey: EncodeWithMetadata,
	ReturnTy: DecodeWithMetadata,
{
	type Target = ReturnTy;
	type IsDefaultable = Defaultable;
	type IsIterable = Iterable;
	type IsFetchable = Fetchable;

	fn pallet_name(&self) -> &str {
		&self.pallet_name
	}

	fn entry_name(&self) -> &str {
		&self.entry_name
	}

	fn append_entry_bytes(&self, _metadata: &Metadata, bytes: &mut Vec<u8>) -> Result<(), Error> {
		for k in &self.storage_entry_keys {
			bytes.extend(&k.0 .0);
		}
		Ok(())
	}

	fn validation_hash(&self) -> Option<[u8; 32]> {
		self.validation_hash
	}
}

pub trait RuntimeTransactions {
	type Deliver: Encode + EncodeAsFields + Send + Sync;
	type Transfer: Encode + EncodeAsFields + Send + Sync;
	type Sudo: Encode + EncodeAsFields + Send + Sync;
	type SendPing: Encode + EncodeAsFields + Send + Sync;
	type ParaRuntimeCall;

	type SendPingParams;
	type TransferParams;

	fn ibc_deliver(messages: Vec<Any>) -> Payload<Self::Deliver>;
	fn ibc_transfer(
		params: Self::TransferParams,
		asset_id: u128,
		amount: u128,
		memo: Option<()>,
	) -> Payload<Self::Transfer>;
	fn sudo_sudo(call: Self::ParaRuntimeCall) -> Payload<Self::Sudo>;
	fn ibc_ping_send_ping(params: Self::SendPingParams) -> Payload<Self::SendPing>;
	fn ibc_increase_counters() -> Self::ParaRuntimeCall;
}

pub trait BeefyAuthoritySetT {
	fn root(&self) -> H256;
	fn len(&self) -> u32;
}

pub trait ParaLifecycleT {
	fn is_parachain(&self) -> bool;
}

pub trait RuntimeStorage {
	type HeadData: Decode + DecodeAsType + AsRef<[u8]> + Into<Vec<u8>> + Sync + Send;
	type Id: From<u32> + Into<u32> + Decode + DecodeAsType + Send + Sync;
	type ParaLifecycle: ParaLifecycleT + Decode + DecodeAsType + Send + Sync;
	type BeefyAuthoritySet: BeefyAuthoritySetT + Codec + EncodeAsType + DecodeAsType + Send + Sync;

	fn timestamp_now() -> Address<StaticStorageMapKey, u64, Yes, Yes, ()>;

	fn paras_heads(x: u32) -> LocalAddress<StaticStorageMapKey, Self::HeadData, Yes, (), Yes>;
	// ) -> LocalAddress<DecodeStaticType<Self::HeadData>, Yes, (), Yes>;

	fn paras_para_lifecycles(
		x: u32,
	) -> LocalAddress<StaticStorageMapKey, Self::ParaLifecycle, Yes, (), Yes>;
	// ) -> LocalAddress<DecodeStaticType<Self::ParaLifecycle>, Yes, (), Yes>;

	fn paras_parachains() -> LocalAddress<StaticStorageMapKey, Vec<Static<Self::Id>>, Yes, Yes, ()>;

	fn grandpa_current_set_id() -> Address<StaticStorageMapKey, u64, Yes, Yes, ()>;

	fn beefy_validator_set_id() -> Address<StaticStorageMapKey, u64, Yes, Yes, ()>;

	fn beefy_authorities(
	) -> LocalAddress<StaticStorageMapKey, Vec<sp_beefy::crypto::Public>, Yes, Yes, ()>;

	fn mmr_leaf_beefy_next_authorities(
	) -> LocalAddress<StaticStorageMapKey, Self::BeefyAuthoritySet, Yes, Yes, ()>;

	fn babe_epoch_start() -> Address<StaticStorageMapKey, (u32, u32), Yes, Yes, ()>;
}

pub trait RuntimeCall {
	fn extract_ibc_deliver_messages(self) -> Option<Vec<Any>>;
}

pub trait EventRecordT {
	type IbcEvent: TryInto<IbcEvent>;

	fn phase(&self) -> Phase;
	fn ibc_events(self) -> Option<Vec<Self::IbcEvent>>;
}

pub trait IbcEventsT {
	type IbcEvent: TryInto<IbcEvent>;

	fn events(self) -> Vec<Self::IbcEvent>;
}

/// This allows end users of this crate return the correct extrinsic metadata required by their
/// runtimes into the transactions signed by this crate.
#[async_trait]
pub trait Config: subxt::Config + Sized {
	/// Asset Id type used by the parachain runtime
	type AssetId: codec::Codec + serde::Serialize + Send + Sync + 'static;
	/// the signature type of the runtime
	type Signature: sp_runtime::traits::Verify + From<<Self as subxt::Config>::Signature> + Decode;
	/// Address type used by the runtime;
	type Address: codec::Codec + From<<Self as subxt::Config>::Address>;
	/// Tip
	type Tip: Default + From<u128> + Send;
	/// Runtime call
	type ParaRuntimeCall: RuntimeCall + Decode + Send;
	/// Parachain runtime event
	type ParaRuntimeEvent;
	/// Parachain events. Used for subscriptions
	type Events: StaticEvent + IbcEventsT;
	/// The event is returned from the subscription
	type EventRecord: Decode + EventRecordT + Send;
	/// Runtime call
	type Storage: RuntimeStorage;
	/// Relay/para-chain transactions
	type Tx: RuntimeTransactions<ParaRuntimeCall = Self::ParaRuntimeCall>;
	/// Parachain signed extra
	type SignedExtra: Decode;

	/// use the subxt client to fetch any neccessary data needed for the extrinsic metadata.
	async fn custom_extrinsic_params(
		client: &OnlineClient<Self>,
	) -> Result<CustomExtrinsicParams<Self>, Error>;
}

pub type CustomExtrinsicParams<T> = <<T as subxt::Config>::ExtrinsicParams as ExtrinsicParams<
	<T as subxt::Config>::Index,
	<T as subxt::Config>::Hash,
>>::OtherParams;
