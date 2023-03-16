#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unreachable_patterns)]
#![allow(clippy::type_complexity)]
#![allow(clippy::useless_format)]
#![allow(non_camel_case_types)]
#![deny(
	unused_imports,
	clippy::useless_conversion,
	bad_style,
	bare_trait_objects,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

//! Pallet IBC
//! Implements the ibc protocol for substrate runtimes.
#[macro_use]
extern crate alloc;

use codec::{Decode, Encode};
use core::fmt::Debug;
use cumulus_primitives_core::ParaId;
pub use pallet::*;
use scale_info::{
	prelude::{
		format,
		string::{String, ToString},
		vec,
	},
	TypeInfo,
};
use sp_runtime::{Either, RuntimeDebug};
use sp_std::{marker::PhantomData, prelude::*, str::FromStr};

mod channel;
mod client;
mod connection;
pub mod errors;
pub mod events;
pub mod ics20;
mod ics23;
pub mod light_clients;
mod port;
pub mod routing;
pub use client::HostConsensusProof;
pub use ibc_primitives::Timeout;
pub use light_client_common;

pub const MODULE_ID: &str = "pallet_ibc";

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Any {
	pub type_url: Vec<u8>,
	pub value: Vec<u8>,
}

impl From<ibc_proto::google::protobuf::Any> for Any {
	fn from(any: ibc_proto::google::protobuf::Any) -> Self {
		Self { type_url: any.type_url.as_bytes().to_vec(), value: any.value }
	}
}

/// Params needed to upgrade clients for all connected chains.
#[derive(
	frame_support::RuntimeDebug, PartialEq, Eq, scale_info::TypeInfo, Encode, Decode, Clone,
)]
pub struct UpgradeParams {
	/// Protobuf encoded client state
	pub client_state: Vec<u8>,
	/// Protobuf encoded consensus state
	pub consensus_state: Vec<u8>,
}

#[derive(
	frame_support::RuntimeDebug, PartialEq, Eq, scale_info::TypeInfo, Encode, Decode, Clone,
)]
pub enum MultiAddress<AccountId> {
	Id(AccountId),
	Raw(Vec<u8>),
}

#[derive(
	frame_support::RuntimeDebug, PartialEq, Eq, scale_info::TypeInfo, Encode, Decode, Clone,
)]
pub struct TransferParams<AccountId> {
	/// Account id or valid utf8 string bytes
	pub to: MultiAddress<AccountId>,
	/// Source channel identifier on host chain
	pub source_channel: u64,
	/// Timeout for this packet
	pub timeout: Timeout,
}

#[derive(
	frame_support::RuntimeDebug, PartialEq, Eq, scale_info::TypeInfo, Encode, Decode, Clone,
)]
pub enum LightClientProtocol {
	Beefy,
	Grandpa,
}
#[cfg(any(test, feature = "runtime-benchmarks"))]
pub(crate) mod benchmarks;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod ics20_fee;
mod impls;
pub mod weight;

pub use weight::WeightInfo;

use crate::ics20::{FlowType, Ics20RateLimiter};

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use core::time::Duration;

	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::{AssetId, Balance},
			ReservableCurrency, UnixTime,
		},
	};
	use frame_system::pallet_prelude::*;
	pub use ibc::signer::Signer;
	use sp_core::crypto::ByteArray;

	use crate::{
		ics20::HandleMemo,
		routing::{Context, ModuleRouter},
	};
	use ibc::{
		applications::transfer::{
			is_sender_chain_source, msgs::transfer::MsgTransfer, Amount, PrefixedCoin,
			PrefixedDenom,
		},
		core::{
			ics02_client::context::{ClientKeeper, ClientReader},
			ics04_channel::context::ChannelReader,
			ics24_host::identifier::{ChannelId, PortId},
		},
		timestamp::Timestamp,
		Height,
	};
	use ibc_primitives::{client_id_from_bytes, get_channel_escrow_address, IbcHandler};
	use light_clients::AnyClientState;
	use sp_runtime::{
		traits::{IdentifyAccount, Saturating},
		AccountId32, BoundedBTreeSet,
	};
	#[cfg(feature = "std")]
	use sp_runtime::{Deserialize, Serialize};
	use sp_std::collections::btree_set::BTreeSet;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + parachain_info::Config + core::fmt::Debug {
		type TimeProvider: UnixTime;
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Currency type of the runtime
		type NativeCurrency: ReservableCurrency<
			<Self as frame_system::Config>::AccountId,
			Balance = Self::Balance,
		>;
		/// Runtime balance type
		type Balance: Balance + From<u128>;
		/// AssetId type
		type AssetId: AssetId + MaybeSerializeDeserialize;
		/// The native asset id, this will use the `NativeCurrency` for all operations.
		#[pallet::constant]
		type NativeAssetId: Get<Self::AssetId>;
		/// Convert ibc denom to asset id and vice versa
		type IbcDenomToAssetIdConversion: DenomToAssetId<Self>;

		/// Prefix for events stored in the Off-chain DB via Indexing API, child trie and connection
		#[pallet::constant]
		type PalletPrefix: Get<&'static [u8]>;

		/// Light client protocol this chain is operating
		#[pallet::constant]
		type LightClientProtocol: Get<LightClientProtocol>;
		/// Account Id Conversion from SS58 string or hex string
		type AccountIdConversion: TryFrom<Signer>
			+ IdentifyAccount<AccountId = <Self as frame_system::Config>::AccountId>
			+ Clone;
		/// Set of traits needed to handle fungible assets
		type Fungibles: Transfer<
				<Self as frame_system::Config>::AccountId,
				Balance = Self::Balance,
				AssetId = Self::AssetId,
			> + Mutate<
				<Self as frame_system::Config>::AccountId,
				Balance = Self::Balance,
				AssetId = Self::AssetId,
			> + Inspect<
				<Self as frame_system::Config>::AccountId,
				Balance = Self::Balance,
				AssetId = Self::AssetId,
			>;
		/// Expected block time in milliseconds
		#[pallet::constant]
		type ExpectedBlockTime: Get<u64>;
		/// Port and Module resolution
		type Router: ModuleRouter;
		/// Minimum connection delay period in seconds for ibc connections that can be created or
		/// accepted. Ensure that this is non-zero in production as it's a critical vulnerability.
		#[pallet::constant]
		type MinimumConnectionDelay: Get<u64>;
		/// ParaId of the runtime
		type ParaId: Get<ParaId>;
		/// Relay chain this runtime is attached to
		type RelayChain: Get<light_client_common::RelayChain>;
		/// benchmarking weight info
		type WeightInfo: WeightInfo;
		/// Origin allowed to unfreeze light clients
		type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// Origin allowed to freeze light clients
		type FreezeOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// Amount to be reserved for client and connection creation
		#[pallet::constant]
		type SpamProtectionDeposit: Get<Self::Balance>;
		type IbcAccountId: Into<AccountId32>;
		type TransferOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::IbcAccountId>;
		type RelayerOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;
		type Ics20RateLimiter: Ics20RateLimiter;
		/// Handle Ics20 Memo
		type HandleMemo: HandleMemo<Self>;
		/// Memo Message types supported by the runtime
		type MemoMessage: codec::Codec
			+ FromStr
			+ ToString
			+ Debug
			+ scale_info::TypeInfo
			+ Clone
			+ Eq;

		type IsSendEnabled: Get<bool>;
		type IsReceiveEnabled: Get<bool>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	/// client_id , Height => Height
	pub type ClientUpdateHeight<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Blake2_128Concat,
		Vec<u8>,
		Vec<u8>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// client_id , Height => Timestamp
	pub type ClientUpdateTime<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, Vec<u8>, Blake2_128Concat, Vec<u8>, u64, OptionQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	pub type ChannelCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	pub type PacketCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// connection_identifier => Vec<(port_id, channel_id)>
	pub type ChannelsConnection<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<(Vec<u8>, Vec<u8>)>, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// counter for clients
	pub type ClientCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// counter for clients
	pub type ConnectionCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// counter for acknowledgments
	pub type AcknowledgementCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// counter for packet receipts
	pub type PacketReceiptCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// client_id => Vec<Connection_id>
	pub type ConnectionClient<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::storage]
	/// Map of asset id to ibc denom pairs (T::AssetId, Vec<u8>)
	/// ibc denoms represented as utf8 string bytes
	pub type IbcAssetIds<T: Config> =
		CountedStorageMap<_, Twox64Concat, T::AssetId, Vec<u8>, OptionQuery>;

	#[pallet::storage]
	/// Map of asset id to ibc denom pairs (Vec<u8>, T::AssetId)
	/// ibc denoms represented as utf8 string bytes
	pub type IbcDenoms<T: Config> =
		CountedStorageMap<_, Twox64Concat, Vec<u8>, T::AssetId, OptionQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// ChannelIds open from this module
	pub type ChannelIds<T: Config> = StorageValue<_, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// Active Escrow addresses
	pub type EscrowAddresses<T: Config> =
		StorageValue<_, BTreeSet<<T as frame_system::Config>::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	/// Consensus heights
	/// Stored as a tuple of (revision_number, revision_height)
	pub type ConsensusHeights<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		BoundedBTreeSet<Height, frame_support::traits::ConstU32<256>>,
		ValueQuery,
	>;

	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct AssetConfig<AssetId> {
		pub id: AssetId,
		pub denom: Vec<u8>,
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		/// This should contain the native currency's asset_id and denom.
		pub assets: Vec<AssetConfig<T::AssetId>>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { assets: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			assert!(
				!self.assets.is_empty(),
				"You must configure the native currency's asset_id and denom!"
			);
			for AssetConfig { id, denom } in &self.assets {
				IbcDenoms::<T>::insert(denom.clone(), id);
				IbcAssetIds::<T>::insert(id, denom);
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Events emitted by the ibc subsystem
		Events { events: Vec<Result<events::IbcEvent, errors::IbcError>> },
		/// An Ibc token transfer has been started
		TokenTransferInitiated {
			from: Vec<u8>,
			to: Vec<u8>,
			ibc_denom: Vec<u8>,
			local_asset_id: Option<T::AssetId>,
			amount: T::Balance,
			is_sender_source: bool,
			source_channel: Vec<u8>,
			destination_channel: Vec<u8>,
		},
		/// A channel has been opened
		ChannelOpened { channel_id: Vec<u8>, port_id: Vec<u8> },
		/// Pallet params updated
		ParamsUpdated { send_enabled: bool, receive_enabled: bool },
		/// An outgoing Ibc token transfer has been completed and burnt
		TokenTransferCompleted {
			from: Vec<u8>,
			to: Vec<u8>,
			ibc_denom: Vec<u8>,
			local_asset_id: Option<T::AssetId>,
			amount: T::Balance,
			is_sender_source: bool,
			source_channel: Vec<u8>,
			destination_channel: Vec<u8>,
		},
		/// Ibc tokens have been received and minted
		TokenReceived {
			from: Vec<u8>,
			to: Vec<u8>,
			ibc_denom: Vec<u8>,
			local_asset_id: Option<T::AssetId>,
			amount: T::Balance,
			is_receiver_source: bool,
			source_channel: Vec<u8>,
			destination_channel: Vec<u8>,
		},
		/// Ibc transfer failed, received an acknowledgement error, tokens have been refunded
		TokenTransferFailed {
			from: Vec<u8>,
			to: Vec<u8>,
			ibc_denom: Vec<u8>,
			local_asset_id: Option<T::AssetId>,
			amount: T::Balance,
			is_sender_source: bool,
			source_channel: Vec<u8>,
			destination_channel: Vec<u8>,
		},
		/// On recv packet was not processed successfully processes
		OnRecvPacketError { msg: Vec<u8> },
		/// Client upgrade path has been set
		ClientUpgradeSet,
		/// Client has been frozen
		ClientFrozen { client_id: Vec<u8>, height: u64, revision_number: u64 },
		/// Asset Admin Account Updated
		AssetAdminUpdated { admin_account: <T as frame_system::Config>::AccountId },
	}

	/// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error processing ibc messages
		ProcessingError,
		/// Error decoding some type
		DecodingError,
		/// Error encoding some type
		EncodingError,
		/// Error generating trie proof
		ProofGenerationError,
		/// Client consensus state not found for height
		ConsensusStateNotFound,
		/// Channel not found
		ChannelNotFound,
		/// Client state not found
		ClientStateNotFound,
		/// Connection not found
		ConnectionNotFound,
		/// Packet commitment wasn't found
		PacketCommitmentNotFound,
		/// Packet receipt wasn't found
		PacketReceiptNotFound,
		/// Packet Acknowledgment wasn't found
		PacketAcknowledgmentNotFound,
		/// Error constructing packet
		SendPacketError,
		/// Invalid channel id
		InvalidChannelId,
		/// Invalid port id
		InvalidPortId,
		/// Other forms of errors
		Other,
		/// Invalid route
		InvalidRoute,
		/// Invalid message for extrinsic
		InvalidMessageType,
		/// The interchain token transfer was not successfully initiated
		TransferInternals,
		TransferSerde,
		TransferOther,
		TransferProtocol,
		TransferSend,
		/// Error Decoding utf8 bytes
		Utf8Error,
		/// Invalid asset id
		InvalidAssetId,
		/// Invalid Ibc denom
		PrefixedDenomParse,
		/// Invalid amount
		InvalidAmount,
		/// Invalid timestamp
		InvalidTimestamp,
		/// Unable to get client revision number
		FailedToGetRevisionNumber,
		/// Invalid params passed
		InvalidParams,
		/// Error opening channel
		ChannelInitError,
		/// Latest height and timestamp for a client not found
		TimestampAndHeightNotFound,
		/// Failed to derive channel escrow address
		ChannelEscrowAddress,
		/// Error writing acknowledgement to storage
		WriteAckError,
		/// Client update time and height not found
		ClientUpdateNotFound,
		/// Error Freezing client
		ClientFreezeFailed,
		/// Access denied
		AccessDenied,
		RateLimiter,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		u32: From<<T as frame_system::Config>::BlockNumber>,
		T: Send + Sync,
		AccountId32: From<<T as frame_system::Config>::AccountId>,
	{
		fn offchain_worker(_n: BlockNumberFor<T>) {
			let _ = Pallet::<T>::packet_cleanup();
		}
	}

	// Dispatch able functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsic", which are often compared to transactions.
	// Dispatch able functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: Send + Sync,
		AccountId32: From<<T as frame_system::Config>::AccountId>,
		u32: From<<T as frame_system::Config>::BlockNumber>,
	{
		#[pallet::call_index(0)]
		#[pallet::weight(crate::weight::deliver::< T > (messages))]
		#[frame_support::transactional]
		pub fn deliver(origin: OriginFor<T>, messages: Vec<Any>) -> DispatchResult {
			use ibc::core::{
				ics02_client::msgs::create_client, ics03_connection::msgs::conn_open_init,
			};
			let sender = T::RelayerOrigin::ensure_origin(origin)?;

			// reserve a fixed deposit for every client and connection created
			// so people don't spam our chain with useless clients.
			let mut ctx = routing::Context::<T>::new();
			let mut reserve_count = 0u128;
			let messages = messages
				.into_iter()
				.filter_map(|message| {
					let type_url = String::from_utf8(message.type_url.clone()).ok()?;
					if matches!(
						type_url.as_str(),
						create_client::TYPE_URL |
							conn_open_init::TYPE_URL |
							ibc::core::ics04_channel::msgs::chan_open_init::TYPE_URL
					) {
						reserve_count += 1;
					}

					Some(Ok(ibc_proto::google::protobuf::Any { type_url, value: message.value }))
				})
				.collect::<Result<Vec<ibc_proto::google::protobuf::Any>, Error<T>>>()?;
			let reserve_amt = T::SpamProtectionDeposit::get().saturating_mul(reserve_count.into());

			if reserve_amt >= T::SpamProtectionDeposit::get() {
				<T::NativeCurrency as ReservableCurrency<
					<T as frame_system::Config>::AccountId,
				>>::reserve(&sender, reserve_amt.into())?;
			}
			Self::execute_ibc_messages(&mut ctx, messages);

			Ok(())
		}

		#[pallet::call_index(1)]
		#[frame_support::transactional]
		#[pallet::weight(<T as Config>::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			params: TransferParams<<T as frame_system::Config>::AccountId>,
			asset_id: T::AssetId,
			amount: T::Balance,
			memo: Option<T::MemoMessage>,
		) -> DispatchResult {
			let origin = T::TransferOrigin::ensure_origin(origin)?.into();
			let denom = T::IbcDenomToAssetIdConversion::from_asset_id_to_denom(asset_id)
				.ok_or_else(|| Error::<T>::InvalidAssetId)?;
			let account_id_32: AccountId32 = origin.into();
			let from = {
				let mut hex_string = hex::encode(account_id_32.to_raw_vec());
				hex_string.insert_str(0, "0x");
				hex_string
			};

			let to = match params.to {
				MultiAddress::Id(id) => {
					// we convert id to hex string instead of ss58 because destination chain could
					// have a different ss58 prefix from source chain
					let account_id_32: AccountId32 = id.into();
					let mut hex_string = hex::encode(account_id_32.to_raw_vec());
					hex_string.insert_str(0, "0x");
					hex_string
				},
				MultiAddress::Raw(bytes) =>
					String::from_utf8(bytes).map_err(|_| Error::<T>::Utf8Error)?,
			};
			let denom =
				PrefixedDenom::from_str(&denom).map_err(|_| Error::<T>::PrefixedDenomParse)?;
			let ibc_amount = Amount::from_str(&format!("{:?}", amount))
				.map_err(|_| Error::<T>::InvalidAmount)?;
			let coin = PrefixedCoin { denom, amount: ibc_amount };
			let source_channel = ChannelId::new(params.source_channel);
			let source_port = PortId::transfer();
			let (latest_height, latest_timestamp) =
				Pallet::<T>::latest_height_and_timestamp(&source_port, &source_channel)
					.map_err(|_| Error::<T>::TimestampAndHeightNotFound)?;

			let (timeout_height, timeout_timestamp) = match params.timeout {
				Timeout::Offset { timestamp, height } => {
					let timestamp = timestamp
						.map(|offset| (latest_timestamp + Duration::from_secs(offset)))
						.transpose()
						.map_err(|_| Error::<T>::InvalidTimestamp)?
						.unwrap_or_default();
					let height = height.map(|offset| latest_height.add(offset)).unwrap_or_default();
					(height, timestamp)
				},
				Timeout::Absolute { timestamp, height } => {
					let timestamp = timestamp
						.map(Timestamp::from_nanoseconds)
						.transpose()
						.map_err(|_| Error::<T>::InvalidTimestamp)?
						.unwrap_or_default();
					let height = height
						.map(|revision_height| {
							Height::new(latest_height.revision_number, revision_height)
						})
						.unwrap_or_default();
					(height, timestamp)
				},
			};

			let msg = MsgTransfer {
				source_port,
				source_channel: source_channel.clone(),
				token: coin.clone(),
				sender: Signer::from_str(&from).map_err(|_| Error::<T>::Utf8Error)?,
				receiver: Signer::from_str(&to).map_err(|_| Error::<T>::Utf8Error)?,
				timeout_height,
				timeout_timestamp,
				memo: memo.map(|memo| memo.to_string()).unwrap_or_default(),
			};

			T::Ics20RateLimiter::allow(&msg, FlowType::Transfer)
				.map_err(|_| Error::<T>::RateLimiter)?;
			let is_sender_source = is_sender_chain_source(
				msg.source_port.clone(),
				msg.source_channel,
				&msg.token.denom,
			);

			if is_sender_source {
				// Store escrow address
				let escrow_address =
					get_channel_escrow_address(&msg.source_port, msg.source_channel)
						.map_err(|_| Error::<T>::ChannelEscrowAddress)?;
				let account_id = T::AccountIdConversion::try_from(escrow_address)
					.map_err(|_| Error::<T>::ChannelEscrowAddress)?
					.into_account();
				let _ = EscrowAddresses::<T>::try_mutate::<_, &'static str, _>(|addresses| {
					if !addresses.contains(&account_id) {
						addresses.insert(account_id);
						Ok(())
					} else {
						Err("Address already exists")
					}
				});
			}

			Pallet::<T>::send_transfer(msg).map_err(|e| {
				log::debug!(target: "pallet_ibc", "[transfer]: error: {:?}", &e);
				use ibc_primitives::Error::*;
				match e {
					SendPacketError { .. } => Error::<T>::TransferSend,
					SendTransferError { .. } => Error::<T>::TransferSend,

					ReceivePacketError { .. } => Error::<T>::TransferProtocol,
					WriteAcknowledgementError { .. } => Error::<T>::TransferProtocol,
					AcknowledgementError { .. } => Error::<T>::TransferProtocol,
					TimeoutError { .. } => Error::<T>::TransferProtocol,

					TimestampOrHeightNotFound { .. } => Error::<T>::TransferInternals,
					ChannelOrPortError { .. } => Error::<T>::TransferInternals,
					ClientStateError { .. } => Error::<T>::TransferInternals,
					ConnectionIdError { .. } => Error::<T>::TransferInternals,
					ClientIdError { .. } => Error::<T>::TransferInternals,
					BindPortError { .. } => Error::<T>::TransferInternals,
					ChannelInitError { .. } => Error::<T>::TransferInternals,
					ChannelCloseError { .. } => Error::<T>::TransferInternals,

					DecodingError { .. } => Error::<T>::TransferSerde,
					ErrorDecodingPrefix => Error::<T>::TransferSerde,

					Other { .. } => Error::<T>::TransferOther,
				}
			})?;
			let ctx = Context::<T>::default();
			let channel_end = ctx
				.channel_end(&(PortId::transfer(), source_channel))
				.map_err(|_| Error::<T>::ChannelNotFound)?;

			Self::deposit_event(Event::<T>::TokenTransferInitiated {
				from: from.as_bytes().to_vec(),
				to: to.as_bytes().to_vec(),
				amount,
				local_asset_id: T::IbcDenomToAssetIdConversion::from_denom_to_asset_id(
					&coin.denom.to_string(),
				)
				.ok(),
				ibc_denom: coin.denom.to_string().as_bytes().to_vec(),
				is_sender_source,
				source_channel: source_channel.to_string().as_bytes().to_vec(),
				destination_channel: channel_end
					.counterparty()
					.channel_id
					.ok_or_else(|| Error::<T>::ChannelNotFound)?
					.to_string()
					.as_bytes()
					.to_vec(),
			});
			Ok(())
		}

		/// We write the consensus & client state under these predefined paths so that
		/// we can produce state proofs of the values to connected chains
		/// in order to execute client upgrades.
		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn upgrade_client(origin: OriginFor<T>, params: UpgradeParams) -> DispatchResult {
			<T as Config>::AdminOrigin::ensure_origin(origin)?;
			const CLIENT_STATE_UPGRADE_PATH: &[u8] = b"client-state-upgrade-path";
			const CONSENSUS_STATE_UPGRADE_PATH: &[u8] = b"consensus-state-upgrade-path";

			sp_io::storage::set(CLIENT_STATE_UPGRADE_PATH, &params.client_state);
			sp_io::storage::set(CONSENSUS_STATE_UPGRADE_PATH, &params.consensus_state);

			Self::deposit_event(Event::<T>::ClientUpgradeSet.into());

			Ok(())
		}

		/// Freeze a client at a specific height
		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn freeze_client(
			origin: OriginFor<T>,
			client_id: Vec<u8>,
			height: u64,
		) -> DispatchResult {
			use ibc::core::ics02_client::client_state::ClientState;
			<T as Config>::FreezeOrigin::ensure_origin(origin)?;
			let client_id =
				client_id_from_bytes(client_id).map_err(|_| Error::<T>::DecodingError)?;
			let mut ctx = routing::Context::<T>::default();
			let client_state =
				ctx.client_state(&client_id).map_err(|_| Error::<T>::ClientStateNotFound)?;
			let frozen_state = match client_state {
				AnyClientState::Grandpa(grandpa) => {
					let latest_height = grandpa.latest_height();
					AnyClientState::wrap(
						&grandpa
							.with_frozen_height(Height::new(latest_height.revision_number, height))
							.map_err(|_| Error::<T>::ClientFreezeFailed)?,
					)
				},
				AnyClientState::Beefy(beefy) => {
					let latest_height = beefy.latest_height();
					AnyClientState::wrap(
						&beefy
							.with_frozen_height(Height::new(latest_height.revision_number, height))
							.map_err(|_| Error::<T>::ClientFreezeFailed)?,
					)
				},
				AnyClientState::Tendermint(tm) => {
					let latest_height = tm.latest_height();
					AnyClientState::wrap(
						&tm.with_frozen_height(Height::new(latest_height.revision_number, height))
							.map_err(|_| Error::<T>::ClientFreezeFailed)?,
					)
				},
				AnyClientState::Wasm(_) => return Err(Error::<T>::ClientFreezeFailed.into()),
				#[cfg(test)]
				AnyClientState::Mock(mut ms) => {
					ms.frozen_height =
						Some(Height::new(ms.latest_height().revision_number, height));
					AnyClientState::wrap(&ms)
				},
			}
			.ok_or_else(|| Error::<T>::ClientFreezeFailed)?;
			let revision_number = frozen_state.latest_height().revision_number;
			ctx.store_client_state(client_id.clone(), frozen_state)
				.map_err(|_| Error::<T>::ClientFreezeFailed)?;

			Self::deposit_event(Event::<T>::ClientFrozen {
				client_id: client_id.as_bytes().to_vec(),
				height,
				revision_number,
			});

			Ok(())
		}
	}
}

/// Result of the `DenomToAssetId::ibc_assets` function.
pub struct IbcAssets<AssetId> {
	/// List of IBC denoms.
	pub denoms: Vec<Vec<u8>>,
	/// Total number of IBC assets on the chain.
	pub total_count: u64,
	/// The next `AssetId` after the last item in the list.
	pub next_id: Option<AssetId>,
}

pub trait DenomToAssetId<T: Config> {
	type Error: Debug;

	/// Get the equivalent asset id for this ibc denom
	/// **Note**
	/// This function should create and register an asset with a valid metadata
	/// if an asset does not exist for this denom
	fn from_denom_to_asset_id(denom: &String) -> Result<T::AssetId, Self::Error>;

	/// Return full denom for given asset id
	fn from_asset_id_to_denom(id: T::AssetId) -> Option<String>;

	/// Returns `IbcAssets` containing a list of assets bound by `limit`.
	/// `start_key` is either an `AssetId` or an offset to start from.
	fn ibc_assets(start_key: Option<Either<T::AssetId, u32>>, limit: u64) -> IbcAssets<T::AssetId>;
}
