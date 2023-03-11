use async_trait::async_trait;
use codec::{Decode, Encode};
use core::borrow::Borrow;
use ibc::events::IbcEvent;
use ibc_proto::google::protobuf::Any;
use sp_core::crypto::AccountId32;
use subxt::{
	client::OnlineClient,
	config::ExtrinsicParams,
	error::Error,
	events::{Phase, StaticEvent},
	metadata::{DecodeStaticType, DecodeWithMetadata, Metadata},
	storage,
	storage::{address::Yes, StaticStorageAddress, StorageAddress},
	tx::StaticTxPayload,
};

/// This represents a statically generated storage lookup address.
pub struct LocalStaticStorageAddress<ReturnTy, Fetchable, Defaultable, Iterable> {
	pub pallet_name: &'static str,
	pub entry_name: &'static str,
	// How to access the specific value at that storage address.
	pub storage_entry_keys: Vec<u8>,
	// Hash provided from static code for validation.
	pub validation_hash: Option<[u8; 32]>,
	pub _marker: std::marker::PhantomData<(ReturnTy, Fetchable, Defaultable, Iterable)>,
}

impl<ReturnTy, Fetchable, Defaultable, Iterable> StorageAddress
	for LocalStaticStorageAddress<ReturnTy, Fetchable, Defaultable, Iterable>
where
	ReturnTy: DecodeWithMetadata,
{
	type Target = ReturnTy;
	type IsDefaultable = Defaultable;
	type IsIterable = Iterable;
	type IsFetchable = Fetchable;

	fn pallet_name(&self) -> &str {
		self.pallet_name
	}

	fn entry_name(&self) -> &str {
		self.entry_name
	}

	fn append_entry_bytes(&self, _metadata: &Metadata, bytes: &mut Vec<u8>) -> Result<(), Error> {
		bytes.extend(&self.storage_entry_keys);
		Ok(())
	}

	fn validation_hash(&self) -> Option<[u8; 32]> {
		self.validation_hash
	}
}

pub trait RuntimeTransactions {
	type Deliver: Encode + Send + Sync;
	type Transfer: Encode + Send + Sync;
	type Sudo: Encode + Send + Sync;
	type SendPing: Encode + Send + Sync;
	type ParaRuntimeCall;

	type SendPingParams;
	type TransferParams;

	fn ibc_deliver(messages: Vec<Any>) -> StaticTxPayload<Self::Deliver>;
	fn ibc_transfer(
		params: Self::TransferParams,
		asset_id: u128,
		amount: u128,
		memo: Option<()>,
	) -> StaticTxPayload<Self::Transfer>;
	fn sudo_sudo(call: Self::ParaRuntimeCall) -> StaticTxPayload<Self::Sudo>;
	fn ibc_ping_send_ping(params: Self::SendPingParams) -> StaticTxPayload<Self::SendPing>;
}

pub trait RuntimeStorage {
	type HeadData: Decode + AsRef<[u8]> + Sync + Send;
	type Id: From<u32>;

	fn timestamp_now() -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, ()>;

	fn paras_heads(
		x: u32,
	) -> LocalStaticStorageAddress<DecodeStaticType<Self::HeadData>, Yes, (), Yes>;

	fn grandpa_current_set_id() -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, ()>;

	fn babe_epoch_start() -> StaticStorageAddress<DecodeStaticType<(u32, u32)>, Yes, Yes, ()>;
}

pub trait RuntimeCall {
	type PalletParams;

	fn extract_ibc_deliver_messages(self) -> Option<Vec<Any>>;
	fn pallet_ibc_set_params(params: Self::PalletParams) -> Self;
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
	/// Runtime call
	type ParaRuntimeCall: RuntimeCall + Decode;
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
