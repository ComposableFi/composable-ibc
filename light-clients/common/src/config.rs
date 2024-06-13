// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use alloc::borrow::Cow;
use async_trait::async_trait;
use ibc::events::IbcEvent;
use ibc_proto::google::protobuf::Any;
use parity_scale_codec::{Decode, Encode};
use sp_core::H256;
use subxt::{
	client::OnlineClient,
	config::ExtrinsicParams,
	error::{MetadataError, StorageAddressError},
	events::{Phase, StaticEvent},
	ext::{
		scale_decode::DecodeAsType,
		scale_encode::{EncodeAsFields, EncodeAsType},
		sp_runtime::scale_info::TypeDef,
	},
	metadata::{
		types::{StorageEntryType, StorageHasher},
		DecodeWithMetadata, EncodeWithMetadata, Metadata,
	},
	storage::{Address, DefaultAddress, StaticStorageKey, StorageKey},
	tx::{DefaultPayload, Payload},
	utils::{Encoded, Static, Yes},
};

// pub type LocalAddress<StorageKey, ReturnTy, Fetchable, Defaultable, Iterable> =
// 	DefaultAddress<StorageKey, ReturnTy, Fetchable, Defaultable, Iterable>;

pub type StaticStorageMapKey = StaticStorageKey<Vec<u8>>;
// pub type StaticStorageMapKey = Static<Encoded>;

///// This represents a statically generated storage lookup address.
pub struct LocalAddress<StorageKey, ReturnTy, Fetchable, Defaultable, Iterable> {
	pub pallet_name: Cow<'static, str>,
	pub entry_name: Cow<'static, str>,
	// How to access the specific value at that storage address.
	pub storage_entry_keys: Vec<StorageKey>,
	// Hash provided from static code for validation.
	pub validation_hash: Option<[u8; 32]>,
	pub _marker: std::marker::PhantomData<(ReturnTy, Fetchable, Defaultable, Iterable)>,
}

impl<Key, ReturnTy, Fetchable, Defaultable, Iterable>
	From<DefaultAddress<Key, ReturnTy, Fetchable, Defaultable, Iterable>>
	for LocalAddress<Key, ReturnTy, Fetchable, Defaultable, Iterable>
where
	Key: StorageKey,
{
	fn from(address: DefaultAddress<Key, ReturnTy, Fetchable, Defaultable, Iterable>) -> Self {
		// SAFETY: layout of the structs should be the same
		unsafe { std::mem::transmute(address) }
	}
}

impl<ReturnTy: DecodeWithMetadata, Fetchable, Defaultable, Iterable>
	LocalAddress<StaticStorageMapKey, ReturnTy, Fetchable, Defaultable, Iterable>
{
	pub fn new<NewReturnTy>(
		storage: DefaultAddress<StaticStorageMapKey, ReturnTy, Fetchable, Defaultable, Iterable>,
	) -> LocalAddress<StaticStorageMapKey, NewReturnTy, Fetchable, Defaultable, Iterable> {
		let storage = LocalAddress::from(storage);
		LocalAddress {
			pallet_name: storage.pallet_name,
			entry_name: storage.entry_name,
			storage_entry_keys: storage.storage_entry_keys,
			validation_hash: storage.validation_hash,
			_marker: Default::default(),
		}
	}
}

fn hash_bytes(input: &[u8], hasher: &StorageHasher, bytes: &mut Vec<u8>) {
	match hasher {
		StorageHasher::Identity => bytes.extend(input),
		StorageHasher::Blake2_128 => bytes.extend(sp_core::hashing::blake2_128(input)),
		StorageHasher::Blake2_128Concat => {
			bytes.extend(sp_core::hashing::blake2_128(input));
			bytes.extend(input);
		},
		StorageHasher::Blake2_256 => bytes.extend(sp_core::hashing::blake2_256(input)),
		StorageHasher::Twox128 => bytes.extend(sp_core::hashing::twox_128(input)),
		StorageHasher::Twox256 => bytes.extend(sp_core::hashing::twox_256(input)),
		StorageHasher::Twox64Concat => {
			bytes.extend(sp_core::hashing::twox_64(input));
			bytes.extend(input);
		},
	}
}

impl<ReturnTy, Fetchable, Defaultable, Iterable> Address
	for LocalAddress<StaticStorageMapKey, ReturnTy, Fetchable, Defaultable, Iterable>
where
	// StorageKey: EncodeWithMetadata,
	ReturnTy: DecodeWithMetadata,
{
	type Keys = StaticStorageMapKey;
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

	fn append_entry_bytes(
		&self,
		metadata: &Metadata,
		bytes: &mut Vec<u8>,
	) -> Result<(), subxt_core::Error> {
		let pallet = metadata.pallet_by_name_err(self.pallet_name())?;
		let storage = pallet
			.storage()
			.ok_or_else(|| MetadataError::StorageNotFoundInPallet(self.pallet_name().to_owned()))?;
		let entry = storage
			.entry_by_name(self.entry_name())
			.ok_or_else(|| MetadataError::StorageEntryNotFound(self.entry_name().to_owned()))?;

		match entry.entry_type() {
			StorageEntryType::Plain(_) =>
				if !self.storage_entry_keys.is_empty() {
					Err(StorageAddressError::TooManyKeys { expected: 0 }.into())
				} else {
					Ok(())
				},
			StorageEntryType::Map { hashers, key_ty, .. } => {
				let ty = metadata
					.types()
					.resolve(*key_ty)
					.ok_or(MetadataError::TypeNotFound(*key_ty))?;

				// If the key is a tuple, we encode each value to the corresponding tuple type.
				// If the key is not a tuple, encode a single value to the key type.
				let type_ids = match &ty.type_def {
					TypeDef::Tuple(tuple) =>
						sp_runtime::Either::Left(tuple.fields.iter().map(|f| f.id)),
					_other => sp_runtime::Either::Right(std::iter::once(*key_ty)),
				};

				if type_ids.len() != self.storage_entry_keys.len() {
					return Err(StorageAddressError::TooManyKeys { expected: type_ids.len() }.into())
				}

				if hashers.len() == 1 {
					// One hasher; hash a tuple of all SCALE encoded bytes with the one hash
					// function.
					let mut input = Vec::new();
					let iter = self.storage_entry_keys.iter().zip(type_ids);
					for (key, type_id) in iter {
						key.encode_storage_key(&mut input, hashers.iter(), type_id)?;
					}
					hash_bytes(&input, &hashers[0], bytes);
					Ok(())
				} else if hashers.len() == type_ids.len() {
					let iter = self.storage_entry_keys.iter().zip(type_ids).zip(hashers);
					// A hasher per field; encode and hash each field independently.
					for ((key, type_id), hasher) in iter {
						let mut input = Vec::new();
						key.encode_with_metadata(type_id, metadata, &mut input)?;
						hash_bytes(&input, hasher, bytes);
					}
					Ok(())
				} else {
					// Mismatch; wrong number of hashers/fields.
					Err(StorageAddressError::WrongNumberOfHashers {
						hashers: hashers.len(),
						fields: type_ids.len(),
					}
					.into())
				}
			},
		}
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
	type MemoMessage;

	fn ibc_deliver(messages: Vec<Any>) -> DefaultPayload<Self::Deliver>;
	fn ibc_transfer(
		params: Self::TransferParams,
		asset_id: u128,
		amount: u128,
		memo: Option<Self::MemoMessage>,
	) -> DefaultPayload<Self::Transfer>;
	fn sudo_sudo(call: Self::ParaRuntimeCall) -> DefaultPayload<Self::Sudo>;
	fn ibc_ping_send_ping(params: Self::SendPingParams) -> DefaultPayload<Self::SendPing>;
	fn ibc_increase_counters() -> Self::ParaRuntimeCall;
}

pub trait BeefyAuthoritySetT {
	fn root(&self) -> H256;
	fn len(&self) -> u32;
}

pub trait ParaLifecycleT {
	fn is_parachain(&self) -> bool;
}

pub trait AsInner {
	type Inner: Encode + Decode + DecodeAsType + EncodeAsType + Send + Sync;

	fn from_inner(inner: Self::Inner) -> Self;
}

pub trait AsInnerEvent {
	type Inner: Encode + Decode + DecodeAsType + EncodeAsType + Send + Sync + StaticEvent;

	fn from_inner(inner: Self::Inner) -> Self;
}

pub trait RuntimeStorage {
	type HeadData: AsRef<[u8]> + Into<Vec<u8>> + Sync + Send + AsInner;
	type Id: From<u32> + Into<u32> + Send + Sync + AsInner;
	type ParaLifecycle: ParaLifecycleT + Send + Sync + AsInner;
	type BeefyAuthoritySet: BeefyAuthoritySetT + Send + Sync + AsInner;

	fn timestamp_now() -> DefaultAddress<StaticStorageMapKey, u64, Yes, Yes, ()>;

	fn paras_heads(
		x: u32,
	) -> LocalAddress<StaticStorageMapKey, <Self::HeadData as AsInner>::Inner, Yes, (), Yes>;

	fn paras_para_lifecycles(
		x: u32,
	) -> LocalAddress<StaticStorageMapKey, <Self::ParaLifecycle as AsInner>::Inner, Yes, (), Yes>;

	fn paras_parachains(
	) -> LocalAddress<StaticStorageMapKey, Vec<Static<<Self::Id as AsInner>::Inner>>, Yes, Yes, ()>;

	fn grandpa_current_set_id() -> DefaultAddress<StaticStorageMapKey, u64, Yes, Yes, ()>;

	fn beefy_validator_set_id() -> DefaultAddress<StaticStorageMapKey, u64, Yes, Yes, ()>;

	fn beefy_authorities() -> LocalAddress<
		StaticStorageMapKey,
		Vec<sp_consensus_beefy::ecdsa_crypto::Public>,
		Yes,
		Yes,
		(),
	>;

	fn mmr_leaf_beefy_next_authorities(
	) -> LocalAddress<StaticStorageMapKey, <Self::BeefyAuthoritySet as AsInner>::Inner, Yes, Yes, ()>;

	fn babe_epoch_start() -> DefaultAddress<StaticStorageMapKey, (u32, u32), Yes, Yes, ()>;
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
	type AssetId: parity_scale_codec::Codec + serde::Serialize + Send + Sync + 'static;
	/// the signature type of the runtime
	type Signature: sp_runtime::traits::Verify + From<<Self as subxt::Config>::Signature> + Decode;
	/// Address type used by the runtime;
	type Address: parity_scale_codec::Codec + From<<Self as subxt::Config>::Address>;
	/// Tip
	type Tip: Default + From<u128> + Send;
	/// Runtime call
	type ParaRuntimeCall: RuntimeCall + Decode + Send;
	/// Parachain runtime event
	type ParaRuntimeEvent: AsInner;
	/// Parachain events. Used for subscriptions
	type Events: IbcEventsT + AsInnerEvent;
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
	) -> Result<CustomExtrinsicParams<Self>, subxt_core::Error>;
}

pub type CustomExtrinsicParams<T> =
	<<T as subxt::Config>::ExtrinsicParams as ExtrinsicParams<T>>::Params;
