#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
pub mod api {
	#[allow(unused_imports)]
	mod root_mod {
		pub use super::*;
	}
	pub static PALLETS: [&str; 5usize] = ["System", "Babe", "Timestamp", "Grandpa", "Paras"];
	#[doc = r" The error type returned when there is a runtime issue."]
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
	#[derive(
		:: subxt :: ext :: codec :: Decode,
		:: subxt :: ext :: codec :: Encode,
		:: subxt :: ext :: scale_decode :: DecodeAsType,
		:: subxt :: ext :: scale_encode :: EncodeAsType,
		Debug,
	)]
	#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
	#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
	pub enum Event {
		#[codec(index = 0)]
		System(system::Event),
		#[codec(index = 11)]
		Grandpa(grandpa::Event),
		#[codec(index = 56)]
		Paras(paras::Event),
	}
	impl ::subxt::events::RootEvent for Event {
		fn root_event(
			pallet_bytes: &[u8],
			pallet_name: &str,
			pallet_ty: u32,
			metadata: &::subxt::Metadata,
		) -> Result<Self, ::subxt::Error> {
			use ::subxt::metadata::DecodeWithMetadata;
			if pallet_name == "System" {
				return Ok(Event::System(system::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Grandpa" {
				return Ok(Event::Grandpa(grandpa::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Paras" {
				return Ok(Event::Paras(paras::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			Err(::subxt::ext::scale_decode::Error::custom(format!(
				"Pallet name '{}' not found in root Event enum",
				pallet_name
			))
			.into())
		}
	}
	pub fn constants() -> ConstantsApi {
		ConstantsApi
	}
	pub fn storage() -> StorageApi {
		StorageApi
	}
	pub fn tx() -> TransactionApi {
		TransactionApi
	}
	pub struct ConstantsApi;
	impl ConstantsApi {
		pub fn system(&self) -> system::constants::ConstantsApi {
			system::constants::ConstantsApi
		}
		pub fn babe(&self) -> babe::constants::ConstantsApi {
			babe::constants::ConstantsApi
		}
		pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
			timestamp::constants::ConstantsApi
		}
		pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
			grandpa::constants::ConstantsApi
		}
		pub fn paras(&self) -> paras::constants::ConstantsApi {
			paras::constants::ConstantsApi
		}
	}
	pub struct StorageApi;
	impl StorageApi {
		pub fn system(&self) -> system::storage::StorageApi {
			system::storage::StorageApi
		}
		pub fn babe(&self) -> babe::storage::StorageApi {
			babe::storage::StorageApi
		}
		pub fn timestamp(&self) -> timestamp::storage::StorageApi {
			timestamp::storage::StorageApi
		}
		pub fn grandpa(&self) -> grandpa::storage::StorageApi {
			grandpa::storage::StorageApi
		}
		pub fn paras(&self) -> paras::storage::StorageApi {
			paras::storage::StorageApi
		}
	}
	pub struct TransactionApi;
	impl TransactionApi {
		pub fn system(&self) -> system::calls::TransactionApi {
			system::calls::TransactionApi
		}
		pub fn babe(&self) -> babe::calls::TransactionApi {
			babe::calls::TransactionApi
		}
		pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
			timestamp::calls::TransactionApi
		}
		pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
			grandpa::calls::TransactionApi
		}
		pub fn paras(&self) -> paras::calls::TransactionApi {
			paras::calls::TransactionApi
		}
	}
	#[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
	pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
		client: &C,
	) -> Result<(), ::subxt::error::MetadataError> {
		let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
		if runtime_metadata_hash !=
			[
				206u8, 48u8, 130u8, 7u8, 136u8, 219u8, 227u8, 195u8, 193u8, 244u8, 112u8, 170u8,
				200u8, 27u8, 161u8, 121u8, 27u8, 110u8, 189u8, 174u8, 246u8, 20u8, 199u8, 103u8,
				245u8, 115u8, 42u8, 77u8, 241u8, 83u8, 4u8, 0u8,
			] {
			Err(::subxt::error::MetadataError::IncompatibleMetadata)
		} else {
			Ok(())
		}
	}
	pub mod system {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Remark {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SetHeapPages {
				pub pages: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SetCode {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SetCodeWithoutChecks {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SetStorage {
				pub items: ::std::vec::Vec<(
					::std::vec::Vec<::core::primitive::u8>,
					::std::vec::Vec<::core::primitive::u8>,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct KillStorage {
				pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct KillPrefix {
				pub prefix: ::std::vec::Vec<::core::primitive::u8>,
				pub subkeys: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RemarkWithEvent {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn remark(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<Remark> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark",
						Remark { remark },
						[
							101u8, 80u8, 195u8, 226u8, 224u8, 247u8, 60u8, 128u8, 3u8, 101u8, 51u8,
							147u8, 96u8, 126u8, 76u8, 230u8, 194u8, 227u8, 191u8, 73u8, 160u8,
							146u8, 87u8, 147u8, 243u8, 28u8, 228u8, 116u8, 224u8, 181u8, 129u8,
							160u8,
						],
					)
				}
				pub fn set_heap_pages(
					&self,
					pages: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<SetHeapPages> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_heap_pages",
						SetHeapPages { pages },
						[
							43u8, 103u8, 128u8, 49u8, 156u8, 136u8, 11u8, 204u8, 80u8, 6u8, 244u8,
							86u8, 171u8, 44u8, 140u8, 225u8, 142u8, 198u8, 43u8, 87u8, 26u8, 45u8,
							125u8, 222u8, 165u8, 254u8, 172u8, 158u8, 39u8, 178u8, 86u8, 87u8,
						],
					)
				}
				pub fn set_code(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<SetCode> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code",
						SetCode { code },
						[
							27u8, 104u8, 244u8, 205u8, 188u8, 254u8, 121u8, 13u8, 106u8, 120u8,
							244u8, 108u8, 97u8, 84u8, 100u8, 68u8, 26u8, 69u8, 93u8, 128u8, 107u8,
							4u8, 3u8, 142u8, 13u8, 134u8, 196u8, 62u8, 113u8, 181u8, 14u8, 40u8,
						],
					)
				}
				pub fn set_code_without_checks(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<SetCodeWithoutChecks> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code_without_checks",
						SetCodeWithoutChecks { code },
						[
							102u8, 160u8, 125u8, 235u8, 30u8, 23u8, 45u8, 239u8, 112u8, 148u8,
							159u8, 158u8, 42u8, 93u8, 206u8, 94u8, 80u8, 250u8, 66u8, 195u8, 60u8,
							40u8, 142u8, 169u8, 183u8, 80u8, 80u8, 96u8, 3u8, 231u8, 99u8, 216u8,
						],
					)
				}
				pub fn set_storage(
					&self,
					items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				) -> ::subxt::tx::Payload<SetStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_storage",
						SetStorage { items },
						[
							74u8, 43u8, 106u8, 255u8, 50u8, 151u8, 192u8, 155u8, 14u8, 90u8, 19u8,
							45u8, 165u8, 16u8, 235u8, 242u8, 21u8, 131u8, 33u8, 172u8, 119u8, 78u8,
							140u8, 10u8, 107u8, 202u8, 122u8, 235u8, 181u8, 191u8, 22u8, 116u8,
						],
					)
				}
				pub fn kill_storage(
					&self,
					keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				) -> ::subxt::tx::Payload<KillStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_storage",
						KillStorage { keys },
						[
							174u8, 174u8, 13u8, 174u8, 75u8, 138u8, 128u8, 235u8, 222u8, 216u8,
							85u8, 18u8, 198u8, 1u8, 138u8, 70u8, 19u8, 108u8, 209u8, 41u8, 228u8,
							67u8, 130u8, 230u8, 160u8, 207u8, 11u8, 180u8, 139u8, 242u8, 41u8,
							15u8,
						],
					)
				}
				pub fn kill_prefix(
					&self,
					prefix: ::std::vec::Vec<::core::primitive::u8>,
					subkeys: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<KillPrefix> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_prefix",
						KillPrefix { prefix, subkeys },
						[
							203u8, 116u8, 217u8, 42u8, 154u8, 215u8, 77u8, 217u8, 13u8, 22u8,
							193u8, 2u8, 128u8, 115u8, 179u8, 115u8, 187u8, 218u8, 129u8, 34u8,
							80u8, 4u8, 173u8, 120u8, 92u8, 35u8, 237u8, 112u8, 201u8, 207u8, 200u8,
							48u8,
						],
					)
				}
				pub fn remark_with_event(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<RemarkWithEvent> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark_with_event",
						RemarkWithEvent { remark },
						[
							123u8, 225u8, 180u8, 179u8, 144u8, 74u8, 27u8, 85u8, 101u8, 75u8,
							134u8, 44u8, 181u8, 25u8, 183u8, 158u8, 14u8, 213u8, 56u8, 225u8,
							136u8, 88u8, 26u8, 114u8, 178u8, 43u8, 176u8, 43u8, 240u8, 84u8, 116u8,
							46u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExtrinsicSuccess {
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExtrinsicFailed {
				pub dispatch_error: runtime_types::sp_runtime::DispatchError,
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CodeUpdated;
			impl ::subxt::events::StaticEvent for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NewAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct KilledAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Remarked {
				pub sender: ::subxt::utils::AccountId32,
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Remarked {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "Remarked";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Account",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							248u8, 178u8, 160u8, 222u8, 45u8, 231u8, 115u8, 164u8, 98u8, 184u8,
							174u8, 206u8, 149u8, 190u8, 175u8, 34u8, 202u8, 230u8, 69u8, 218u8,
							83u8, 43u8, 170u8, 41u8, 106u8, 77u8, 233u8, 97u8, 114u8, 14u8, 155u8,
							131u8,
						],
					)
				}
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Account",
						Vec::new(),
						[
							248u8, 178u8, 160u8, 222u8, 45u8, 231u8, 115u8, 164u8, 98u8, 184u8,
							174u8, 206u8, 149u8, 190u8, 175u8, 34u8, 202u8, 230u8, 69u8, 218u8,
							83u8, 43u8, 170u8, 41u8, 106u8, 77u8, 233u8, 97u8, 114u8, 14u8, 155u8,
							131u8,
						],
					)
				}
				pub fn extrinsic_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicCount",
						vec![],
						[
							223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8,
							222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8,
							41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8, 163u8, 231u8,
						],
					)
				}
				pub fn block_weight(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockWeight",
						vec![],
						[
							120u8, 67u8, 71u8, 163u8, 36u8, 202u8, 52u8, 106u8, 143u8, 155u8,
							144u8, 87u8, 142u8, 241u8, 232u8, 183u8, 56u8, 235u8, 27u8, 237u8,
							20u8, 202u8, 33u8, 85u8, 189u8, 0u8, 28u8, 52u8, 198u8, 40u8, 219u8,
							54u8,
						],
					)
				}
				pub fn all_extrinsics_len(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"AllExtrinsicsLen",
						vec![],
						[
							202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8, 74u8, 93u8, 164u8,
							254u8, 248u8, 254u8, 192u8, 32u8, 117u8, 96u8, 149u8, 53u8, 145u8,
							219u8, 64u8, 234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8, 28u8,
							134u8, 60u8,
						],
					)
				}
				pub fn block_hash(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
							21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
							80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
						],
					)
				}
				pub fn block_hash_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						Vec::new(),
						[
							50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
							21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
							80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
						],
					)
				}
				pub fn extrinsic_data(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicData",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
							254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
							59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
						],
					)
				}
				pub fn extrinsic_data_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicData",
						Vec::new(),
						[
							210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
							254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
							59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
						],
					)
				}
				pub fn number(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Number",
						vec![],
						[
							228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8, 235u8,
							246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8, 204u8,
							36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8, 235u8,
							164u8, 191u8,
						],
					)
				}
				pub fn parent_hash(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ParentHash",
						vec![],
						[
							232u8, 206u8, 177u8, 119u8, 38u8, 57u8, 233u8, 50u8, 225u8, 49u8,
							169u8, 176u8, 210u8, 51u8, 231u8, 176u8, 234u8, 186u8, 188u8, 112u8,
							15u8, 152u8, 195u8, 232u8, 201u8, 97u8, 208u8, 249u8, 9u8, 163u8, 69u8,
							36u8,
						],
					)
				}
				pub fn digest(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_runtime::generic::digest::Digest,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Digest",
						vec![],
						[
							83u8, 141u8, 200u8, 132u8, 182u8, 55u8, 197u8, 122u8, 13u8, 159u8,
							31u8, 42u8, 60u8, 191u8, 89u8, 221u8, 242u8, 47u8, 199u8, 213u8, 48u8,
							216u8, 131u8, 168u8, 245u8, 82u8, 56u8, 190u8, 62u8, 69u8, 96u8, 37u8,
						],
					)
				}
				pub fn events(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::frame_system::EventRecord<
							runtime_types::polkadot_runtime::RuntimeEvent,
							::subxt::utils::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Events",
						vec![],
						[
							4u8, 136u8, 15u8, 48u8, 79u8, 139u8, 254u8, 86u8, 195u8, 173u8, 54u8,
							2u8, 95u8, 168u8, 133u8, 59u8, 91u8, 209u8, 154u8, 132u8, 176u8, 29u8,
							46u8, 103u8, 80u8, 28u8, 207u8, 9u8, 241u8, 171u8, 98u8, 98u8,
						],
					)
				}
				pub fn event_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventCount",
						vec![],
						[
							236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8,
							203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8,
							161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8, 66u8,
							112u8,
						],
					)
				}
				pub fn event_topics(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventTopics",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
							63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
							111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
							38u8,
						],
					)
				}
				pub fn event_topics_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventTopics",
						Vec::new(),
						[
							205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
							63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
							111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
							38u8,
						],
					)
				}
				pub fn last_runtime_upgrade(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::LastRuntimeUpgradeInfo,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"LastRuntimeUpgrade",
						vec![],
						[
							52u8, 37u8, 117u8, 111u8, 57u8, 130u8, 196u8, 14u8, 99u8, 77u8, 91u8,
							126u8, 178u8, 249u8, 78u8, 34u8, 9u8, 194u8, 92u8, 105u8, 113u8, 81u8,
							185u8, 127u8, 245u8, 184u8, 60u8, 29u8, 234u8, 182u8, 96u8, 196u8,
						],
					)
				}
				pub fn upgraded_to_u32_ref_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"UpgradedToU32RefCount",
						vec![],
						[
							171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8, 175u8,
							178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8,
							83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8, 131u8, 204u8,
						],
					)
				}
				pub fn upgraded_to_triple_ref_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"UpgradedToTripleRefCount",
						vec![],
						[
							90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8, 201u8,
							210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8, 103u8,
							155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8, 40u8,
							185u8,
						],
					)
				}
				pub fn execution_phase(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::Phase,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExecutionPhase",
						vec![],
						[
							230u8, 183u8, 221u8, 135u8, 226u8, 223u8, 55u8, 104u8, 138u8, 224u8,
							103u8, 156u8, 222u8, 99u8, 203u8, 199u8, 164u8, 168u8, 193u8, 133u8,
							201u8, 155u8, 63u8, 95u8, 17u8, 206u8, 165u8, 123u8, 161u8, 33u8,
							172u8, 93u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn block_weights(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockWeights>
				{
					::subxt::constants::Address::new_static(
						"System",
						"BlockWeights",
						[
							118u8, 253u8, 239u8, 217u8, 145u8, 115u8, 85u8, 86u8, 172u8, 248u8,
							139u8, 32u8, 158u8, 126u8, 172u8, 188u8, 197u8, 105u8, 145u8, 235u8,
							171u8, 50u8, 31u8, 225u8, 167u8, 187u8, 241u8, 87u8, 6u8, 17u8, 234u8,
							185u8,
						],
					)
				}
				pub fn block_length(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockLength> {
					::subxt::constants::Address::new_static(
						"System",
						"BlockLength",
						[
							116u8, 184u8, 225u8, 228u8, 207u8, 203u8, 4u8, 220u8, 234u8, 198u8,
							150u8, 108u8, 205u8, 87u8, 194u8, 131u8, 229u8, 51u8, 140u8, 4u8, 47u8,
							12u8, 200u8, 144u8, 153u8, 62u8, 51u8, 39u8, 138u8, 205u8, 203u8,
							236u8,
						],
					)
				}
				pub fn block_hash_count(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"System",
						"BlockHashCount",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn db_weight(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_weights::RuntimeDbWeight> {
					::subxt::constants::Address::new_static(
						"System",
						"DbWeight",
						[
							124u8, 162u8, 190u8, 149u8, 49u8, 177u8, 162u8, 231u8, 62u8, 167u8,
							199u8, 181u8, 43u8, 232u8, 185u8, 116u8, 195u8, 51u8, 233u8, 223u8,
							20u8, 129u8, 246u8, 13u8, 65u8, 180u8, 64u8, 9u8, 157u8, 59u8, 245u8,
							118u8,
						],
					)
				}
				pub fn version(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_version::RuntimeVersion> {
					::subxt::constants::Address::new_static(
						"System",
						"Version",
						[
							93u8, 98u8, 57u8, 243u8, 229u8, 8u8, 234u8, 231u8, 72u8, 230u8, 139u8,
							47u8, 63u8, 181u8, 17u8, 2u8, 220u8, 231u8, 104u8, 237u8, 185u8, 143u8,
							165u8, 253u8, 188u8, 76u8, 147u8, 12u8, 170u8, 26u8, 74u8, 200u8,
						],
					)
				}
				pub fn ss58_prefix(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
					::subxt::constants::Address::new_static(
						"System",
						"SS58Prefix",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
							41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
							90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}
			}
		}
	}
	pub mod babe {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ReportEquivocation {
				pub equivocation_proof: ::std::boxed::Box<
					runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::sp_runtime::generic::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
						runtime_types::sp_consensus_babe::app::Public,
					>,
				>,
				pub key_owner_proof: runtime_types::sp_session::MembershipProof,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ReportEquivocationUnsigned {
				pub equivocation_proof: ::std::boxed::Box<
					runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::sp_runtime::generic::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
						runtime_types::sp_consensus_babe::app::Public,
					>,
				>,
				pub key_owner_proof: runtime_types::sp_session::MembershipProof,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PlanConfigChange {
				pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn report_equivocation(
					&self,
					equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::sp_runtime::generic::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
						runtime_types::sp_consensus_babe::app::Public,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::Payload<ReportEquivocation> {
					::subxt::tx::Payload::new_static(
						"Babe",
						"report_equivocation",
						ReportEquivocation {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							177u8, 237u8, 107u8, 138u8, 237u8, 233u8, 30u8, 195u8, 112u8, 176u8,
							185u8, 113u8, 157u8, 221u8, 134u8, 151u8, 62u8, 151u8, 64u8, 164u8,
							254u8, 112u8, 2u8, 94u8, 175u8, 79u8, 160u8, 3u8, 72u8, 145u8, 244u8,
							137u8,
						],
					)
				}
				pub fn report_equivocation_unsigned(
					&self,
					equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::sp_runtime::generic::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
						runtime_types::sp_consensus_babe::app::Public,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::Payload<ReportEquivocationUnsigned> {
					::subxt::tx::Payload::new_static(
						"Babe",
						"report_equivocation_unsigned",
						ReportEquivocationUnsigned {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							56u8, 103u8, 238u8, 118u8, 61u8, 192u8, 222u8, 87u8, 254u8, 24u8,
							138u8, 219u8, 210u8, 85u8, 201u8, 147u8, 128u8, 49u8, 199u8, 144u8,
							46u8, 158u8, 163u8, 31u8, 101u8, 224u8, 72u8, 98u8, 68u8, 120u8, 215u8,
							19u8,
						],
					)
				}
				pub fn plan_config_change(
					&self,
					config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
				) -> ::subxt::tx::Payload<PlanConfigChange> {
					::subxt::tx::Payload::new_static(
						"Babe",
						"plan_config_change",
						PlanConfigChange { config },
						[
							229u8, 157u8, 41u8, 58u8, 56u8, 4u8, 52u8, 107u8, 104u8, 20u8, 42u8,
							110u8, 1u8, 17u8, 45u8, 196u8, 30u8, 135u8, 63u8, 46u8, 40u8, 137u8,
							209u8, 37u8, 24u8, 108u8, 251u8, 189u8, 77u8, 208u8, 74u8, 32u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn epoch_index(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"EpochIndex",
						vec![],
						[
							51u8, 27u8, 91u8, 156u8, 118u8, 99u8, 46u8, 219u8, 190u8, 147u8, 205u8,
							23u8, 106u8, 169u8, 121u8, 218u8, 208u8, 235u8, 135u8, 127u8, 243u8,
							41u8, 55u8, 243u8, 235u8, 122u8, 57u8, 86u8, 37u8, 90u8, 208u8, 71u8,
						],
					)
				}
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
						runtime_types::sp_consensus_babe::app::Public,
						::core::primitive::u64,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"Authorities",
						vec![],
						[
							61u8, 8u8, 133u8, 111u8, 169u8, 120u8, 0u8, 213u8, 31u8, 159u8, 204u8,
							212u8, 18u8, 205u8, 93u8, 84u8, 140u8, 108u8, 136u8, 209u8, 234u8,
							107u8, 145u8, 9u8, 204u8, 224u8, 105u8, 9u8, 238u8, 241u8, 65u8, 30u8,
						],
					)
				}
				pub fn genesis_slot(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_slots::Slot,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"GenesisSlot",
						vec![],
						[
							234u8, 127u8, 243u8, 100u8, 124u8, 160u8, 66u8, 248u8, 48u8, 218u8,
							61u8, 52u8, 54u8, 142u8, 158u8, 77u8, 32u8, 63u8, 156u8, 39u8, 94u8,
							255u8, 192u8, 238u8, 170u8, 118u8, 58u8, 42u8, 199u8, 61u8, 199u8,
							77u8,
						],
					)
				}
				pub fn current_slot(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_slots::Slot,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"CurrentSlot",
						vec![],
						[
							139u8, 237u8, 185u8, 137u8, 251u8, 179u8, 69u8, 167u8, 133u8, 168u8,
							204u8, 64u8, 178u8, 123u8, 92u8, 250u8, 119u8, 190u8, 208u8, 178u8,
							208u8, 176u8, 124u8, 187u8, 74u8, 165u8, 33u8, 78u8, 161u8, 206u8, 8u8,
							108u8,
						],
					)
				}
				pub fn randomness(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					[::core::primitive::u8; 32usize],
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"Randomness",
						vec![],
						[
							191u8, 197u8, 25u8, 164u8, 104u8, 248u8, 247u8, 193u8, 244u8, 60u8,
							181u8, 195u8, 248u8, 90u8, 41u8, 199u8, 82u8, 123u8, 72u8, 126u8, 18u8,
							17u8, 128u8, 215u8, 34u8, 251u8, 227u8, 70u8, 166u8, 10u8, 104u8,
							140u8,
						],
					)
				}
				pub fn pending_epoch_config_change(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"PendingEpochConfigChange",
						vec![],
						[
							4u8, 201u8, 0u8, 204u8, 47u8, 246u8, 4u8, 185u8, 163u8, 242u8, 242u8,
							152u8, 29u8, 222u8, 71u8, 127u8, 49u8, 203u8, 206u8, 180u8, 244u8,
							50u8, 80u8, 49u8, 199u8, 97u8, 3u8, 170u8, 156u8, 139u8, 106u8, 113u8,
						],
					)
				}
				pub fn next_randomness(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					[::core::primitive::u8; 32usize],
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"NextRandomness",
						vec![],
						[
							185u8, 98u8, 45u8, 109u8, 253u8, 38u8, 238u8, 221u8, 240u8, 29u8, 38u8,
							107u8, 118u8, 117u8, 131u8, 115u8, 21u8, 255u8, 203u8, 81u8, 243u8,
							251u8, 91u8, 60u8, 163u8, 202u8, 125u8, 193u8, 173u8, 234u8, 166u8,
							92u8,
						],
					)
				}
				pub fn next_authorities(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
						runtime_types::sp_consensus_babe::app::Public,
						::core::primitive::u64,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"NextAuthorities",
						vec![],
						[
							201u8, 193u8, 164u8, 18u8, 155u8, 253u8, 124u8, 163u8, 143u8, 73u8,
							212u8, 20u8, 241u8, 108u8, 110u8, 5u8, 171u8, 66u8, 224u8, 208u8, 10u8,
							65u8, 148u8, 164u8, 1u8, 12u8, 216u8, 83u8, 20u8, 226u8, 254u8, 183u8,
						],
					)
				}
				pub fn segment_index(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"SegmentIndex",
						vec![],
						[
							128u8, 45u8, 87u8, 58u8, 174u8, 152u8, 241u8, 156u8, 56u8, 192u8, 19u8,
							45u8, 75u8, 160u8, 35u8, 253u8, 145u8, 11u8, 178u8, 81u8, 114u8, 117u8,
							112u8, 107u8, 163u8, 208u8, 240u8, 151u8, 102u8, 176u8, 246u8, 5u8,
						],
					)
				}
				pub fn under_construction(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						[::core::primitive::u8; 32usize],
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"UnderConstruction",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							180u8, 4u8, 149u8, 245u8, 231u8, 92u8, 99u8, 170u8, 254u8, 172u8,
							182u8, 3u8, 152u8, 156u8, 132u8, 196u8, 140u8, 97u8, 7u8, 84u8, 220u8,
							89u8, 195u8, 177u8, 235u8, 51u8, 98u8, 144u8, 73u8, 238u8, 59u8, 164u8,
						],
					)
				}
				pub fn under_construction_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						[::core::primitive::u8; 32usize],
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"UnderConstruction",
						Vec::new(),
						[
							180u8, 4u8, 149u8, 245u8, 231u8, 92u8, 99u8, 170u8, 254u8, 172u8,
							182u8, 3u8, 152u8, 156u8, 132u8, 196u8, 140u8, 97u8, 7u8, 84u8, 220u8,
							89u8, 195u8, 177u8, 235u8, 51u8, 98u8, 144u8, 73u8, 238u8, 59u8, 164u8,
						],
					)
				}
				pub fn initialized(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::option::Option<runtime_types::sp_consensus_babe::digests::PreDigest>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"Initialized",
						vec![],
						[
							40u8, 135u8, 28u8, 144u8, 247u8, 208u8, 48u8, 220u8, 46u8, 60u8, 131u8,
							190u8, 196u8, 235u8, 126u8, 66u8, 34u8, 14u8, 32u8, 131u8, 71u8, 46u8,
							62u8, 207u8, 177u8, 213u8, 167u8, 34u8, 199u8, 29u8, 16u8, 236u8,
						],
					)
				}
				pub fn author_vrf_randomness(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::option::Option<[::core::primitive::u8; 32usize]>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"AuthorVrfRandomness",
						vec![],
						[
							66u8, 235u8, 74u8, 252u8, 222u8, 135u8, 19u8, 28u8, 74u8, 191u8, 170u8,
							197u8, 207u8, 127u8, 77u8, 121u8, 138u8, 138u8, 110u8, 187u8, 34u8,
							14u8, 230u8, 43u8, 241u8, 241u8, 63u8, 163u8, 53u8, 179u8, 250u8,
							247u8,
						],
					)
				}
				pub fn epoch_start(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::core::primitive::u32),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"EpochStart",
						vec![],
						[
							196u8, 39u8, 241u8, 20u8, 150u8, 180u8, 136u8, 4u8, 195u8, 205u8,
							218u8, 10u8, 130u8, 131u8, 168u8, 243u8, 207u8, 249u8, 58u8, 195u8,
							177u8, 119u8, 110u8, 243u8, 241u8, 3u8, 245u8, 56u8, 157u8, 5u8, 68u8,
							60u8,
						],
					)
				}
				pub fn lateness(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"Lateness",
						vec![],
						[
							229u8, 230u8, 224u8, 89u8, 49u8, 213u8, 198u8, 236u8, 144u8, 56u8,
							193u8, 234u8, 62u8, 242u8, 191u8, 199u8, 105u8, 131u8, 74u8, 63u8,
							75u8, 1u8, 210u8, 49u8, 3u8, 128u8, 18u8, 77u8, 219u8, 146u8, 60u8,
							88u8,
						],
					)
				}
				pub fn epoch_config(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_babe::BabeEpochConfiguration,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"EpochConfig",
						vec![],
						[
							41u8, 118u8, 141u8, 244u8, 72u8, 17u8, 125u8, 203u8, 43u8, 153u8,
							203u8, 119u8, 117u8, 223u8, 123u8, 133u8, 73u8, 235u8, 130u8, 21u8,
							160u8, 167u8, 16u8, 173u8, 177u8, 35u8, 117u8, 97u8, 149u8, 49u8,
							220u8, 24u8,
						],
					)
				}
				pub fn next_epoch_config(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_babe::BabeEpochConfiguration,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"NextEpochConfig",
						vec![],
						[
							111u8, 182u8, 144u8, 180u8, 92u8, 146u8, 102u8, 249u8, 196u8, 229u8,
							226u8, 30u8, 25u8, 198u8, 133u8, 9u8, 136u8, 95u8, 11u8, 151u8, 139u8,
							156u8, 105u8, 228u8, 181u8, 12u8, 175u8, 148u8, 174u8, 33u8, 233u8,
							228u8,
						],
					)
				}
				pub fn skipped_epochs(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u64,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"SkippedEpochs",
						vec![],
						[
							187u8, 66u8, 178u8, 110u8, 247u8, 41u8, 128u8, 194u8, 173u8, 197u8,
							28u8, 219u8, 112u8, 75u8, 9u8, 184u8, 51u8, 12u8, 121u8, 117u8, 176u8,
							213u8, 139u8, 144u8, 122u8, 72u8, 243u8, 105u8, 248u8, 63u8, 6u8, 87u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn epoch_duration(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Babe",
						"EpochDuration",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				pub fn expected_block_time(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Babe",
						"ExpectedBlockTime",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				pub fn max_authorities(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Babe",
						"MaxAuthorities",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod timestamp {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Set {
				#[codec(compact)]
				pub now: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set(&self, now: ::core::primitive::u64) -> ::subxt::tx::Payload<Set> {
					::subxt::tx::Payload::new_static(
						"Timestamp",
						"set",
						Set { now },
						[
							6u8, 97u8, 172u8, 236u8, 118u8, 238u8, 228u8, 114u8, 15u8, 115u8,
							102u8, 85u8, 66u8, 151u8, 16u8, 33u8, 187u8, 17u8, 166u8, 88u8, 127u8,
							214u8, 182u8, 51u8, 168u8, 88u8, 43u8, 101u8, 185u8, 8u8, 1u8, 28u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn now(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Timestamp",
						"Now",
						vec![],
						[
							148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8, 144u8,
							221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8, 202u8,
							185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8, 250u8,
						],
					)
				}
				pub fn did_update(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Timestamp",
						"DidUpdate",
						vec![],
						[
							70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8, 175u8,
							13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8, 167u8, 85u8,
							27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn minimum_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Timestamp",
						"MinimumPeriod",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod grandpa {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ReportEquivocation {
				pub equivocation_proof: ::std::boxed::Box<
					runtime_types::sp_consensus_grandpa::EquivocationProof<
						::subxt::utils::H256,
						::core::primitive::u32,
					>,
				>,
				pub key_owner_proof: runtime_types::sp_session::MembershipProof,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ReportEquivocationUnsigned {
				pub equivocation_proof: ::std::boxed::Box<
					runtime_types::sp_consensus_grandpa::EquivocationProof<
						::subxt::utils::H256,
						::core::primitive::u32,
					>,
				>,
				pub key_owner_proof: runtime_types::sp_session::MembershipProof,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NoteStalled {
				pub delay: ::core::primitive::u32,
				pub best_finalized_block_number: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn report_equivocation(
					&self,
					equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
						::subxt::utils::H256,
						::core::primitive::u32,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::Payload<ReportEquivocation> {
					::subxt::tx::Payload::new_static(
						"Grandpa",
						"report_equivocation",
						ReportEquivocation {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							156u8, 162u8, 189u8, 89u8, 60u8, 156u8, 129u8, 176u8, 62u8, 35u8,
							214u8, 7u8, 68u8, 245u8, 130u8, 117u8, 30u8, 3u8, 73u8, 218u8, 142u8,
							82u8, 13u8, 141u8, 124u8, 19u8, 53u8, 138u8, 70u8, 4u8, 40u8, 32u8,
						],
					)
				}
				pub fn report_equivocation_unsigned(
					&self,
					equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
						::subxt::utils::H256,
						::core::primitive::u32,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::Payload<ReportEquivocationUnsigned> {
					::subxt::tx::Payload::new_static(
						"Grandpa",
						"report_equivocation_unsigned",
						ReportEquivocationUnsigned {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							166u8, 26u8, 217u8, 185u8, 215u8, 37u8, 174u8, 170u8, 137u8, 160u8,
							151u8, 43u8, 246u8, 86u8, 58u8, 18u8, 248u8, 73u8, 99u8, 161u8, 158u8,
							93u8, 212u8, 186u8, 224u8, 253u8, 234u8, 18u8, 151u8, 111u8, 227u8,
							249u8,
						],
					)
				}
				pub fn note_stalled(
					&self,
					delay: ::core::primitive::u32,
					best_finalized_block_number: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<NoteStalled> {
					::subxt::tx::Payload::new_static(
						"Grandpa",
						"note_stalled",
						NoteStalled { delay, best_finalized_block_number },
						[
							197u8, 236u8, 137u8, 32u8, 46u8, 200u8, 144u8, 13u8, 89u8, 181u8,
							235u8, 73u8, 167u8, 131u8, 174u8, 93u8, 42u8, 136u8, 238u8, 59u8,
							129u8, 60u8, 83u8, 100u8, 5u8, 182u8, 99u8, 250u8, 145u8, 180u8, 1u8,
							199u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_grandpa::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NewAuthorities {
				pub authority_set: ::std::vec::Vec<(
					runtime_types::sp_consensus_grandpa::app::Public,
					::core::primitive::u64,
				)>,
			}
			impl ::subxt::events::StaticEvent for NewAuthorities {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "NewAuthorities";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Paused;
			impl ::subxt::events::StaticEvent for Paused {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "Paused";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Resumed;
			impl ::subxt::events::StaticEvent for Resumed {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "Resumed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn state(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"State",
						vec![],
						[
							211u8, 149u8, 114u8, 217u8, 206u8, 194u8, 115u8, 67u8, 12u8, 218u8,
							246u8, 213u8, 208u8, 29u8, 216u8, 104u8, 2u8, 39u8, 123u8, 172u8,
							252u8, 210u8, 52u8, 129u8, 147u8, 237u8, 244u8, 68u8, 252u8, 169u8,
							97u8, 148u8,
						],
					)
				}
				pub fn pending_change(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"PendingChange",
						vec![],
						[
							178u8, 24u8, 140u8, 7u8, 8u8, 196u8, 18u8, 58u8, 3u8, 226u8, 181u8,
							47u8, 155u8, 160u8, 70u8, 12u8, 75u8, 189u8, 38u8, 255u8, 104u8, 141u8,
							64u8, 34u8, 134u8, 201u8, 102u8, 21u8, 75u8, 81u8, 218u8, 60u8,
						],
					)
				}
				pub fn next_forced(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"NextForced",
						vec![],
						[
							99u8, 43u8, 245u8, 201u8, 60u8, 9u8, 122u8, 99u8, 188u8, 29u8, 67u8,
							6u8, 193u8, 133u8, 179u8, 67u8, 202u8, 208u8, 62u8, 179u8, 19u8, 169u8,
							196u8, 119u8, 107u8, 75u8, 100u8, 3u8, 121u8, 18u8, 80u8, 156u8,
						],
					)
				}
				pub fn stalled(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::core::primitive::u32),
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"Stalled",
						vec![],
						[
							219u8, 8u8, 37u8, 78u8, 150u8, 55u8, 0u8, 57u8, 201u8, 170u8, 186u8,
							189u8, 56u8, 161u8, 44u8, 15u8, 53u8, 178u8, 224u8, 208u8, 231u8,
							109u8, 14u8, 209u8, 57u8, 205u8, 237u8, 153u8, 231u8, 156u8, 24u8,
							185u8,
						],
					)
				}
				pub fn current_set_id(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"CurrentSetId",
						vec![],
						[
							129u8, 7u8, 62u8, 101u8, 199u8, 60u8, 56u8, 33u8, 54u8, 158u8, 20u8,
							178u8, 244u8, 145u8, 189u8, 197u8, 157u8, 163u8, 116u8, 36u8, 105u8,
							52u8, 149u8, 244u8, 108u8, 94u8, 109u8, 111u8, 244u8, 137u8, 7u8,
							108u8,
						],
					)
				}
				pub fn set_id_session(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"SetIdSession",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
							166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
							108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
							134u8,
						],
					)
				}
				pub fn set_id_session_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"SetIdSession",
						Vec::new(),
						[
							91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
							166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
							108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
							134u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn max_authorities(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Grandpa",
						"MaxAuthorities",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_set_id_session_entries(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Grandpa",
						"MaxSetIdSessionEntries",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod paras {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceSetCurrentCode {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceSetCurrentHead {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub new_head: runtime_types::polkadot_parachain::primitives::HeadData,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceScheduleCodeUpgrade {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
				pub relay_parent_number: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceNoteNewHead {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub new_head: runtime_types::polkadot_parachain::primitives::HeadData,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceQueueAction {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AddTrustedValidationCode {
				pub validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PokeUnusedValidationCode {
				pub validation_code_hash:
					runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct IncludePvfCheckStatement {
				pub stmt: runtime_types::polkadot_primitives::v4::PvfCheckStatement,
				pub signature: runtime_types::polkadot_primitives::v4::validator_app::Signature,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn force_set_current_code(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
				) -> ::subxt::tx::Payload<ForceSetCurrentCode> {
					::subxt::tx::Payload::new_static(
						"Paras",
						"force_set_current_code",
						ForceSetCurrentCode { para, new_code },
						[
							56u8, 59u8, 48u8, 185u8, 106u8, 99u8, 250u8, 32u8, 207u8, 2u8, 4u8,
							110u8, 165u8, 131u8, 22u8, 33u8, 248u8, 175u8, 186u8, 6u8, 118u8, 51u8,
							74u8, 239u8, 68u8, 122u8, 148u8, 242u8, 193u8, 131u8, 6u8, 135u8,
						],
					)
				}
				pub fn force_set_current_head(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					new_head: runtime_types::polkadot_parachain::primitives::HeadData,
				) -> ::subxt::tx::Payload<ForceSetCurrentHead> {
					::subxt::tx::Payload::new_static(
						"Paras",
						"force_set_current_head",
						ForceSetCurrentHead { para, new_head },
						[
							203u8, 70u8, 33u8, 168u8, 133u8, 64u8, 146u8, 137u8, 156u8, 104u8,
							183u8, 26u8, 74u8, 227u8, 154u8, 224u8, 75u8, 85u8, 143u8, 51u8, 60u8,
							194u8, 59u8, 94u8, 100u8, 84u8, 194u8, 100u8, 153u8, 9u8, 222u8, 63u8,
						],
					)
				}
				pub fn force_schedule_code_upgrade(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
					relay_parent_number: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<ForceScheduleCodeUpgrade> {
					::subxt::tx::Payload::new_static(
						"Paras",
						"force_schedule_code_upgrade",
						ForceScheduleCodeUpgrade { para, new_code, relay_parent_number },
						[
							30u8, 210u8, 178u8, 31u8, 48u8, 144u8, 167u8, 117u8, 220u8, 36u8,
							175u8, 220u8, 145u8, 193u8, 20u8, 98u8, 149u8, 130u8, 66u8, 54u8, 20u8,
							204u8, 231u8, 116u8, 203u8, 179u8, 253u8, 106u8, 55u8, 58u8, 116u8,
							109u8,
						],
					)
				}
				pub fn force_note_new_head(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					new_head: runtime_types::polkadot_parachain::primitives::HeadData,
				) -> ::subxt::tx::Payload<ForceNoteNewHead> {
					::subxt::tx::Payload::new_static(
						"Paras",
						"force_note_new_head",
						ForceNoteNewHead { para, new_head },
						[
							83u8, 93u8, 166u8, 142u8, 213u8, 1u8, 243u8, 73u8, 192u8, 164u8, 104u8,
							206u8, 99u8, 250u8, 31u8, 222u8, 231u8, 54u8, 12u8, 45u8, 92u8, 74u8,
							248u8, 50u8, 180u8, 86u8, 251u8, 172u8, 227u8, 88u8, 45u8, 127u8,
						],
					)
				}
				pub fn force_queue_action(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::Payload<ForceQueueAction> {
					::subxt::tx::Payload::new_static(
						"Paras",
						"force_queue_action",
						ForceQueueAction { para },
						[
							195u8, 243u8, 79u8, 34u8, 111u8, 246u8, 109u8, 90u8, 251u8, 137u8,
							48u8, 23u8, 117u8, 29u8, 26u8, 200u8, 37u8, 64u8, 36u8, 254u8, 224u8,
							99u8, 165u8, 246u8, 8u8, 76u8, 250u8, 36u8, 141u8, 67u8, 185u8, 17u8,
						],
					)
				}
				pub fn add_trusted_validation_code(
					&self,
					validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
				) -> ::subxt::tx::Payload<AddTrustedValidationCode> {
					::subxt::tx::Payload::new_static(
						"Paras",
						"add_trusted_validation_code",
						AddTrustedValidationCode { validation_code },
						[
							160u8, 199u8, 245u8, 178u8, 58u8, 65u8, 79u8, 199u8, 53u8, 60u8, 84u8,
							225u8, 2u8, 145u8, 154u8, 204u8, 165u8, 171u8, 173u8, 223u8, 59u8,
							196u8, 37u8, 12u8, 243u8, 158u8, 77u8, 184u8, 58u8, 64u8, 133u8, 71u8,
						],
					)
				}
				pub fn poke_unused_validation_code(
					&self,
					validation_code_hash : runtime_types :: polkadot_parachain :: primitives :: ValidationCodeHash,
				) -> ::subxt::tx::Payload<PokeUnusedValidationCode> {
					::subxt::tx::Payload::new_static(
						"Paras",
						"poke_unused_validation_code",
						PokeUnusedValidationCode { validation_code_hash },
						[
							98u8, 9u8, 24u8, 180u8, 8u8, 144u8, 36u8, 28u8, 111u8, 83u8, 162u8,
							160u8, 66u8, 119u8, 177u8, 117u8, 143u8, 233u8, 241u8, 128u8, 189u8,
							118u8, 241u8, 30u8, 74u8, 171u8, 193u8, 177u8, 233u8, 12u8, 254u8,
							146u8,
						],
					)
				}
				pub fn include_pvf_check_statement(
					&self,
					stmt: runtime_types::polkadot_primitives::v4::PvfCheckStatement,
					signature: runtime_types::polkadot_primitives::v4::validator_app::Signature,
				) -> ::subxt::tx::Payload<IncludePvfCheckStatement> {
					::subxt::tx::Payload::new_static(
						"Paras",
						"include_pvf_check_statement",
						IncludePvfCheckStatement { stmt, signature },
						[
							22u8, 136u8, 241u8, 59u8, 36u8, 249u8, 239u8, 255u8, 169u8, 117u8,
							19u8, 58u8, 214u8, 16u8, 135u8, 65u8, 13u8, 250u8, 5u8, 41u8, 144u8,
							29u8, 207u8, 73u8, 215u8, 221u8, 1u8, 253u8, 123u8, 110u8, 6u8, 196u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::polkadot_runtime_parachains::paras::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CurrentCodeUpdated(pub runtime_types::polkadot_parachain::primitives::Id);
			impl ::subxt::events::StaticEvent for CurrentCodeUpdated {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "CurrentCodeUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CurrentHeadUpdated(pub runtime_types::polkadot_parachain::primitives::Id);
			impl ::subxt::events::StaticEvent for CurrentHeadUpdated {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "CurrentHeadUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CodeUpgradeScheduled(pub runtime_types::polkadot_parachain::primitives::Id);
			impl ::subxt::events::StaticEvent for CodeUpgradeScheduled {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "CodeUpgradeScheduled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NewHeadNoted(pub runtime_types::polkadot_parachain::primitives::Id);
			impl ::subxt::events::StaticEvent for NewHeadNoted {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "NewHeadNoted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ActionQueued(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for ActionQueued {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "ActionQueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PvfCheckStarted(
				pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
				pub runtime_types::polkadot_parachain::primitives::Id,
			);
			impl ::subxt::events::StaticEvent for PvfCheckStarted {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "PvfCheckStarted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PvfCheckAccepted(
				pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
				pub runtime_types::polkadot_parachain::primitives::Id,
			);
			impl ::subxt::events::StaticEvent for PvfCheckAccepted {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "PvfCheckAccepted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PvfCheckRejected(
				pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
				pub runtime_types::polkadot_parachain::primitives::Id,
			);
			impl ::subxt::events::StaticEvent for PvfCheckRejected {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "PvfCheckRejected";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn pvf_active_vote_map(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_runtime_parachains::paras::PvfCheckActiveVoteState<
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"PvfActiveVoteMap",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							84u8, 214u8, 221u8, 221u8, 244u8, 56u8, 135u8, 87u8, 252u8, 39u8,
							188u8, 13u8, 196u8, 25u8, 214u8, 186u8, 152u8, 181u8, 190u8, 39u8,
							235u8, 211u8, 236u8, 114u8, 67u8, 85u8, 138u8, 43u8, 248u8, 134u8,
							124u8, 73u8,
						],
					)
				}
				pub fn pvf_active_vote_map_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_runtime_parachains::paras::PvfCheckActiveVoteState<
						::core::primitive::u32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"PvfActiveVoteMap",
						Vec::new(),
						[
							84u8, 214u8, 221u8, 221u8, 244u8, 56u8, 135u8, 87u8, 252u8, 39u8,
							188u8, 13u8, 196u8, 25u8, 214u8, 186u8, 152u8, 181u8, 190u8, 39u8,
							235u8, 211u8, 236u8, 114u8, 67u8, 85u8, 138u8, 43u8, 248u8, 134u8,
							124u8, 73u8,
						],
					)
				}
				pub fn pvf_active_vote_list(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"PvfActiveVoteList",
						vec![],
						[
							196u8, 23u8, 108u8, 162u8, 29u8, 33u8, 49u8, 219u8, 127u8, 26u8, 241u8,
							58u8, 102u8, 43u8, 156u8, 3u8, 87u8, 153u8, 195u8, 96u8, 68u8, 132u8,
							170u8, 162u8, 18u8, 156u8, 121u8, 63u8, 53u8, 91u8, 68u8, 69u8,
						],
					)
				}
				pub fn parachains(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"Parachains",
						vec![],
						[
							85u8, 234u8, 218u8, 69u8, 20u8, 169u8, 235u8, 6u8, 69u8, 126u8, 28u8,
							18u8, 57u8, 93u8, 238u8, 7u8, 167u8, 221u8, 75u8, 35u8, 36u8, 4u8,
							46u8, 55u8, 234u8, 123u8, 122u8, 173u8, 13u8, 205u8, 58u8, 226u8,
						],
					)
				}
				pub fn para_lifecycles(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"ParaLifecycles",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							221u8, 103u8, 112u8, 222u8, 86u8, 2u8, 172u8, 187u8, 174u8, 106u8, 4u8,
							253u8, 35u8, 73u8, 18u8, 78u8, 25u8, 31u8, 124u8, 110u8, 81u8, 62u8,
							215u8, 228u8, 183u8, 132u8, 138u8, 213u8, 186u8, 209u8, 191u8, 186u8,
						],
					)
				}
				pub fn para_lifecycles_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"ParaLifecycles",
						Vec::new(),
						[
							221u8, 103u8, 112u8, 222u8, 86u8, 2u8, 172u8, 187u8, 174u8, 106u8, 4u8,
							253u8, 35u8, 73u8, 18u8, 78u8, 25u8, 31u8, 124u8, 110u8, 81u8, 62u8,
							215u8, 228u8, 183u8, 132u8, 138u8, 213u8, 186u8, 209u8, 191u8, 186u8,
						],
					)
				}
				pub fn heads(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::HeadData,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"Heads",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							122u8, 38u8, 181u8, 121u8, 245u8, 100u8, 136u8, 233u8, 237u8, 248u8,
							127u8, 2u8, 147u8, 41u8, 202u8, 242u8, 238u8, 70u8, 55u8, 200u8, 15u8,
							106u8, 138u8, 108u8, 192u8, 61u8, 158u8, 134u8, 131u8, 142u8, 70u8,
							3u8,
						],
					)
				}
				pub fn heads_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::HeadData,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"Heads",
						Vec::new(),
						[
							122u8, 38u8, 181u8, 121u8, 245u8, 100u8, 136u8, 233u8, 237u8, 248u8,
							127u8, 2u8, 147u8, 41u8, 202u8, 242u8, 238u8, 70u8, 55u8, 200u8, 15u8,
							106u8, 138u8, 108u8, 192u8, 61u8, 158u8, 134u8, 131u8, 142u8, 70u8,
							3u8,
						],
					)
				}
				pub fn current_code_hash(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"CurrentCodeHash",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							179u8, 145u8, 45u8, 44u8, 130u8, 240u8, 50u8, 128u8, 190u8, 133u8,
							66u8, 85u8, 47u8, 141u8, 56u8, 87u8, 131u8, 99u8, 170u8, 203u8, 8u8,
							51u8, 123u8, 73u8, 206u8, 30u8, 173u8, 35u8, 157u8, 195u8, 104u8,
							236u8,
						],
					)
				}
				pub fn current_code_hash_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"CurrentCodeHash",
						Vec::new(),
						[
							179u8, 145u8, 45u8, 44u8, 130u8, 240u8, 50u8, 128u8, 190u8, 133u8,
							66u8, 85u8, 47u8, 141u8, 56u8, 87u8, 131u8, 99u8, 170u8, 203u8, 8u8,
							51u8, 123u8, 73u8, 206u8, 30u8, 173u8, 35u8, 157u8, 195u8, 104u8,
							236u8,
						],
					)
				}
				pub fn past_code_hash(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"PastCodeHash",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							241u8, 112u8, 128u8, 226u8, 163u8, 193u8, 77u8, 78u8, 177u8, 146u8,
							31u8, 143u8, 44u8, 140u8, 159u8, 134u8, 221u8, 129u8, 36u8, 224u8,
							46u8, 119u8, 245u8, 253u8, 55u8, 22u8, 137u8, 187u8, 71u8, 94u8, 88u8,
							124u8,
						],
					)
				}
				pub fn past_code_hash_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"PastCodeHash",
						Vec::new(),
						[
							241u8, 112u8, 128u8, 226u8, 163u8, 193u8, 77u8, 78u8, 177u8, 146u8,
							31u8, 143u8, 44u8, 140u8, 159u8, 134u8, 221u8, 129u8, 36u8, 224u8,
							46u8, 119u8, 245u8, 253u8, 55u8, 22u8, 137u8, 187u8, 71u8, 94u8, 88u8,
							124u8,
						],
					)
				}
				pub fn past_code_meta(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_runtime_parachains::paras::ParaPastCodeMeta<
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"PastCodeMeta",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							251u8, 52u8, 126u8, 12u8, 21u8, 178u8, 151u8, 195u8, 153u8, 17u8,
							215u8, 242u8, 118u8, 192u8, 86u8, 72u8, 36u8, 97u8, 245u8, 134u8,
							155u8, 117u8, 85u8, 93u8, 225u8, 209u8, 63u8, 164u8, 168u8, 72u8,
							171u8, 228u8,
						],
					)
				}
				pub fn past_code_meta_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_runtime_parachains::paras::ParaPastCodeMeta<
						::core::primitive::u32,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"PastCodeMeta",
						Vec::new(),
						[
							251u8, 52u8, 126u8, 12u8, 21u8, 178u8, 151u8, 195u8, 153u8, 17u8,
							215u8, 242u8, 118u8, 192u8, 86u8, 72u8, 36u8, 97u8, 245u8, 134u8,
							155u8, 117u8, 85u8, 93u8, 225u8, 209u8, 63u8, 164u8, 168u8, 72u8,
							171u8, 228u8,
						],
					)
				}
				pub fn past_code_pruning(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"PastCodePruning",
						vec![],
						[
							59u8, 26u8, 175u8, 129u8, 174u8, 27u8, 239u8, 77u8, 38u8, 130u8, 37u8,
							134u8, 62u8, 28u8, 218u8, 176u8, 16u8, 137u8, 175u8, 90u8, 248u8, 44u8,
							248u8, 172u8, 231u8, 6u8, 36u8, 190u8, 109u8, 237u8, 228u8, 135u8,
						],
					)
				}
				pub fn future_code_upgrades(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"FutureCodeUpgrades",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							40u8, 134u8, 185u8, 28u8, 48u8, 105u8, 152u8, 229u8, 166u8, 235u8,
							32u8, 173u8, 64u8, 63u8, 151u8, 157u8, 190u8, 214u8, 7u8, 8u8, 6u8,
							128u8, 21u8, 104u8, 175u8, 71u8, 130u8, 207u8, 158u8, 115u8, 172u8,
							149u8,
						],
					)
				}
				pub fn future_code_upgrades_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"FutureCodeUpgrades",
						Vec::new(),
						[
							40u8, 134u8, 185u8, 28u8, 48u8, 105u8, 152u8, 229u8, 166u8, 235u8,
							32u8, 173u8, 64u8, 63u8, 151u8, 157u8, 190u8, 214u8, 7u8, 8u8, 6u8,
							128u8, 21u8, 104u8, 175u8, 71u8, 130u8, 207u8, 158u8, 115u8, 172u8,
							149u8,
						],
					)
				}
				pub fn future_code_hash(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"FutureCodeHash",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							252u8, 24u8, 95u8, 200u8, 207u8, 91u8, 66u8, 103u8, 54u8, 171u8, 191u8,
							187u8, 73u8, 170u8, 132u8, 59u8, 205u8, 125u8, 68u8, 194u8, 122u8,
							124u8, 190u8, 53u8, 241u8, 225u8, 131u8, 53u8, 44u8, 145u8, 244u8,
							216u8,
						],
					)
				}
				pub fn future_code_hash_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"FutureCodeHash",
						Vec::new(),
						[
							252u8, 24u8, 95u8, 200u8, 207u8, 91u8, 66u8, 103u8, 54u8, 171u8, 191u8,
							187u8, 73u8, 170u8, 132u8, 59u8, 205u8, 125u8, 68u8, 194u8, 122u8,
							124u8, 190u8, 53u8, 241u8, 225u8, 131u8, 53u8, 44u8, 145u8, 244u8,
							216u8,
						],
					)
				}
				pub fn upgrade_go_ahead_signal(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_primitives::v4::UpgradeGoAhead,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"UpgradeGoAheadSignal",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							159u8, 47u8, 247u8, 154u8, 54u8, 20u8, 235u8, 49u8, 74u8, 41u8, 65u8,
							51u8, 52u8, 187u8, 242u8, 6u8, 84u8, 144u8, 200u8, 176u8, 222u8, 232u8,
							70u8, 50u8, 70u8, 97u8, 61u8, 249u8, 245u8, 120u8, 98u8, 183u8,
						],
					)
				}
				pub fn upgrade_go_ahead_signal_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_primitives::v4::UpgradeGoAhead,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"UpgradeGoAheadSignal",
						Vec::new(),
						[
							159u8, 47u8, 247u8, 154u8, 54u8, 20u8, 235u8, 49u8, 74u8, 41u8, 65u8,
							51u8, 52u8, 187u8, 242u8, 6u8, 84u8, 144u8, 200u8, 176u8, 222u8, 232u8,
							70u8, 50u8, 70u8, 97u8, 61u8, 249u8, 245u8, 120u8, 98u8, 183u8,
						],
					)
				}
				pub fn upgrade_restriction_signal(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_primitives::v4::UpgradeRestriction,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"UpgradeRestrictionSignal",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							86u8, 190u8, 41u8, 79u8, 66u8, 68u8, 46u8, 87u8, 212u8, 176u8, 255u8,
							134u8, 104u8, 121u8, 44u8, 143u8, 248u8, 100u8, 35u8, 157u8, 91u8,
							165u8, 118u8, 38u8, 49u8, 171u8, 158u8, 163u8, 45u8, 92u8, 44u8, 11u8,
						],
					)
				}
				pub fn upgrade_restriction_signal_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_primitives::v4::UpgradeRestriction,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"UpgradeRestrictionSignal",
						Vec::new(),
						[
							86u8, 190u8, 41u8, 79u8, 66u8, 68u8, 46u8, 87u8, 212u8, 176u8, 255u8,
							134u8, 104u8, 121u8, 44u8, 143u8, 248u8, 100u8, 35u8, 157u8, 91u8,
							165u8, 118u8, 38u8, 49u8, 171u8, 158u8, 163u8, 45u8, 92u8, 44u8, 11u8,
						],
					)
				}
				pub fn upgrade_cooldowns(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"UpgradeCooldowns",
						vec![],
						[
							205u8, 236u8, 140u8, 145u8, 241u8, 245u8, 60u8, 68u8, 23u8, 175u8,
							226u8, 174u8, 154u8, 107u8, 243u8, 197u8, 61u8, 218u8, 7u8, 24u8,
							109u8, 221u8, 178u8, 80u8, 242u8, 123u8, 33u8, 119u8, 5u8, 241u8,
							179u8, 13u8,
						],
					)
				}
				pub fn upcoming_upgrades(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"UpcomingUpgrades",
						vec![],
						[
							165u8, 112u8, 215u8, 149u8, 125u8, 175u8, 206u8, 195u8, 214u8, 173u8,
							0u8, 144u8, 46u8, 197u8, 55u8, 204u8, 144u8, 68u8, 158u8, 156u8, 145u8,
							230u8, 173u8, 101u8, 255u8, 108u8, 52u8, 199u8, 142u8, 37u8, 55u8,
							32u8,
						],
					)
				}
				pub fn actions_queue(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"ActionsQueue",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							209u8, 106u8, 198u8, 228u8, 148u8, 75u8, 246u8, 248u8, 12u8, 143u8,
							175u8, 56u8, 168u8, 243u8, 67u8, 166u8, 59u8, 92u8, 219u8, 184u8, 1u8,
							34u8, 241u8, 66u8, 245u8, 48u8, 174u8, 41u8, 123u8, 16u8, 178u8, 161u8,
						],
					)
				}
				pub fn actions_queue_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"ActionsQueue",
						Vec::new(),
						[
							209u8, 106u8, 198u8, 228u8, 148u8, 75u8, 246u8, 248u8, 12u8, 143u8,
							175u8, 56u8, 168u8, 243u8, 67u8, 166u8, 59u8, 92u8, 219u8, 184u8, 1u8,
							34u8, 241u8, 66u8, 245u8, 48u8, 174u8, 41u8, 123u8, 16u8, 178u8, 161u8,
						],
					)
				}
				pub fn upcoming_paras_genesis(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"UpcomingParasGenesis",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							134u8, 111u8, 59u8, 49u8, 28u8, 111u8, 6u8, 57u8, 109u8, 75u8, 75u8,
							53u8, 91u8, 150u8, 86u8, 38u8, 223u8, 50u8, 107u8, 75u8, 200u8, 61u8,
							211u8, 7u8, 105u8, 251u8, 243u8, 18u8, 220u8, 195u8, 216u8, 167u8,
						],
					)
				}
				pub fn upcoming_paras_genesis_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"UpcomingParasGenesis",
						Vec::new(),
						[
							134u8, 111u8, 59u8, 49u8, 28u8, 111u8, 6u8, 57u8, 109u8, 75u8, 75u8,
							53u8, 91u8, 150u8, 86u8, 38u8, 223u8, 50u8, 107u8, 75u8, 200u8, 61u8,
							211u8, 7u8, 105u8, 251u8, 243u8, 18u8, 220u8, 195u8, 216u8, 167u8,
						],
					)
				}
				pub fn code_by_hash_refs(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"CodeByHashRefs",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							24u8, 6u8, 23u8, 50u8, 105u8, 203u8, 126u8, 161u8, 0u8, 5u8, 121u8,
							165u8, 204u8, 106u8, 68u8, 91u8, 84u8, 126u8, 29u8, 239u8, 236u8,
							138u8, 32u8, 237u8, 241u8, 226u8, 190u8, 233u8, 160u8, 143u8, 88u8,
							168u8,
						],
					)
				}
				pub fn code_by_hash_refs_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"CodeByHashRefs",
						Vec::new(),
						[
							24u8, 6u8, 23u8, 50u8, 105u8, 203u8, 126u8, 161u8, 0u8, 5u8, 121u8,
							165u8, 204u8, 106u8, 68u8, 91u8, 84u8, 126u8, 29u8, 239u8, 236u8,
							138u8, 32u8, 237u8, 241u8, 226u8, 190u8, 233u8, 160u8, 143u8, 88u8,
							168u8,
						],
					)
				}
				pub fn code_by_hash(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::ValidationCode,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"CodeByHash",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							58u8, 104u8, 36u8, 34u8, 226u8, 210u8, 253u8, 90u8, 23u8, 3u8, 6u8,
							202u8, 9u8, 44u8, 107u8, 108u8, 41u8, 149u8, 255u8, 173u8, 119u8,
							206u8, 201u8, 180u8, 32u8, 193u8, 44u8, 232u8, 73u8, 15u8, 210u8,
							226u8,
						],
					)
				}
				pub fn code_by_hash_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::ValidationCode,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Paras",
						"CodeByHash",
						Vec::new(),
						[
							58u8, 104u8, 36u8, 34u8, 226u8, 210u8, 253u8, 90u8, 23u8, 3u8, 6u8,
							202u8, 9u8, 44u8, 107u8, 108u8, 41u8, 149u8, 255u8, 173u8, 119u8,
							206u8, 201u8, 180u8, 32u8, 193u8, 44u8, 232u8, 73u8, 15u8, 210u8,
							226u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn unsigned_priority(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Paras",
						"UnsignedPriority",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod bounded_collections {
			use super::runtime_types;
			pub mod bounded_btree_map {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BoundedBTreeMap<_0, _1>(pub ::subxt::utils::KeyedVec<_0, _1>);
			}
			pub mod bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
			pub mod weak_bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
		}
		pub mod finality_grandpa {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Equivocation<_0, _1, _2> {
				pub round_number: ::core::primitive::u64,
				pub identity: _0,
				pub first: (_1, _2),
				pub second: (_1, _2),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Precommit<_0, _1> {
				pub target_hash: _0,
				pub target_number: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Prevote<_0, _1> {
				pub target_hash: _0,
				pub target_number: _1,
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DispatchInfo {
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PostDispatchInfo {
					pub actual_weight:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum RawOrigin<_0> {
					#[codec(index = 0)]
					Root,
					#[codec(index = 1)]
					Signed(_0),
					#[codec(index = 2)]
					None,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod messages {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum ProcessMessageError {
						#[codec(index = 0)]
						BadFormat,
						#[codec(index = 1)]
						Corrupt,
						#[codec(index = 2)]
						Unsupported,
						#[codec(index = 3)]
						Overweight(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 4)]
						Yield,
					}
				}
				pub mod misc {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct WrapperOpaque<_0>(
						#[codec(compact)] pub ::core::primitive::u32,
						pub _0,
					);
				}
				pub mod preimages {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Bounded<_0> {
						#[codec(index = 0)]
						Legacy {
							hash: ::subxt::utils::H256,
						},
						#[codec(index = 1)]
						Inline(
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 2)]
						Lookup {
							hash: ::subxt::utils::H256,
							len: ::core::primitive::u32,
						},
						__Ignore(::core::marker::PhantomData<_0>),
					}
				}
				pub mod schedule {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum DispatchTime<_0> {
						#[codec(index = 0)]
						At(_0),
						#[codec(index = 1)]
						After(_0),
					}
				}
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
					}
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PalletId(pub [::core::primitive::u8; 8usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
						::core::primitive::u32,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockWeights {
					pub base_block: runtime_types::sp_weights::weight_v2::Weight,
					pub max_block: runtime_types::sp_weights::weight_v2::Weight,
					pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WeightsPerClass {
					pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
					pub max_extrinsic:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub max_total:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub reserved:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					remark { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 2)]
					set_code { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 3)]
					set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 5)]
					kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
					#[codec(index = 6)]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InvalidSpecName,
					#[codec(index = 1)]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					NonDefaultComposite,
					#[codec(index = 4)]
					NonZeroRefCount,
					#[codec(index = 5)]
					CallFiltered,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 1)]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 2)]
					CodeUpdated,
					#[codec(index = 3)]
					NewAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 4)]
					KilledAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 5)]
					Remarked { sender: ::subxt::utils::AccountId32, hash: ::subxt::utils::H256 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: _0,
				pub providers: _0,
				pub sufficients: _0,
				pub data: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
		}
		pub mod pallet_babe {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					report_equivocation {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_slots::EquivocationProof<
								runtime_types::sp_runtime::generic::header::Header<
									::core::primitive::u32,
									runtime_types::sp_runtime::traits::BlakeTwo256,
								>,
								runtime_types::sp_consensus_babe::app::Public,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 1)]
					report_equivocation_unsigned {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_slots::EquivocationProof<
								runtime_types::sp_runtime::generic::header::Header<
									::core::primitive::u32,
									runtime_types::sp_runtime::traits::BlakeTwo256,
								>,
								runtime_types::sp_consensus_babe::app::Public,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 2)]
					plan_config_change {
						config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InvalidEquivocationProof,
					#[codec(index = 1)]
					InvalidKeyOwnershipProof,
					#[codec(index = 2)]
					DuplicateOffenceReport,
					#[codec(index = 3)]
					InvalidConfiguration,
				}
			}
		}
		pub mod pallet_bags_list {
			use super::runtime_types;
			pub mod list {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Bag {
					pub head: ::core::option::Option<::subxt::utils::AccountId32>,
					pub tail: ::core::option::Option<::subxt::utils::AccountId32>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum ListError {
					#[codec(index = 0)]
					Duplicate,
					#[codec(index = 1)]
					NotHeavier,
					#[codec(index = 2)]
					NotInSameBag,
					#[codec(index = 3)]
					NodeNotFound,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Node {
					pub id: ::subxt::utils::AccountId32,
					pub prev: ::core::option::Option<::subxt::utils::AccountId32>,
					pub next: ::core::option::Option<::subxt::utils::AccountId32>,
					pub bag_upper: ::core::primitive::u64,
					pub score: ::core::primitive::u64,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					rebag {
						dislocated: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 1)]
					put_in_front_of {
						lighter: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					List(runtime_types::pallet_bags_list::list::ListError),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Rebagged {
						who: ::subxt::utils::AccountId32,
						from: ::core::primitive::u64,
						to: ::core::primitive::u64,
					},
					#[codec(index = 1)]
					ScoreUpdated {
						who: ::subxt::utils::AccountId32,
						new_score: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					transfer_allow_death {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					set_balance_deprecated {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						old_reserved: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					force_transfer {
						source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					transfer_keep_alive {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					transfer_all {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					force_unreserve {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					upgrade_accounts { who: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 7)]
					transfer {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					force_set_balance {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					VestingBalance,
					#[codec(index = 1)]
					LiquidityRestrictions,
					#[codec(index = 2)]
					InsufficientBalance,
					#[codec(index = 3)]
					ExistentialDeposit,
					#[codec(index = 4)]
					Expendability,
					#[codec(index = 5)]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					DeadAccount,
					#[codec(index = 7)]
					TooManyReserves,
					#[codec(index = 8)]
					TooManyHolds,
					#[codec(index = 9)]
					TooManyFreezes,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Endowed {
						account: ::subxt::utils::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					DustLost {
						account: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					Transfer {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					BalanceSet { who: ::subxt::utils::AccountId32, free: ::core::primitive::u128 },
					#[codec(index = 4)]
					Reserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 5)]
					Unreserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 6)]
					ReserveRepatriated {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					Deposit { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 8)]
					Withdraw { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 9)]
					Slashed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 10)]
					Minted { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 11)]
					Burned { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 12)]
					Suspended { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 13)]
					Restored { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 14)]
					Upgraded { who: ::subxt::utils::AccountId32 },
					#[codec(index = 15)]
					Issued { amount: ::core::primitive::u128 },
					#[codec(index = 16)]
					Rescinded { amount: ::core::primitive::u128 },
					#[codec(index = 17)]
					Locked { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 18)]
					Unlocked { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 19)]
					Frozen { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 20)]
					Thawed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AccountData<_0> {
					pub free: _0,
					pub reserved: _0,
					pub frozen: _0,
					pub flags: runtime_types::pallet_balances::types::ExtraFlags,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BalanceLock<_0> {
					pub id: [::core::primitive::u8; 8usize],
					pub amount: _0,
					pub reasons: runtime_types::pallet_balances::types::Reasons,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ExtraFlags(pub ::core::primitive::u128);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct IdAmount<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Reasons {
					#[codec(index = 0)]
					Fee,
					#[codec(index = 1)]
					Misc,
					#[codec(index = 2)]
					All,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReserveData<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
			}
		}
		pub mod pallet_bounties {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					propose_bounty {
						#[codec(compact)]
						value: ::core::primitive::u128,
						description: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					approve_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					propose_curator {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
						curator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						fee: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					unassign_curator {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					accept_curator {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					award_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
						beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 6)]
					claim_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					close_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					extend_bounty_expiry {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
						remark: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InsufficientProposersBalance,
					#[codec(index = 1)]
					InvalidIndex,
					#[codec(index = 2)]
					ReasonTooBig,
					#[codec(index = 3)]
					UnexpectedStatus,
					#[codec(index = 4)]
					RequireCurator,
					#[codec(index = 5)]
					InvalidValue,
					#[codec(index = 6)]
					InvalidFee,
					#[codec(index = 7)]
					PendingPayout,
					#[codec(index = 8)]
					Premature,
					#[codec(index = 9)]
					HasActiveChildBounty,
					#[codec(index = 10)]
					TooManyQueued,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					BountyProposed { index: ::core::primitive::u32 },
					#[codec(index = 1)]
					BountyRejected { index: ::core::primitive::u32, bond: ::core::primitive::u128 },
					#[codec(index = 2)]
					BountyBecameActive { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					BountyAwarded {
						index: ::core::primitive::u32,
						beneficiary: ::subxt::utils::AccountId32,
					},
					#[codec(index = 4)]
					BountyClaimed {
						index: ::core::primitive::u32,
						payout: ::core::primitive::u128,
						beneficiary: ::subxt::utils::AccountId32,
					},
					#[codec(index = 5)]
					BountyCanceled { index: ::core::primitive::u32 },
					#[codec(index = 6)]
					BountyExtended { index: ::core::primitive::u32 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Bounty<_0, _1, _2> {
				pub proposer: _0,
				pub value: _1,
				pub fee: _1,
				pub curator_deposit: _1,
				pub bond: _1,
				pub status: runtime_types::pallet_bounties::BountyStatus<_0, _2>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum BountyStatus<_0, _1> {
				#[codec(index = 0)]
				Proposed,
				#[codec(index = 1)]
				Approved,
				#[codec(index = 2)]
				Funded,
				#[codec(index = 3)]
				CuratorProposed { curator: _0 },
				#[codec(index = 4)]
				Active { curator: _0, update_due: _1 },
				#[codec(index = 5)]
				PendingPayout { curator: _0, beneficiary: _0, unlock_at: _1 },
			}
		}
		pub mod pallet_child_bounties {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					add_child_bounty {
						#[codec(compact)]
						parent_bounty_id: ::core::primitive::u32,
						#[codec(compact)]
						value: ::core::primitive::u128,
						description: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					propose_curator {
						#[codec(compact)]
						parent_bounty_id: ::core::primitive::u32,
						#[codec(compact)]
						child_bounty_id: ::core::primitive::u32,
						curator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						fee: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					accept_curator {
						#[codec(compact)]
						parent_bounty_id: ::core::primitive::u32,
						#[codec(compact)]
						child_bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					unassign_curator {
						#[codec(compact)]
						parent_bounty_id: ::core::primitive::u32,
						#[codec(compact)]
						child_bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					award_child_bounty {
						#[codec(compact)]
						parent_bounty_id: ::core::primitive::u32,
						#[codec(compact)]
						child_bounty_id: ::core::primitive::u32,
						beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 5)]
					claim_child_bounty {
						#[codec(compact)]
						parent_bounty_id: ::core::primitive::u32,
						#[codec(compact)]
						child_bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					close_child_bounty {
						#[codec(compact)]
						parent_bounty_id: ::core::primitive::u32,
						#[codec(compact)]
						child_bounty_id: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					ParentBountyNotActive,
					#[codec(index = 1)]
					InsufficientBountyBalance,
					#[codec(index = 2)]
					TooManyChildBounties,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Added { index: ::core::primitive::u32, child_index: ::core::primitive::u32 },
					#[codec(index = 1)]
					Awarded {
						index: ::core::primitive::u32,
						child_index: ::core::primitive::u32,
						beneficiary: ::subxt::utils::AccountId32,
					},
					#[codec(index = 2)]
					Claimed {
						index: ::core::primitive::u32,
						child_index: ::core::primitive::u32,
						payout: ::core::primitive::u128,
						beneficiary: ::subxt::utils::AccountId32,
					},
					#[codec(index = 3)]
					Canceled { index: ::core::primitive::u32, child_index: ::core::primitive::u32 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChildBounty<_0, _1, _2> {
				pub parent_bounty: _2,
				pub value: _1,
				pub fee: _1,
				pub curator_deposit: _1,
				pub status: runtime_types::pallet_child_bounties::ChildBountyStatus<_0, _2>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ChildBountyStatus<_0, _1> {
				#[codec(index = 0)]
				Added,
				#[codec(index = 1)]
				CuratorProposed { curator: _0 },
				#[codec(index = 2)]
				Active { curator: _0 },
				#[codec(index = 3)]
				PendingPayout { curator: _0, beneficiary: _0, unlock_at: _1 },
			}
		}
		pub mod pallet_collective {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					set_members {
						new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
						prime: ::core::option::Option<::subxt::utils::AccountId32>,
						old_count: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					execute {
						proposal: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					propose {
						#[codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					vote {
						proposal: ::subxt::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					disapprove_proposal { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 6)]
					close {
						proposal_hash: ::subxt::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					NotMember,
					#[codec(index = 1)]
					DuplicateProposal,
					#[codec(index = 2)]
					ProposalMissing,
					#[codec(index = 3)]
					WrongIndex,
					#[codec(index = 4)]
					DuplicateVote,
					#[codec(index = 5)]
					AlreadyInitialized,
					#[codec(index = 6)]
					TooEarly,
					#[codec(index = 7)]
					TooManyProposals,
					#[codec(index = 8)]
					WrongProposalWeight,
					#[codec(index = 9)]
					WrongProposalLength,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Proposed {
						account: ::subxt::utils::AccountId32,
						proposal_index: ::core::primitive::u32,
						proposal_hash: ::subxt::utils::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					Voted {
						account: ::subxt::utils::AccountId32,
						proposal_hash: ::subxt::utils::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					Approved { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 3)]
					Disapproved { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 4)]
					Executed {
						proposal_hash: ::subxt::utils::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 5)]
					MemberExecuted {
						proposal_hash: ::subxt::utils::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 6)]
					Closed {
						proposal_hash: ::subxt::utils::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RawOrigin<_0> {
				#[codec(index = 0)]
				Members(::core::primitive::u32, ::core::primitive::u32),
				#[codec(index = 1)]
				Member(_0),
				#[codec(index = 2)]
				_Phantom,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Votes<_0, _1> {
				pub index: _1,
				pub threshold: _1,
				pub ayes: ::std::vec::Vec<_0>,
				pub nays: ::std::vec::Vec<_0>,
				pub end: _1,
			}
		}
		pub mod pallet_conviction_voting {
			use super::runtime_types;
			pub mod conviction {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Conviction {
					#[codec(index = 0)]
					None,
					#[codec(index = 1)]
					Locked1x,
					#[codec(index = 2)]
					Locked2x,
					#[codec(index = 3)]
					Locked3x,
					#[codec(index = 4)]
					Locked4x,
					#[codec(index = 5)]
					Locked5x,
					#[codec(index = 6)]
					Locked6x,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					vote {
						#[codec(compact)]
						poll_index: ::core::primitive::u32,
						vote: runtime_types::pallet_conviction_voting::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 1)]
					delegate {
						class: ::core::primitive::u16,
						to: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						conviction: runtime_types::pallet_conviction_voting::conviction::Conviction,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					undelegate { class: ::core::primitive::u16 },
					#[codec(index = 3)]
					unlock {
						class: ::core::primitive::u16,
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 4)]
					remove_vote {
						class: ::core::option::Option<::core::primitive::u16>,
						index: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					remove_other_vote {
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						class: ::core::primitive::u16,
						index: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					NotOngoing,
					#[codec(index = 1)]
					NotVoter,
					#[codec(index = 2)]
					NoPermission,
					#[codec(index = 3)]
					NoPermissionYet,
					#[codec(index = 4)]
					AlreadyDelegating,
					#[codec(index = 5)]
					AlreadyVoting,
					#[codec(index = 6)]
					InsufficientFunds,
					#[codec(index = 7)]
					NotDelegating,
					#[codec(index = 8)]
					Nonsense,
					#[codec(index = 9)]
					MaxVotesReached,
					#[codec(index = 10)]
					ClassNeeded,
					#[codec(index = 11)]
					BadClass,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Delegated(::subxt::utils::AccountId32, ::subxt::utils::AccountId32),
					#[codec(index = 1)]
					Undelegated(::subxt::utils::AccountId32),
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Delegations<_0> {
					pub votes: _0,
					pub capital: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Tally<_0> {
					pub ayes: _0,
					pub nays: _0,
					pub support: _0,
				}
			}
			pub mod vote {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum AccountVote<_0> {
					#[codec(index = 0)]
					Standard {
						vote: runtime_types::pallet_conviction_voting::vote::Vote,
						balance: _0,
					},
					#[codec(index = 1)]
					Split { aye: _0, nay: _0 },
					#[codec(index = 2)]
					SplitAbstain { aye: _0, nay: _0, abstain: _0 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Casting<_0, _1, _2> {
					pub votes: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						_1,
						runtime_types::pallet_conviction_voting::vote::AccountVote<_0>,
					)>,
					pub delegations:
						runtime_types::pallet_conviction_voting::types::Delegations<_0>,
					pub prior: runtime_types::pallet_conviction_voting::vote::PriorLock<_1, _0>,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Delegating<_0, _1, _2> {
					pub balance: _0,
					pub target: _1,
					pub conviction: runtime_types::pallet_conviction_voting::conviction::Conviction,
					pub delegations:
						runtime_types::pallet_conviction_voting::types::Delegations<_0>,
					pub prior: runtime_types::pallet_conviction_voting::vote::PriorLock<_2, _0>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PriorLock<_0, _1>(pub _0, pub _1);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Vote(pub ::core::primitive::u8);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Voting<_0, _1, _2, _3> {
					#[codec(index = 0)]
					Casting(runtime_types::pallet_conviction_voting::vote::Casting<_0, _2, _2>),
					#[codec(index = 1)]
					Delegating(
						runtime_types::pallet_conviction_voting::vote::Delegating<_0, _1, _2>,
					),
					__Ignore(::core::marker::PhantomData<_3>),
				}
			}
		}
		pub mod pallet_democracy {
			use super::runtime_types;
			pub mod conviction {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Conviction {
					#[codec(index = 0)]
					None,
					#[codec(index = 1)]
					Locked1x,
					#[codec(index = 2)]
					Locked2x,
					#[codec(index = 3)]
					Locked3x,
					#[codec(index = 4)]
					Locked4x,
					#[codec(index = 5)]
					Locked5x,
					#[codec(index = 6)]
					Locked6x,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					propose {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::polkadot_runtime::RuntimeCall,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					second {
						#[codec(compact)]
						proposal: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					vote {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 3)]
					emergency_cancel { ref_index: ::core::primitive::u32 },
					#[codec(index = 4)]
					external_propose {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::polkadot_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 5)]
					external_propose_majority {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::polkadot_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 6)]
					external_propose_default {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::polkadot_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 7)]
					fast_track {
						proposal_hash: ::subxt::utils::H256,
						voting_period: ::core::primitive::u32,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					veto_external { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 9)]
					cancel_referendum {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					delegate {
						to: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					undelegate,
					#[codec(index = 12)]
					clear_public_proposals,
					#[codec(index = 13)]
					unlock { target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> },
					#[codec(index = 14)]
					remove_vote { index: ::core::primitive::u32 },
					#[codec(index = 15)]
					remove_other_vote {
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						index: ::core::primitive::u32,
					},
					#[codec(index = 16)]
					blacklist {
						proposal_hash: ::subxt::utils::H256,
						maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 17)]
					cancel_proposal {
						#[codec(compact)]
						prop_index: ::core::primitive::u32,
					},
					#[codec(index = 18)]
					set_metadata {
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						maybe_hash: ::core::option::Option<::subxt::utils::H256>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					ValueLow,
					#[codec(index = 1)]
					ProposalMissing,
					#[codec(index = 2)]
					AlreadyCanceled,
					#[codec(index = 3)]
					DuplicateProposal,
					#[codec(index = 4)]
					ProposalBlacklisted,
					#[codec(index = 5)]
					NotSimpleMajority,
					#[codec(index = 6)]
					InvalidHash,
					#[codec(index = 7)]
					NoProposal,
					#[codec(index = 8)]
					AlreadyVetoed,
					#[codec(index = 9)]
					ReferendumInvalid,
					#[codec(index = 10)]
					NoneWaiting,
					#[codec(index = 11)]
					NotVoter,
					#[codec(index = 12)]
					NoPermission,
					#[codec(index = 13)]
					AlreadyDelegating,
					#[codec(index = 14)]
					InsufficientFunds,
					#[codec(index = 15)]
					NotDelegating,
					#[codec(index = 16)]
					VotesExist,
					#[codec(index = 17)]
					InstantNotAllowed,
					#[codec(index = 18)]
					Nonsense,
					#[codec(index = 19)]
					WrongUpperBound,
					#[codec(index = 20)]
					MaxVotesReached,
					#[codec(index = 21)]
					TooMany,
					#[codec(index = 22)]
					VotingPeriodLow,
					#[codec(index = 23)]
					PreimageNotExist,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Proposed {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					Tabled {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					ExternalTabled,
					#[codec(index = 3)]
					Started {
						ref_index: ::core::primitive::u32,
						threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					},
					#[codec(index = 4)]
					Passed { ref_index: ::core::primitive::u32 },
					#[codec(index = 5)]
					NotPassed { ref_index: ::core::primitive::u32 },
					#[codec(index = 6)]
					Cancelled { ref_index: ::core::primitive::u32 },
					#[codec(index = 7)]
					Delegated {
						who: ::subxt::utils::AccountId32,
						target: ::subxt::utils::AccountId32,
					},
					#[codec(index = 8)]
					Undelegated { account: ::subxt::utils::AccountId32 },
					#[codec(index = 9)]
					Vetoed {
						who: ::subxt::utils::AccountId32,
						proposal_hash: ::subxt::utils::H256,
						until: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					Blacklisted { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 11)]
					Voted {
						voter: ::subxt::utils::AccountId32,
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 12)]
					Seconded {
						seconder: ::subxt::utils::AccountId32,
						prop_index: ::core::primitive::u32,
					},
					#[codec(index = 13)]
					ProposalCanceled { prop_index: ::core::primitive::u32 },
					#[codec(index = 14)]
					MetadataSet {
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: ::subxt::utils::H256,
					},
					#[codec(index = 15)]
					MetadataCleared {
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: ::subxt::utils::H256,
					},
					#[codec(index = 16)]
					MetadataTransferred {
						prev_owner: runtime_types::pallet_democracy::types::MetadataOwner,
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: ::subxt::utils::H256,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Delegations<_0> {
					pub votes: _0,
					pub capital: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum MetadataOwner {
					#[codec(index = 0)]
					External,
					#[codec(index = 1)]
					Proposal(::core::primitive::u32),
					#[codec(index = 2)]
					Referendum(::core::primitive::u32),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum ReferendumInfo<_0, _1, _2> {
					#[codec(index = 0)]
					Ongoing(runtime_types::pallet_democracy::types::ReferendumStatus<_0, _1, _2>),
					#[codec(index = 1)]
					Finished { approved: ::core::primitive::bool, end: _0 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReferendumStatus<_0, _1, _2> {
					pub end: _0,
					pub proposal: _1,
					pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					pub delay: _0,
					pub tally: runtime_types::pallet_democracy::types::Tally<_2>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Tally<_0> {
					pub ayes: _0,
					pub nays: _0,
					pub turnout: _0,
				}
			}
			pub mod vote {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum AccountVote<_0> {
					#[codec(index = 0)]
					Standard { vote: runtime_types::pallet_democracy::vote::Vote, balance: _0 },
					#[codec(index = 1)]
					Split { aye: _0, nay: _0 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PriorLock<_0, _1>(pub _0, pub _1);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Vote(pub ::core::primitive::u8);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Voting<_0, _1, _2> {
					#[codec(index = 0)]
					Direct {
						votes: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							_2,
							runtime_types::pallet_democracy::vote::AccountVote<_0>,
						)>,
						delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
						prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
					},
					#[codec(index = 1)]
					Delegating {
						balance: _0,
						target: _1,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
						prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
					},
				}
			}
			pub mod vote_threshold {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum VoteThreshold {
					#[codec(index = 0)]
					SuperMajorityApprove,
					#[codec(index = 1)]
					SuperMajorityAgainst,
					#[codec(index = 2)]
					SimpleMajority,
				}
			}
		}
		pub mod pallet_election_provider_multi_phase {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					# [codec (index = 0)] submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: polkadot_runtime :: NposCompactSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , # [codec (index = 1)] set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < runtime_types :: sp_npos_elections :: ElectionScore > , } , # [codec (index = 2)] set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: utils :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: utils :: AccountId32 > ,) > , } , # [codec (index = 3)] submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: polkadot_runtime :: NposCompactSolution16 > > , } , # [codec (index = 4)] governance_fallback { maybe_max_voters : :: core :: option :: Option < :: core :: primitive :: u32 > , maybe_max_targets : :: core :: option :: Option < :: core :: primitive :: u32 > , } , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					PreDispatchEarlySubmission,
					#[codec(index = 1)]
					PreDispatchWrongWinnerCount,
					#[codec(index = 2)]
					PreDispatchWeakSubmission,
					#[codec(index = 3)]
					SignedQueueFull,
					#[codec(index = 4)]
					SignedCannotPayDeposit,
					#[codec(index = 5)]
					SignedInvalidWitness,
					#[codec(index = 6)]
					SignedTooMuchWeight,
					#[codec(index = 7)]
					OcwCallWrongEra,
					#[codec(index = 8)]
					MissingSnapshotMetadata,
					#[codec(index = 9)]
					InvalidSubmissionIndex,
					#[codec(index = 10)]
					CallNotAllowed,
					#[codec(index = 11)]
					FallbackFailed,
					#[codec(index = 12)]
					BoundNotMet,
					#[codec(index = 13)]
					TooManyWinners,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					SolutionStored {
						compute:
							runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
						origin: ::core::option::Option<::subxt::utils::AccountId32>,
						prev_ejected: ::core::primitive::bool,
					},
					#[codec(index = 1)]
					ElectionFinalized {
						compute:
							runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
						score: runtime_types::sp_npos_elections::ElectionScore,
					},
					#[codec(index = 2)]
					ElectionFailed,
					#[codec(index = 3)]
					Rewarded {
						account: ::subxt::utils::AccountId32,
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					Slashed { account: ::subxt::utils::AccountId32, value: ::core::primitive::u128 },
					#[codec(index = 5)]
					PhaseTransitioned {
						from: runtime_types::pallet_election_provider_multi_phase::Phase<
							::core::primitive::u32,
						>,
						to: runtime_types::pallet_election_provider_multi_phase::Phase<
							::core::primitive::u32,
						>,
						round: ::core::primitive::u32,
					},
				}
			}
			pub mod signed {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SignedSubmission<_0, _1, _2> {
					pub who: _0,
					pub deposit: _1,
					pub raw_solution:
						runtime_types::pallet_election_provider_multi_phase::RawSolution<_2>,
					pub call_fee: _1,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ElectionCompute {
				#[codec(index = 0)]
				OnChain,
				#[codec(index = 1)]
				Signed,
				#[codec(index = 2)]
				Unsigned,
				#[codec(index = 3)]
				Fallback,
				#[codec(index = 4)]
				Emergency,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Phase<_0> {
				#[codec(index = 0)]
				Off,
				#[codec(index = 1)]
				Signed,
				#[codec(index = 2)]
				Unsigned((::core::primitive::bool, _0)),
				#[codec(index = 3)]
				Emergency,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RawSolution<_0> {
				pub solution: _0,
				pub score: runtime_types::sp_npos_elections::ElectionScore,
				pub round: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ReadySolution {
				pub supports: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
					::subxt::utils::AccountId32,
					runtime_types::sp_npos_elections::Support<::subxt::utils::AccountId32>,
				)>,
				pub score: runtime_types::sp_npos_elections::ElectionScore,
				pub compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RoundSnapshot<_0, _1> {
				pub voters: ::std::vec::Vec<_1>,
				pub targets: ::std::vec::Vec<_0>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SolutionOrSnapshotSize {
				#[codec(compact)]
				pub voters: ::core::primitive::u32,
				#[codec(compact)]
				pub targets: ::core::primitive::u32,
			}
		}
		pub mod pallet_elections_phragmen {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					vote {
						votes: ::std::vec::Vec<::subxt::utils::AccountId32>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					remove_voter,
					#[codec(index = 2)]
					submit_candidacy {
						#[codec(compact)]
						candidate_count: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					renounce_candidacy {
						renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
					},
					#[codec(index = 4)]
					remove_member {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						slash_bond: ::core::primitive::bool,
						rerun_election: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					clean_defunct_voters {
						num_voters: ::core::primitive::u32,
						num_defunct: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					UnableToVote,
					#[codec(index = 1)]
					NoVotes,
					#[codec(index = 2)]
					TooManyVotes,
					#[codec(index = 3)]
					MaximumVotesExceeded,
					#[codec(index = 4)]
					LowBalance,
					#[codec(index = 5)]
					UnableToPayBond,
					#[codec(index = 6)]
					MustBeVoter,
					#[codec(index = 7)]
					DuplicatedCandidate,
					#[codec(index = 8)]
					TooManyCandidates,
					#[codec(index = 9)]
					MemberSubmit,
					#[codec(index = 10)]
					RunnerUpSubmit,
					#[codec(index = 11)]
					InsufficientCandidateFunds,
					#[codec(index = 12)]
					NotMember,
					#[codec(index = 13)]
					InvalidWitnessData,
					#[codec(index = 14)]
					InvalidVoteCount,
					#[codec(index = 15)]
					InvalidRenouncing,
					#[codec(index = 16)]
					InvalidReplacement,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					NewTerm {
						new_members:
							::std::vec::Vec<(::subxt::utils::AccountId32, ::core::primitive::u128)>,
					},
					#[codec(index = 1)]
					EmptyTerm,
					#[codec(index = 2)]
					ElectionError,
					#[codec(index = 3)]
					MemberKicked { member: ::subxt::utils::AccountId32 },
					#[codec(index = 4)]
					Renounced { candidate: ::subxt::utils::AccountId32 },
					#[codec(index = 5)]
					CandidateSlashed {
						candidate: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					SeatHolderSlashed {
						seat_holder: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Renouncing {
				#[codec(index = 0)]
				Member,
				#[codec(index = 1)]
				RunnerUp,
				#[codec(index = 2)]
				Candidate(#[codec(compact)] ::core::primitive::u32),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SeatHolder<_0, _1> {
				pub who: _0,
				pub stake: _1,
				pub deposit: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Voter<_0, _1> {
				pub votes: ::std::vec::Vec<_0>,
				pub stake: _1,
				pub deposit: _1,
			}
		}
		pub mod pallet_fast_unstake {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					register_fast_unstake,
					#[codec(index = 1)]
					deregister,
					#[codec(index = 2)]
					control { eras_to_check: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					NotController,
					#[codec(index = 1)]
					AlreadyQueued,
					#[codec(index = 2)]
					NotFullyBonded,
					#[codec(index = 3)]
					NotQueued,
					#[codec(index = 4)]
					AlreadyHead,
					#[codec(index = 5)]
					CallNotAllowed,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Unstaked {
						stash: ::subxt::utils::AccountId32,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					Slashed { stash: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 2)]
					InternalError,
					#[codec(index = 3)]
					BatchChecked { eras: ::std::vec::Vec<::core::primitive::u32> },
					#[codec(index = 4)]
					BatchFinished { size: ::core::primitive::u32 },
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UnstakeRequest {
					pub stashes: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					)>,
					pub checked: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u32,
					>,
				}
			}
		}
		pub mod pallet_grandpa {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					report_equivocation {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_grandpa::EquivocationProof<
								::subxt::utils::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 1)]
					report_equivocation_unsigned {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_grandpa::EquivocationProof<
								::subxt::utils::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 2)]
					note_stalled {
						delay: ::core::primitive::u32,
						best_finalized_block_number: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					PauseFailed,
					#[codec(index = 1)]
					ResumeFailed,
					#[codec(index = 2)]
					ChangePending,
					#[codec(index = 3)]
					TooSoon,
					#[codec(index = 4)]
					InvalidKeyOwnershipProof,
					#[codec(index = 5)]
					InvalidEquivocationProof,
					#[codec(index = 6)]
					DuplicateOffenceReport,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					NewAuthorities {
						authority_set: ::std::vec::Vec<(
							runtime_types::sp_consensus_grandpa::app::Public,
							::core::primitive::u64,
						)>,
					},
					#[codec(index = 1)]
					Paused,
					#[codec(index = 2)]
					Resumed,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct StoredPendingChange<_0> {
				pub scheduled_at: _0,
				pub delay: _0,
				pub next_authorities:
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
						runtime_types::sp_consensus_grandpa::app::Public,
						::core::primitive::u64,
					)>,
				pub forced: ::core::option::Option<_0>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum StoredState<_0> {
				#[codec(index = 0)]
				Live,
				#[codec(index = 1)]
				PendingPause { scheduled_at: _0, delay: _0 },
				#[codec(index = 2)]
				Paused,
				#[codec(index = 3)]
				PendingResume { scheduled_at: _0, delay: _0 },
			}
		}
		pub mod pallet_identity {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					add_registrar {
						account: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 1)]
					set_identity {
						info:
							::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
					},
					#[codec(index = 2)]
					set_subs {
						subs: ::std::vec::Vec<(
							::subxt::utils::AccountId32,
							runtime_types::pallet_identity::types::Data,
						)>,
					},
					#[codec(index = 3)]
					clear_identity,
					#[codec(index = 4)]
					request_judgement {
						#[codec(compact)]
						reg_index: ::core::primitive::u32,
						#[codec(compact)]
						max_fee: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					cancel_request { reg_index: ::core::primitive::u32 },
					#[codec(index = 6)]
					set_fee {
						#[codec(compact)]
						index: ::core::primitive::u32,
						#[codec(compact)]
						fee: ::core::primitive::u128,
					},
					#[codec(index = 7)]
					set_account_id {
						#[codec(compact)]
						index: ::core::primitive::u32,
						new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 8)]
					set_fields {
						#[codec(compact)]
						index: ::core::primitive::u32,
						fields: runtime_types::pallet_identity::types::BitFlags<
							runtime_types::pallet_identity::types::IdentityField,
						>,
					},
					#[codec(index = 9)]
					provide_judgement {
						#[codec(compact)]
						reg_index: ::core::primitive::u32,
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						judgement: runtime_types::pallet_identity::types::Judgement<
							::core::primitive::u128,
						>,
						identity: ::subxt::utils::H256,
					},
					#[codec(index = 10)]
					kill_identity {
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 11)]
					add_sub {
						sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						data: runtime_types::pallet_identity::types::Data,
					},
					#[codec(index = 12)]
					rename_sub {
						sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						data: runtime_types::pallet_identity::types::Data,
					},
					#[codec(index = 13)]
					remove_sub {
						sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 14)]
					quit_sub,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					TooManySubAccounts,
					#[codec(index = 1)]
					NotFound,
					#[codec(index = 2)]
					NotNamed,
					#[codec(index = 3)]
					EmptyIndex,
					#[codec(index = 4)]
					FeeChanged,
					#[codec(index = 5)]
					NoIdentity,
					#[codec(index = 6)]
					StickyJudgement,
					#[codec(index = 7)]
					JudgementGiven,
					#[codec(index = 8)]
					InvalidJudgement,
					#[codec(index = 9)]
					InvalidIndex,
					#[codec(index = 10)]
					InvalidTarget,
					#[codec(index = 11)]
					TooManyFields,
					#[codec(index = 12)]
					TooManyRegistrars,
					#[codec(index = 13)]
					AlreadyClaimed,
					#[codec(index = 14)]
					NotSub,
					#[codec(index = 15)]
					NotOwned,
					#[codec(index = 16)]
					JudgementForDifferentIdentity,
					#[codec(index = 17)]
					JudgementPaymentFailed,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					IdentitySet { who: ::subxt::utils::AccountId32 },
					#[codec(index = 1)]
					IdentityCleared {
						who: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					IdentityKilled {
						who: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					JudgementRequested {
						who: ::subxt::utils::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					JudgementUnrequested {
						who: ::subxt::utils::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					JudgementGiven {
						target: ::subxt::utils::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					RegistrarAdded { registrar_index: ::core::primitive::u32 },
					#[codec(index = 7)]
					SubIdentityAdded {
						sub: ::subxt::utils::AccountId32,
						main: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					SubIdentityRemoved {
						sub: ::subxt::utils::AccountId32,
						main: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					SubIdentityRevoked {
						sub: ::subxt::utils::AccountId32,
						main: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BitFlags<_0>(
					pub ::core::primitive::u64,
					#[codec(skip)] pub ::core::marker::PhantomData<_0>,
				);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Data {
					#[codec(index = 0)]
					None,
					#[codec(index = 1)]
					Raw0([::core::primitive::u8; 0usize]),
					#[codec(index = 2)]
					Raw1([::core::primitive::u8; 1usize]),
					#[codec(index = 3)]
					Raw2([::core::primitive::u8; 2usize]),
					#[codec(index = 4)]
					Raw3([::core::primitive::u8; 3usize]),
					#[codec(index = 5)]
					Raw4([::core::primitive::u8; 4usize]),
					#[codec(index = 6)]
					Raw5([::core::primitive::u8; 5usize]),
					#[codec(index = 7)]
					Raw6([::core::primitive::u8; 6usize]),
					#[codec(index = 8)]
					Raw7([::core::primitive::u8; 7usize]),
					#[codec(index = 9)]
					Raw8([::core::primitive::u8; 8usize]),
					#[codec(index = 10)]
					Raw9([::core::primitive::u8; 9usize]),
					#[codec(index = 11)]
					Raw10([::core::primitive::u8; 10usize]),
					#[codec(index = 12)]
					Raw11([::core::primitive::u8; 11usize]),
					#[codec(index = 13)]
					Raw12([::core::primitive::u8; 12usize]),
					#[codec(index = 14)]
					Raw13([::core::primitive::u8; 13usize]),
					#[codec(index = 15)]
					Raw14([::core::primitive::u8; 14usize]),
					#[codec(index = 16)]
					Raw15([::core::primitive::u8; 15usize]),
					#[codec(index = 17)]
					Raw16([::core::primitive::u8; 16usize]),
					#[codec(index = 18)]
					Raw17([::core::primitive::u8; 17usize]),
					#[codec(index = 19)]
					Raw18([::core::primitive::u8; 18usize]),
					#[codec(index = 20)]
					Raw19([::core::primitive::u8; 19usize]),
					#[codec(index = 21)]
					Raw20([::core::primitive::u8; 20usize]),
					#[codec(index = 22)]
					Raw21([::core::primitive::u8; 21usize]),
					#[codec(index = 23)]
					Raw22([::core::primitive::u8; 22usize]),
					#[codec(index = 24)]
					Raw23([::core::primitive::u8; 23usize]),
					#[codec(index = 25)]
					Raw24([::core::primitive::u8; 24usize]),
					#[codec(index = 26)]
					Raw25([::core::primitive::u8; 25usize]),
					#[codec(index = 27)]
					Raw26([::core::primitive::u8; 26usize]),
					#[codec(index = 28)]
					Raw27([::core::primitive::u8; 27usize]),
					#[codec(index = 29)]
					Raw28([::core::primitive::u8; 28usize]),
					#[codec(index = 30)]
					Raw29([::core::primitive::u8; 29usize]),
					#[codec(index = 31)]
					Raw30([::core::primitive::u8; 30usize]),
					#[codec(index = 32)]
					Raw31([::core::primitive::u8; 31usize]),
					#[codec(index = 33)]
					Raw32([::core::primitive::u8; 32usize]),
					#[codec(index = 34)]
					BlakeTwo256([::core::primitive::u8; 32usize]),
					#[codec(index = 35)]
					Sha256([::core::primitive::u8; 32usize]),
					#[codec(index = 36)]
					Keccak256([::core::primitive::u8; 32usize]),
					#[codec(index = 37)]
					ShaThree256([::core::primitive::u8; 32usize]),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum IdentityField {
					#[codec(index = 1)]
					Display,
					#[codec(index = 2)]
					Legal,
					#[codec(index = 4)]
					Web,
					#[codec(index = 8)]
					Riot,
					#[codec(index = 16)]
					Email,
					#[codec(index = 32)]
					PgpFingerprint,
					#[codec(index = 64)]
					Image,
					#[codec(index = 128)]
					Twitter,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct IdentityInfo {
					pub additional: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::pallet_identity::types::Data,
						runtime_types::pallet_identity::types::Data,
					)>,
					pub display: runtime_types::pallet_identity::types::Data,
					pub legal: runtime_types::pallet_identity::types::Data,
					pub web: runtime_types::pallet_identity::types::Data,
					pub riot: runtime_types::pallet_identity::types::Data,
					pub email: runtime_types::pallet_identity::types::Data,
					pub pgp_fingerprint: ::core::option::Option<[::core::primitive::u8; 20usize]>,
					pub image: runtime_types::pallet_identity::types::Data,
					pub twitter: runtime_types::pallet_identity::types::Data,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Judgement<_0> {
					#[codec(index = 0)]
					Unknown,
					#[codec(index = 1)]
					FeePaid(_0),
					#[codec(index = 2)]
					Reasonable,
					#[codec(index = 3)]
					KnownGood,
					#[codec(index = 4)]
					OutOfDate,
					#[codec(index = 5)]
					LowQuality,
					#[codec(index = 6)]
					Erroneous,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RegistrarInfo<_0, _1> {
					pub account: _1,
					pub fee: _0,
					pub fields: runtime_types::pallet_identity::types::BitFlags<
						runtime_types::pallet_identity::types::IdentityField,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Registration<_0> {
					pub judgements: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u32,
						runtime_types::pallet_identity::types::Judgement<_0>,
					)>,
					pub deposit: _0,
					pub info: runtime_types::pallet_identity::types::IdentityInfo,
				}
			}
		}
		pub mod pallet_im_online {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					heartbeat {
						heartbeat:
							runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
						signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InvalidKey,
					#[codec(index = 1)]
					DuplicatedHeartbeat,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					HeartbeatReceived {
						authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
					},
					#[codec(index = 1)]
					AllGood,
					#[codec(index = 2)]
					SomeOffline {
						offline: ::std::vec::Vec<(
							::subxt::utils::AccountId32,
							runtime_types::pallet_staking::Exposure<
								::subxt::utils::AccountId32,
								::core::primitive::u128,
							>,
						)>,
					},
				}
			}
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BoundedOpaqueNetworkState {
				pub peer_id: runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
					::core::primitive::u8,
				>,
				pub external_addresses:
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Heartbeat<_0> {
				pub block_number: _0,
				pub network_state: runtime_types::sp_core::offchain::OpaqueNetworkState,
				pub session_index: _0,
				pub authority_index: _0,
				pub validators_len: _0,
			}
		}
		pub mod pallet_indices {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					claim { index: ::core::primitive::u32 },
					#[codec(index = 1)]
					transfer {
						new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					free { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					force_transfer {
						new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						index: ::core::primitive::u32,
						freeze: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					freeze { index: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					NotAssigned,
					#[codec(index = 1)]
					NotOwner,
					#[codec(index = 2)]
					InUse,
					#[codec(index = 3)]
					NotTransfer,
					#[codec(index = 4)]
					Permanent,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					IndexAssigned {
						who: ::subxt::utils::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					IndexFreed { index: ::core::primitive::u32 },
					#[codec(index = 2)]
					IndexFrozen { index: ::core::primitive::u32, who: ::subxt::utils::AccountId32 },
				}
			}
		}
		pub mod pallet_membership {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					add_member {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 1)]
					remove_member {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 2)]
					swap_member {
						remove: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						add: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 3)]
					reset_members { members: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 4)]
					change_key {
						new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 5)]
					set_prime { who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> },
					#[codec(index = 6)]
					clear_prime,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					AlreadyMember,
					#[codec(index = 1)]
					NotMember,
					#[codec(index = 2)]
					TooManyMembers,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					MemberAdded,
					#[codec(index = 1)]
					MemberRemoved,
					#[codec(index = 2)]
					MembersSwapped,
					#[codec(index = 3)]
					MembersReset,
					#[codec(index = 4)]
					KeyChanged,
					#[codec(index = 5)]
					Dummy,
				}
			}
		}
		pub mod pallet_message_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					# [codec (index = 0)] reap_page { message_origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , page_index : :: core :: primitive :: u32 , } , # [codec (index = 1)] execute_overweight { message_origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , page : :: core :: primitive :: u32 , index : :: core :: primitive :: u32 , weight_limit : runtime_types :: sp_weights :: weight_v2 :: Weight , } , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					NotReapable,
					#[codec(index = 1)]
					NoPage,
					#[codec(index = 2)]
					NoMessage,
					#[codec(index = 3)]
					AlreadyProcessed,
					#[codec(index = 4)]
					Queued,
					#[codec(index = 5)]
					InsufficientWeight,
					#[codec(index = 6)]
					TemporarilyUnprocessable,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					# [codec (index = 0)] ProcessingFailed { id : [:: core :: primitive :: u8 ; 32usize] , origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , error : runtime_types :: frame_support :: traits :: messages :: ProcessMessageError , } , # [codec (index = 1)] Processed { id : [:: core :: primitive :: u8 ; 32usize] , origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , weight_used : runtime_types :: sp_weights :: weight_v2 :: Weight , success : :: core :: primitive :: bool , } , # [codec (index = 2)] OverweightEnqueued { id : [:: core :: primitive :: u8 ; 32usize] , origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , page_index : :: core :: primitive :: u32 , message_index : :: core :: primitive :: u32 , } , # [codec (index = 3)] PageReaped { origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , index : :: core :: primitive :: u32 , } , }
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BookState<_0> {
				pub begin: ::core::primitive::u32,
				pub end: ::core::primitive::u32,
				pub count: ::core::primitive::u32,
				pub ready_neighbours:
					::core::option::Option<runtime_types::pallet_message_queue::Neighbours<_0>>,
				pub message_count: ::core::primitive::u64,
				pub size: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Neighbours<_0> {
				pub prev: _0,
				pub next: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Page<_0> {
				pub remaining: _0,
				pub remaining_size: _0,
				pub first_index: _0,
				pub first: _0,
				pub last: _0,
				pub heap: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u8,
				>,
			}
		}
		pub mod pallet_multisig {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					as_multi_threshold_1 {
						other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					as_multi {
						threshold: ::core::primitive::u16,
						other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
						maybe_timepoint: ::core::option::Option<
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						>,
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					approve_as_multi {
						threshold: ::core::primitive::u16,
						other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
						maybe_timepoint: ::core::option::Option<
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						>,
						call_hash: [::core::primitive::u8; 32usize],
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 3)]
					cancel_as_multi {
						threshold: ::core::primitive::u16,
						other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						call_hash: [::core::primitive::u8; 32usize],
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					MinimumThreshold,
					#[codec(index = 1)]
					AlreadyApproved,
					#[codec(index = 2)]
					NoApprovalsNeeded,
					#[codec(index = 3)]
					TooFewSignatories,
					#[codec(index = 4)]
					TooManySignatories,
					#[codec(index = 5)]
					SignatoriesOutOfOrder,
					#[codec(index = 6)]
					SenderInSignatories,
					#[codec(index = 7)]
					NotFound,
					#[codec(index = 8)]
					NotOwner,
					#[codec(index = 9)]
					NoTimepoint,
					#[codec(index = 10)]
					WrongTimepoint,
					#[codec(index = 11)]
					UnexpectedTimepoint,
					#[codec(index = 12)]
					MaxWeightTooLow,
					#[codec(index = 13)]
					AlreadyStored,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					NewMultisig {
						approving: ::subxt::utils::AccountId32,
						multisig: ::subxt::utils::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 1)]
					MultisigApproval {
						approving: ::subxt::utils::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::utils::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 2)]
					MultisigExecuted {
						approving: ::subxt::utils::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::utils::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 3)]
					MultisigCancelled {
						cancelling: ::subxt::utils::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::utils::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Multisig<_0, _1, _2> {
				pub when: runtime_types::pallet_multisig::Timepoint<_0>,
				pub deposit: _1,
				pub depositor: _2,
				pub approvals: runtime_types::bounded_collections::bounded_vec::BoundedVec<_2>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Timepoint<_0> {
				pub height: _0,
				pub index: _0,
			}
		}
		pub mod pallet_nomination_pools {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					join {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						pool_id: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					bond_extra {
						extra: runtime_types::pallet_nomination_pools::BondExtra<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 2)]
					claim_payout,
					#[codec(index = 3)]
					unbond {
						member_account:
							::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						unbonding_points: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					pool_withdraw_unbonded {
						pool_id: ::core::primitive::u32,
						num_slashing_spans: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					withdraw_unbonded {
						member_account:
							::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						num_slashing_spans: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					create {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						root: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						nominator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						bouncer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 7)]
					create_with_pool_id {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						root: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						nominator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						bouncer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						pool_id: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					nominate {
						pool_id: ::core::primitive::u32,
						validators: ::std::vec::Vec<::subxt::utils::AccountId32>,
					},
					#[codec(index = 9)]
					set_state {
						pool_id: ::core::primitive::u32,
						state: runtime_types::pallet_nomination_pools::PoolState,
					},
					#[codec(index = 10)]
					set_metadata {
						pool_id: ::core::primitive::u32,
						metadata: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 11)]
					set_configs {
						min_join_bond: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u128,
						>,
						min_create_bond: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u128,
						>,
						max_pools: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u32,
						>,
						max_members: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u32,
						>,
						max_members_per_pool: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u32,
						>,
						global_max_commission: runtime_types::pallet_nomination_pools::ConfigOp<
							runtime_types::sp_arithmetic::per_things::Perbill,
						>,
					},
					#[codec(index = 12)]
					update_roles {
						pool_id: ::core::primitive::u32,
						new_root: runtime_types::pallet_nomination_pools::ConfigOp<
							::subxt::utils::AccountId32,
						>,
						new_nominator: runtime_types::pallet_nomination_pools::ConfigOp<
							::subxt::utils::AccountId32,
						>,
						new_bouncer: runtime_types::pallet_nomination_pools::ConfigOp<
							::subxt::utils::AccountId32,
						>,
					},
					#[codec(index = 13)]
					chill { pool_id: ::core::primitive::u32 },
					#[codec(index = 14)]
					bond_extra_other {
						member: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						extra: runtime_types::pallet_nomination_pools::BondExtra<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 15)]
					set_claim_permission {
						permission: runtime_types::pallet_nomination_pools::ClaimPermission,
					},
					#[codec(index = 16)]
					claim_payout_other { other: ::subxt::utils::AccountId32 },
					#[codec(index = 17)]
					set_commission {
						pool_id: ::core::primitive::u32,
						new_commission: ::core::option::Option<(
							runtime_types::sp_arithmetic::per_things::Perbill,
							::subxt::utils::AccountId32,
						)>,
					},
					#[codec(index = 18)]
					set_commission_max {
						pool_id: ::core::primitive::u32,
						max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
					},
					#[codec(index = 19)]
					set_commission_change_rate {
						pool_id: ::core::primitive::u32,
						change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
							::core::primitive::u32,
						>,
					},
					#[codec(index = 20)]
					claim_commission { pool_id: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum DefensiveError {
					#[codec(index = 0)]
					NotEnoughSpaceInUnbondPool,
					#[codec(index = 1)]
					PoolNotFound,
					#[codec(index = 2)]
					RewardPoolNotFound,
					#[codec(index = 3)]
					SubPoolsNotFound,
					#[codec(index = 4)]
					BondedStashKilledPrematurely,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					PoolNotFound,
					#[codec(index = 1)]
					PoolMemberNotFound,
					#[codec(index = 2)]
					RewardPoolNotFound,
					#[codec(index = 3)]
					SubPoolsNotFound,
					#[codec(index = 4)]
					AccountBelongsToOtherPool,
					#[codec(index = 5)]
					FullyUnbonding,
					#[codec(index = 6)]
					MaxUnbondingLimit,
					#[codec(index = 7)]
					CannotWithdrawAny,
					#[codec(index = 8)]
					MinimumBondNotMet,
					#[codec(index = 9)]
					OverflowRisk,
					#[codec(index = 10)]
					NotDestroying,
					#[codec(index = 11)]
					NotNominator,
					#[codec(index = 12)]
					NotKickerOrDestroying,
					#[codec(index = 13)]
					NotOpen,
					#[codec(index = 14)]
					MaxPools,
					#[codec(index = 15)]
					MaxPoolMembers,
					#[codec(index = 16)]
					CanNotChangeState,
					#[codec(index = 17)]
					DoesNotHavePermission,
					#[codec(index = 18)]
					MetadataExceedsMaxLen,
					#[codec(index = 19)]
					Defensive(runtime_types::pallet_nomination_pools::pallet::DefensiveError),
					#[codec(index = 20)]
					PartialUnbondNotAllowedPermissionlessly,
					#[codec(index = 21)]
					MaxCommissionRestricted,
					#[codec(index = 22)]
					CommissionExceedsMaximum,
					#[codec(index = 23)]
					CommissionChangeThrottled,
					#[codec(index = 24)]
					CommissionChangeRateNotAllowed,
					#[codec(index = 25)]
					NoPendingCommission,
					#[codec(index = 26)]
					NoCommissionCurrentSet,
					#[codec(index = 27)]
					PoolIdInUse,
					#[codec(index = 28)]
					InvalidPoolId,
					#[codec(index = 29)]
					BondExtraRestricted,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Created {
						depositor: ::subxt::utils::AccountId32,
						pool_id: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					Bonded {
						member: ::subxt::utils::AccountId32,
						pool_id: ::core::primitive::u32,
						bonded: ::core::primitive::u128,
						joined: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					PaidOut {
						member: ::subxt::utils::AccountId32,
						pool_id: ::core::primitive::u32,
						payout: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					Unbonded {
						member: ::subxt::utils::AccountId32,
						pool_id: ::core::primitive::u32,
						balance: ::core::primitive::u128,
						points: ::core::primitive::u128,
						era: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					Withdrawn {
						member: ::subxt::utils::AccountId32,
						pool_id: ::core::primitive::u32,
						balance: ::core::primitive::u128,
						points: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					Destroyed { pool_id: ::core::primitive::u32 },
					#[codec(index = 6)]
					StateChanged {
						pool_id: ::core::primitive::u32,
						new_state: runtime_types::pallet_nomination_pools::PoolState,
					},
					#[codec(index = 7)]
					MemberRemoved {
						pool_id: ::core::primitive::u32,
						member: ::subxt::utils::AccountId32,
					},
					#[codec(index = 8)]
					RolesUpdated {
						root: ::core::option::Option<::subxt::utils::AccountId32>,
						bouncer: ::core::option::Option<::subxt::utils::AccountId32>,
						nominator: ::core::option::Option<::subxt::utils::AccountId32>,
					},
					#[codec(index = 9)]
					PoolSlashed {
						pool_id: ::core::primitive::u32,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					UnbondingPoolSlashed {
						pool_id: ::core::primitive::u32,
						era: ::core::primitive::u32,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					PoolCommissionUpdated {
						pool_id: ::core::primitive::u32,
						current: ::core::option::Option<(
							runtime_types::sp_arithmetic::per_things::Perbill,
							::subxt::utils::AccountId32,
						)>,
					},
					#[codec(index = 12)]
					PoolMaxCommissionUpdated {
						pool_id: ::core::primitive::u32,
						max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
					},
					#[codec(index = 13)]
					PoolCommissionChangeRateUpdated {
						pool_id: ::core::primitive::u32,
						change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
							::core::primitive::u32,
						>,
					},
					#[codec(index = 14)]
					PoolCommissionClaimed {
						pool_id: ::core::primitive::u32,
						commission: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum BondExtra<_0> {
				#[codec(index = 0)]
				FreeBalance(_0),
				#[codec(index = 1)]
				Rewards,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondedPoolInner {
				pub commission: runtime_types::pallet_nomination_pools::Commission,
				pub member_counter: ::core::primitive::u32,
				pub points: ::core::primitive::u128,
				pub roles:
					runtime_types::pallet_nomination_pools::PoolRoles<::subxt::utils::AccountId32>,
				pub state: runtime_types::pallet_nomination_pools::PoolState,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ClaimPermission {
				#[codec(index = 0)]
				Permissioned,
				#[codec(index = 1)]
				PermissionlessCompound,
				#[codec(index = 2)]
				PermissionlessWithdraw,
				#[codec(index = 3)]
				PermissionlessAll,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Commission {
				pub current: ::core::option::Option<(
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::utils::AccountId32,
				)>,
				pub max: ::core::option::Option<runtime_types::sp_arithmetic::per_things::Perbill>,
				pub change_rate: ::core::option::Option<
					runtime_types::pallet_nomination_pools::CommissionChangeRate<
						::core::primitive::u32,
					>,
				>,
				pub throttle_from: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CommissionChangeRate<_0> {
				pub max_increase: runtime_types::sp_arithmetic::per_things::Perbill,
				pub min_delay: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ConfigOp<_0> {
				#[codec(index = 0)]
				Noop,
				#[codec(index = 1)]
				Set(_0),
				#[codec(index = 2)]
				Remove,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PoolMember {
				pub pool_id: ::core::primitive::u32,
				pub points: ::core::primitive::u128,
				pub last_recorded_reward_counter:
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
				pub unbonding_eras:
					runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
						::core::primitive::u32,
						::core::primitive::u128,
					>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PoolRoles<_0> {
				pub depositor: _0,
				pub root: ::core::option::Option<_0>,
				pub nominator: ::core::option::Option<_0>,
				pub bouncer: ::core::option::Option<_0>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum PoolState {
				#[codec(index = 0)]
				Open,
				#[codec(index = 1)]
				Blocked,
				#[codec(index = 2)]
				Destroying,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RewardPool {
				pub last_recorded_reward_counter:
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
				pub last_recorded_total_payouts: ::core::primitive::u128,
				pub total_rewards_claimed: ::core::primitive::u128,
				pub total_commission_pending: ::core::primitive::u128,
				pub total_commission_claimed: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SubPools {
				pub no_era: runtime_types::pallet_nomination_pools::UnbondPool,
				pub with_era:
					runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
						::core::primitive::u32,
						runtime_types::pallet_nomination_pools::UnbondPool,
					>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnbondPool {
				pub points: ::core::primitive::u128,
				pub balance: ::core::primitive::u128,
			}
		}
		pub mod pallet_offences {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Offence {
						kind: [::core::primitive::u8; 16usize],
						timeslot: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
			}
		}
		pub mod pallet_preimage {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					note_preimage { bytes: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					unnote_preimage { hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					request_preimage { hash: ::subxt::utils::H256 },
					#[codec(index = 3)]
					unrequest_preimage { hash: ::subxt::utils::H256 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					TooBig,
					#[codec(index = 1)]
					AlreadyNoted,
					#[codec(index = 2)]
					NotAuthorized,
					#[codec(index = 3)]
					NotNoted,
					#[codec(index = 4)]
					Requested,
					#[codec(index = 5)]
					NotRequested,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Noted { hash: ::subxt::utils::H256 },
					#[codec(index = 1)]
					Requested { hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					Cleared { hash: ::subxt::utils::H256 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RequestStatus<_0, _1> {
				#[codec(index = 0)]
				Unrequested { deposit: (_0, _1), len: ::core::primitive::u32 },
				#[codec(index = 1)]
				Requested {
					deposit: ::core::option::Option<(_0, _1)>,
					count: ::core::primitive::u32,
					len: ::core::option::Option<::core::primitive::u32>,
				},
			}
		}
		pub mod pallet_proxy {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					proxy {
						real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						force_proxy_type:
							::core::option::Option<runtime_types::polkadot_runtime::ProxyType>,
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					add_proxy {
						delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						proxy_type: runtime_types::polkadot_runtime::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					remove_proxy {
						delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						proxy_type: runtime_types::polkadot_runtime::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					remove_proxies,
					#[codec(index = 4)]
					create_pure {
						proxy_type: runtime_types::polkadot_runtime::ProxyType,
						delay: ::core::primitive::u32,
						index: ::core::primitive::u16,
					},
					#[codec(index = 5)]
					kill_pure {
						spawner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						proxy_type: runtime_types::polkadot_runtime::ProxyType,
						index: ::core::primitive::u16,
						#[codec(compact)]
						height: ::core::primitive::u32,
						#[codec(compact)]
						ext_index: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					announce {
						real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						call_hash: ::subxt::utils::H256,
					},
					#[codec(index = 7)]
					remove_announcement {
						real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						call_hash: ::subxt::utils::H256,
					},
					#[codec(index = 8)]
					reject_announcement {
						delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						call_hash: ::subxt::utils::H256,
					},
					#[codec(index = 9)]
					proxy_announced {
						delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						force_proxy_type:
							::core::option::Option<runtime_types::polkadot_runtime::ProxyType>,
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					TooMany,
					#[codec(index = 1)]
					NotFound,
					#[codec(index = 2)]
					NotProxy,
					#[codec(index = 3)]
					Unproxyable,
					#[codec(index = 4)]
					Duplicate,
					#[codec(index = 5)]
					NoPermission,
					#[codec(index = 6)]
					Unannounced,
					#[codec(index = 7)]
					NoSelfProxy,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					ProxyExecuted {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					PureCreated {
						pure: ::subxt::utils::AccountId32,
						who: ::subxt::utils::AccountId32,
						proxy_type: runtime_types::polkadot_runtime::ProxyType,
						disambiguation_index: ::core::primitive::u16,
					},
					#[codec(index = 2)]
					Announced {
						real: ::subxt::utils::AccountId32,
						proxy: ::subxt::utils::AccountId32,
						call_hash: ::subxt::utils::H256,
					},
					#[codec(index = 3)]
					ProxyAdded {
						delegator: ::subxt::utils::AccountId32,
						delegatee: ::subxt::utils::AccountId32,
						proxy_type: runtime_types::polkadot_runtime::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					ProxyRemoved {
						delegator: ::subxt::utils::AccountId32,
						delegatee: ::subxt::utils::AccountId32,
						proxy_type: runtime_types::polkadot_runtime::ProxyType,
						delay: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Announcement<_0, _1, _2> {
				pub real: _0,
				pub call_hash: _1,
				pub height: _2,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ProxyDefinition<_0, _1, _2> {
				pub delegate: _0,
				pub proxy_type: _1,
				pub delay: _2,
			}
		}
		pub mod pallet_referenda {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					submit {
						proposal_origin:
							::std::boxed::Box<runtime_types::polkadot_runtime::OriginCaller>,
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::polkadot_runtime::RuntimeCall,
						>,
						enactment_moment:
							runtime_types::frame_support::traits::schedule::DispatchTime<
								::core::primitive::u32,
							>,
					},
					#[codec(index = 1)]
					place_decision_deposit { index: ::core::primitive::u32 },
					#[codec(index = 2)]
					refund_decision_deposit { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					cancel { index: ::core::primitive::u32 },
					#[codec(index = 4)]
					kill { index: ::core::primitive::u32 },
					#[codec(index = 5)]
					nudge_referendum { index: ::core::primitive::u32 },
					#[codec(index = 6)]
					one_fewer_deciding { track: ::core::primitive::u16 },
					#[codec(index = 7)]
					refund_submission_deposit { index: ::core::primitive::u32 },
					#[codec(index = 8)]
					set_metadata {
						index: ::core::primitive::u32,
						maybe_hash: ::core::option::Option<::subxt::utils::H256>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					NotOngoing,
					#[codec(index = 1)]
					HasDeposit,
					#[codec(index = 2)]
					BadTrack,
					#[codec(index = 3)]
					Full,
					#[codec(index = 4)]
					QueueEmpty,
					#[codec(index = 5)]
					BadReferendum,
					#[codec(index = 6)]
					NothingToDo,
					#[codec(index = 7)]
					NoTrack,
					#[codec(index = 8)]
					Unfinished,
					#[codec(index = 9)]
					NoPermission,
					#[codec(index = 10)]
					NoDeposit,
					#[codec(index = 11)]
					BadStatus,
					#[codec(index = 12)]
					PreimageNotExist,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Submitted {
						index: ::core::primitive::u32,
						track: ::core::primitive::u16,
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::polkadot_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 1)]
					DecisionDepositPlaced {
						index: ::core::primitive::u32,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					DecisionDepositRefunded {
						index: ::core::primitive::u32,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					DepositSlashed {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					DecisionStarted {
						index: ::core::primitive::u32,
						track: ::core::primitive::u16,
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::polkadot_runtime::RuntimeCall,
						>,
						tally: runtime_types::pallet_conviction_voting::types::Tally<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 5)]
					ConfirmStarted { index: ::core::primitive::u32 },
					#[codec(index = 6)]
					ConfirmAborted { index: ::core::primitive::u32 },
					#[codec(index = 7)]
					Confirmed {
						index: ::core::primitive::u32,
						tally: runtime_types::pallet_conviction_voting::types::Tally<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 8)]
					Approved { index: ::core::primitive::u32 },
					#[codec(index = 9)]
					Rejected {
						index: ::core::primitive::u32,
						tally: runtime_types::pallet_conviction_voting::types::Tally<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 10)]
					TimedOut {
						index: ::core::primitive::u32,
						tally: runtime_types::pallet_conviction_voting::types::Tally<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 11)]
					Cancelled {
						index: ::core::primitive::u32,
						tally: runtime_types::pallet_conviction_voting::types::Tally<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 12)]
					Killed {
						index: ::core::primitive::u32,
						tally: runtime_types::pallet_conviction_voting::types::Tally<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 13)]
					SubmissionDepositRefunded {
						index: ::core::primitive::u32,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 14)]
					MetadataSet { index: ::core::primitive::u32, hash: ::subxt::utils::H256 },
					#[codec(index = 15)]
					MetadataCleared { index: ::core::primitive::u32, hash: ::subxt::utils::H256 },
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Curve {
					#[codec(index = 0)]
					LinearDecreasing {
						length: runtime_types::sp_arithmetic::per_things::Perbill,
						floor: runtime_types::sp_arithmetic::per_things::Perbill,
						ceil: runtime_types::sp_arithmetic::per_things::Perbill,
					},
					#[codec(index = 1)]
					SteppedDecreasing {
						begin: runtime_types::sp_arithmetic::per_things::Perbill,
						end: runtime_types::sp_arithmetic::per_things::Perbill,
						step: runtime_types::sp_arithmetic::per_things::Perbill,
						period: runtime_types::sp_arithmetic::per_things::Perbill,
					},
					#[codec(index = 2)]
					Reciprocal {
						factor: runtime_types::sp_arithmetic::fixed_point::FixedI64,
						x_offset: runtime_types::sp_arithmetic::fixed_point::FixedI64,
						y_offset: runtime_types::sp_arithmetic::fixed_point::FixedI64,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DecidingStatus<_0> {
					pub since: _0,
					pub confirming: ::core::option::Option<_0>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Deposit<_0, _1> {
					pub who: _0,
					pub amount: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum ReferendumInfo<_0, _1, _2, _3, _4, _5, _6, _7> {
					#[codec(index = 0)]
					Ongoing(
						runtime_types::pallet_referenda::types::ReferendumStatus<
							_0,
							_1,
							_2,
							_3,
							_4,
							_5,
							_6,
							_7,
						>,
					),
					#[codec(index = 1)]
					Approved(
						_2,
						::core::option::Option<
							runtime_types::pallet_referenda::types::Deposit<_6, _4>,
						>,
						::core::option::Option<
							runtime_types::pallet_referenda::types::Deposit<_6, _4>,
						>,
					),
					#[codec(index = 2)]
					Rejected(
						_2,
						::core::option::Option<
							runtime_types::pallet_referenda::types::Deposit<_6, _4>,
						>,
						::core::option::Option<
							runtime_types::pallet_referenda::types::Deposit<_6, _4>,
						>,
					),
					#[codec(index = 3)]
					Cancelled(
						_2,
						::core::option::Option<
							runtime_types::pallet_referenda::types::Deposit<_6, _4>,
						>,
						::core::option::Option<
							runtime_types::pallet_referenda::types::Deposit<_6, _4>,
						>,
					),
					#[codec(index = 4)]
					TimedOut(
						_2,
						::core::option::Option<
							runtime_types::pallet_referenda::types::Deposit<_6, _4>,
						>,
						::core::option::Option<
							runtime_types::pallet_referenda::types::Deposit<_6, _4>,
						>,
					),
					#[codec(index = 5)]
					Killed(_2),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReferendumStatus<_0, _1, _2, _3, _4, _5, _6, _7> {
					pub track: _0,
					pub origin: _1,
					pub proposal: _3,
					pub enactment: runtime_types::frame_support::traits::schedule::DispatchTime<_2>,
					pub submitted: _2,
					pub submission_deposit: runtime_types::pallet_referenda::types::Deposit<_6, _4>,
					pub decision_deposit: ::core::option::Option<
						runtime_types::pallet_referenda::types::Deposit<_6, _4>,
					>,
					pub deciding: ::core::option::Option<
						runtime_types::pallet_referenda::types::DecidingStatus<_2>,
					>,
					pub tally: _5,
					pub in_queue: ::core::primitive::bool,
					pub alarm: ::core::option::Option<(_2, _7)>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TrackInfo<_0, _1> {
					pub name: ::std::string::String,
					pub max_deciding: _1,
					pub decision_deposit: _0,
					pub prepare_period: _1,
					pub decision_period: _1,
					pub confirm_period: _1,
					pub min_enactment_period: _1,
					pub min_approval: runtime_types::pallet_referenda::types::Curve,
					pub min_support: runtime_types::pallet_referenda::types::Curve,
				}
			}
		}
		pub mod pallet_scheduler {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					schedule {
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					cancel { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 2)]
					schedule_named {
						id: [::core::primitive::u8; 32usize],
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
					},
					#[codec(index = 3)]
					cancel_named { id: [::core::primitive::u8; 32usize] },
					#[codec(index = 4)]
					schedule_after {
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
					},
					#[codec(index = 5)]
					schedule_named_after {
						id: [::core::primitive::u8; 32usize],
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					FailedToSchedule,
					#[codec(index = 1)]
					NotFound,
					#[codec(index = 2)]
					TargetBlockNumberInPast,
					#[codec(index = 3)]
					RescheduleNoChange,
					#[codec(index = 4)]
					Named,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Scheduled { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 1)]
					Canceled { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 2)]
					Dispatched {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 3)]
					CallUnavailable {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 4)]
					PeriodicFailed {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 5)]
					PermanentlyOverweight {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Scheduled<_0, _1, _2, _3, _4> {
				pub maybe_id: ::core::option::Option<_0>,
				pub priority: ::core::primitive::u8,
				pub call: _1,
				pub maybe_periodic: ::core::option::Option<(_2, _2)>,
				pub origin: _3,
				#[codec(skip)]
				pub __subxt_unused_type_params: ::core::marker::PhantomData<_4>,
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					set_keys {
						keys: runtime_types::polkadot_runtime::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					purge_keys,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InvalidProof,
					#[codec(index = 1)]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					DuplicatedKey,
					#[codec(index = 3)]
					NoKeys,
					#[codec(index = 4)]
					NoAccount,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					NewSession { session_index: ::core::primitive::u32 },
				}
			}
		}
		pub mod pallet_staking {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						#[codec(index = 0)]
						bond {
							#[codec(compact)]
							value: ::core::primitive::u128,
							payee: runtime_types::pallet_staking::RewardDestination<
								::subxt::utils::AccountId32,
							>,
						},
						#[codec(index = 1)]
						bond_extra {
							#[codec(compact)]
							max_additional: ::core::primitive::u128,
						},
						#[codec(index = 2)]
						unbond {
							#[codec(compact)]
							value: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						withdraw_unbonded { num_slashing_spans: ::core::primitive::u32 },
						#[codec(index = 4)]
						validate { prefs: runtime_types::pallet_staking::ValidatorPrefs },
						#[codec(index = 5)]
						nominate {
							targets: ::std::vec::Vec<
								::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
							>,
						},
						#[codec(index = 6)]
						chill,
						#[codec(index = 7)]
						set_payee {
							payee: runtime_types::pallet_staking::RewardDestination<
								::subxt::utils::AccountId32,
							>,
						},
						#[codec(index = 8)]
						set_controller,
						#[codec(index = 9)]
						set_validator_count {
							#[codec(compact)]
							new: ::core::primitive::u32,
						},
						#[codec(index = 10)]
						increase_validator_count {
							#[codec(compact)]
							additional: ::core::primitive::u32,
						},
						#[codec(index = 11)]
						scale_validator_count {
							factor: runtime_types::sp_arithmetic::per_things::Percent,
						},
						#[codec(index = 12)]
						force_no_eras,
						#[codec(index = 13)]
						force_new_era,
						#[codec(index = 14)]
						set_invulnerables {
							invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
						},
						#[codec(index = 15)]
						force_unstake {
							stash: ::subxt::utils::AccountId32,
							num_slashing_spans: ::core::primitive::u32,
						},
						#[codec(index = 16)]
						force_new_era_always,
						#[codec(index = 17)]
						cancel_deferred_slash {
							era: ::core::primitive::u32,
							slash_indices: ::std::vec::Vec<::core::primitive::u32>,
						},
						#[codec(index = 18)]
						payout_stakers {
							validator_stash: ::subxt::utils::AccountId32,
							era: ::core::primitive::u32,
						},
						#[codec(index = 19)]
						rebond {
							#[codec(compact)]
							value: ::core::primitive::u128,
						},
						#[codec(index = 20)]
						reap_stash {
							stash: ::subxt::utils::AccountId32,
							num_slashing_spans: ::core::primitive::u32,
						},
						#[codec(index = 21)]
						kick {
							who: ::std::vec::Vec<
								::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
							>,
						},
						#[codec(index = 22)]
						set_staking_configs {
							min_nominator_bond:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u128,
								>,
							min_validator_bond:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u128,
								>,
							max_nominator_count:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u32,
								>,
							max_validator_count:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u32,
								>,
							chill_threshold:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									runtime_types::sp_arithmetic::per_things::Percent,
								>,
							min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
								runtime_types::sp_arithmetic::per_things::Perbill,
							>,
						},
						#[codec(index = 23)]
						chill_other { controller: ::subxt::utils::AccountId32 },
						#[codec(index = 24)]
						force_apply_min_commission { validator_stash: ::subxt::utils::AccountId32 },
						#[codec(index = 25)]
						set_min_commission {
							new: runtime_types::sp_arithmetic::per_things::Perbill,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum ConfigOp<_0> {
						#[codec(index = 0)]
						Noop,
						#[codec(index = 1)]
						Set(_0),
						#[codec(index = 2)]
						Remove,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						NotController,
						#[codec(index = 1)]
						NotStash,
						#[codec(index = 2)]
						AlreadyBonded,
						#[codec(index = 3)]
						AlreadyPaired,
						#[codec(index = 4)]
						EmptyTargets,
						#[codec(index = 5)]
						DuplicateIndex,
						#[codec(index = 6)]
						InvalidSlashIndex,
						#[codec(index = 7)]
						InsufficientBond,
						#[codec(index = 8)]
						NoMoreChunks,
						#[codec(index = 9)]
						NoUnlockChunk,
						#[codec(index = 10)]
						FundedTarget,
						#[codec(index = 11)]
						InvalidEraToReward,
						#[codec(index = 12)]
						InvalidNumberOfNominations,
						#[codec(index = 13)]
						NotSortedAndUnique,
						#[codec(index = 14)]
						AlreadyClaimed,
						#[codec(index = 15)]
						IncorrectHistoryDepth,
						#[codec(index = 16)]
						IncorrectSlashingSpans,
						#[codec(index = 17)]
						BadState,
						#[codec(index = 18)]
						TooManyTargets,
						#[codec(index = 19)]
						BadTarget,
						#[codec(index = 20)]
						CannotChillOther,
						#[codec(index = 21)]
						TooManyNominators,
						#[codec(index = 22)]
						TooManyValidators,
						#[codec(index = 23)]
						CommissionTooLow,
						#[codec(index = 24)]
						BoundNotMet,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Event {
						#[codec(index = 0)]
						EraPaid {
							era_index: ::core::primitive::u32,
							validator_payout: ::core::primitive::u128,
							remainder: ::core::primitive::u128,
						},
						#[codec(index = 1)]
						Rewarded {
							stash: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 2)]
						Slashed {
							staker: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						SlashReported {
							validator: ::subxt::utils::AccountId32,
							fraction: runtime_types::sp_arithmetic::per_things::Perbill,
							slash_era: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						OldSlashingReportDiscarded { session_index: ::core::primitive::u32 },
						#[codec(index = 5)]
						StakersElected,
						#[codec(index = 6)]
						Bonded {
							stash: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 7)]
						Unbonded {
							stash: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 8)]
						Withdrawn {
							stash: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 9)]
						Kicked {
							nominator: ::subxt::utils::AccountId32,
							stash: ::subxt::utils::AccountId32,
						},
						#[codec(index = 10)]
						StakingElectionFailed,
						#[codec(index = 11)]
						Chilled { stash: ::subxt::utils::AccountId32 },
						#[codec(index = 12)]
						PayoutStarted {
							era_index: ::core::primitive::u32,
							validator_stash: ::subxt::utils::AccountId32,
						},
						#[codec(index = 13)]
						ValidatorPrefsSet {
							stash: ::subxt::utils::AccountId32,
							prefs: runtime_types::pallet_staking::ValidatorPrefs,
						},
						#[codec(index = 14)]
						ForceEra { mode: runtime_types::pallet_staking::Forcing },
					}
				}
			}
			pub mod slashing {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SlashingSpans {
					pub span_index: ::core::primitive::u32,
					pub last_start: ::core::primitive::u32,
					pub last_nonzero_slash: ::core::primitive::u32,
					pub prior: ::std::vec::Vec<::core::primitive::u32>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SpanRecord<_0> {
					pub slashed: _0,
					pub paid_out: _0,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ActiveEraInfo {
				pub index: ::core::primitive::u32,
				pub start: ::core::option::Option<::core::primitive::u64>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EraRewardPoints<_0> {
				pub total: ::core::primitive::u32,
				pub individual: ::subxt::utils::KeyedVec<_0, ::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Exposure<_0, _1> {
				#[codec(compact)]
				pub total: _1,
				#[codec(compact)]
				pub own: _1,
				pub others:
					::std::vec::Vec<runtime_types::pallet_staking::IndividualExposure<_0, _1>>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Forcing {
				#[codec(index = 0)]
				NotForcing,
				#[codec(index = 1)]
				ForceNew,
				#[codec(index = 2)]
				ForceNone,
				#[codec(index = 3)]
				ForceAlways,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct IndividualExposure<_0, _1> {
				pub who: _0,
				#[codec(compact)]
				pub value: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Nominations {
				pub targets: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::subxt::utils::AccountId32,
				>,
				pub submitted_in: ::core::primitive::u32,
				pub suppressed: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RewardDestination<_0> {
				#[codec(index = 0)]
				Staked,
				#[codec(index = 1)]
				Stash,
				#[codec(index = 2)]
				Controller,
				#[codec(index = 3)]
				Account(_0),
				#[codec(index = 4)]
				None,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct StakingLedger {
				pub stash: ::subxt::utils::AccountId32,
				#[codec(compact)]
				pub total: ::core::primitive::u128,
				#[codec(compact)]
				pub active: ::core::primitive::u128,
				pub unlocking: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					runtime_types::pallet_staking::UnlockChunk<::core::primitive::u128>,
				>,
				pub claimed_rewards: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnappliedSlash<_0, _1> {
				pub validator: _0,
				pub own: _1,
				pub others: ::std::vec::Vec<(_0, _1)>,
				pub reporters: ::std::vec::Vec<_0>,
				pub payout: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnlockChunk<_0> {
				#[codec(compact)]
				pub value: _0,
				#[codec(compact)]
				pub era: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ValidatorPrefs {
				#[codec(compact)]
				pub commission: runtime_types::sp_arithmetic::per_things::Perbill,
				pub blocked: ::core::primitive::bool,
			}
		}
		pub mod pallet_timestamp {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					set {
						#[codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_tips {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					report_awesome {
						reason: ::std::vec::Vec<::core::primitive::u8>,
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 1)]
					retract_tip { hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					tip_new {
						reason: ::std::vec::Vec<::core::primitive::u8>,
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						tip_value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					tip {
						hash: ::subxt::utils::H256,
						#[codec(compact)]
						tip_value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					close_tip { hash: ::subxt::utils::H256 },
					#[codec(index = 5)]
					slash_tip { hash: ::subxt::utils::H256 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					ReasonTooBig,
					#[codec(index = 1)]
					AlreadyKnown,
					#[codec(index = 2)]
					UnknownTip,
					#[codec(index = 3)]
					NotFinder,
					#[codec(index = 4)]
					StillOpen,
					#[codec(index = 5)]
					Premature,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					NewTip { tip_hash: ::subxt::utils::H256 },
					#[codec(index = 1)]
					TipClosing { tip_hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					TipClosed {
						tip_hash: ::subxt::utils::H256,
						who: ::subxt::utils::AccountId32,
						payout: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					TipRetracted { tip_hash: ::subxt::utils::H256 },
					#[codec(index = 4)]
					TipSlashed {
						tip_hash: ::subxt::utils::H256,
						finder: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OpenTip<_0, _1, _2, _3> {
				pub reason: _3,
				pub who: _0,
				pub finder: _0,
				pub deposit: _1,
				pub closes: ::core::option::Option<_2>,
				pub tips: ::std::vec::Vec<(_0, _1)>,
				pub finders_fee: ::core::primitive::bool,
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					TransactionFeePaid {
						who: ::subxt::utils::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Releases {
				#[codec(index = 0)]
				V1Ancient,
				#[codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_treasury {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					propose_spend {
						#[codec(compact)]
						value: ::core::primitive::u128,
						beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 1)]
					reject_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					approve_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					spend {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 4)]
					remove_approval {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InsufficientProposersBalance,
					#[codec(index = 1)]
					InvalidIndex,
					#[codec(index = 2)]
					TooManyApprovals,
					#[codec(index = 3)]
					InsufficientPermission,
					#[codec(index = 4)]
					ProposalNotApproved,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Proposed { proposal_index: ::core::primitive::u32 },
					#[codec(index = 1)]
					Spending { budget_remaining: ::core::primitive::u128 },
					#[codec(index = 2)]
					Awarded {
						proposal_index: ::core::primitive::u32,
						award: ::core::primitive::u128,
						account: ::subxt::utils::AccountId32,
					},
					#[codec(index = 3)]
					Rejected {
						proposal_index: ::core::primitive::u32,
						slashed: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					Burnt { burnt_funds: ::core::primitive::u128 },
					#[codec(index = 5)]
					Rollover { rollover_balance: ::core::primitive::u128 },
					#[codec(index = 6)]
					Deposit { value: ::core::primitive::u128 },
					#[codec(index = 7)]
					SpendApproved {
						proposal_index: ::core::primitive::u32,
						amount: ::core::primitive::u128,
						beneficiary: ::subxt::utils::AccountId32,
					},
					#[codec(index = 8)]
					UpdatedInactive {
						reactivated: ::core::primitive::u128,
						deactivated: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Proposal<_0, _1> {
				pub proposer: _0,
				pub value: _1,
				pub beneficiary: _0,
				pub bond: _1,
			}
		}
		pub mod pallet_utility {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					batch { calls: ::std::vec::Vec<runtime_types::polkadot_runtime::RuntimeCall> },
					#[codec(index = 1)]
					as_derivative {
						index: ::core::primitive::u16,
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
					},
					#[codec(index = 2)]
					batch_all {
						calls: ::std::vec::Vec<runtime_types::polkadot_runtime::RuntimeCall>,
					},
					#[codec(index = 3)]
					dispatch_as {
						as_origin: ::std::boxed::Box<runtime_types::polkadot_runtime::OriginCaller>,
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
					},
					#[codec(index = 4)]
					force_batch {
						calls: ::std::vec::Vec<runtime_types::polkadot_runtime::RuntimeCall>,
					},
					#[codec(index = 5)]
					with_weight {
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					TooManyCalls,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					BatchInterrupted {
						index: ::core::primitive::u32,
						error: runtime_types::sp_runtime::DispatchError,
					},
					#[codec(index = 1)]
					BatchCompleted,
					#[codec(index = 2)]
					BatchCompletedWithErrors,
					#[codec(index = 3)]
					ItemCompleted,
					#[codec(index = 4)]
					ItemFailed { error: runtime_types::sp_runtime::DispatchError },
					#[codec(index = 5)]
					DispatchedAs {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_vesting {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					vest,
					#[codec(index = 1)]
					vest_other {
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 2)]
					vested_transfer {
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
							::core::primitive::u128,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					force_vested_transfer {
						source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
							::core::primitive::u128,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 4)]
					merge_schedules {
						schedule1_index: ::core::primitive::u32,
						schedule2_index: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					NotVesting,
					#[codec(index = 1)]
					AtMaxVestingSchedules,
					#[codec(index = 2)]
					AmountLow,
					#[codec(index = 3)]
					ScheduleIndexOutOfBounds,
					#[codec(index = 4)]
					InvalidScheduleParams,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					VestingUpdated {
						account: ::subxt::utils::AccountId32,
						unvested: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					VestingCompleted { account: ::subxt::utils::AccountId32 },
				}
			}
			pub mod vesting_info {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct VestingInfo<_0, _1> {
					pub locked: _0,
					pub per_block: _0,
					pub starting_block: _1,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Releases {
				#[codec(index = 0)]
				V0,
				#[codec(index = 1)]
				V1,
			}
		}
		pub mod pallet_whitelist {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					whitelist_call { call_hash: ::subxt::utils::H256 },
					#[codec(index = 1)]
					remove_whitelisted_call { call_hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					dispatch_whitelisted_call {
						call_hash: ::subxt::utils::H256,
						call_encoded_len: ::core::primitive::u32,
						call_weight_witness: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 3)]
					dispatch_whitelisted_call_with_preimage {
						call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					UnavailablePreImage,
					#[codec(index = 1)]
					UndecodableCall,
					#[codec(index = 2)]
					InvalidCallWeightWitness,
					#[codec(index = 3)]
					CallIsNotWhitelisted,
					#[codec(index = 4)]
					CallAlreadyWhitelisted,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					CallWhitelisted { call_hash: ::subxt::utils::H256 },
					#[codec(index = 1)]
					WhitelistedCallRemoved { call_hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					WhitelistedCallDispatched {
						call_hash: ::subxt::utils::H256,
						result: ::core::result::Result<
							runtime_types::frame_support::dispatch::PostDispatchInfo,
							runtime_types::sp_runtime::DispatchErrorWithPostInfo<
								runtime_types::frame_support::dispatch::PostDispatchInfo,
							>,
						>,
					},
				}
			}
		}
		pub mod pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					send {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
					},
					#[codec(index = 1)]
					teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					execute {
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 4)]
					force_xcm_version {
						location:
							::std::boxed::Box<runtime_types::xcm::v3::multilocation::MultiLocation>,
						xcm_version: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					force_default_xcm_version {
						maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 6)]
					force_subscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 7)]
					force_unsubscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 8)]
					limited_reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 9)]
					limited_teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 10)]
					force_suspension { suspended: ::core::primitive::bool },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					Unreachable,
					#[codec(index = 1)]
					SendFailure,
					#[codec(index = 2)]
					Filtered,
					#[codec(index = 3)]
					UnweighableMessage,
					#[codec(index = 4)]
					DestinationNotInvertible,
					#[codec(index = 5)]
					Empty,
					#[codec(index = 6)]
					CannotReanchor,
					#[codec(index = 7)]
					TooManyAssets,
					#[codec(index = 8)]
					InvalidOrigin,
					#[codec(index = 9)]
					BadVersion,
					#[codec(index = 10)]
					BadLocation,
					#[codec(index = 11)]
					NoSubscription,
					#[codec(index = 12)]
					AlreadySubscribed,
					#[codec(index = 13)]
					InvalidAsset,
					#[codec(index = 14)]
					LowBalance,
					#[codec(index = 15)]
					TooManyLocks,
					#[codec(index = 16)]
					AccountNotSovereign,
					#[codec(index = 17)]
					FeesNotMet,
					#[codec(index = 18)]
					LockNotFound,
					#[codec(index = 19)]
					InUse,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Attempted(runtime_types::xcm::v3::traits::Outcome),
					#[codec(index = 1)]
					Sent(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::Xcm,
					),
					#[codec(index = 2)]
					UnexpectedResponse(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 3)]
					ResponseReady(::core::primitive::u64, runtime_types::xcm::v3::Response),
					#[codec(index = 4)]
					Notified(::core::primitive::u64, ::core::primitive::u8, ::core::primitive::u8),
					#[codec(index = 5)]
					NotifyOverweight(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
						runtime_types::sp_weights::weight_v2::Weight,
						runtime_types::sp_weights::weight_v2::Weight,
					),
					#[codec(index = 6)]
					NotifyDispatchError(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 7)]
					NotifyDecodeFailed(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 8)]
					InvalidResponder(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 9)]
					InvalidResponderVersion(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 10)]
					ResponseTaken(::core::primitive::u64),
					#[codec(index = 11)]
					AssetsTrapped(
						::subxt::utils::H256,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
					#[codec(index = 12)]
					VersionChangeNotified(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u32,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 13)]
					SupportedVersionChanged(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec(index = 14)]
					NotifyTargetSendFail(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v3::traits::Error,
					),
					#[codec(index = 15)]
					NotifyTargetMigrationFail(
						runtime_types::xcm::VersionedMultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 16)]
					InvalidQuerierVersion(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 17)]
					InvalidQuerier(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 18)]
					VersionNotifyStarted(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 19)]
					VersionNotifyRequested(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 20)]
					VersionNotifyUnrequested(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 21)]
					FeesPaid(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 22)]
					AssetsClaimed(
						::subxt::utils::H256,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Origin {
					#[codec(index = 0)]
					Xcm(runtime_types::xcm::v3::multilocation::MultiLocation),
					#[codec(index = 1)]
					Response(runtime_types::xcm::v3::multilocation::MultiLocation),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum QueryStatus<_0> {
					#[codec(index = 0)]
					Pending {
						responder: runtime_types::xcm::VersionedMultiLocation,
						maybe_match_querier:
							::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
						maybe_notify:
							::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
						timeout: _0,
					},
					#[codec(index = 1)]
					VersionNotifier {
						origin: runtime_types::xcm::VersionedMultiLocation,
						is_active: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					Ready { response: runtime_types::xcm::VersionedResponse, at: _0 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemoteLockedFungibleRecord<_0> {
					pub amount: ::core::primitive::u128,
					pub owner: runtime_types::xcm::VersionedMultiLocation,
					pub locker: runtime_types::xcm::VersionedMultiLocation,
					pub consumers: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						_0,
						::core::primitive::u128,
					)>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum VersionMigrationStage {
					#[codec(index = 0)]
					MigrateSupportedVersion,
					#[codec(index = 1)]
					MigrateVersionNotifiers,
					#[codec(index = 2)]
					NotifyCurrentTargets(
						::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					),
					#[codec(index = 3)]
					MigrateAndNotifyOldTargets,
				}
			}
		}
		pub mod polkadot_core_primitives {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CandidateHash(pub ::subxt::utils::H256);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OutboundHrmpMessage<_0> {
				pub recipient: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod polkadot_parachain {
			use super::runtime_types;
			pub mod primitives {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct HrmpChannelId {
					pub sender: runtime_types::polkadot_parachain::primitives::Id,
					pub recipient: runtime_types::polkadot_parachain::primitives::Id,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Id(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ValidationCode(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ValidationCodeHash(pub ::subxt::utils::H256);
			}
		}
		pub mod polkadot_primitives {
			use super::runtime_types;
			pub mod v4 {
				use super::runtime_types;
				pub mod assignment_app {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
				}
				pub mod collator_app {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
				}
				pub mod executor_params {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum ExecutorParam {
						#[codec(index = 1)]
						MaxMemoryPages(::core::primitive::u32),
						#[codec(index = 2)]
						StackLogicalMax(::core::primitive::u32),
						#[codec(index = 3)]
						StackNativeMax(::core::primitive::u32),
						#[codec(index = 4)]
						PrecheckingMaxMemory(::core::primitive::u64),
						#[codec(index = 5)]
						PvfPrepTimeout(
							runtime_types::polkadot_primitives::v4::PvfPrepTimeoutKind,
							::core::primitive::u64,
						),
						#[codec(index = 6)]
						PvfExecTimeout(
							runtime_types::polkadot_primitives::v4::PvfExecTimeoutKind,
							::core::primitive::u64,
						),
						#[codec(index = 7)]
						WasmExtBulkMemory,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct ExecutorParams(
						pub  ::std::vec::Vec<
							runtime_types::polkadot_primitives::v4::executor_params::ExecutorParam,
						>,
					);
				}
				pub mod signed {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct UncheckedSigned<_0, _1> {
						pub payload: _0,
						pub validator_index: runtime_types::polkadot_primitives::v4::ValidatorIndex,
						pub signature:
							runtime_types::polkadot_primitives::v4::validator_app::Signature,
						#[codec(skip)]
						pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
					}
				}
				pub mod validator_app {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AvailabilityBitfield(
					pub  ::subxt::utils::bits::DecodedBits<
						::core::primitive::u8,
						::subxt::utils::bits::Lsb0,
					>,
				);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BackedCandidate<_0> {
					pub candidate:
						runtime_types::polkadot_primitives::v4::CommittedCandidateReceipt<_0>,
					pub validity_votes: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v4::ValidityAttestation,
					>,
					pub validator_indices: ::subxt::utils::bits::DecodedBits<
						::core::primitive::u8,
						::subxt::utils::bits::Lsb0,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CandidateCommitments<_0> {
					pub upward_messages:
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::std::vec::Vec<::core::primitive::u8>,
						>,
					pub horizontal_messages:
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
								runtime_types::polkadot_parachain::primitives::Id,
							>,
						>,
					pub new_validation_code: ::core::option::Option<
						runtime_types::polkadot_parachain::primitives::ValidationCode,
					>,
					pub head_data: runtime_types::polkadot_parachain::primitives::HeadData,
					pub processed_downward_messages: _0,
					pub hrmp_watermark: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CandidateDescriptor<_0> {
					pub para_id: runtime_types::polkadot_parachain::primitives::Id,
					pub relay_parent: _0,
					pub collator: runtime_types::polkadot_primitives::v4::collator_app::Public,
					pub persisted_validation_data_hash: _0,
					pub pov_hash: _0,
					pub erasure_root: _0,
					pub signature: runtime_types::polkadot_primitives::v4::collator_app::Signature,
					pub para_head: _0,
					pub validation_code_hash:
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CandidateReceipt<_0> {
					pub descriptor: runtime_types::polkadot_primitives::v4::CandidateDescriptor<_0>,
					pub commitments_hash: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CommittedCandidateReceipt<_0> {
					pub descriptor: runtime_types::polkadot_primitives::v4::CandidateDescriptor<_0>,
					pub commitments: runtime_types::polkadot_primitives::v4::CandidateCommitments<
						::core::primitive::u32,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CoreIndex(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum CoreOccupied {
					#[codec(index = 0)]
					Parathread(runtime_types::polkadot_primitives::v4::ParathreadEntry),
					#[codec(index = 1)]
					Parachain,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DisputeState<_0> {
					pub validators_for: ::subxt::utils::bits::DecodedBits<
						::core::primitive::u8,
						::subxt::utils::bits::Lsb0,
					>,
					pub validators_against: ::subxt::utils::bits::DecodedBits<
						::core::primitive::u8,
						::subxt::utils::bits::Lsb0,
					>,
					pub start: _0,
					pub concluded_at: ::core::option::Option<_0>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum DisputeStatement {
					#[codec(index = 0)]
					Valid(runtime_types::polkadot_primitives::v4::ValidDisputeStatementKind),
					#[codec(index = 1)]
					Invalid(runtime_types::polkadot_primitives::v4::InvalidDisputeStatementKind),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DisputeStatementSet {
					pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
					pub session: ::core::primitive::u32,
					pub statements: ::std::vec::Vec<(
						runtime_types::polkadot_primitives::v4::DisputeStatement,
						runtime_types::polkadot_primitives::v4::ValidatorIndex,
						runtime_types::polkadot_primitives::v4::validator_app::Signature,
					)>,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct GroupIndex(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct IndexedVec<_0, _1>(
					pub ::std::vec::Vec<_1>,
					#[codec(skip)] pub ::core::marker::PhantomData<_0>,
				);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct InherentData<_0> {
					pub bitfields: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v4::signed::UncheckedSigned<
							runtime_types::polkadot_primitives::v4::AvailabilityBitfield,
							runtime_types::polkadot_primitives::v4::AvailabilityBitfield,
						>,
					>,
					pub backed_candidates: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v4::BackedCandidate<
							::subxt::utils::H256,
						>,
					>,
					pub disputes: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v4::DisputeStatementSet,
					>,
					pub parent_header: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum InvalidDisputeStatementKind {
					#[codec(index = 0)]
					Explicit,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ParathreadClaim(
					pub runtime_types::polkadot_parachain::primitives::Id,
					pub runtime_types::polkadot_primitives::v4::collator_app::Public,
				);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ParathreadEntry {
					pub claim: runtime_types::polkadot_primitives::v4::ParathreadClaim,
					pub retries: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PvfCheckStatement {
					pub accept: ::core::primitive::bool,
					pub subject: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					pub session_index: ::core::primitive::u32,
					pub validator_index: runtime_types::polkadot_primitives::v4::ValidatorIndex,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum PvfExecTimeoutKind {
					#[codec(index = 0)]
					Backing,
					#[codec(index = 1)]
					Approval,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum PvfPrepTimeoutKind {
					#[codec(index = 0)]
					Precheck,
					#[codec(index = 1)]
					Lenient,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ScrapedOnChainVotes<_0> {
					pub session: ::core::primitive::u32,
					pub backing_validators_per_candidate: ::std::vec::Vec<(
						runtime_types::polkadot_primitives::v4::CandidateReceipt<_0>,
						::std::vec::Vec<(
							runtime_types::polkadot_primitives::v4::ValidatorIndex,
							runtime_types::polkadot_primitives::v4::ValidityAttestation,
						)>,
					)>,
					pub disputes: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v4::DisputeStatementSet,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SessionInfo {
					pub active_validator_indices:
						::std::vec::Vec<runtime_types::polkadot_primitives::v4::ValidatorIndex>,
					pub random_seed: [::core::primitive::u8; 32usize],
					pub dispute_period: ::core::primitive::u32,
					pub validators: runtime_types::polkadot_primitives::v4::IndexedVec<
						runtime_types::polkadot_primitives::v4::ValidatorIndex,
						runtime_types::polkadot_primitives::v4::validator_app::Public,
					>,
					pub discovery_keys:
						::std::vec::Vec<runtime_types::sp_authority_discovery::app::Public>,
					pub assignment_keys: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v4::assignment_app::Public,
					>,
					pub validator_groups: runtime_types::polkadot_primitives::v4::IndexedVec<
						runtime_types::polkadot_primitives::v4::GroupIndex,
						::std::vec::Vec<runtime_types::polkadot_primitives::v4::ValidatorIndex>,
					>,
					pub n_cores: ::core::primitive::u32,
					pub zeroth_delay_tranche_width: ::core::primitive::u32,
					pub relay_vrf_modulo_samples: ::core::primitive::u32,
					pub n_delay_tranches: ::core::primitive::u32,
					pub no_show_slots: ::core::primitive::u32,
					pub needed_approvals: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum UpgradeGoAhead {
					#[codec(index = 0)]
					Abort,
					#[codec(index = 1)]
					GoAhead,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum UpgradeRestriction {
					#[codec(index = 0)]
					Present,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum ValidDisputeStatementKind {
					#[codec(index = 0)]
					Explicit,
					#[codec(index = 1)]
					BackingSeconded(::subxt::utils::H256),
					#[codec(index = 2)]
					BackingValid(::subxt::utils::H256),
					#[codec(index = 3)]
					ApprovalChecking,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ValidatorIndex(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum ValidityAttestation {
					#[codec(index = 1)]
					Implicit(runtime_types::polkadot_primitives::v4::validator_app::Signature),
					#[codec(index = 2)]
					Explicit(runtime_types::polkadot_primitives::v4::validator_app::Signature),
				}
			}
			pub mod vstaging {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AsyncBackingParams {
					pub max_candidate_depth: ::core::primitive::u32,
					pub allowed_ancestry_len: ::core::primitive::u32,
				}
			}
		}
		pub mod polkadot_runtime {
			use super::runtime_types;
			pub mod governance {
				use super::runtime_types;
				pub mod origins {
					use super::runtime_types;
					pub mod pallet_custom_origins {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub enum Origin {
							#[codec(index = 0)]
							StakingAdmin,
							#[codec(index = 1)]
							Treasurer,
							#[codec(index = 2)]
							FellowshipAdmin,
							#[codec(index = 3)]
							GeneralAdmin,
							#[codec(index = 4)]
							AuctionAdmin,
							#[codec(index = 5)]
							LeaseAdmin,
							#[codec(index = 6)]
							ReferendumCanceller,
							#[codec(index = 7)]
							ReferendumKiller,
							#[codec(index = 8)]
							SmallTipper,
							#[codec(index = 9)]
							BigTipper,
							#[codec(index = 10)]
							SmallSpender,
							#[codec(index = 11)]
							MediumSpender,
							#[codec(index = 12)]
							BigSpender,
							#[codec(index = 13)]
							WhitelistedCaller,
						}
					}
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NposCompactSolution16 {
				pub votes1: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes2: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					),
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes3: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 2usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes4: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 3usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes5: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 4usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes6: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 5usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes7: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 6usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes8: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 7usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes9: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 8usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes10: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 9usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes11: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 10usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes12: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 11usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes13: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 12usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes14: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 13usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes15: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 14usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
				pub votes16: ::std::vec::Vec<(
					::subxt::ext::codec::Compact<::core::primitive::u32>,
					[(
						::subxt::ext::codec::Compact<::core::primitive::u16>,
						::subxt::ext::codec::Compact<
							runtime_types::sp_arithmetic::per_things::PerU16,
						>,
					); 15usize],
					::subxt::ext::codec::Compact<::core::primitive::u16>,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum OriginCaller {
				# [codec (index = 0)] system (runtime_types :: frame_support :: dispatch :: RawOrigin < :: subxt :: utils :: AccountId32 > ,) , # [codec (index = 15)] Council (runtime_types :: pallet_collective :: RawOrigin < :: subxt :: utils :: AccountId32 > ,) , # [codec (index = 16)] TechnicalCommittee (runtime_types :: pallet_collective :: RawOrigin < :: subxt :: utils :: AccountId32 > ,) , # [codec (index = 22)] Origins (runtime_types :: polkadot_runtime :: governance :: origins :: pallet_custom_origins :: Origin ,) , # [codec (index = 50)] ParachainsOrigin (runtime_types :: polkadot_runtime_parachains :: origin :: pallet :: Origin ,) , # [codec (index = 99)] XcmPallet (runtime_types :: pallet_xcm :: pallet :: Origin ,) , # [codec (index = 6)] Void (runtime_types :: sp_core :: Void ,) , }
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ProxyType {
				#[codec(index = 0)]
				Any,
				#[codec(index = 1)]
				NonTransfer,
				#[codec(index = 2)]
				Governance,
				#[codec(index = 3)]
				Staking,
				#[codec(index = 5)]
				IdentityJudgement,
				#[codec(index = 6)]
				CancelProxy,
				#[codec(index = 7)]
				Auction,
				#[codec(index = 8)]
				NominationPools,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Runtime;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeCall {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Call),
				#[codec(index = 10)]
				Preimage(runtime_types::pallet_preimage::pallet::Call),
				#[codec(index = 2)]
				Babe(runtime_types::pallet_babe::pallet::Call),
				#[codec(index = 3)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 4)]
				Indices(runtime_types::pallet_indices::pallet::Call),
				#[codec(index = 5)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 7)]
				Staking(runtime_types::pallet_staking::pallet::pallet::Call),
				#[codec(index = 9)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 11)]
				Grandpa(runtime_types::pallet_grandpa::pallet::Call),
				#[codec(index = 12)]
				ImOnline(runtime_types::pallet_im_online::pallet::Call),
				#[codec(index = 14)]
				Democracy(runtime_types::pallet_democracy::pallet::Call),
				#[codec(index = 15)]
				Council(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 16)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 17)]
				PhragmenElection(runtime_types::pallet_elections_phragmen::pallet::Call),
				#[codec(index = 18)]
				TechnicalMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 19)]
				Treasury(runtime_types::pallet_treasury::pallet::Call),
				#[codec(index = 20)]
				ConvictionVoting(runtime_types::pallet_conviction_voting::pallet::Call),
				#[codec(index = 21)]
				Referenda(runtime_types::pallet_referenda::pallet::Call),
				#[codec(index = 23)]
				Whitelist(runtime_types::pallet_whitelist::pallet::Call),
				#[codec(index = 24)]
				Claims(runtime_types::polkadot_runtime_common::claims::pallet::Call),
				#[codec(index = 25)]
				Vesting(runtime_types::pallet_vesting::pallet::Call),
				#[codec(index = 26)]
				Utility(runtime_types::pallet_utility::pallet::Call),
				#[codec(index = 28)]
				Identity(runtime_types::pallet_identity::pallet::Call),
				#[codec(index = 29)]
				Proxy(runtime_types::pallet_proxy::pallet::Call),
				#[codec(index = 30)]
				Multisig(runtime_types::pallet_multisig::pallet::Call),
				#[codec(index = 34)]
				Bounties(runtime_types::pallet_bounties::pallet::Call),
				#[codec(index = 38)]
				ChildBounties(runtime_types::pallet_child_bounties::pallet::Call),
				#[codec(index = 35)]
				Tips(runtime_types::pallet_tips::pallet::Call),
				#[codec(index = 36)]
				ElectionProviderMultiPhase(
					runtime_types::pallet_election_provider_multi_phase::pallet::Call,
				),
				#[codec(index = 37)]
				VoterList(runtime_types::pallet_bags_list::pallet::Call),
				#[codec(index = 39)]
				NominationPools(runtime_types::pallet_nomination_pools::pallet::Call),
				#[codec(index = 40)]
				FastUnstake(runtime_types::pallet_fast_unstake::pallet::Call),
				#[codec(index = 51)]
				Configuration(
					runtime_types::polkadot_runtime_parachains::configuration::pallet::Call,
				),
				#[codec(index = 52)]
				ParasShared(runtime_types::polkadot_runtime_parachains::shared::pallet::Call),
				#[codec(index = 53)]
				ParaInclusion(runtime_types::polkadot_runtime_parachains::inclusion::pallet::Call),
				#[codec(index = 54)]
				ParaInherent(
					runtime_types::polkadot_runtime_parachains::paras_inherent::pallet::Call,
				),
				#[codec(index = 56)]
				Paras(runtime_types::polkadot_runtime_parachains::paras::pallet::Call),
				#[codec(index = 57)]
				Initializer(runtime_types::polkadot_runtime_parachains::initializer::pallet::Call),
				#[codec(index = 60)]
				Hrmp(runtime_types::polkadot_runtime_parachains::hrmp::pallet::Call),
				#[codec(index = 62)]
				ParasDisputes(runtime_types::polkadot_runtime_parachains::disputes::pallet::Call),
				#[codec(index = 63)]
				ParasSlashing(
					runtime_types::polkadot_runtime_parachains::disputes::slashing::pallet::Call,
				),
				#[codec(index = 70)]
				Registrar(runtime_types::polkadot_runtime_common::paras_registrar::pallet::Call),
				#[codec(index = 71)]
				Slots(runtime_types::polkadot_runtime_common::slots::pallet::Call),
				#[codec(index = 72)]
				Auctions(runtime_types::polkadot_runtime_common::auctions::pallet::Call),
				#[codec(index = 73)]
				Crowdloan(runtime_types::polkadot_runtime_common::crowdloan::pallet::Call),
				#[codec(index = 99)]
				XcmPallet(runtime_types::pallet_xcm::pallet::Call),
				#[codec(index = 100)]
				MessageQueue(runtime_types::pallet_message_queue::pallet::Call),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeEvent {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 1)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Event),
				#[codec(index = 10)]
				Preimage(runtime_types::pallet_preimage::pallet::Event),
				#[codec(index = 4)]
				Indices(runtime_types::pallet_indices::pallet::Event),
				#[codec(index = 5)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 32)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 7)]
				Staking(runtime_types::pallet_staking::pallet::pallet::Event),
				#[codec(index = 8)]
				Offences(runtime_types::pallet_offences::pallet::Event),
				#[codec(index = 9)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 11)]
				Grandpa(runtime_types::pallet_grandpa::pallet::Event),
				#[codec(index = 12)]
				ImOnline(runtime_types::pallet_im_online::pallet::Event),
				#[codec(index = 14)]
				Democracy(runtime_types::pallet_democracy::pallet::Event),
				#[codec(index = 15)]
				Council(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 16)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 17)]
				PhragmenElection(runtime_types::pallet_elections_phragmen::pallet::Event),
				#[codec(index = 18)]
				TechnicalMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 19)]
				Treasury(runtime_types::pallet_treasury::pallet::Event),
				#[codec(index = 20)]
				ConvictionVoting(runtime_types::pallet_conviction_voting::pallet::Event),
				#[codec(index = 21)]
				Referenda(runtime_types::pallet_referenda::pallet::Event),
				#[codec(index = 23)]
				Whitelist(runtime_types::pallet_whitelist::pallet::Event),
				#[codec(index = 24)]
				Claims(runtime_types::polkadot_runtime_common::claims::pallet::Event),
				#[codec(index = 25)]
				Vesting(runtime_types::pallet_vesting::pallet::Event),
				#[codec(index = 26)]
				Utility(runtime_types::pallet_utility::pallet::Event),
				#[codec(index = 28)]
				Identity(runtime_types::pallet_identity::pallet::Event),
				#[codec(index = 29)]
				Proxy(runtime_types::pallet_proxy::pallet::Event),
				#[codec(index = 30)]
				Multisig(runtime_types::pallet_multisig::pallet::Event),
				#[codec(index = 34)]
				Bounties(runtime_types::pallet_bounties::pallet::Event),
				#[codec(index = 38)]
				ChildBounties(runtime_types::pallet_child_bounties::pallet::Event),
				#[codec(index = 35)]
				Tips(runtime_types::pallet_tips::pallet::Event),
				#[codec(index = 36)]
				ElectionProviderMultiPhase(
					runtime_types::pallet_election_provider_multi_phase::pallet::Event,
				),
				#[codec(index = 37)]
				VoterList(runtime_types::pallet_bags_list::pallet::Event),
				#[codec(index = 39)]
				NominationPools(runtime_types::pallet_nomination_pools::pallet::Event),
				#[codec(index = 40)]
				FastUnstake(runtime_types::pallet_fast_unstake::pallet::Event),
				#[codec(index = 53)]
				ParaInclusion(runtime_types::polkadot_runtime_parachains::inclusion::pallet::Event),
				#[codec(index = 56)]
				Paras(runtime_types::polkadot_runtime_parachains::paras::pallet::Event),
				#[codec(index = 60)]
				Hrmp(runtime_types::polkadot_runtime_parachains::hrmp::pallet::Event),
				#[codec(index = 62)]
				ParasDisputes(runtime_types::polkadot_runtime_parachains::disputes::pallet::Event),
				#[codec(index = 70)]
				Registrar(runtime_types::polkadot_runtime_common::paras_registrar::pallet::Event),
				#[codec(index = 71)]
				Slots(runtime_types::polkadot_runtime_common::slots::pallet::Event),
				#[codec(index = 72)]
				Auctions(runtime_types::polkadot_runtime_common::auctions::pallet::Event),
				#[codec(index = 73)]
				Crowdloan(runtime_types::polkadot_runtime_common::crowdloan::pallet::Event),
				#[codec(index = 99)]
				XcmPallet(runtime_types::pallet_xcm::pallet::Event),
				#[codec(index = 100)]
				MessageQueue(runtime_types::pallet_message_queue::pallet::Event),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SessionKeys {
				pub grandpa: runtime_types::sp_consensus_grandpa::app::Public,
				pub babe: runtime_types::sp_consensus_babe::app::Public,
				pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
				pub para_validator: runtime_types::polkadot_primitives::v4::validator_app::Public,
				pub para_assignment: runtime_types::polkadot_primitives::v4::assignment_app::Public,
				pub authority_discovery: runtime_types::sp_authority_discovery::app::Public,
			}
		}
		pub mod polkadot_runtime_common {
			use super::runtime_types;
			pub mod auctions {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						#[codec(index = 0)]
						new_auction {
							#[codec(compact)]
							duration: ::core::primitive::u32,
							#[codec(compact)]
							lease_period_index: ::core::primitive::u32,
						},
						#[codec(index = 1)]
						bid {
							#[codec(compact)]
							para: runtime_types::polkadot_parachain::primitives::Id,
							#[codec(compact)]
							auction_index: ::core::primitive::u32,
							#[codec(compact)]
							first_slot: ::core::primitive::u32,
							#[codec(compact)]
							last_slot: ::core::primitive::u32,
							#[codec(compact)]
							amount: ::core::primitive::u128,
						},
						#[codec(index = 2)]
						cancel_auction,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						AuctionInProgress,
						#[codec(index = 1)]
						LeasePeriodInPast,
						#[codec(index = 2)]
						ParaNotRegistered,
						#[codec(index = 3)]
						NotCurrentAuction,
						#[codec(index = 4)]
						NotAuction,
						#[codec(index = 5)]
						AuctionEnded,
						#[codec(index = 6)]
						AlreadyLeasedOut,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Event {
						#[codec(index = 0)]
						AuctionStarted {
							auction_index: ::core::primitive::u32,
							lease_period: ::core::primitive::u32,
							ending: ::core::primitive::u32,
						},
						#[codec(index = 1)]
						AuctionClosed { auction_index: ::core::primitive::u32 },
						#[codec(index = 2)]
						Reserved {
							bidder: ::subxt::utils::AccountId32,
							extra_reserved: ::core::primitive::u128,
							total_amount: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						Unreserved {
							bidder: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 4)]
						ReserveConfiscated {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							leaser: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 5)]
						BidAccepted {
							bidder: ::subxt::utils::AccountId32,
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							amount: ::core::primitive::u128,
							first_slot: ::core::primitive::u32,
							last_slot: ::core::primitive::u32,
						},
						#[codec(index = 6)]
						WinningOffset {
							auction_index: ::core::primitive::u32,
							block_number: ::core::primitive::u32,
						},
					}
				}
			}
			pub mod claims {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						#[codec(index = 0)]
						claim {
							dest: ::subxt::utils::AccountId32,
							ethereum_signature:
								runtime_types::polkadot_runtime_common::claims::EcdsaSignature,
						},
						#[codec(index = 1)]
						mint_claim {
							who: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
							value: ::core::primitive::u128,
							vesting_schedule: ::core::option::Option<(
								::core::primitive::u128,
								::core::primitive::u128,
								::core::primitive::u32,
							)>,
							statement: ::core::option::Option<
								runtime_types::polkadot_runtime_common::claims::StatementKind,
							>,
						},
						#[codec(index = 2)]
						claim_attest {
							dest: ::subxt::utils::AccountId32,
							ethereum_signature:
								runtime_types::polkadot_runtime_common::claims::EcdsaSignature,
							statement: ::std::vec::Vec<::core::primitive::u8>,
						},
						#[codec(index = 3)]
						attest { statement: ::std::vec::Vec<::core::primitive::u8> },
						#[codec(index = 4)]
						move_claim {
							old: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
							new: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
							maybe_preclaim: ::core::option::Option<::subxt::utils::AccountId32>,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						InvalidEthereumSignature,
						#[codec(index = 1)]
						SignerHasNoClaim,
						#[codec(index = 2)]
						SenderHasNoClaim,
						#[codec(index = 3)]
						PotUnderflow,
						#[codec(index = 4)]
						InvalidStatement,
						#[codec(index = 5)]
						VestedBalanceExists,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Event {
						#[codec(index = 0)]
						Claimed {
							who: ::subxt::utils::AccountId32,
							ethereum_address:
								runtime_types::polkadot_runtime_common::claims::EthereumAddress,
							amount: ::core::primitive::u128,
						},
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct EcdsaSignature(pub [::core::primitive::u8; 65usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct EthereumAddress(pub [::core::primitive::u8; 20usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PrevalidateAttests;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum StatementKind {
					#[codec(index = 0)]
					Regular,
					#[codec(index = 1)]
					Saft,
				}
			}
			pub mod crowdloan {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						#[codec(index = 0)]
						create {
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
							#[codec(compact)]
							cap: ::core::primitive::u128,
							#[codec(compact)]
							first_period: ::core::primitive::u32,
							#[codec(compact)]
							last_period: ::core::primitive::u32,
							#[codec(compact)]
							end: ::core::primitive::u32,
							verifier:
								::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
						},
						#[codec(index = 1)]
						contribute {
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
							#[codec(compact)]
							value: ::core::primitive::u128,
							signature:
								::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
						},
						#[codec(index = 2)]
						withdraw {
							who: ::subxt::utils::AccountId32,
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 3)]
						refund {
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 4)]
						dissolve {
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 5)]
						edit {
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
							#[codec(compact)]
							cap: ::core::primitive::u128,
							#[codec(compact)]
							first_period: ::core::primitive::u32,
							#[codec(compact)]
							last_period: ::core::primitive::u32,
							#[codec(compact)]
							end: ::core::primitive::u32,
							verifier:
								::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
						},
						#[codec(index = 6)]
						add_memo {
							index: runtime_types::polkadot_parachain::primitives::Id,
							memo: ::std::vec::Vec<::core::primitive::u8>,
						},
						#[codec(index = 7)]
						poke { index: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 8)]
						contribute_all {
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
							signature:
								::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						FirstPeriodInPast,
						#[codec(index = 1)]
						FirstPeriodTooFarInFuture,
						#[codec(index = 2)]
						LastPeriodBeforeFirstPeriod,
						#[codec(index = 3)]
						LastPeriodTooFarInFuture,
						#[codec(index = 4)]
						CannotEndInPast,
						#[codec(index = 5)]
						EndTooFarInFuture,
						#[codec(index = 6)]
						Overflow,
						#[codec(index = 7)]
						ContributionTooSmall,
						#[codec(index = 8)]
						InvalidParaId,
						#[codec(index = 9)]
						CapExceeded,
						#[codec(index = 10)]
						ContributionPeriodOver,
						#[codec(index = 11)]
						InvalidOrigin,
						#[codec(index = 12)]
						NotParachain,
						#[codec(index = 13)]
						LeaseActive,
						#[codec(index = 14)]
						BidOrLeaseActive,
						#[codec(index = 15)]
						FundNotEnded,
						#[codec(index = 16)]
						NoContributions,
						#[codec(index = 17)]
						NotReadyToDissolve,
						#[codec(index = 18)]
						InvalidSignature,
						#[codec(index = 19)]
						MemoTooLarge,
						#[codec(index = 20)]
						AlreadyInNewRaise,
						#[codec(index = 21)]
						VrfDelayInProgress,
						#[codec(index = 22)]
						NoLeasePeriod,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Event {
						#[codec(index = 0)]
						Created { para_id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 1)]
						Contributed {
							who: ::subxt::utils::AccountId32,
							fund_index: runtime_types::polkadot_parachain::primitives::Id,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 2)]
						Withdrew {
							who: ::subxt::utils::AccountId32,
							fund_index: runtime_types::polkadot_parachain::primitives::Id,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						PartiallyRefunded {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 4)]
						AllRefunded { para_id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 5)]
						Dissolved { para_id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 6)]
						HandleBidResult {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							result: ::core::result::Result<
								(),
								runtime_types::sp_runtime::DispatchError,
							>,
						},
						#[codec(index = 7)]
						Edited { para_id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 8)]
						MemoUpdated {
							who: ::subxt::utils::AccountId32,
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							memo: ::std::vec::Vec<::core::primitive::u8>,
						},
						#[codec(index = 9)]
						AddedToNewRaise {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
						},
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FundInfo<_0, _1, _2, _3> {
					pub depositor: _0,
					pub verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
					pub deposit: _1,
					pub raised: _1,
					pub end: _2,
					pub cap: _1,
					pub last_contribution:
						runtime_types::polkadot_runtime_common::crowdloan::LastContribution<_2>,
					pub first_period: _2,
					pub last_period: _2,
					pub fund_index: _2,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum LastContribution<_0> {
					#[codec(index = 0)]
					Never,
					#[codec(index = 1)]
					PreEnding(_0),
					#[codec(index = 2)]
					Ending(_0),
				}
			}
			pub mod paras_registrar {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						#[codec(index = 0)]
						register {
							id: runtime_types::polkadot_parachain::primitives::Id,
							genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
							validation_code:
								runtime_types::polkadot_parachain::primitives::ValidationCode,
						},
						#[codec(index = 1)]
						force_register {
							who: ::subxt::utils::AccountId32,
							deposit: ::core::primitive::u128,
							id: runtime_types::polkadot_parachain::primitives::Id,
							genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
							validation_code:
								runtime_types::polkadot_parachain::primitives::ValidationCode,
						},
						#[codec(index = 2)]
						deregister { id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 3)]
						swap {
							id: runtime_types::polkadot_parachain::primitives::Id,
							other: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 4)]
						remove_lock { para: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 5)]
						reserve,
						#[codec(index = 6)]
						add_lock { para: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 7)]
						schedule_code_upgrade {
							para: runtime_types::polkadot_parachain::primitives::Id,
							new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
						},
						#[codec(index = 8)]
						set_current_head {
							para: runtime_types::polkadot_parachain::primitives::Id,
							new_head: runtime_types::polkadot_parachain::primitives::HeadData,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						NotRegistered,
						#[codec(index = 1)]
						AlreadyRegistered,
						#[codec(index = 2)]
						NotOwner,
						#[codec(index = 3)]
						CodeTooLarge,
						#[codec(index = 4)]
						HeadDataTooLarge,
						#[codec(index = 5)]
						NotParachain,
						#[codec(index = 6)]
						NotParathread,
						#[codec(index = 7)]
						CannotDeregister,
						#[codec(index = 8)]
						CannotDowngrade,
						#[codec(index = 9)]
						CannotUpgrade,
						#[codec(index = 10)]
						ParaLocked,
						#[codec(index = 11)]
						NotReserved,
						#[codec(index = 12)]
						EmptyCode,
						#[codec(index = 13)]
						CannotSwap,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Event {
						#[codec(index = 0)]
						Registered {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							manager: ::subxt::utils::AccountId32,
						},
						#[codec(index = 1)]
						Deregistered { para_id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 2)]
						Reserved {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							who: ::subxt::utils::AccountId32,
						},
						#[codec(index = 3)]
						Swapped {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							other_id: runtime_types::polkadot_parachain::primitives::Id,
						},
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ParaInfo<_0, _1> {
					pub manager: _0,
					pub deposit: _1,
					pub locked: ::core::primitive::bool,
				}
			}
			pub mod slots {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						#[codec(index = 0)]
						force_lease {
							para: runtime_types::polkadot_parachain::primitives::Id,
							leaser: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
							period_begin: ::core::primitive::u32,
							period_count: ::core::primitive::u32,
						},
						#[codec(index = 1)]
						clear_all_leases { para: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 2)]
						trigger_onboard { para: runtime_types::polkadot_parachain::primitives::Id },
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						ParaNotOnboarding,
						#[codec(index = 1)]
						LeaseError,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Event {
						#[codec(index = 0)]
						NewLeasePeriod { lease_period: ::core::primitive::u32 },
						#[codec(index = 1)]
						Leased {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							leaser: ::subxt::utils::AccountId32,
							period_begin: ::core::primitive::u32,
							period_count: ::core::primitive::u32,
							extra_reserved: ::core::primitive::u128,
							total_amount: ::core::primitive::u128,
						},
					}
				}
			}
		}
		pub mod polkadot_runtime_parachains {
			use super::runtime_types;
			pub mod configuration {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						# [codec (index = 0)] set_validation_upgrade_cooldown { new : :: core :: primitive :: u32 , } , # [codec (index = 1)] set_validation_upgrade_delay { new : :: core :: primitive :: u32 , } , # [codec (index = 2)] set_code_retention_period { new : :: core :: primitive :: u32 , } , # [codec (index = 3)] set_max_code_size { new : :: core :: primitive :: u32 , } , # [codec (index = 4)] set_max_pov_size { new : :: core :: primitive :: u32 , } , # [codec (index = 5)] set_max_head_data_size { new : :: core :: primitive :: u32 , } , # [codec (index = 6)] set_parathread_cores { new : :: core :: primitive :: u32 , } , # [codec (index = 7)] set_parathread_retries { new : :: core :: primitive :: u32 , } , # [codec (index = 8)] set_group_rotation_frequency { new : :: core :: primitive :: u32 , } , # [codec (index = 9)] set_chain_availability_period { new : :: core :: primitive :: u32 , } , # [codec (index = 10)] set_thread_availability_period { new : :: core :: primitive :: u32 , } , # [codec (index = 11)] set_scheduling_lookahead { new : :: core :: primitive :: u32 , } , # [codec (index = 12)] set_max_validators_per_core { new : :: core :: option :: Option < :: core :: primitive :: u32 > , } , # [codec (index = 13)] set_max_validators { new : :: core :: option :: Option < :: core :: primitive :: u32 > , } , # [codec (index = 14)] set_dispute_period { new : :: core :: primitive :: u32 , } , # [codec (index = 15)] set_dispute_post_conclusion_acceptance_period { new : :: core :: primitive :: u32 , } , # [codec (index = 18)] set_no_show_slots { new : :: core :: primitive :: u32 , } , # [codec (index = 19)] set_n_delay_tranches { new : :: core :: primitive :: u32 , } , # [codec (index = 20)] set_zeroth_delay_tranche_width { new : :: core :: primitive :: u32 , } , # [codec (index = 21)] set_needed_approvals { new : :: core :: primitive :: u32 , } , # [codec (index = 22)] set_relay_vrf_modulo_samples { new : :: core :: primitive :: u32 , } , # [codec (index = 23)] set_max_upward_queue_count { new : :: core :: primitive :: u32 , } , # [codec (index = 24)] set_max_upward_queue_size { new : :: core :: primitive :: u32 , } , # [codec (index = 25)] set_max_downward_message_size { new : :: core :: primitive :: u32 , } , # [codec (index = 27)] set_max_upward_message_size { new : :: core :: primitive :: u32 , } , # [codec (index = 28)] set_max_upward_message_num_per_candidate { new : :: core :: primitive :: u32 , } , # [codec (index = 29)] set_hrmp_open_request_ttl { new : :: core :: primitive :: u32 , } , # [codec (index = 30)] set_hrmp_sender_deposit { new : :: core :: primitive :: u128 , } , # [codec (index = 31)] set_hrmp_recipient_deposit { new : :: core :: primitive :: u128 , } , # [codec (index = 32)] set_hrmp_channel_max_capacity { new : :: core :: primitive :: u32 , } , # [codec (index = 33)] set_hrmp_channel_max_total_size { new : :: core :: primitive :: u32 , } , # [codec (index = 34)] set_hrmp_max_parachain_inbound_channels { new : :: core :: primitive :: u32 , } , # [codec (index = 35)] set_hrmp_max_parathread_inbound_channels { new : :: core :: primitive :: u32 , } , # [codec (index = 36)] set_hrmp_channel_max_message_size { new : :: core :: primitive :: u32 , } , # [codec (index = 37)] set_hrmp_max_parachain_outbound_channels { new : :: core :: primitive :: u32 , } , # [codec (index = 38)] set_hrmp_max_parathread_outbound_channels { new : :: core :: primitive :: u32 , } , # [codec (index = 39)] set_hrmp_max_message_num_per_candidate { new : :: core :: primitive :: u32 , } , # [codec (index = 41)] set_pvf_checking_enabled { new : :: core :: primitive :: bool , } , # [codec (index = 42)] set_pvf_voting_ttl { new : :: core :: primitive :: u32 , } , # [codec (index = 43)] set_minimum_validation_upgrade_delay { new : :: core :: primitive :: u32 , } , # [codec (index = 44)] set_bypass_consistency_check { new : :: core :: primitive :: bool , } , # [codec (index = 45)] set_async_backing_params { new : runtime_types :: polkadot_primitives :: vstaging :: AsyncBackingParams , } , # [codec (index = 46)] set_executor_params { new : runtime_types :: polkadot_primitives :: v4 :: executor_params :: ExecutorParams , } , }
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						InvalidNewValue,
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct HostConfiguration<_0> {
					pub max_code_size: _0,
					pub max_head_data_size: _0,
					pub max_upward_queue_count: _0,
					pub max_upward_queue_size: _0,
					pub max_upward_message_size: _0,
					pub max_upward_message_num_per_candidate: _0,
					pub hrmp_max_message_num_per_candidate: _0,
					pub validation_upgrade_cooldown: _0,
					pub validation_upgrade_delay: _0,
					pub async_backing_params:
						runtime_types::polkadot_primitives::vstaging::AsyncBackingParams,
					pub max_pov_size: _0,
					pub max_downward_message_size: _0,
					pub hrmp_max_parachain_outbound_channels: _0,
					pub hrmp_max_parathread_outbound_channels: _0,
					pub hrmp_sender_deposit: ::core::primitive::u128,
					pub hrmp_recipient_deposit: ::core::primitive::u128,
					pub hrmp_channel_max_capacity: _0,
					pub hrmp_channel_max_total_size: _0,
					pub hrmp_max_parachain_inbound_channels: _0,
					pub hrmp_max_parathread_inbound_channels: _0,
					pub hrmp_channel_max_message_size: _0,
					pub executor_params:
						runtime_types::polkadot_primitives::v4::executor_params::ExecutorParams,
					pub code_retention_period: _0,
					pub parathread_cores: _0,
					pub parathread_retries: _0,
					pub group_rotation_frequency: _0,
					pub chain_availability_period: _0,
					pub thread_availability_period: _0,
					pub scheduling_lookahead: _0,
					pub max_validators_per_core: ::core::option::Option<_0>,
					pub max_validators: ::core::option::Option<_0>,
					pub dispute_period: _0,
					pub dispute_post_conclusion_acceptance_period: _0,
					pub no_show_slots: _0,
					pub n_delay_tranches: _0,
					pub zeroth_delay_tranche_width: _0,
					pub needed_approvals: _0,
					pub relay_vrf_modulo_samples: _0,
					pub pvf_checking_enabled: ::core::primitive::bool,
					pub pvf_voting_ttl: _0,
					pub minimum_validation_upgrade_delay: _0,
				}
			}
			pub mod disputes {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						#[codec(index = 0)]
						force_unfreeze,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						DuplicateDisputeStatementSets,
						#[codec(index = 1)]
						AncientDisputeStatement,
						#[codec(index = 2)]
						ValidatorIndexOutOfBounds,
						#[codec(index = 3)]
						InvalidSignature,
						#[codec(index = 4)]
						DuplicateStatement,
						#[codec(index = 5)]
						SingleSidedDispute,
						#[codec(index = 6)]
						MaliciousBacker,
						#[codec(index = 7)]
						MissingBackingVotes,
						#[codec(index = 8)]
						UnconfirmedDispute,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Event {
						#[codec(index = 0)]
						DisputeInitiated(
							runtime_types::polkadot_core_primitives::CandidateHash,
							runtime_types::polkadot_runtime_parachains::disputes::DisputeLocation,
						),
						#[codec(index = 1)]
						DisputeConcluded(
							runtime_types::polkadot_core_primitives::CandidateHash,
							runtime_types::polkadot_runtime_parachains::disputes::DisputeResult,
						),
						#[codec(index = 2)]
						Revert(::core::primitive::u32),
					}
				}
				pub mod slashing {
					use super::runtime_types;
					pub mod pallet {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub enum Call {
							# [codec (index = 0)] report_dispute_lost_unsigned { dispute_proof : :: std :: boxed :: Box < runtime_types :: polkadot_runtime_parachains :: disputes :: slashing :: DisputeProof > , key_owner_proof : runtime_types :: sp_session :: MembershipProof , } , }
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub enum Error {
							#[codec(index = 0)]
							InvalidKeyOwnershipProof,
							#[codec(index = 1)]
							InvalidSessionIndex,
							#[codec(index = 2)]
							InvalidCandidateHash,
							#[codec(index = 3)]
							InvalidValidatorIndex,
							#[codec(index = 4)]
							ValidatorIndexIdMismatch,
							#[codec(index = 5)]
							DuplicateSlashingReport,
						}
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct DisputeProof { pub time_slot : runtime_types :: polkadot_runtime_parachains :: disputes :: slashing :: DisputesTimeSlot , pub kind : runtime_types :: polkadot_runtime_parachains :: disputes :: slashing :: SlashingOffenceKind , pub validator_index : runtime_types :: polkadot_primitives :: v4 :: ValidatorIndex , pub validator_id : runtime_types :: polkadot_primitives :: v4 :: validator_app :: Public , }
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct DisputesTimeSlot {
						pub session_index: ::core::primitive::u32,
						pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct PendingSlashes { pub keys : :: subxt :: utils :: KeyedVec < runtime_types :: polkadot_primitives :: v4 :: ValidatorIndex , runtime_types :: polkadot_primitives :: v4 :: validator_app :: Public > , pub kind : runtime_types :: polkadot_runtime_parachains :: disputes :: slashing :: SlashingOffenceKind , }
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum SlashingOffenceKind {
						#[codec(index = 0)]
						ForInvalid,
						#[codec(index = 1)]
						AgainstValid,
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum DisputeLocation {
					#[codec(index = 0)]
					Local,
					#[codec(index = 1)]
					Remote,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum DisputeResult {
					#[codec(index = 0)]
					Valid,
					#[codec(index = 1)]
					Invalid,
				}
			}
			pub mod hrmp {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						#[codec(index = 0)]
						hrmp_init_open_channel {
							recipient: runtime_types::polkadot_parachain::primitives::Id,
							proposed_max_capacity: ::core::primitive::u32,
							proposed_max_message_size: ::core::primitive::u32,
						},
						#[codec(index = 1)]
						hrmp_accept_open_channel {
							sender: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 2)]
						hrmp_close_channel {
							channel_id:
								runtime_types::polkadot_parachain::primitives::HrmpChannelId,
						},
						#[codec(index = 3)]
						force_clean_hrmp {
							para: runtime_types::polkadot_parachain::primitives::Id,
							inbound: ::core::primitive::u32,
							outbound: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						force_process_hrmp_open { channels: ::core::primitive::u32 },
						#[codec(index = 5)]
						force_process_hrmp_close { channels: ::core::primitive::u32 },
						#[codec(index = 6)]
						hrmp_cancel_open_request {
							channel_id:
								runtime_types::polkadot_parachain::primitives::HrmpChannelId,
							open_requests: ::core::primitive::u32,
						},
						#[codec(index = 7)]
						force_open_hrmp_channel {
							sender: runtime_types::polkadot_parachain::primitives::Id,
							recipient: runtime_types::polkadot_parachain::primitives::Id,
							max_capacity: ::core::primitive::u32,
							max_message_size: ::core::primitive::u32,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						OpenHrmpChannelToSelf,
						#[codec(index = 1)]
						OpenHrmpChannelInvalidRecipient,
						#[codec(index = 2)]
						OpenHrmpChannelZeroCapacity,
						#[codec(index = 3)]
						OpenHrmpChannelCapacityExceedsLimit,
						#[codec(index = 4)]
						OpenHrmpChannelZeroMessageSize,
						#[codec(index = 5)]
						OpenHrmpChannelMessageSizeExceedsLimit,
						#[codec(index = 6)]
						OpenHrmpChannelAlreadyExists,
						#[codec(index = 7)]
						OpenHrmpChannelAlreadyRequested,
						#[codec(index = 8)]
						OpenHrmpChannelLimitExceeded,
						#[codec(index = 9)]
						AcceptHrmpChannelDoesntExist,
						#[codec(index = 10)]
						AcceptHrmpChannelAlreadyConfirmed,
						#[codec(index = 11)]
						AcceptHrmpChannelLimitExceeded,
						#[codec(index = 12)]
						CloseHrmpChannelUnauthorized,
						#[codec(index = 13)]
						CloseHrmpChannelDoesntExist,
						#[codec(index = 14)]
						CloseHrmpChannelAlreadyUnderway,
						#[codec(index = 15)]
						CancelHrmpOpenChannelUnauthorized,
						#[codec(index = 16)]
						OpenHrmpChannelDoesntExist,
						#[codec(index = 17)]
						OpenHrmpChannelAlreadyConfirmed,
						#[codec(index = 18)]
						WrongWitness,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Event {
						#[codec(index = 0)]
						OpenChannelRequested(
							runtime_types::polkadot_parachain::primitives::Id,
							runtime_types::polkadot_parachain::primitives::Id,
							::core::primitive::u32,
							::core::primitive::u32,
						),
						#[codec(index = 1)]
						OpenChannelCanceled(
							runtime_types::polkadot_parachain::primitives::Id,
							runtime_types::polkadot_parachain::primitives::HrmpChannelId,
						),
						#[codec(index = 2)]
						OpenChannelAccepted(
							runtime_types::polkadot_parachain::primitives::Id,
							runtime_types::polkadot_parachain::primitives::Id,
						),
						#[codec(index = 3)]
						ChannelClosed(
							runtime_types::polkadot_parachain::primitives::Id,
							runtime_types::polkadot_parachain::primitives::HrmpChannelId,
						),
						#[codec(index = 4)]
						HrmpChannelForceOpened(
							runtime_types::polkadot_parachain::primitives::Id,
							runtime_types::polkadot_parachain::primitives::Id,
							::core::primitive::u32,
							::core::primitive::u32,
						),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct HrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<::subxt::utils::H256>,
					pub sender_deposit: ::core::primitive::u128,
					pub recipient_deposit: ::core::primitive::u128,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct HrmpOpenChannelRequest {
					pub confirmed: ::core::primitive::bool,
					pub _age: ::core::primitive::u32,
					pub sender_deposit: ::core::primitive::u128,
					pub max_message_size: ::core::primitive::u32,
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
				}
			}
			pub mod inclusion {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						UnsortedOrDuplicateValidatorIndices,
						#[codec(index = 1)]
						UnsortedOrDuplicateDisputeStatementSet,
						#[codec(index = 2)]
						UnsortedOrDuplicateBackedCandidates,
						#[codec(index = 3)]
						UnexpectedRelayParent,
						#[codec(index = 4)]
						WrongBitfieldSize,
						#[codec(index = 5)]
						BitfieldAllZeros,
						#[codec(index = 6)]
						BitfieldDuplicateOrUnordered,
						#[codec(index = 7)]
						ValidatorIndexOutOfBounds,
						#[codec(index = 8)]
						InvalidBitfieldSignature,
						#[codec(index = 9)]
						UnscheduledCandidate,
						#[codec(index = 10)]
						CandidateScheduledBeforeParaFree,
						#[codec(index = 11)]
						WrongCollator,
						#[codec(index = 12)]
						ScheduledOutOfOrder,
						#[codec(index = 13)]
						HeadDataTooLarge,
						#[codec(index = 14)]
						PrematureCodeUpgrade,
						#[codec(index = 15)]
						NewCodeTooLarge,
						#[codec(index = 16)]
						CandidateNotInParentContext,
						#[codec(index = 17)]
						InvalidGroupIndex,
						#[codec(index = 18)]
						InsufficientBacking,
						#[codec(index = 19)]
						InvalidBacking,
						#[codec(index = 20)]
						NotCollatorSigned,
						#[codec(index = 21)]
						ValidationDataHashMismatch,
						#[codec(index = 22)]
						IncorrectDownwardMessageHandling,
						#[codec(index = 23)]
						InvalidUpwardMessages,
						#[codec(index = 24)]
						HrmpWatermarkMishandling,
						#[codec(index = 25)]
						InvalidOutboundHrmp,
						#[codec(index = 26)]
						InvalidValidationCodeHash,
						#[codec(index = 27)]
						ParaHeadMismatch,
						#[codec(index = 28)]
						BitfieldReferencesFreedCore,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Event {
						#[codec(index = 0)]
						CandidateBacked(
							runtime_types::polkadot_primitives::v4::CandidateReceipt<
								::subxt::utils::H256,
							>,
							runtime_types::polkadot_parachain::primitives::HeadData,
							runtime_types::polkadot_primitives::v4::CoreIndex,
							runtime_types::polkadot_primitives::v4::GroupIndex,
						),
						#[codec(index = 1)]
						CandidateIncluded(
							runtime_types::polkadot_primitives::v4::CandidateReceipt<
								::subxt::utils::H256,
							>,
							runtime_types::polkadot_parachain::primitives::HeadData,
							runtime_types::polkadot_primitives::v4::CoreIndex,
							runtime_types::polkadot_primitives::v4::GroupIndex,
						),
						#[codec(index = 2)]
						CandidateTimedOut(
							runtime_types::polkadot_primitives::v4::CandidateReceipt<
								::subxt::utils::H256,
							>,
							runtime_types::polkadot_parachain::primitives::HeadData,
							runtime_types::polkadot_primitives::v4::CoreIndex,
						),
						#[codec(index = 3)]
						UpwardMessagesReceived {
							from: runtime_types::polkadot_parachain::primitives::Id,
							count: ::core::primitive::u32,
						},
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum AggregateMessageOrigin {
					#[codec(index = 0)]
					Ump(runtime_types::polkadot_runtime_parachains::inclusion::UmpQueueId),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AvailabilityBitfieldRecord<_0> {
					pub bitfield: runtime_types::polkadot_primitives::v4::AvailabilityBitfield,
					pub submitted_at: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CandidatePendingAvailability<_0, _1> {
					pub core: runtime_types::polkadot_primitives::v4::CoreIndex,
					pub hash: runtime_types::polkadot_core_primitives::CandidateHash,
					pub descriptor: runtime_types::polkadot_primitives::v4::CandidateDescriptor<_0>,
					pub availability_votes: ::subxt::utils::bits::DecodedBits<
						::core::primitive::u8,
						::subxt::utils::bits::Lsb0,
					>,
					pub backers: ::subxt::utils::bits::DecodedBits<
						::core::primitive::u8,
						::subxt::utils::bits::Lsb0,
					>,
					pub relay_parent_number: _1,
					pub backed_in_number: _1,
					pub backing_group: runtime_types::polkadot_primitives::v4::GroupIndex,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum UmpQueueId {
					#[codec(index = 0)]
					Para(runtime_types::polkadot_parachain::primitives::Id),
				}
			}
			pub mod initializer {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						#[codec(index = 0)]
						force_approve { up_to: ::core::primitive::u32 },
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BufferedSessionChange {
					pub validators: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v4::validator_app::Public,
					>,
					pub queued: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v4::validator_app::Public,
					>,
					pub session_index: ::core::primitive::u32,
				}
			}
			pub mod origin {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Origin {
						#[codec(index = 0)]
						Parachain(runtime_types::polkadot_parachain::primitives::Id),
					}
				}
			}
			pub mod paras {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						#[codec(index = 0)]
						force_set_current_code {
							para: runtime_types::polkadot_parachain::primitives::Id,
							new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
						},
						#[codec(index = 1)]
						force_set_current_head {
							para: runtime_types::polkadot_parachain::primitives::Id,
							new_head: runtime_types::polkadot_parachain::primitives::HeadData,
						},
						#[codec(index = 2)]
						force_schedule_code_upgrade {
							para: runtime_types::polkadot_parachain::primitives::Id,
							new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
							relay_parent_number: ::core::primitive::u32,
						},
						#[codec(index = 3)]
						force_note_new_head {
							para: runtime_types::polkadot_parachain::primitives::Id,
							new_head: runtime_types::polkadot_parachain::primitives::HeadData,
						},
						#[codec(index = 4)]
						force_queue_action {
							para: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 5)]
						add_trusted_validation_code {
							validation_code:
								runtime_types::polkadot_parachain::primitives::ValidationCode,
						},
						#[codec(index = 6)]
						poke_unused_validation_code {
							validation_code_hash:
								runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
						},
						#[codec(index = 7)]
						include_pvf_check_statement {
							stmt: runtime_types::polkadot_primitives::v4::PvfCheckStatement,
							signature:
								runtime_types::polkadot_primitives::v4::validator_app::Signature,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						NotRegistered,
						#[codec(index = 1)]
						CannotOnboard,
						#[codec(index = 2)]
						CannotOffboard,
						#[codec(index = 3)]
						CannotUpgrade,
						#[codec(index = 4)]
						CannotDowngrade,
						#[codec(index = 5)]
						PvfCheckStatementStale,
						#[codec(index = 6)]
						PvfCheckStatementFuture,
						#[codec(index = 7)]
						PvfCheckValidatorIndexOutOfBounds,
						#[codec(index = 8)]
						PvfCheckInvalidSignature,
						#[codec(index = 9)]
						PvfCheckDoubleVote,
						#[codec(index = 10)]
						PvfCheckSubjectInvalid,
						#[codec(index = 11)]
						CannotUpgradeCode,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Event {
						#[codec(index = 0)]
						CurrentCodeUpdated(runtime_types::polkadot_parachain::primitives::Id),
						#[codec(index = 1)]
						CurrentHeadUpdated(runtime_types::polkadot_parachain::primitives::Id),
						#[codec(index = 2)]
						CodeUpgradeScheduled(runtime_types::polkadot_parachain::primitives::Id),
						#[codec(index = 3)]
						NewHeadNoted(runtime_types::polkadot_parachain::primitives::Id),
						#[codec(index = 4)]
						ActionQueued(
							runtime_types::polkadot_parachain::primitives::Id,
							::core::primitive::u32,
						),
						#[codec(index = 5)]
						PvfCheckStarted(
							runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
							runtime_types::polkadot_parachain::primitives::Id,
						),
						#[codec(index = 6)]
						PvfCheckAccepted(
							runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
							runtime_types::polkadot_parachain::primitives::Id,
						),
						#[codec(index = 7)]
						PvfCheckRejected(
							runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
							runtime_types::polkadot_parachain::primitives::Id,
						),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ParaGenesisArgs {
					pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
					pub validation_code:
						runtime_types::polkadot_parachain::primitives::ValidationCode,
					pub para_kind: ::core::primitive::bool,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum ParaLifecycle {
					#[codec(index = 0)]
					Onboarding,
					#[codec(index = 1)]
					Parathread,
					#[codec(index = 2)]
					Parachain,
					#[codec(index = 3)]
					UpgradingParathread,
					#[codec(index = 4)]
					DowngradingParachain,
					#[codec(index = 5)]
					OffboardingParathread,
					#[codec(index = 6)]
					OffboardingParachain,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ParaPastCodeMeta<_0> {
					pub upgrade_times: ::std::vec::Vec<
						runtime_types::polkadot_runtime_parachains::paras::ReplacementTimes<_0>,
					>,
					pub last_pruned: ::core::option::Option<_0>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PvfCheckActiveVoteState<_0> {
					pub votes_accept: ::subxt::utils::bits::DecodedBits<
						::core::primitive::u8,
						::subxt::utils::bits::Lsb0,
					>,
					pub votes_reject: ::subxt::utils::bits::DecodedBits<
						::core::primitive::u8,
						::subxt::utils::bits::Lsb0,
					>,
					pub age: _0,
					pub created_at: _0,
					pub causes: ::std::vec::Vec<
						runtime_types::polkadot_runtime_parachains::paras::PvfCheckCause<_0>,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum PvfCheckCause<_0> {
					#[codec(index = 0)]
					Onboarding(runtime_types::polkadot_parachain::primitives::Id),
					#[codec(index = 1)]
					Upgrade {
						id: runtime_types::polkadot_parachain::primitives::Id,
						relay_parent_number: _0,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReplacementTimes<_0> {
					pub expected_at: _0,
					pub activated_at: _0,
				}
			}
			pub mod paras_inherent {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {
						#[codec(index = 0)]
						enter {
							data: runtime_types::polkadot_primitives::v4::InherentData<
								runtime_types::sp_runtime::generic::header::Header<
									::core::primitive::u32,
									runtime_types::sp_runtime::traits::BlakeTwo256,
								>,
							>,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						TooManyInclusionInherents,
						#[codec(index = 1)]
						InvalidParentHeader,
						#[codec(index = 2)]
						CandidateConcludedInvalid,
						#[codec(index = 3)]
						InherentOverweight,
						#[codec(index = 4)]
						DisputeStatementsUnsortedOrDuplicates,
						#[codec(index = 5)]
						DisputeInvalid,
					}
				}
			}
			pub mod scheduler {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum AssignmentKind {
					#[codec(index = 0)]
					Parachain,
					#[codec(index = 1)]
					Parathread(
						runtime_types::polkadot_primitives::v4::collator_app::Public,
						::core::primitive::u32,
					),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CoreAssignment {
					pub core: runtime_types::polkadot_primitives::v4::CoreIndex,
					pub para_id: runtime_types::polkadot_parachain::primitives::Id,
					pub kind: runtime_types::polkadot_runtime_parachains::scheduler::AssignmentKind,
					pub group_idx: runtime_types::polkadot_primitives::v4::GroupIndex,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ParathreadClaimQueue {
					pub queue: ::std::vec::Vec<
						runtime_types::polkadot_runtime_parachains::scheduler::QueuedParathread,
					>,
					pub next_core_offset: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueuedParathread {
					pub claim: runtime_types::polkadot_primitives::v4::ParathreadEntry,
					pub core_offset: ::core::primitive::u32,
				}
			}
			pub mod shared {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Call {}
				}
			}
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FixedI64(pub ::core::primitive::i64);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PerU16(pub ::core::primitive::u16);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Perbill(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Percent(pub ::core::primitive::u8);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Permill(pub ::core::primitive::u32);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
		}
		pub mod sp_authority_discovery {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub runtime_types::sp_core::sr25519::Public);
			}
		}
		pub mod sp_consensus_babe {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub runtime_types::sp_core::sr25519::Public);
			}
			pub mod digests {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum NextConfigDescriptor {
					#[codec(index = 1)]
					V1 {
						c: (::core::primitive::u64, ::core::primitive::u64),
						allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum PreDigest {
					#[codec(index = 1)]
					Primary(runtime_types::sp_consensus_babe::digests::PrimaryPreDigest),
					#[codec(index = 2)]
					SecondaryPlain(
						runtime_types::sp_consensus_babe::digests::SecondaryPlainPreDigest,
					),
					#[codec(index = 3)]
					SecondaryVRF(runtime_types::sp_consensus_babe::digests::SecondaryVRFPreDigest),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PrimaryPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
					pub vrf_signature: runtime_types::sp_core::sr25519::vrf::VrfSignature,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SecondaryPlainPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SecondaryVRFPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
					pub vrf_signature: runtime_types::sp_core::sr25519::vrf::VrfSignature,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum AllowedSlots {
				#[codec(index = 0)]
				PrimarySlots,
				#[codec(index = 1)]
				PrimaryAndSecondaryPlainSlots,
				#[codec(index = 2)]
				PrimaryAndSecondaryVRFSlots,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BabeEpochConfiguration {
				pub c: (::core::primitive::u64, ::core::primitive::u64),
				pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
			}
		}
		pub mod sp_consensus_grandpa {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub runtime_types::sp_core::ed25519::Public);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Equivocation<_0, _1> {
				#[codec(index = 0)]
				Prevote(
					runtime_types::finality_grandpa::Equivocation<
						runtime_types::sp_consensus_grandpa::app::Public,
						runtime_types::finality_grandpa::Prevote<_0, _1>,
						runtime_types::sp_consensus_grandpa::app::Signature,
					>,
				),
				#[codec(index = 1)]
				Precommit(
					runtime_types::finality_grandpa::Equivocation<
						runtime_types::sp_consensus_grandpa::app::Public,
						runtime_types::finality_grandpa::Precommit<_0, _1>,
						runtime_types::sp_consensus_grandpa::app::Signature,
					>,
				),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EquivocationProof<_0, _1> {
				pub set_id: ::core::primitive::u64,
				pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EquivocationProof<_0, _1> {
				pub offender: _1,
				pub slot: runtime_types::sp_consensus_slots::Slot,
				pub first_header: _0,
				pub second_header: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub [::core::primitive::u8; 33usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod offchain {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct OpaqueNetworkState {
					pub peer_id: runtime_types::sp_core::OpaquePeerId,
					pub external_addresses:
						::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
				}
			}
			pub mod sr25519 {
				use super::runtime_types;
				pub mod vrf {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct VrfSignature {
						pub output: [::core::primitive::u8; 32usize],
						pub proof: [::core::primitive::u8; 64usize],
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Void {}
		}
		pub mod sp_npos_elections {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ElectionScore {
				pub minimal_stake: ::core::primitive::u128,
				pub sum_stake: ::core::primitive::u128,
				pub sum_stake_squared: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Support<_0> {
				pub total: ::core::primitive::u128,
				pub voters: ::std::vec::Vec<(_0, ::core::primitive::u128)>,
			}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum DigestItem {
						#[codec(index = 6)]
						PreRuntime(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 4)]
						Consensus(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 5)]
						Seal(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 0)]
						Other(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 8)]
						RuntimeEnvironmentUpdated,
					}
				}
				pub mod era {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Era {
						#[codec(index = 0)]
						Immortal,
						#[codec(index = 1)]
						Mortal1(::core::primitive::u8),
						#[codec(index = 2)]
						Mortal2(::core::primitive::u8),
						#[codec(index = 3)]
						Mortal3(::core::primitive::u8),
						#[codec(index = 4)]
						Mortal4(::core::primitive::u8),
						#[codec(index = 5)]
						Mortal5(::core::primitive::u8),
						#[codec(index = 6)]
						Mortal6(::core::primitive::u8),
						#[codec(index = 7)]
						Mortal7(::core::primitive::u8),
						#[codec(index = 8)]
						Mortal8(::core::primitive::u8),
						#[codec(index = 9)]
						Mortal9(::core::primitive::u8),
						#[codec(index = 10)]
						Mortal10(::core::primitive::u8),
						#[codec(index = 11)]
						Mortal11(::core::primitive::u8),
						#[codec(index = 12)]
						Mortal12(::core::primitive::u8),
						#[codec(index = 13)]
						Mortal13(::core::primitive::u8),
						#[codec(index = 14)]
						Mortal14(::core::primitive::u8),
						#[codec(index = 15)]
						Mortal15(::core::primitive::u8),
						#[codec(index = 16)]
						Mortal16(::core::primitive::u8),
						#[codec(index = 17)]
						Mortal17(::core::primitive::u8),
						#[codec(index = 18)]
						Mortal18(::core::primitive::u8),
						#[codec(index = 19)]
						Mortal19(::core::primitive::u8),
						#[codec(index = 20)]
						Mortal20(::core::primitive::u8),
						#[codec(index = 21)]
						Mortal21(::core::primitive::u8),
						#[codec(index = 22)]
						Mortal22(::core::primitive::u8),
						#[codec(index = 23)]
						Mortal23(::core::primitive::u8),
						#[codec(index = 24)]
						Mortal24(::core::primitive::u8),
						#[codec(index = 25)]
						Mortal25(::core::primitive::u8),
						#[codec(index = 26)]
						Mortal26(::core::primitive::u8),
						#[codec(index = 27)]
						Mortal27(::core::primitive::u8),
						#[codec(index = 28)]
						Mortal28(::core::primitive::u8),
						#[codec(index = 29)]
						Mortal29(::core::primitive::u8),
						#[codec(index = 30)]
						Mortal30(::core::primitive::u8),
						#[codec(index = 31)]
						Mortal31(::core::primitive::u8),
						#[codec(index = 32)]
						Mortal32(::core::primitive::u8),
						#[codec(index = 33)]
						Mortal33(::core::primitive::u8),
						#[codec(index = 34)]
						Mortal34(::core::primitive::u8),
						#[codec(index = 35)]
						Mortal35(::core::primitive::u8),
						#[codec(index = 36)]
						Mortal36(::core::primitive::u8),
						#[codec(index = 37)]
						Mortal37(::core::primitive::u8),
						#[codec(index = 38)]
						Mortal38(::core::primitive::u8),
						#[codec(index = 39)]
						Mortal39(::core::primitive::u8),
						#[codec(index = 40)]
						Mortal40(::core::primitive::u8),
						#[codec(index = 41)]
						Mortal41(::core::primitive::u8),
						#[codec(index = 42)]
						Mortal42(::core::primitive::u8),
						#[codec(index = 43)]
						Mortal43(::core::primitive::u8),
						#[codec(index = 44)]
						Mortal44(::core::primitive::u8),
						#[codec(index = 45)]
						Mortal45(::core::primitive::u8),
						#[codec(index = 46)]
						Mortal46(::core::primitive::u8),
						#[codec(index = 47)]
						Mortal47(::core::primitive::u8),
						#[codec(index = 48)]
						Mortal48(::core::primitive::u8),
						#[codec(index = 49)]
						Mortal49(::core::primitive::u8),
						#[codec(index = 50)]
						Mortal50(::core::primitive::u8),
						#[codec(index = 51)]
						Mortal51(::core::primitive::u8),
						#[codec(index = 52)]
						Mortal52(::core::primitive::u8),
						#[codec(index = 53)]
						Mortal53(::core::primitive::u8),
						#[codec(index = 54)]
						Mortal54(::core::primitive::u8),
						#[codec(index = 55)]
						Mortal55(::core::primitive::u8),
						#[codec(index = 56)]
						Mortal56(::core::primitive::u8),
						#[codec(index = 57)]
						Mortal57(::core::primitive::u8),
						#[codec(index = 58)]
						Mortal58(::core::primitive::u8),
						#[codec(index = 59)]
						Mortal59(::core::primitive::u8),
						#[codec(index = 60)]
						Mortal60(::core::primitive::u8),
						#[codec(index = 61)]
						Mortal61(::core::primitive::u8),
						#[codec(index = 62)]
						Mortal62(::core::primitive::u8),
						#[codec(index = 63)]
						Mortal63(::core::primitive::u8),
						#[codec(index = 64)]
						Mortal64(::core::primitive::u8),
						#[codec(index = 65)]
						Mortal65(::core::primitive::u8),
						#[codec(index = 66)]
						Mortal66(::core::primitive::u8),
						#[codec(index = 67)]
						Mortal67(::core::primitive::u8),
						#[codec(index = 68)]
						Mortal68(::core::primitive::u8),
						#[codec(index = 69)]
						Mortal69(::core::primitive::u8),
						#[codec(index = 70)]
						Mortal70(::core::primitive::u8),
						#[codec(index = 71)]
						Mortal71(::core::primitive::u8),
						#[codec(index = 72)]
						Mortal72(::core::primitive::u8),
						#[codec(index = 73)]
						Mortal73(::core::primitive::u8),
						#[codec(index = 74)]
						Mortal74(::core::primitive::u8),
						#[codec(index = 75)]
						Mortal75(::core::primitive::u8),
						#[codec(index = 76)]
						Mortal76(::core::primitive::u8),
						#[codec(index = 77)]
						Mortal77(::core::primitive::u8),
						#[codec(index = 78)]
						Mortal78(::core::primitive::u8),
						#[codec(index = 79)]
						Mortal79(::core::primitive::u8),
						#[codec(index = 80)]
						Mortal80(::core::primitive::u8),
						#[codec(index = 81)]
						Mortal81(::core::primitive::u8),
						#[codec(index = 82)]
						Mortal82(::core::primitive::u8),
						#[codec(index = 83)]
						Mortal83(::core::primitive::u8),
						#[codec(index = 84)]
						Mortal84(::core::primitive::u8),
						#[codec(index = 85)]
						Mortal85(::core::primitive::u8),
						#[codec(index = 86)]
						Mortal86(::core::primitive::u8),
						#[codec(index = 87)]
						Mortal87(::core::primitive::u8),
						#[codec(index = 88)]
						Mortal88(::core::primitive::u8),
						#[codec(index = 89)]
						Mortal89(::core::primitive::u8),
						#[codec(index = 90)]
						Mortal90(::core::primitive::u8),
						#[codec(index = 91)]
						Mortal91(::core::primitive::u8),
						#[codec(index = 92)]
						Mortal92(::core::primitive::u8),
						#[codec(index = 93)]
						Mortal93(::core::primitive::u8),
						#[codec(index = 94)]
						Mortal94(::core::primitive::u8),
						#[codec(index = 95)]
						Mortal95(::core::primitive::u8),
						#[codec(index = 96)]
						Mortal96(::core::primitive::u8),
						#[codec(index = 97)]
						Mortal97(::core::primitive::u8),
						#[codec(index = 98)]
						Mortal98(::core::primitive::u8),
						#[codec(index = 99)]
						Mortal99(::core::primitive::u8),
						#[codec(index = 100)]
						Mortal100(::core::primitive::u8),
						#[codec(index = 101)]
						Mortal101(::core::primitive::u8),
						#[codec(index = 102)]
						Mortal102(::core::primitive::u8),
						#[codec(index = 103)]
						Mortal103(::core::primitive::u8),
						#[codec(index = 104)]
						Mortal104(::core::primitive::u8),
						#[codec(index = 105)]
						Mortal105(::core::primitive::u8),
						#[codec(index = 106)]
						Mortal106(::core::primitive::u8),
						#[codec(index = 107)]
						Mortal107(::core::primitive::u8),
						#[codec(index = 108)]
						Mortal108(::core::primitive::u8),
						#[codec(index = 109)]
						Mortal109(::core::primitive::u8),
						#[codec(index = 110)]
						Mortal110(::core::primitive::u8),
						#[codec(index = 111)]
						Mortal111(::core::primitive::u8),
						#[codec(index = 112)]
						Mortal112(::core::primitive::u8),
						#[codec(index = 113)]
						Mortal113(::core::primitive::u8),
						#[codec(index = 114)]
						Mortal114(::core::primitive::u8),
						#[codec(index = 115)]
						Mortal115(::core::primitive::u8),
						#[codec(index = 116)]
						Mortal116(::core::primitive::u8),
						#[codec(index = 117)]
						Mortal117(::core::primitive::u8),
						#[codec(index = 118)]
						Mortal118(::core::primitive::u8),
						#[codec(index = 119)]
						Mortal119(::core::primitive::u8),
						#[codec(index = 120)]
						Mortal120(::core::primitive::u8),
						#[codec(index = 121)]
						Mortal121(::core::primitive::u8),
						#[codec(index = 122)]
						Mortal122(::core::primitive::u8),
						#[codec(index = 123)]
						Mortal123(::core::primitive::u8),
						#[codec(index = 124)]
						Mortal124(::core::primitive::u8),
						#[codec(index = 125)]
						Mortal125(::core::primitive::u8),
						#[codec(index = 126)]
						Mortal126(::core::primitive::u8),
						#[codec(index = 127)]
						Mortal127(::core::primitive::u8),
						#[codec(index = 128)]
						Mortal128(::core::primitive::u8),
						#[codec(index = 129)]
						Mortal129(::core::primitive::u8),
						#[codec(index = 130)]
						Mortal130(::core::primitive::u8),
						#[codec(index = 131)]
						Mortal131(::core::primitive::u8),
						#[codec(index = 132)]
						Mortal132(::core::primitive::u8),
						#[codec(index = 133)]
						Mortal133(::core::primitive::u8),
						#[codec(index = 134)]
						Mortal134(::core::primitive::u8),
						#[codec(index = 135)]
						Mortal135(::core::primitive::u8),
						#[codec(index = 136)]
						Mortal136(::core::primitive::u8),
						#[codec(index = 137)]
						Mortal137(::core::primitive::u8),
						#[codec(index = 138)]
						Mortal138(::core::primitive::u8),
						#[codec(index = 139)]
						Mortal139(::core::primitive::u8),
						#[codec(index = 140)]
						Mortal140(::core::primitive::u8),
						#[codec(index = 141)]
						Mortal141(::core::primitive::u8),
						#[codec(index = 142)]
						Mortal142(::core::primitive::u8),
						#[codec(index = 143)]
						Mortal143(::core::primitive::u8),
						#[codec(index = 144)]
						Mortal144(::core::primitive::u8),
						#[codec(index = 145)]
						Mortal145(::core::primitive::u8),
						#[codec(index = 146)]
						Mortal146(::core::primitive::u8),
						#[codec(index = 147)]
						Mortal147(::core::primitive::u8),
						#[codec(index = 148)]
						Mortal148(::core::primitive::u8),
						#[codec(index = 149)]
						Mortal149(::core::primitive::u8),
						#[codec(index = 150)]
						Mortal150(::core::primitive::u8),
						#[codec(index = 151)]
						Mortal151(::core::primitive::u8),
						#[codec(index = 152)]
						Mortal152(::core::primitive::u8),
						#[codec(index = 153)]
						Mortal153(::core::primitive::u8),
						#[codec(index = 154)]
						Mortal154(::core::primitive::u8),
						#[codec(index = 155)]
						Mortal155(::core::primitive::u8),
						#[codec(index = 156)]
						Mortal156(::core::primitive::u8),
						#[codec(index = 157)]
						Mortal157(::core::primitive::u8),
						#[codec(index = 158)]
						Mortal158(::core::primitive::u8),
						#[codec(index = 159)]
						Mortal159(::core::primitive::u8),
						#[codec(index = 160)]
						Mortal160(::core::primitive::u8),
						#[codec(index = 161)]
						Mortal161(::core::primitive::u8),
						#[codec(index = 162)]
						Mortal162(::core::primitive::u8),
						#[codec(index = 163)]
						Mortal163(::core::primitive::u8),
						#[codec(index = 164)]
						Mortal164(::core::primitive::u8),
						#[codec(index = 165)]
						Mortal165(::core::primitive::u8),
						#[codec(index = 166)]
						Mortal166(::core::primitive::u8),
						#[codec(index = 167)]
						Mortal167(::core::primitive::u8),
						#[codec(index = 168)]
						Mortal168(::core::primitive::u8),
						#[codec(index = 169)]
						Mortal169(::core::primitive::u8),
						#[codec(index = 170)]
						Mortal170(::core::primitive::u8),
						#[codec(index = 171)]
						Mortal171(::core::primitive::u8),
						#[codec(index = 172)]
						Mortal172(::core::primitive::u8),
						#[codec(index = 173)]
						Mortal173(::core::primitive::u8),
						#[codec(index = 174)]
						Mortal174(::core::primitive::u8),
						#[codec(index = 175)]
						Mortal175(::core::primitive::u8),
						#[codec(index = 176)]
						Mortal176(::core::primitive::u8),
						#[codec(index = 177)]
						Mortal177(::core::primitive::u8),
						#[codec(index = 178)]
						Mortal178(::core::primitive::u8),
						#[codec(index = 179)]
						Mortal179(::core::primitive::u8),
						#[codec(index = 180)]
						Mortal180(::core::primitive::u8),
						#[codec(index = 181)]
						Mortal181(::core::primitive::u8),
						#[codec(index = 182)]
						Mortal182(::core::primitive::u8),
						#[codec(index = 183)]
						Mortal183(::core::primitive::u8),
						#[codec(index = 184)]
						Mortal184(::core::primitive::u8),
						#[codec(index = 185)]
						Mortal185(::core::primitive::u8),
						#[codec(index = 186)]
						Mortal186(::core::primitive::u8),
						#[codec(index = 187)]
						Mortal187(::core::primitive::u8),
						#[codec(index = 188)]
						Mortal188(::core::primitive::u8),
						#[codec(index = 189)]
						Mortal189(::core::primitive::u8),
						#[codec(index = 190)]
						Mortal190(::core::primitive::u8),
						#[codec(index = 191)]
						Mortal191(::core::primitive::u8),
						#[codec(index = 192)]
						Mortal192(::core::primitive::u8),
						#[codec(index = 193)]
						Mortal193(::core::primitive::u8),
						#[codec(index = 194)]
						Mortal194(::core::primitive::u8),
						#[codec(index = 195)]
						Mortal195(::core::primitive::u8),
						#[codec(index = 196)]
						Mortal196(::core::primitive::u8),
						#[codec(index = 197)]
						Mortal197(::core::primitive::u8),
						#[codec(index = 198)]
						Mortal198(::core::primitive::u8),
						#[codec(index = 199)]
						Mortal199(::core::primitive::u8),
						#[codec(index = 200)]
						Mortal200(::core::primitive::u8),
						#[codec(index = 201)]
						Mortal201(::core::primitive::u8),
						#[codec(index = 202)]
						Mortal202(::core::primitive::u8),
						#[codec(index = 203)]
						Mortal203(::core::primitive::u8),
						#[codec(index = 204)]
						Mortal204(::core::primitive::u8),
						#[codec(index = 205)]
						Mortal205(::core::primitive::u8),
						#[codec(index = 206)]
						Mortal206(::core::primitive::u8),
						#[codec(index = 207)]
						Mortal207(::core::primitive::u8),
						#[codec(index = 208)]
						Mortal208(::core::primitive::u8),
						#[codec(index = 209)]
						Mortal209(::core::primitive::u8),
						#[codec(index = 210)]
						Mortal210(::core::primitive::u8),
						#[codec(index = 211)]
						Mortal211(::core::primitive::u8),
						#[codec(index = 212)]
						Mortal212(::core::primitive::u8),
						#[codec(index = 213)]
						Mortal213(::core::primitive::u8),
						#[codec(index = 214)]
						Mortal214(::core::primitive::u8),
						#[codec(index = 215)]
						Mortal215(::core::primitive::u8),
						#[codec(index = 216)]
						Mortal216(::core::primitive::u8),
						#[codec(index = 217)]
						Mortal217(::core::primitive::u8),
						#[codec(index = 218)]
						Mortal218(::core::primitive::u8),
						#[codec(index = 219)]
						Mortal219(::core::primitive::u8),
						#[codec(index = 220)]
						Mortal220(::core::primitive::u8),
						#[codec(index = 221)]
						Mortal221(::core::primitive::u8),
						#[codec(index = 222)]
						Mortal222(::core::primitive::u8),
						#[codec(index = 223)]
						Mortal223(::core::primitive::u8),
						#[codec(index = 224)]
						Mortal224(::core::primitive::u8),
						#[codec(index = 225)]
						Mortal225(::core::primitive::u8),
						#[codec(index = 226)]
						Mortal226(::core::primitive::u8),
						#[codec(index = 227)]
						Mortal227(::core::primitive::u8),
						#[codec(index = 228)]
						Mortal228(::core::primitive::u8),
						#[codec(index = 229)]
						Mortal229(::core::primitive::u8),
						#[codec(index = 230)]
						Mortal230(::core::primitive::u8),
						#[codec(index = 231)]
						Mortal231(::core::primitive::u8),
						#[codec(index = 232)]
						Mortal232(::core::primitive::u8),
						#[codec(index = 233)]
						Mortal233(::core::primitive::u8),
						#[codec(index = 234)]
						Mortal234(::core::primitive::u8),
						#[codec(index = 235)]
						Mortal235(::core::primitive::u8),
						#[codec(index = 236)]
						Mortal236(::core::primitive::u8),
						#[codec(index = 237)]
						Mortal237(::core::primitive::u8),
						#[codec(index = 238)]
						Mortal238(::core::primitive::u8),
						#[codec(index = 239)]
						Mortal239(::core::primitive::u8),
						#[codec(index = 240)]
						Mortal240(::core::primitive::u8),
						#[codec(index = 241)]
						Mortal241(::core::primitive::u8),
						#[codec(index = 242)]
						Mortal242(::core::primitive::u8),
						#[codec(index = 243)]
						Mortal243(::core::primitive::u8),
						#[codec(index = 244)]
						Mortal244(::core::primitive::u8),
						#[codec(index = 245)]
						Mortal245(::core::primitive::u8),
						#[codec(index = 246)]
						Mortal246(::core::primitive::u8),
						#[codec(index = 247)]
						Mortal247(::core::primitive::u8),
						#[codec(index = 248)]
						Mortal248(::core::primitive::u8),
						#[codec(index = 249)]
						Mortal249(::core::primitive::u8),
						#[codec(index = 250)]
						Mortal250(::core::primitive::u8),
						#[codec(index = 251)]
						Mortal251(::core::primitive::u8),
						#[codec(index = 252)]
						Mortal252(::core::primitive::u8),
						#[codec(index = 253)]
						Mortal253(::core::primitive::u8),
						#[codec(index = 254)]
						Mortal254(::core::primitive::u8),
						#[codec(index = 255)]
						Mortal255(::core::primitive::u8),
					}
				}
				pub mod header {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Header<_0, _1> {
						pub parent_hash: ::subxt::utils::H256,
						#[codec(compact)]
						pub number: _0,
						pub state_root: ::subxt::utils::H256,
						pub extrinsics_root: ::subxt::utils::H256,
						pub digest: runtime_types::sp_runtime::generic::digest::Digest,
						#[codec(skip)]
						pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
					}
				}
				pub mod unchecked_extrinsic {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
						pub ::std::vec::Vec<::core::primitive::u8>,
						#[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
					);
				}
			}
			pub mod traits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlakeTwo256;
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				TooManyConsumers,
				#[codec(index = 7)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 8)]
				Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
				#[codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
				#[codec(index = 10)]
				Exhausted,
				#[codec(index = 11)]
				Corruption,
				#[codec(index = 12)]
				Unavailable,
				#[codec(index = 13)]
				RootNotAllowed,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DispatchErrorWithPostInfo<_0> {
				pub post_info: _0,
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Signature),
				#[codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Signature),
				#[codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Signature),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum MultiSigner {
				#[codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Public),
				#[codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Public),
				#[codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Public),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum TokenError {
				#[codec(index = 0)]
				FundsUnavailable,
				#[codec(index = 1)]
				OnlyProvider,
				#[codec(index = 2)]
				BelowMinimum,
				#[codec(index = 3)]
				CannotCreate,
				#[codec(index = 4)]
				UnknownAsset,
				#[codec(index = 5)]
				Frozen,
				#[codec(index = 6)]
				Unsupported,
				#[codec(index = 7)]
				CannotCreateHold,
				#[codec(index = 8)]
				NotExpendable,
				#[codec(index = 9)]
				Blocked,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum TransactionalError {
				#[codec(index = 0)]
				LimitReached,
				#[codec(index = 1)]
				NoLayer,
			}
		}
		pub mod sp_session {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MembershipProof {
				pub session: ::core::primitive::u32,
				pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				pub validator_count: ::core::primitive::u32,
			}
		}
		pub mod sp_staking {
			use super::runtime_types;
			pub mod offence {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct OffenceDetails<_0, _1> {
					pub offender: _1,
					pub reporters: ::std::vec::Vec<_0>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RuntimeVersion {
				pub spec_name: ::std::string::String,
				pub impl_name: ::std::string::String,
				pub authoring_version: ::core::primitive::u32,
				pub spec_version: ::core::primitive::u32,
				pub impl_version: ::core::primitive::u32,
				pub apis:
					::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
				pub transaction_version: ::core::primitive::u32,
				pub state_version: ::core::primitive::u8,
			}
		}
		pub mod sp_weights {
			use super::runtime_types;
			pub mod weight_v2 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Weight {
					#[codec(compact)]
					pub ref_time: ::core::primitive::u64,
					#[codec(compact)]
					pub proof_size: ::core::primitive::u64,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RuntimeDbWeight {
				pub read: ::core::primitive::u64,
				pub write: ::core::primitive::u64,
			}
		}
		pub mod xcm {
			use super::runtime_types;
			pub mod double_encoded {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DoubleEncoded {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v2 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: runtime_types::xcm::v2::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: runtime_types::xcm::v2::NetworkId,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: runtime_types::xcm::v2::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey(
							runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v2::BodyId,
							part: runtime_types::xcm::v2::BodyPart,
						},
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v2::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
						#[codec(index = 6)]
						Blob(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v2::multiasset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v2::multiasset::AssetId,
						pub fun: runtime_types::xcm::v2::multiasset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v2::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v2::multiasset::WildMultiAsset),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v2::multiasset::MultiAsset>,
					);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v2::multiasset::AssetId,
							fun: runtime_types::xcm::v2::multiasset::WildFungibility,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v2::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v2::multilocation::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						MultiLocationFull,
						#[codec(index = 5)]
						MultiLocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						UnhandledXcmVersion,
						#[codec(index = 23)]
						WeightLimitReached(::core::primitive::u64),
						#[codec(index = 24)]
						Barrier,
						#[codec(index = 25)]
						WeightNotComputable,
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum BodyId {
					#[codec(index = 0)]
					Unit,
					#[codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					Index(#[codec(compact)] ::core::primitive::u32),
					#[codec(index = 3)]
					Executive,
					#[codec(index = 4)]
					Technical,
					#[codec(index = 5)]
					Legislative,
					#[codec(index = 6)]
					Judicial,
					#[codec(index = 7)]
					Defense,
					#[codec(index = 8)]
					Administration,
					#[codec(index = 9)]
					Treasury,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum BodyPart {
					#[codec(index = 0)]
					Voice,
					#[codec(index = 1)]
					Members {
						#[codec(compact)]
						count: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					Fraction {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					AtLeastProportion {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					MoreThanProportion {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v2::Response,
						#[codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v2::OriginKind,
						#[codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
					#[codec(index = 12)]
					ReportError {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						receive: runtime_types::xcm::v2::multiasset::MultiAssets,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 18)]
					QueryHolding {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v2::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v2::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v2::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum NetworkId {
					#[codec(index = 0)]
					Any,
					#[codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					Polkadot,
					#[codec(index = 3)]
					Kusama,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum OriginKind {
					#[codec(index = 0)]
					Native,
					#[codec(index = 1)]
					SovereignAccount,
					#[codec(index = 2)]
					Superuser,
					#[codec(index = 3)]
					Xcm,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v2::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(#[codec(compact)] ::core::primitive::u64),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
			}
			pub mod v3 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum BodyId {
						#[codec(index = 0)]
						Unit,
						#[codec(index = 1)]
						Moniker([::core::primitive::u8; 4usize]),
						#[codec(index = 2)]
						Index(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						Executive,
						#[codec(index = 4)]
						Technical,
						#[codec(index = 5)]
						Legislative,
						#[codec(index = 6)]
						Judicial,
						#[codec(index = 7)]
						Defense,
						#[codec(index = 8)]
						Administration,
						#[codec(index = 9)]
						Treasury,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum BodyPart {
						#[codec(index = 0)]
						Voice,
						#[codec(index = 1)]
						Members {
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
						#[codec(index = 2)]
						Fraction {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 3)]
						AtLeastProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						MoreThanProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey {
							length: ::core::primitive::u8,
							data: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v3::junction::BodyId,
							part: runtime_types::xcm::v3::junction::BodyPart,
						},
						#[codec(index = 9)]
						GlobalConsensus(runtime_types::xcm::v3::junction::NetworkId),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum NetworkId {
						#[codec(index = 0)]
						ByGenesis([::core::primitive::u8; 32usize]),
						#[codec(index = 1)]
						ByFork {
							block_number: ::core::primitive::u64,
							block_hash: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
						#[codec(index = 4)]
						Westend,
						#[codec(index = 5)]
						Rococo,
						#[codec(index = 6)]
						Wococo,
						#[codec(index = 7)]
						Ethereum {
							#[codec(compact)]
							chain_id: ::core::primitive::u64,
						},
						#[codec(index = 8)]
						BitcoinCore,
						#[codec(index = 9)]
						BitcoinCash,
					}
				}
				pub mod junctions {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v3::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v3::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v3::multiasset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v3::multiasset::AssetId,
						pub fun: runtime_types::xcm::v3::multiasset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v3::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v3::multiasset::WildMultiAsset),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v3::multiasset::MultiAsset>,
					);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
						},
						#[codec(index = 2)]
						AllCounted(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						AllOfCounted {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v3::junctions::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						LocationFull,
						#[codec(index = 5)]
						LocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						ExpectationFalse,
						#[codec(index = 23)]
						PalletNotFound,
						#[codec(index = 24)]
						NameMismatch,
						#[codec(index = 25)]
						VersionIncompatible,
						#[codec(index = 26)]
						HoldingWouldOverflow,
						#[codec(index = 27)]
						ExportError,
						#[codec(index = 28)]
						ReanchorFailed,
						#[codec(index = 29)]
						NoDeal,
						#[codec(index = 30)]
						FeesNotMet,
						#[codec(index = 31)]
						LockError,
						#[codec(index = 32)]
						NoPermission,
						#[codec(index = 33)]
						Unanchored,
						#[codec(index = 34)]
						NotDepositable,
						#[codec(index = 35)]
						UnhandledXcmVersion,
						#[codec(index = 36)]
						WeightLimitReached(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 37)]
						Barrier,
						#[codec(index = 38)]
						WeightNotComputable,
						#[codec(index = 39)]
						ExceedsStackLimit,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Outcome {
						#[codec(index = 0)]
						Complete(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 1)]
						Incomplete(
							runtime_types::sp_weights::weight_v2::Weight,
							runtime_types::xcm::v3::traits::Error,
						),
						#[codec(index = 2)]
						Error(runtime_types::xcm::v3::traits::Error),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v3::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v2::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						want: runtime_types::xcm::v3::multiasset::MultiAssets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v3::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v3::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v3::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::xcm::v3::junction::NetworkId,
						destination: runtime_types::xcm::v3::junctions::Junctions,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						unlocker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						target: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						owner: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						locker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::xcm::v3::multilocation::MultiLocation),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum MaybeErrorCode {
					#[codec(index = 0)]
					Success,
					#[codec(index = 1)]
					Error(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					TruncatedError(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PalletInfo {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					#[codec(compact)]
					pub major: ::core::primitive::u32,
					#[codec(compact)]
					pub minor: ::core::primitive::u32,
					#[codec(compact)]
					pub patch: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryResponseInfo {
					pub destination: runtime_types::xcm::v3::multilocation::MultiLocation,
					#[codec(compact)]
					pub query_id: ::core::primitive::u64,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
					#[codec(index = 4)]
					PalletsInfo(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::xcm::v3::PalletInfo,
						>,
					),
					#[codec(index = 5)]
					DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(runtime_types::sp_weights::weight_v2::Weight),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction>);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedAssetId {
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::AssetId),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedMultiAssets {
				#[codec(index = 1)]
				V2(runtime_types::xcm::v2::multiasset::MultiAssets),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::MultiAssets),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedMultiLocation {
				#[codec(index = 1)]
				V2(runtime_types::xcm::v2::multilocation::MultiLocation),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multilocation::MultiLocation),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedResponse {
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Response),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Response),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedXcm {
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Xcm),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Xcm),
			}
		}
	}
}
