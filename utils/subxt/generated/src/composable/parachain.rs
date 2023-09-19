#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
pub mod api {
	#[allow(unused_imports)]
	mod root_mod {
		pub use super::*;
	}
	pub static PALLETS: [&str; 5usize] = ["System", "Timestamp", "Sudo", "AssetsRegistry", "Ibc"];
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
		#[codec(index = 2)]
		Sudo(sudo::Event),
		#[codec(index = 59)]
		AssetsRegistry(assets_registry::Event),
		#[codec(index = 190)]
		Ibc(ibc::Event),
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
			if pallet_name == "Sudo" {
				return Ok(Event::Sudo(sudo::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "AssetsRegistry" {
				return Ok(Event::AssetsRegistry(assets_registry::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Ibc" {
				return Ok(Event::Ibc(ibc::Event::decode_with_metadata(
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
		pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
			timestamp::constants::ConstantsApi
		}
		pub fn assets_registry(&self) -> assets_registry::constants::ConstantsApi {
			assets_registry::constants::ConstantsApi
		}
		pub fn ibc(&self) -> ibc::constants::ConstantsApi {
			ibc::constants::ConstantsApi
		}
	}
	pub struct StorageApi;
	impl StorageApi {
		pub fn system(&self) -> system::storage::StorageApi {
			system::storage::StorageApi
		}
		pub fn timestamp(&self) -> timestamp::storage::StorageApi {
			timestamp::storage::StorageApi
		}
		pub fn sudo(&self) -> sudo::storage::StorageApi {
			sudo::storage::StorageApi
		}
		pub fn assets_registry(&self) -> assets_registry::storage::StorageApi {
			assets_registry::storage::StorageApi
		}
		pub fn ibc(&self) -> ibc::storage::StorageApi {
			ibc::storage::StorageApi
		}
	}
	pub struct TransactionApi;
	impl TransactionApi {
		pub fn system(&self) -> system::calls::TransactionApi {
			system::calls::TransactionApi
		}
		pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
			timestamp::calls::TransactionApi
		}
		pub fn sudo(&self) -> sudo::calls::TransactionApi {
			sudo::calls::TransactionApi
		}
		pub fn assets_registry(&self) -> assets_registry::calls::TransactionApi {
			assets_registry::calls::TransactionApi
		}
		pub fn ibc(&self) -> ibc::calls::TransactionApi {
			ibc::calls::TransactionApi
		}
	}
	#[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
	pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
		client: &C,
	) -> Result<(), ::subxt::error::MetadataError> {
		let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
		if runtime_metadata_hash !=
			[
				18u8, 35u8, 65u8, 16u8, 194u8, 126u8, 231u8, 175u8, 14u8, 204u8, 109u8, 98u8, 50u8,
				194u8, 106u8, 25u8, 56u8, 224u8, 17u8, 17u8, 141u8, 43u8, 208u8, 188u8, 123u8,
				216u8, 18u8, 68u8, 27u8, 204u8, 61u8, 214u8,
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
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
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
							176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
							114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
							42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
						],
					)
				}
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
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
							176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
							114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
							42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
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
							runtime_types::composable_runtime::RuntimeEvent,
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
							207u8, 24u8, 144u8, 237u8, 51u8, 120u8, 18u8, 105u8, 192u8, 112u8,
							85u8, 233u8, 201u8, 133u8, 182u8, 156u8, 58u8, 160u8, 112u8, 60u8, 6u8,
							159u8, 22u8, 144u8, 112u8, 254u8, 141u8, 18u8, 26u8, 183u8, 251u8,
							115u8,
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
	pub mod sudo {
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
			pub struct Sudo {
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SudoUncheckedWeight {
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SetKey {
				pub new: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
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
			pub struct SudoAs {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn sudo(
					&self,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<Sudo> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo",
						Sudo { call: ::std::boxed::Box::new(call) },
						[
							165u8, 12u8, 249u8, 135u8, 122u8, 100u8, 1u8, 54u8, 178u8, 110u8,
							103u8, 88u8, 173u8, 234u8, 139u8, 255u8, 211u8, 100u8, 25u8, 12u8,
							127u8, 196u8, 253u8, 224u8, 47u8, 63u8, 123u8, 213u8, 135u8, 219u8,
							53u8, 161u8,
						],
					)
				}
				pub fn sudo_unchecked_weight(
					&self,
					call: runtime_types::composable_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<SudoUncheckedWeight> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_unchecked_weight",
						SudoUncheckedWeight { call: ::std::boxed::Box::new(call), weight },
						[
							81u8, 211u8, 66u8, 240u8, 106u8, 150u8, 22u8, 14u8, 252u8, 112u8,
							159u8, 112u8, 117u8, 244u8, 206u8, 185u8, 92u8, 47u8, 34u8, 195u8,
							211u8, 252u8, 191u8, 161u8, 111u8, 157u8, 12u8, 204u8, 163u8, 252u8,
							163u8, 247u8,
						],
					)
				}
				pub fn set_key(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<SetKey> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"set_key",
						SetKey { new },
						[
							34u8, 116u8, 170u8, 18u8, 106u8, 17u8, 231u8, 159u8, 110u8, 246u8, 2u8,
							27u8, 161u8, 155u8, 163u8, 41u8, 138u8, 7u8, 81u8, 98u8, 230u8, 182u8,
							23u8, 222u8, 240u8, 117u8, 173u8, 232u8, 192u8, 55u8, 92u8, 208u8,
						],
					)
				}
				pub fn sudo_as(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<SudoAs> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_as",
						SudoAs { who, call: ::std::boxed::Box::new(call) },
						[
							131u8, 202u8, 146u8, 145u8, 156u8, 80u8, 164u8, 104u8, 187u8, 58u8,
							182u8, 114u8, 5u8, 76u8, 18u8, 87u8, 125u8, 131u8, 172u8, 114u8, 245u8,
							188u8, 43u8, 105u8, 47u8, 159u8, 53u8, 172u8, 149u8, 55u8, 155u8, 42u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
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
			pub struct Sudid {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct KeyChanged {
				pub old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SudoAsDone {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for SudoAsDone {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "SudoAsDone";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn key(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Sudo",
						"Key",
						vec![],
						[
							244u8, 73u8, 188u8, 136u8, 218u8, 163u8, 68u8, 179u8, 122u8, 173u8,
							34u8, 108u8, 137u8, 28u8, 182u8, 16u8, 196u8, 92u8, 138u8, 34u8, 102u8,
							80u8, 199u8, 88u8, 107u8, 207u8, 36u8, 22u8, 168u8, 167u8, 20u8, 142u8,
						],
					)
				}
			}
		}
	}
	pub mod assets_registry {
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
			pub struct RegisterAsset {
				pub protocol_id: [::core::primitive::u8; 4usize],
				pub nonce: ::core::primitive::u64,
				pub location:
					::core::option::Option<runtime_types::primitives::currency::ForeignAssetId>,
				pub asset_info:
					runtime_types::composable_traits::assets::AssetInfo<::core::primitive::u128>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpdateAsset {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub asset_info: runtime_types::composable_traits::assets::AssetInfoUpdate<
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
			pub struct SetMinFee {
				pub target_parachain_id: ::core::primitive::u32,
				pub foreign_asset_id: runtime_types::primitives::currency::ForeignAssetId,
				pub amount: ::core::option::Option<::core::primitive::u128>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpdateAssetLocation {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub location:
					::core::option::Option<runtime_types::primitives::currency::ForeignAssetId>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn register_asset(
					&self,
					protocol_id: [::core::primitive::u8; 4usize],
					nonce: ::core::primitive::u64,
					location: ::core::option::Option<
						runtime_types::primitives::currency::ForeignAssetId,
					>,
					asset_info: runtime_types::composable_traits::assets::AssetInfo<
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::Payload<RegisterAsset> {
					::subxt::tx::Payload::new_static(
						"AssetsRegistry",
						"register_asset",
						RegisterAsset { protocol_id, nonce, location, asset_info },
						[
							114u8, 180u8, 167u8, 148u8, 36u8, 115u8, 16u8, 122u8, 7u8, 26u8, 200u8,
							219u8, 71u8, 87u8, 218u8, 92u8, 204u8, 234u8, 169u8, 232u8, 217u8,
							201u8, 95u8, 119u8, 21u8, 56u8, 1u8, 45u8, 114u8, 0u8, 203u8, 226u8,
						],
					)
				}
				pub fn update_asset(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					asset_info: runtime_types::composable_traits::assets::AssetInfoUpdate<
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::Payload<UpdateAsset> {
					::subxt::tx::Payload::new_static(
						"AssetsRegistry",
						"update_asset",
						UpdateAsset { asset_id, asset_info },
						[
							174u8, 162u8, 224u8, 233u8, 204u8, 95u8, 136u8, 25u8, 59u8, 157u8,
							214u8, 159u8, 224u8, 211u8, 132u8, 172u8, 70u8, 80u8, 127u8, 203u8,
							8u8, 47u8, 230u8, 169u8, 67u8, 189u8, 199u8, 137u8, 83u8, 33u8, 41u8,
							21u8,
						],
					)
				}
				pub fn set_min_fee(
					&self,
					target_parachain_id: ::core::primitive::u32,
					foreign_asset_id: runtime_types::primitives::currency::ForeignAssetId,
					amount: ::core::option::Option<::core::primitive::u128>,
				) -> ::subxt::tx::Payload<SetMinFee> {
					::subxt::tx::Payload::new_static(
						"AssetsRegistry",
						"set_min_fee",
						SetMinFee { target_parachain_id, foreign_asset_id, amount },
						[
							33u8, 96u8, 135u8, 52u8, 165u8, 254u8, 163u8, 23u8, 149u8, 239u8,
							138u8, 96u8, 119u8, 6u8, 92u8, 239u8, 113u8, 68u8, 28u8, 18u8, 15u8,
							129u8, 30u8, 52u8, 233u8, 128u8, 174u8, 68u8, 99u8, 34u8, 151u8, 230u8,
						],
					)
				}
				pub fn update_asset_location(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					location: ::core::option::Option<
						runtime_types::primitives::currency::ForeignAssetId,
					>,
				) -> ::subxt::tx::Payload<UpdateAssetLocation> {
					::subxt::tx::Payload::new_static(
						"AssetsRegistry",
						"update_asset_location",
						UpdateAssetLocation { asset_id, location },
						[
							244u8, 203u8, 171u8, 83u8, 97u8, 77u8, 51u8, 19u8, 162u8, 23u8, 246u8,
							89u8, 191u8, 220u8, 231u8, 162u8, 60u8, 137u8, 2u8, 136u8, 170u8,
							131u8, 188u8, 173u8, 181u8, 110u8, 118u8, 6u8, 229u8, 190u8, 31u8,
							60u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_assets_registry::pallet::Event;
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
			pub struct AssetRegistered {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub location:
					::core::option::Option<runtime_types::primitives::currency::ForeignAssetId>,
				pub asset_info:
					runtime_types::composable_traits::assets::AssetInfo<::core::primitive::u128>,
			}
			impl ::subxt::events::StaticEvent for AssetRegistered {
				const PALLET: &'static str = "AssetsRegistry";
				const EVENT: &'static str = "AssetRegistered";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetUpdated {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub asset_info: runtime_types::composable_traits::assets::AssetInfoUpdate<
					::core::primitive::u128,
				>,
			}
			impl ::subxt::events::StaticEvent for AssetUpdated {
				const PALLET: &'static str = "AssetsRegistry";
				const EVENT: &'static str = "AssetUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetLocationUpdated {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub location: runtime_types::primitives::currency::ForeignAssetId,
			}
			impl ::subxt::events::StaticEvent for AssetLocationUpdated {
				const PALLET: &'static str = "AssetsRegistry";
				const EVENT: &'static str = "AssetLocationUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetLocationRemoved {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			impl ::subxt::events::StaticEvent for AssetLocationRemoved {
				const PALLET: &'static str = "AssetsRegistry";
				const EVENT: &'static str = "AssetLocationRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MinFeeUpdated {
				pub target_parachain_id: ::core::primitive::u32,
				pub foreign_asset_id: runtime_types::primitives::currency::ForeignAssetId,
				pub amount: ::core::option::Option<::core::primitive::u128>,
			}
			impl ::subxt::events::StaticEvent for MinFeeUpdated {
				const PALLET: &'static str = "AssetsRegistry";
				const EVENT: &'static str = "MinFeeUpdated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn local_to_foreign(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitives::currency::ForeignAssetId,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"LocalToForeign",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							176u8, 240u8, 139u8, 24u8, 119u8, 179u8, 119u8, 240u8, 88u8, 131u8,
							7u8, 10u8, 86u8, 65u8, 195u8, 83u8, 130u8, 130u8, 208u8, 50u8, 218u8,
							86u8, 40u8, 46u8, 57u8, 189u8, 69u8, 126u8, 166u8, 111u8, 32u8, 174u8,
						],
					)
				}
				pub fn local_to_foreign_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitives::currency::ForeignAssetId,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"LocalToForeign",
						Vec::new(),
						[
							176u8, 240u8, 139u8, 24u8, 119u8, 179u8, 119u8, 240u8, 88u8, 131u8,
							7u8, 10u8, 86u8, 65u8, 195u8, 83u8, 130u8, 130u8, 208u8, 50u8, 218u8,
							86u8, 40u8, 46u8, 57u8, 189u8, 69u8, 126u8, 166u8, 111u8, 32u8, 174u8,
						],
					)
				}
				pub fn foreign_to_local(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::ForeignAssetId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitives::currency::CurrencyId,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"ForeignToLocal",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							185u8, 118u8, 135u8, 81u8, 214u8, 111u8, 139u8, 184u8, 97u8, 114u8,
							147u8, 90u8, 126u8, 237u8, 117u8, 70u8, 101u8, 74u8, 161u8, 243u8,
							50u8, 105u8, 35u8, 135u8, 50u8, 130u8, 171u8, 95u8, 160u8, 123u8,
							142u8, 235u8,
						],
					)
				}
				pub fn foreign_to_local_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitives::currency::CurrencyId,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"ForeignToLocal",
						Vec::new(),
						[
							185u8, 118u8, 135u8, 81u8, 214u8, 111u8, 139u8, 184u8, 97u8, 114u8,
							147u8, 90u8, 126u8, 237u8, 117u8, 70u8, 101u8, 74u8, 161u8, 243u8,
							50u8, 105u8, 35u8, 135u8, 50u8, 130u8, 171u8, 95u8, 160u8, 123u8,
							142u8, 235u8,
						],
					)
				}
				pub fn min_fee_amounts(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::primitives::currency::ForeignAssetId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"MinFeeAmounts",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							163u8, 140u8, 185u8, 251u8, 253u8, 56u8, 117u8, 169u8, 30u8, 82u8,
							95u8, 163u8, 151u8, 164u8, 132u8, 213u8, 85u8, 89u8, 239u8, 108u8,
							87u8, 0u8, 93u8, 173u8, 229u8, 68u8, 152u8, 156u8, 98u8, 111u8, 30u8,
							142u8,
						],
					)
				}
				pub fn min_fee_amounts_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"MinFeeAmounts",
						Vec::new(),
						[
							163u8, 140u8, 185u8, 251u8, 253u8, 56u8, 117u8, 169u8, 30u8, 82u8,
							95u8, 163u8, 151u8, 164u8, 132u8, 213u8, 85u8, 89u8, 239u8, 108u8,
							87u8, 0u8, 93u8, 173u8, 229u8, 68u8, 152u8, 156u8, 98u8, 111u8, 30u8,
							142u8,
						],
					)
				}
				pub fn asset_ratio(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_traits::currency::Rational64,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetRatio",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							17u8, 185u8, 148u8, 160u8, 80u8, 45u8, 67u8, 207u8, 123u8, 100u8, 65u8,
							207u8, 92u8, 14u8, 93u8, 164u8, 238u8, 254u8, 92u8, 62u8, 210u8, 29u8,
							165u8, 103u8, 62u8, 253u8, 104u8, 46u8, 87u8, 188u8, 2u8, 95u8,
						],
					)
				}
				pub fn asset_ratio_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_traits::currency::Rational64,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetRatio",
						Vec::new(),
						[
							17u8, 185u8, 148u8, 160u8, 80u8, 45u8, 67u8, 207u8, 123u8, 100u8, 65u8,
							207u8, 92u8, 14u8, 93u8, 164u8, 238u8, 254u8, 92u8, 62u8, 210u8, 29u8,
							165u8, 103u8, 62u8, 253u8, 104u8, 46u8, 87u8, 188u8, 2u8, 95u8,
						],
					)
				}
				pub fn existential_deposit(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"ExistentialDeposit",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							127u8, 223u8, 220u8, 128u8, 43u8, 19u8, 146u8, 89u8, 158u8, 164u8,
							31u8, 163u8, 151u8, 74u8, 146u8, 4u8, 168u8, 12u8, 221u8, 73u8, 32u8,
							38u8, 71u8, 33u8, 228u8, 117u8, 213u8, 172u8, 26u8, 210u8, 228u8,
							107u8,
						],
					)
				}
				pub fn existential_deposit_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"ExistentialDeposit",
						Vec::new(),
						[
							127u8, 223u8, 220u8, 128u8, 43u8, 19u8, 146u8, 89u8, 158u8, 164u8,
							31u8, 163u8, 151u8, 74u8, 146u8, 4u8, 168u8, 12u8, 221u8, 73u8, 32u8,
							38u8, 71u8, 33u8, 228u8, 117u8, 213u8, 172u8, 26u8, 210u8, 228u8,
							107u8,
						],
					)
				}				pub fn asset_name (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: primitives :: currency :: CurrencyId > ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetName",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							155u8, 153u8, 205u8, 218u8, 104u8, 221u8, 182u8, 75u8, 75u8, 220u8,
							223u8, 196u8, 241u8, 95u8, 74u8, 117u8, 209u8, 128u8, 19u8, 93u8,
							137u8, 215u8, 238u8, 133u8, 17u8, 66u8, 102u8, 165u8, 63u8, 139u8,
							242u8, 216u8,
						],
					)
				}				pub fn asset_name_root (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetName",
						Vec::new(),
						[
							155u8, 153u8, 205u8, 218u8, 104u8, 221u8, 182u8, 75u8, 75u8, 220u8,
							223u8, 196u8, 241u8, 95u8, 74u8, 117u8, 209u8, 128u8, 19u8, 93u8,
							137u8, 215u8, 238u8, 133u8, 17u8, 66u8, 102u8, 165u8, 63u8, 139u8,
							242u8, 216u8,
						],
					)
				}				pub fn asset_symbol (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: primitives :: currency :: CurrencyId > ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetSymbol",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							33u8, 238u8, 10u8, 174u8, 250u8, 38u8, 250u8, 233u8, 199u8, 123u8,
							195u8, 71u8, 37u8, 117u8, 179u8, 18u8, 168u8, 10u8, 229u8, 196u8,
							122u8, 233u8, 252u8, 173u8, 114u8, 244u8, 200u8, 191u8, 208u8, 209u8,
							251u8, 139u8,
						],
					)
				}				pub fn asset_symbol_root (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetSymbol",
						Vec::new(),
						[
							33u8, 238u8, 10u8, 174u8, 250u8, 38u8, 250u8, 233u8, 199u8, 123u8,
							195u8, 71u8, 37u8, 117u8, 179u8, 18u8, 168u8, 10u8, 229u8, 196u8,
							122u8, 233u8, 252u8, 173u8, 114u8, 244u8, 200u8, 191u8, 208u8, 209u8,
							251u8, 139u8,
						],
					)
				}
				pub fn asset_decimals(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u8,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetDecimals",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							35u8, 231u8, 230u8, 103u8, 90u8, 169u8, 185u8, 105u8, 170u8, 88u8,
							22u8, 22u8, 8u8, 45u8, 92u8, 85u8, 20u8, 170u8, 19u8, 14u8, 66u8,
							142u8, 213u8, 90u8, 202u8, 63u8, 15u8, 230u8, 68u8, 255u8, 178u8,
							238u8,
						],
					)
				}
				pub fn asset_decimals_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u8,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetDecimals",
						Vec::new(),
						[
							35u8, 231u8, 230u8, 103u8, 90u8, 169u8, 185u8, 105u8, 170u8, 88u8,
							22u8, 22u8, 8u8, 45u8, 92u8, 85u8, 20u8, 170u8, 19u8, 14u8, 66u8,
							142u8, 213u8, 90u8, 202u8, 63u8, 15u8, 230u8, 68u8, 255u8, 178u8,
							238u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn network_id(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"AssetsRegistry",
						"NetworkId",
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
	pub mod ibc {
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
			pub struct Deliver {
				pub messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Transfer {
				pub params: runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
				pub memo: ::core::option::Option<::std::string::String>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpgradeClient {
				pub params: runtime_types::pallet_ibc::UpgradeParams,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct FreezeClient {
				pub client_id: ::std::vec::Vec<::core::primitive::u8>,
				pub height: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct IncreaseCounters;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AddChannelsToFeelessChannelList {
				pub source_channel: ::core::primitive::u64,
				pub destination_channel: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RemoveChannelsFromFeelessChannelList {
				pub source_channel: ::core::primitive::u64,
				pub destination_channel: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SetChildStorage {
				pub key: ::std::vec::Vec<::core::primitive::u8>,
				pub value: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SubstituteClientState {
				pub client_id: ::std::string::String,
				pub height: runtime_types::ibc::core::ics02_client::height::Height,
				pub client_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
				pub consensus_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn deliver(
					&self,
					messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
				) -> ::subxt::tx::Payload<Deliver> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"deliver",
						Deliver { messages },
						[
							145u8, 52u8, 143u8, 156u8, 204u8, 181u8, 57u8, 94u8, 85u8, 140u8,
							149u8, 91u8, 203u8, 234u8, 129u8, 192u8, 215u8, 195u8, 53u8, 180u8,
							98u8, 195u8, 113u8, 238u8, 228u8, 88u8, 1u8, 36u8, 173u8, 185u8, 129u8,
							108u8,
						],
					)
				}
				pub fn transfer(
					&self,
					params: runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
					memo: ::core::option::Option<::std::string::String>,
				) -> ::subxt::tx::Payload<Transfer> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"transfer",
						Transfer { params, asset_id, amount, memo },
						[
							41u8, 191u8, 254u8, 178u8, 218u8, 14u8, 149u8, 146u8, 80u8, 247u8,
							198u8, 233u8, 37u8, 24u8, 139u8, 11u8, 211u8, 99u8, 94u8, 17u8, 86u8,
							157u8, 187u8, 67u8, 80u8, 218u8, 88u8, 117u8, 151u8, 11u8, 29u8, 70u8,
						],
					)
				}
				pub fn upgrade_client(
					&self,
					params: runtime_types::pallet_ibc::UpgradeParams,
				) -> ::subxt::tx::Payload<UpgradeClient> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"upgrade_client",
						UpgradeClient { params },
						[
							113u8, 38u8, 218u8, 101u8, 66u8, 187u8, 155u8, 238u8, 185u8, 82u8,
							16u8, 26u8, 35u8, 122u8, 0u8, 124u8, 239u8, 54u8, 134u8, 255u8, 40u8,
							224u8, 3u8, 49u8, 200u8, 214u8, 212u8, 165u8, 224u8, 19u8, 11u8, 28u8,
						],
					)
				}
				pub fn freeze_client(
					&self,
					client_id: ::std::vec::Vec<::core::primitive::u8>,
					height: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<FreezeClient> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"freeze_client",
						FreezeClient { client_id, height },
						[
							71u8, 208u8, 79u8, 70u8, 132u8, 43u8, 127u8, 140u8, 240u8, 38u8, 18u8,
							3u8, 50u8, 179u8, 10u8, 210u8, 28u8, 174u8, 60u8, 81u8, 229u8, 211u8,
							120u8, 55u8, 109u8, 191u8, 182u8, 207u8, 37u8, 52u8, 224u8, 239u8,
						],
					)
				}
				pub fn increase_counters(&self) -> ::subxt::tx::Payload<IncreaseCounters> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"increase_counters",
						IncreaseCounters {},
						[
							129u8, 134u8, 128u8, 27u8, 104u8, 56u8, 20u8, 231u8, 100u8, 38u8, 28u8,
							242u8, 126u8, 191u8, 89u8, 243u8, 178u8, 248u8, 49u8, 138u8, 185u8,
							219u8, 112u8, 238u8, 14u8, 149u8, 67u8, 37u8, 109u8, 119u8, 85u8, 99u8,
						],
					)
				}
				pub fn add_channels_to_feeless_channel_list(
					&self,
					source_channel: ::core::primitive::u64,
					destination_channel: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<AddChannelsToFeelessChannelList> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"add_channels_to_feeless_channel_list",
						AddChannelsToFeelessChannelList { source_channel, destination_channel },
						[
							94u8, 90u8, 107u8, 98u8, 113u8, 134u8, 183u8, 32u8, 208u8, 138u8,
							173u8, 24u8, 152u8, 97u8, 73u8, 1u8, 95u8, 126u8, 203u8, 112u8, 13u8,
							122u8, 126u8, 7u8, 141u8, 110u8, 13u8, 185u8, 252u8, 71u8, 163u8, 18u8,
						],
					)
				}
				pub fn remove_channels_from_feeless_channel_list(
					&self,
					source_channel: ::core::primitive::u64,
					destination_channel: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<RemoveChannelsFromFeelessChannelList> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"remove_channels_from_feeless_channel_list",
						RemoveChannelsFromFeelessChannelList {
							source_channel,
							destination_channel,
						},
						[
							56u8, 207u8, 158u8, 148u8, 9u8, 34u8, 243u8, 213u8, 138u8, 143u8, 10u8,
							115u8, 118u8, 197u8, 187u8, 250u8, 210u8, 187u8, 169u8, 157u8, 158u8,
							61u8, 241u8, 90u8, 117u8, 123u8, 239u8, 105u8, 99u8, 196u8, 254u8,
							116u8,
						],
					)
				}
				pub fn set_child_storage(
					&self,
					key: ::std::vec::Vec<::core::primitive::u8>,
					value: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<SetChildStorage> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"set_child_storage",
						SetChildStorage { key, value },
						[
							54u8, 168u8, 178u8, 188u8, 166u8, 223u8, 180u8, 182u8, 208u8, 217u8,
							154u8, 231u8, 21u8, 88u8, 211u8, 188u8, 63u8, 192u8, 34u8, 236u8,
							153u8, 118u8, 18u8, 41u8, 198u8, 99u8, 241u8, 132u8, 58u8, 170u8, 40u8,
							74u8,
						],
					)
				}
				pub fn substitute_client_state(
					&self,
					client_id: ::std::string::String,
					height: runtime_types::ibc::core::ics02_client::height::Height,
					client_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
					consensus_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<SubstituteClientState> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"substitute_client_state",
						SubstituteClientState {
							client_id,
							height,
							client_state_bytes,
							consensus_state_bytes,
						},
						[
							156u8, 107u8, 88u8, 238u8, 241u8, 13u8, 71u8, 9u8, 14u8, 67u8, 82u8,
							154u8, 205u8, 108u8, 253u8, 145u8, 3u8, 251u8, 93u8, 169u8, 43u8, 26u8,
							16u8, 209u8, 148u8, 111u8, 99u8, 155u8, 32u8, 145u8, 19u8, 149u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_ibc::pallet::Event;
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
			pub struct Events {
				pub events: ::std::vec::Vec<
					::core::result::Result<
						runtime_types::pallet_ibc::events::IbcEvent,
						runtime_types::pallet_ibc::errors::IbcError,
					>,
				>,
			}
			impl ::subxt::events::StaticEvent for Events {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "Events";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TokenTransferInitiated {
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
				pub amount: ::core::primitive::u128,
				pub is_sender_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenTransferInitiated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenTransferInitiated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChannelOpened {
				pub channel_id: ::std::vec::Vec<::core::primitive::u8>,
				pub port_id: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ChannelOpened {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChannelOpened";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ParamsUpdated {
				pub send_enabled: ::core::primitive::bool,
				pub receive_enabled: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for ParamsUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ParamsUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TokenTransferCompleted {
				pub from: runtime_types::ibc::signer::Signer,
				pub to: runtime_types::ibc::signer::Signer,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
				pub amount: ::core::primitive::u128,
				pub is_sender_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenTransferCompleted {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenTransferCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TokenReceived {
				pub from: runtime_types::ibc::signer::Signer,
				pub to: runtime_types::ibc::signer::Signer,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
				pub amount: ::core::primitive::u128,
				pub is_receiver_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenReceived {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenReceived";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TokenTransferFailed {
				pub from: runtime_types::ibc::signer::Signer,
				pub to: runtime_types::ibc::signer::Signer,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
				pub amount: ::core::primitive::u128,
				pub is_sender_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenTransferFailed {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenTransferFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TokenTransferTimeout {
				pub from: runtime_types::ibc::signer::Signer,
				pub to: runtime_types::ibc::signer::Signer,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
				pub amount: ::core::primitive::u128,
				pub is_sender_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenTransferTimeout {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenTransferTimeout";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OnRecvPacketError {
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for OnRecvPacketError {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "OnRecvPacketError";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ClientUpgradeSet;
			impl ::subxt::events::StaticEvent for ClientUpgradeSet {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientUpgradeSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ClientFrozen {
				pub client_id: ::std::vec::Vec<::core::primitive::u8>,
				pub height: ::core::primitive::u64,
				pub revision_number: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ClientFrozen {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientFrozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetAdminUpdated {
				pub admin_account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for AssetAdminUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "AssetAdminUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct FeeLessChannelIdsAdded {
				pub source_channel: ::core::primitive::u64,
				pub destination_channel: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for FeeLessChannelIdsAdded {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "FeeLessChannelIdsAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct FeeLessChannelIdsRemoved {
				pub source_channel: ::core::primitive::u64,
				pub destination_channel: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for FeeLessChannelIdsRemoved {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "FeeLessChannelIdsRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChargingFeeOnTransferInitiated {
				pub sequence: ::core::primitive::u64,
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
				pub amount: ::core::primitive::u128,
				pub is_flat_fee: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ChargingFeeOnTransferInitiated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChargingFeeOnTransferInitiated";
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
			pub struct ChargingFeeConfirmed {
				pub sequence: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ChargingFeeConfirmed {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChargingFeeConfirmed";
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
			pub struct ChargingFeeTimeout {
				pub sequence: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ChargingFeeTimeout {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChargingFeeTimeout";
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
			pub struct ChargingFeeFailedAcknowledgement {
				pub sequence: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ChargingFeeFailedAcknowledgement {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChargingFeeFailedAcknowledgement";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChildStateUpdated;
			impl ::subxt::events::StaticEvent for ChildStateUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChildStateUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ClientStateSubstituted {
				pub client_id: ::std::string::String,
				pub height: runtime_types::ibc::core::ics02_client::height::Height,
			}
			impl ::subxt::events::StaticEvent for ClientStateSubstituted {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientStateSubstituted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoStarted {
				pub account_id: ::subxt::utils::AccountId32,
				pub memo: ::core::option::Option<::std::string::String>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoStarted {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoStarted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoIbcTokenTransferSuccess {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
				pub channel: ::core::primitive::u64,
				pub next_memo: ::core::option::Option<::std::string::String>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoIbcTokenTransferSuccess {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoIbcTokenTransferSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoIbcTokenTransferFailedWithReason {
				pub from: ::subxt::utils::AccountId32,
				pub memo: ::std::string::String,
				pub reason: ::core::primitive::u8,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoIbcTokenTransferFailedWithReason {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoIbcTokenTransferFailedWithReason";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoIbcTokenTransferFailed {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
				pub channel: ::core::primitive::u64,
				pub next_memo: ::core::option::Option<::std::string::String>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoIbcTokenTransferFailed {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoIbcTokenTransferFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoXcmSuccess {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub para_id: ::core::option::Option<::core::primitive::u32>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoXcmSuccess {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoXcmSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecuteMemoXcmFailed {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub para_id: ::core::option::Option<::core::primitive::u32>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoXcmFailed {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoXcmFailed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn client_update_height(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ClientUpdateHeight",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							169u8, 6u8, 192u8, 186u8, 79u8, 156u8, 202u8, 105u8, 213u8, 28u8,
							186u8, 112u8, 216u8, 170u8, 8u8, 166u8, 181u8, 179u8, 111u8, 212u8,
							35u8, 121u8, 7u8, 86u8, 212u8, 69u8, 66u8, 3u8, 19u8, 220u8, 114u8,
							167u8,
						],
					)
				}
				pub fn client_update_height_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ClientUpdateHeight",
						Vec::new(),
						[
							169u8, 6u8, 192u8, 186u8, 79u8, 156u8, 202u8, 105u8, 213u8, 28u8,
							186u8, 112u8, 216u8, 170u8, 8u8, 166u8, 181u8, 179u8, 111u8, 212u8,
							35u8, 121u8, 7u8, 86u8, 212u8, 69u8, 66u8, 3u8, 19u8, 220u8, 114u8,
							167u8,
						],
					)
				}
				pub fn service_charge_out(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ServiceChargeOut",
						vec![],
						[
							3u8, 153u8, 106u8, 100u8, 56u8, 235u8, 77u8, 52u8, 230u8, 105u8, 155u8,
							35u8, 156u8, 113u8, 41u8, 45u8, 92u8, 253u8, 248u8, 97u8, 201u8, 101u8,
							18u8, 85u8, 248u8, 6u8, 200u8, 191u8, 42u8, 67u8, 172u8, 151u8,
						],
					)
				}
				pub fn client_update_time(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ClientUpdateTime",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							98u8, 194u8, 46u8, 221u8, 34u8, 111u8, 178u8, 66u8, 21u8, 234u8, 174u8,
							27u8, 188u8, 45u8, 219u8, 211u8, 68u8, 207u8, 23u8, 228u8, 175u8,
							165u8, 179u8, 18u8, 219u8, 248u8, 34u8, 60u8, 202u8, 106u8, 171u8,
							68u8,
						],
					)
				}
				pub fn client_update_time_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ClientUpdateTime",
						Vec::new(),
						[
							98u8, 194u8, 46u8, 221u8, 34u8, 111u8, 178u8, 66u8, 21u8, 234u8, 174u8,
							27u8, 188u8, 45u8, 219u8, 211u8, 68u8, 207u8, 23u8, 228u8, 175u8,
							165u8, 179u8, 18u8, 219u8, 248u8, 34u8, 60u8, 202u8, 106u8, 171u8,
							68u8,
						],
					)
				}
				pub fn channel_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ChannelCounter",
						vec![],
						[
							227u8, 20u8, 185u8, 41u8, 83u8, 61u8, 150u8, 45u8, 251u8, 243u8, 199u8,
							188u8, 94u8, 160u8, 194u8, 25u8, 245u8, 89u8, 69u8, 105u8, 37u8, 220u8,
							143u8, 106u8, 244u8, 161u8, 215u8, 129u8, 220u8, 79u8, 193u8, 255u8,
						],
					)
				}
				pub fn packet_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PacketCounter",
						vec![],
						[
							90u8, 180u8, 164u8, 48u8, 0u8, 236u8, 78u8, 205u8, 206u8, 248u8, 91u8,
							28u8, 64u8, 96u8, 73u8, 159u8, 230u8, 81u8, 41u8, 88u8, 165u8, 107u8,
							85u8, 85u8, 56u8, 240u8, 122u8, 230u8, 165u8, 216u8, 232u8, 223u8,
						],
					)
				}
				pub fn channels_connection(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ChannelsConnection",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							175u8, 74u8, 214u8, 39u8, 82u8, 72u8, 28u8, 110u8, 105u8, 136u8, 218u8,
							218u8, 110u8, 111u8, 182u8, 21u8, 180u8, 80u8, 66u8, 44u8, 85u8, 138u8,
							56u8, 102u8, 121u8, 201u8, 111u8, 240u8, 73u8, 7u8, 8u8, 115u8,
						],
					)
				}
				pub fn channels_connection_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ChannelsConnection",
						Vec::new(),
						[
							175u8, 74u8, 214u8, 39u8, 82u8, 72u8, 28u8, 110u8, 105u8, 136u8, 218u8,
							218u8, 110u8, 111u8, 182u8, 21u8, 180u8, 80u8, 66u8, 44u8, 85u8, 138u8,
							56u8, 102u8, 121u8, 201u8, 111u8, 240u8, 73u8, 7u8, 8u8, 115u8,
						],
					)
				}
				pub fn fee_less_channel_ids(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"FeeLessChannelIds",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							190u8, 190u8, 60u8, 41u8, 38u8, 36u8, 108u8, 181u8, 192u8, 34u8, 88u8,
							107u8, 188u8, 195u8, 107u8, 232u8, 197u8, 153u8, 16u8, 234u8, 161u8,
							255u8, 37u8, 78u8, 4u8, 7u8, 40u8, 52u8, 9u8, 110u8, 150u8, 216u8,
						],
					)
				}
				pub fn fee_less_channel_ids_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"FeeLessChannelIds",
						Vec::new(),
						[
							190u8, 190u8, 60u8, 41u8, 38u8, 36u8, 108u8, 181u8, 192u8, 34u8, 88u8,
							107u8, 188u8, 195u8, 107u8, 232u8, 197u8, 153u8, 16u8, 234u8, 161u8,
							255u8, 37u8, 78u8, 4u8, 7u8, 40u8, 52u8, 9u8, 110u8, 150u8, 216u8,
						],
					)
				}
				pub fn sequence_fee(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"SequenceFee",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							42u8, 117u8, 77u8, 205u8, 205u8, 54u8, 177u8, 179u8, 247u8, 26u8, 4u8,
							45u8, 160u8, 255u8, 209u8, 2u8, 203u8, 10u8, 54u8, 6u8, 155u8, 44u8,
							186u8, 242u8, 212u8, 31u8, 55u8, 45u8, 90u8, 9u8, 77u8, 119u8,
						],
					)
				}
				pub fn sequence_fee_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"SequenceFee",
						Vec::new(),
						[
							42u8, 117u8, 77u8, 205u8, 205u8, 54u8, 177u8, 179u8, 247u8, 26u8, 4u8,
							45u8, 160u8, 255u8, 209u8, 2u8, 203u8, 10u8, 54u8, 6u8, 155u8, 44u8,
							186u8, 242u8, 212u8, 31u8, 55u8, 45u8, 90u8, 9u8, 77u8, 119u8,
						],
					)
				}
				pub fn client_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ClientCounter",
						vec![],
						[
							236u8, 121u8, 82u8, 20u8, 184u8, 116u8, 197u8, 237u8, 123u8, 236u8,
							221u8, 90u8, 88u8, 89u8, 224u8, 171u8, 143u8, 145u8, 221u8, 242u8,
							195u8, 89u8, 31u8, 185u8, 251u8, 92u8, 250u8, 50u8, 47u8, 171u8, 31u8,
							124u8,
						],
					)
				}
				pub fn connection_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ConnectionCounter",
						vec![],
						[
							155u8, 106u8, 125u8, 78u8, 71u8, 212u8, 217u8, 208u8, 192u8, 39u8,
							220u8, 52u8, 226u8, 76u8, 191u8, 4u8, 196u8, 118u8, 136u8, 3u8, 90u8,
							155u8, 168u8, 89u8, 103u8, 155u8, 220u8, 150u8, 4u8, 118u8, 164u8, 2u8,
						],
					)
				}
				pub fn acknowledgement_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"AcknowledgementCounter",
						vec![],
						[
							169u8, 90u8, 79u8, 11u8, 28u8, 186u8, 46u8, 204u8, 147u8, 76u8, 77u8,
							162u8, 45u8, 137u8, 52u8, 50u8, 67u8, 212u8, 51u8, 49u8, 156u8, 128u8,
							213u8, 182u8, 158u8, 71u8, 152u8, 162u8, 250u8, 196u8, 71u8, 170u8,
						],
					)
				}
				pub fn packet_receipt_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PacketReceiptCounter",
						vec![],
						[
							149u8, 146u8, 199u8, 15u8, 127u8, 62u8, 135u8, 23u8, 188u8, 232u8,
							218u8, 239u8, 194u8, 219u8, 28u8, 34u8, 21u8, 153u8, 149u8, 155u8,
							26u8, 39u8, 50u8, 201u8, 88u8, 36u8, 177u8, 239u8, 55u8, 51u8, 141u8,
							241u8,
						],
					)
				}
				pub fn connection_client(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ConnectionClient",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							134u8, 166u8, 43u8, 43u8, 142u8, 200u8, 83u8, 81u8, 252u8, 1u8, 153u8,
							167u8, 197u8, 170u8, 154u8, 242u8, 241u8, 178u8, 166u8, 147u8, 223u8,
							188u8, 118u8, 48u8, 40u8, 203u8, 29u8, 17u8, 120u8, 250u8, 79u8, 111u8,
						],
					)
				}
				pub fn connection_client_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ConnectionClient",
						Vec::new(),
						[
							134u8, 166u8, 43u8, 43u8, 142u8, 200u8, 83u8, 81u8, 252u8, 1u8, 153u8,
							167u8, 197u8, 170u8, 154u8, 242u8, 241u8, 178u8, 166u8, 147u8, 223u8,
							188u8, 118u8, 48u8, 40u8, 203u8, 29u8, 17u8, 120u8, 250u8, 79u8, 111u8,
						],
					)
				}
				pub fn ibc_asset_ids(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"IbcAssetIds",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							95u8, 163u8, 91u8, 54u8, 240u8, 186u8, 53u8, 241u8, 234u8, 178u8,
							228u8, 71u8, 139u8, 33u8, 124u8, 205u8, 97u8, 75u8, 125u8, 55u8, 234u8,
							37u8, 128u8, 129u8, 119u8, 196u8, 227u8, 212u8, 43u8, 154u8, 153u8,
							214u8,
						],
					)
				}
				pub fn ibc_asset_ids_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"IbcAssetIds",
						Vec::new(),
						[
							95u8, 163u8, 91u8, 54u8, 240u8, 186u8, 53u8, 241u8, 234u8, 178u8,
							228u8, 71u8, 139u8, 33u8, 124u8, 205u8, 97u8, 75u8, 125u8, 55u8, 234u8,
							37u8, 128u8, 129u8, 119u8, 196u8, 227u8, 212u8, 43u8, 154u8, 153u8,
							214u8,
						],
					)
				}
				pub fn counter_for_ibc_asset_ids(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"CounterForIbcAssetIds",
						vec![],
						[
							115u8, 253u8, 221u8, 253u8, 101u8, 232u8, 174u8, 9u8, 2u8, 79u8, 212u8,
							243u8, 233u8, 90u8, 34u8, 251u8, 140u8, 100u8, 153u8, 60u8, 240u8,
							60u8, 36u8, 90u8, 81u8, 31u8, 241u8, 224u8, 157u8, 89u8, 194u8, 105u8,
						],
					)
				}
				pub fn ibc_denoms(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitives::currency::CurrencyId,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"IbcDenoms",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							66u8, 43u8, 17u8, 209u8, 96u8, 87u8, 219u8, 181u8, 26u8, 207u8, 178u8,
							232u8, 121u8, 119u8, 194u8, 108u8, 240u8, 228u8, 95u8, 246u8, 247u8,
							223u8, 55u8, 66u8, 128u8, 110u8, 200u8, 161u8, 164u8, 229u8, 205u8,
							8u8,
						],
					)
				}
				pub fn ibc_denoms_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitives::currency::CurrencyId,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"IbcDenoms",
						Vec::new(),
						[
							66u8, 43u8, 17u8, 209u8, 96u8, 87u8, 219u8, 181u8, 26u8, 207u8, 178u8,
							232u8, 121u8, 119u8, 194u8, 108u8, 240u8, 228u8, 95u8, 246u8, 247u8,
							223u8, 55u8, 66u8, 128u8, 110u8, 200u8, 161u8, 164u8, 229u8, 205u8,
							8u8,
						],
					)
				}
				pub fn counter_for_ibc_denoms(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"CounterForIbcDenoms",
						vec![],
						[
							254u8, 185u8, 194u8, 13u8, 31u8, 147u8, 250u8, 253u8, 56u8, 226u8,
							58u8, 135u8, 7u8, 228u8, 50u8, 135u8, 175u8, 249u8, 5u8, 148u8, 42u8,
							24u8, 125u8, 212u8, 245u8, 134u8, 213u8, 102u8, 17u8, 2u8, 135u8,
							194u8,
						],
					)
				}
				pub fn channel_ids(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ChannelIds",
						vec![],
						[
							21u8, 104u8, 164u8, 90u8, 17u8, 181u8, 235u8, 0u8, 67u8, 67u8, 164u8,
							217u8, 126u8, 131u8, 214u8, 38u8, 143u8, 134u8, 215u8, 173u8, 53u8,
							154u8, 118u8, 215u8, 175u8, 207u8, 14u8, 134u8, 241u8, 215u8, 188u8,
							69u8,
						],
					)
				}
				pub fn escrow_addresses(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"EscrowAddresses",
						vec![],
						[
							184u8, 122u8, 32u8, 42u8, 200u8, 130u8, 61u8, 220u8, 100u8, 177u8,
							62u8, 197u8, 90u8, 210u8, 142u8, 56u8, 70u8, 70u8, 59u8, 246u8, 203u8,
							118u8, 32u8, 237u8, 123u8, 115u8, 73u8, 67u8, 175u8, 199u8, 167u8,
							25u8,
						],
					)
				}
				pub fn consensus_heights(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
						runtime_types::ibc::core::ics02_client::height::Height,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ConsensusHeights",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							238u8, 213u8, 231u8, 8u8, 158u8, 84u8, 100u8, 101u8, 78u8, 142u8,
							125u8, 133u8, 128u8, 92u8, 138u8, 184u8, 144u8, 221u8, 58u8, 101u8,
							206u8, 217u8, 140u8, 30u8, 206u8, 26u8, 242u8, 223u8, 113u8, 46u8,
							227u8, 240u8,
						],
					)
				}
				pub fn consensus_heights_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
						runtime_types::ibc::core::ics02_client::height::Height,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"ConsensusHeights",
						Vec::new(),
						[
							238u8, 213u8, 231u8, 8u8, 158u8, 84u8, 100u8, 101u8, 78u8, 142u8,
							125u8, 133u8, 128u8, 92u8, 138u8, 184u8, 144u8, 221u8, 58u8, 101u8,
							206u8, 217u8, 140u8, 30u8, 206u8, 26u8, 242u8, 223u8, 113u8, 46u8,
							227u8, 240u8,
						],
					)
				}
				pub fn send_packets(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"SendPackets",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							146u8, 209u8, 240u8, 254u8, 152u8, 240u8, 128u8, 54u8, 44u8, 236u8,
							62u8, 147u8, 168u8, 51u8, 64u8, 89u8, 223u8, 204u8, 166u8, 170u8, 28u8,
							93u8, 150u8, 165u8, 173u8, 122u8, 44u8, 215u8, 219u8, 10u8, 249u8,
							133u8,
						],
					)
				}
				pub fn send_packets_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"SendPackets",
						Vec::new(),
						[
							146u8, 209u8, 240u8, 254u8, 152u8, 240u8, 128u8, 54u8, 44u8, 236u8,
							62u8, 147u8, 168u8, 51u8, 64u8, 89u8, 223u8, 204u8, 166u8, 170u8, 28u8,
							93u8, 150u8, 165u8, 173u8, 122u8, 44u8, 215u8, 219u8, 10u8, 249u8,
							133u8,
						],
					)
				}
				pub fn recv_packets(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"RecvPackets",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							131u8, 144u8, 185u8, 91u8, 9u8, 190u8, 201u8, 215u8, 217u8, 147u8,
							53u8, 137u8, 26u8, 201u8, 49u8, 43u8, 53u8, 129u8, 103u8, 20u8, 150u8,
							103u8, 169u8, 2u8, 147u8, 89u8, 43u8, 198u8, 144u8, 125u8, 96u8, 131u8,
						],
					)
				}
				pub fn recv_packets_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"RecvPackets",
						Vec::new(),
						[
							131u8, 144u8, 185u8, 91u8, 9u8, 190u8, 201u8, 215u8, 217u8, 147u8,
							53u8, 137u8, 26u8, 201u8, 49u8, 43u8, 53u8, 129u8, 103u8, 20u8, 150u8,
							103u8, 169u8, 2u8, 147u8, 89u8, 43u8, 198u8, 144u8, 125u8, 96u8, 131u8,
						],
					)
				}
				pub fn acks(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"Acks",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							193u8, 242u8, 208u8, 150u8, 1u8, 103u8, 241u8, 98u8, 227u8, 26u8,
							158u8, 161u8, 14u8, 230u8, 134u8, 210u8, 154u8, 177u8, 14u8, 216u8,
							134u8, 72u8, 102u8, 128u8, 21u8, 77u8, 164u8, 95u8, 40u8, 96u8, 205u8,
							4u8,
						],
					)
				}
				pub fn acks_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"Acks",
						Vec::new(),
						[
							193u8, 242u8, 208u8, 150u8, 1u8, 103u8, 241u8, 98u8, 227u8, 26u8,
							158u8, 161u8, 14u8, 230u8, 134u8, 210u8, 154u8, 177u8, 14u8, 216u8,
							134u8, 72u8, 102u8, 128u8, 21u8, 77u8, 164u8, 95u8, 40u8, 96u8, 205u8,
							4u8,
						],
					)
				}
				pub fn pending_send_packet_seqs(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::vec::Vec<::core::primitive::u64>, ::core::primitive::u64),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PendingSendPacketSeqs",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							177u8, 235u8, 231u8, 90u8, 78u8, 216u8, 49u8, 192u8, 170u8, 167u8,
							215u8, 146u8, 83u8, 146u8, 76u8, 117u8, 40u8, 104u8, 7u8, 182u8, 56u8,
							30u8, 14u8, 255u8, 236u8, 34u8, 176u8, 197u8, 78u8, 220u8, 34u8, 224u8,
						],
					)
				}
				pub fn pending_send_packet_seqs_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::vec::Vec<::core::primitive::u64>, ::core::primitive::u64),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PendingSendPacketSeqs",
						Vec::new(),
						[
							177u8, 235u8, 231u8, 90u8, 78u8, 216u8, 49u8, 192u8, 170u8, 167u8,
							215u8, 146u8, 83u8, 146u8, 76u8, 117u8, 40u8, 104u8, 7u8, 182u8, 56u8,
							30u8, 14u8, 255u8, 236u8, 34u8, 176u8, 197u8, 78u8, 220u8, 34u8, 224u8,
						],
					)
				}
				pub fn pending_recv_packet_seqs(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::vec::Vec<::core::primitive::u64>, ::core::primitive::u64),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PendingRecvPacketSeqs",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							206u8, 66u8, 91u8, 112u8, 240u8, 28u8, 169u8, 232u8, 243u8, 211u8,
							174u8, 107u8, 109u8, 148u8, 165u8, 170u8, 28u8, 213u8, 221u8, 180u8,
							188u8, 250u8, 94u8, 128u8, 92u8, 177u8, 207u8, 36u8, 190u8, 3u8, 72u8,
							154u8,
						],
					)
				}
				pub fn pending_recv_packet_seqs_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::vec::Vec<::core::primitive::u64>, ::core::primitive::u64),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PendingRecvPacketSeqs",
						Vec::new(),
						[
							206u8, 66u8, 91u8, 112u8, 240u8, 28u8, 169u8, 232u8, 243u8, 211u8,
							174u8, 107u8, 109u8, 148u8, 165u8, 170u8, 28u8, 213u8, 221u8, 180u8,
							188u8, 250u8, 94u8, 128u8, 92u8, 177u8, 207u8, 36u8, 190u8, 3u8, 72u8,
							154u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn native_asset_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::primitives::currency::CurrencyId> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"NativeAssetId",
						[
							150u8, 207u8, 49u8, 178u8, 254u8, 209u8, 81u8, 36u8, 235u8, 117u8,
							62u8, 166u8, 4u8, 173u8, 64u8, 189u8, 19u8, 182u8, 131u8, 166u8, 234u8,
							145u8, 83u8, 23u8, 246u8, 20u8, 47u8, 34u8, 66u8, 162u8, 146u8, 49u8,
						],
					)
				}
				pub fn pallet_prefix(
					&self,
				) -> ::subxt::constants::Address<::std::vec::Vec<::core::primitive::u8>> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"PalletPrefix",
						[
							106u8, 50u8, 57u8, 116u8, 43u8, 202u8, 37u8, 248u8, 102u8, 22u8, 62u8,
							22u8, 242u8, 54u8, 152u8, 168u8, 107u8, 64u8, 72u8, 172u8, 124u8, 40u8,
							42u8, 110u8, 104u8, 145u8, 31u8, 144u8, 242u8, 189u8, 145u8, 208u8,
						],
					)
				}
				pub fn light_client_protocol(
					&self,
				) -> ::subxt::constants::Address<runtime_types::pallet_ibc::LightClientProtocol> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"LightClientProtocol",
						[
							200u8, 116u8, 16u8, 82u8, 41u8, 118u8, 80u8, 243u8, 53u8, 143u8, 22u8,
							2u8, 167u8, 246u8, 7u8, 151u8, 169u8, 50u8, 102u8, 67u8, 255u8, 148u8,
							204u8, 202u8, 89u8, 187u8, 48u8, 204u8, 36u8, 113u8, 253u8, 204u8,
						],
					)
				}
				pub fn expected_block_time(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"ExpectedBlockTime",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				pub fn minimum_connection_delay(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"MinimumConnectionDelay",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				pub fn spam_protection_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"SpamProtectionDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn clean_up_packets_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Ibc",
						"CleanUpPacketsPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn service_charge_out(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Perbill>
				{
					::subxt::constants::Address::new_static(
						"Ibc",
						"ServiceChargeOut",
						[
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
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
			pub mod bounded_btree_set {
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
				pub struct BoundedBTreeSet<_0>(pub ::std::vec::Vec<_0>);
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
		pub mod common {
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
			pub struct MaxStringSize;
		}
		pub mod composable_runtime {
			use super::runtime_types;
			pub mod opaque {
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
				pub struct SessionKeys {
					pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
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
			pub enum OriginCaller {
				#[codec(index = 0)]
				system(
					runtime_types::frame_support::dispatch::RawOrigin<::subxt::utils::AccountId32>,
				),
				#[codec(index = 30)]
				Council(runtime_types::pallet_collective::RawOrigin<::subxt::utils::AccountId32>),
				#[codec(index = 72)]
				TechnicalCommittee(
					runtime_types::pallet_collective::RawOrigin<::subxt::utils::AccountId32>,
				),
				#[codec(index = 74)]
				ReleaseCommittee(
					runtime_types::pallet_collective::RawOrigin<::subxt::utils::AccountId32>,
				),
				#[codec(index = 41)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Origin),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Origin),
				#[codec(index = 79)]
				Origins(runtime_types::pallet_custom_origins::pallet::Origin),
				#[codec(index = 7)]
				Void(runtime_types::sp_core::Void),
			}
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
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 2)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
				#[codec(index = 12)]
				AssetTxPayment(runtime_types::pallet_asset_tx_payment::pallet::Call),
				#[codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Call),
				#[codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 8)]
				Multisig(runtime_types::pallet_multisig::pallet::Call),
				#[codec(index = 10)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
				#[codec(index = 11)]
				ParachainInfo(runtime_types::parachain_info::pallet::Call),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 30)]
				Council(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 31)]
				CouncilMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 32)]
				Treasury(runtime_types::pallet_treasury::pallet::Call),
				#[codec(index = 33)]
				Democracy(runtime_types::pallet_democracy::pallet::Call),
				#[codec(index = 72)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 73)]
				TechnicalCommitteeMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 74)]
				ReleaseCommittee(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 75)]
				ReleaseMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 34)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Call),
				#[codec(index = 35)]
				Utility(runtime_types::pallet_utility::pallet::Call),
				#[codec(index = 36)]
				Preimage(runtime_types::pallet_preimage::pallet::Call),
				#[codec(index = 37)]
				Proxy(runtime_types::pallet_proxy::pallet::Call),
				#[codec(index = 40)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
				#[codec(index = 41)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
				#[codec(index = 43)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
				#[codec(index = 44)]
				XTokens(runtime_types::orml_xtokens::module::Call),
				#[codec(index = 45)]
				UnknownTokens(runtime_types::orml_unknown_tokens::module::Call),
				#[codec(index = 52)]
				Tokens(runtime_types::orml_tokens::module::Call),
				#[codec(index = 53)]
				CurrencyFactory(runtime_types::pallet_currency_factory::pallet::Call),
				#[codec(index = 56)]
				CrowdloanRewards(runtime_types::pallet_crowdloan_rewards::pallet::Call),
				#[codec(index = 57)]
				Assets(runtime_types::pallet_assets::pallet::Call),
				#[codec(index = 59)]
				AssetsRegistry(runtime_types::pallet_assets_registry::pallet::Call),
				#[codec(index = 76)]
				Referenda(runtime_types::pallet_referenda::pallet::Call),
				#[codec(index = 77)]
				ConvictionVoting(runtime_types::pallet_conviction_voting::pallet::Call),
				#[codec(index = 78)]
				OpenGovBalances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 80)]
				Whitelist(runtime_types::pallet_whitelist::pallet::Call),
				#[codec(index = 100)]
				CallFilter(runtime_types::pallet_call_filter::pallet::Call),
				#[codec(index = 190)]
				Ibc(runtime_types::pallet_ibc::pallet::Call),
				#[codec(index = 191)]
				Ics20Fee(runtime_types::pallet_ibc::ics20_fee::pallet::Call),
				#[codec(index = 192)]
				PalletMultihopXcmIbc(runtime_types::pallet_multihop_xcm_ibc::pallet::Call),
			}
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
				#[codec(index = 2)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
				#[codec(index = 4)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Event),
				#[codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 8)]
				Multisig(runtime_types::pallet_multisig::pallet::Event),
				#[codec(index = 10)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 30)]
				Council(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 31)]
				CouncilMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 32)]
				Treasury(runtime_types::pallet_treasury::pallet::Event),
				#[codec(index = 33)]
				Democracy(runtime_types::pallet_democracy::pallet::Event),
				#[codec(index = 72)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 73)]
				TechnicalCommitteeMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 74)]
				ReleaseCommittee(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 75)]
				ReleaseMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 34)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Event),
				#[codec(index = 35)]
				Utility(runtime_types::pallet_utility::pallet::Event),
				#[codec(index = 36)]
				Preimage(runtime_types::pallet_preimage::pallet::Event),
				#[codec(index = 37)]
				Proxy(runtime_types::pallet_proxy::pallet::Event),
				#[codec(index = 40)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
				#[codec(index = 41)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
				#[codec(index = 43)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
				#[codec(index = 44)]
				XTokens(runtime_types::orml_xtokens::module::Event),
				#[codec(index = 45)]
				UnknownTokens(runtime_types::orml_unknown_tokens::module::Event),
				#[codec(index = 52)]
				Tokens(runtime_types::orml_tokens::module::Event),
				#[codec(index = 53)]
				CurrencyFactory(runtime_types::pallet_currency_factory::pallet::Event),
				#[codec(index = 56)]
				CrowdloanRewards(runtime_types::pallet_crowdloan_rewards::pallet::Event),
				#[codec(index = 59)]
				AssetsRegistry(runtime_types::pallet_assets_registry::pallet::Event),
				#[codec(index = 76)]
				Referenda(runtime_types::pallet_referenda::pallet::Event),
				#[codec(index = 77)]
				ConvictionVoting(runtime_types::pallet_conviction_voting::pallet::Event),
				#[codec(index = 78)]
				OpenGovBalances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 80)]
				Whitelist(runtime_types::pallet_whitelist::pallet::Event),
				#[codec(index = 100)]
				CallFilter(runtime_types::pallet_call_filter::pallet::Event),
				#[codec(index = 190)]
				Ibc(runtime_types::pallet_ibc::pallet::Event),
				#[codec(index = 191)]
				Ics20Fee(runtime_types::pallet_ibc::ics20_fee::pallet::Event),
				#[codec(index = 192)]
				PalletMultihopXcmIbc(runtime_types::pallet_multihop_xcm_ibc::pallet::Event),
			}
		}
		pub mod composable_support {
			use super::runtime_types;
			pub mod collections {
				use super::runtime_types;
				pub mod vec {
					use super::runtime_types;
					pub mod bounded {
						use super::runtime_types;
						pub mod bi_bounded_vec {
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
							pub struct BiBoundedVec<_0> {
								pub inner: ::std::vec::Vec<_0>,
							}
						}
					}
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
			}
		}
		pub mod composable_traits {
			use super::runtime_types;
			pub mod account_proxy {
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
				pub enum ProxyType {
					#[codec(index = 0)]
					Any,
					#[codec(index = 1)]
					Governance,
					#[codec(index = 2)]
					CancelProxy,
					#[codec(index = 3)]
					Bridge,
					#[codec(index = 4)]
					Assets,
					#[codec(index = 5)]
					Defi,
					#[codec(index = 6)]
					Oracle,
					#[codec(index = 7)]
					Contracts,
				}
			}
			pub mod assets {
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
				pub struct AssetInfo < _0 > { pub name : :: core :: option :: Option < runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > > , pub symbol : :: core :: option :: Option < runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > > , pub decimals : :: core :: option :: Option < :: core :: primitive :: u8 > , pub existential_deposit : _0 , pub ratio : :: core :: option :: Option < runtime_types :: composable_traits :: currency :: Rational64 > , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AssetInfoUpdate < _0 > { pub name : runtime_types :: composable_traits :: storage :: UpdateValue < :: core :: option :: Option < runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > > > , pub symbol : runtime_types :: composable_traits :: storage :: UpdateValue < :: core :: option :: Option < runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > > > , pub decimals : runtime_types :: composable_traits :: storage :: UpdateValue < :: core :: option :: Option < :: core :: primitive :: u8 > > , pub existential_deposit : runtime_types :: composable_traits :: storage :: UpdateValue < _0 > , pub ratio : runtime_types :: composable_traits :: storage :: UpdateValue < :: core :: option :: Option < runtime_types :: composable_traits :: currency :: Rational64 > > , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BasicAssetMetadata { pub symbol : runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , pub name : runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , }
			}
			pub mod currency {
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
				pub struct Rational64 {
					pub n: ::core::primitive::u64,
					pub d: ::core::primitive::u64,
				}
			}
			pub mod storage {
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
				pub enum UpdateValue<_0> {
					#[codec(index = 0)]
					Ignore,
					#[codec(index = 1)]
					Set(_0),
				}
			}
			pub mod xcm {
				use super::runtime_types;
				pub mod memo {
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
					pub enum ChainHop {
						#[codec(index = 0)]
						SubstrateIbc,
						#[codec(index = 1)]
						CosmosIbc,
						#[codec(index = 2)]
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
					pub struct ChainInfo {
						pub chain_id: ::core::primitive::u32,
						pub order: ::core::primitive::u8,
						pub channel_id: ::core::primitive::u64,
						pub timestamp: ::core::option::Option<::core::primitive::u64>,
						pub height: ::core::option::Option<::core::primitive::u64>,
						pub retries: ::core::option::Option<::core::primitive::u8>,
						pub timeout: ::core::option::Option<::core::primitive::u64>,
						pub chain_hop: runtime_types::composable_traits::xcm::memo::ChainHop,
						pub para_id: ::core::option::Option<::core::primitive::u32>,
					}
				}
			}
		}
		pub mod cumulus_pallet_dmp_queue {
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
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: runtime_types::sp_weights::weight_v2::Weight,
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
					Unknown,
					#[codec(index = 1)]
					OverLimit,
				}
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
					InvalidFormat { message_id: [::core::primitive::u8; 32usize] },
					#[codec(index = 1)]
					UnsupportedVersion { message_id: [::core::primitive::u8; 32usize] },
					#[codec(index = 2)]
					ExecutedDownward {
						message_id: [::core::primitive::u8; 32usize],
						outcome: runtime_types::xcm::v3::traits::Outcome,
					},
					#[codec(index = 3)]
					WeightExhausted {
						message_id: [::core::primitive::u8; 32usize],
						remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 4)]
					OverweightEnqueued {
						message_id: [::core::primitive::u8; 32usize],
						overweight_index: ::core::primitive::u64,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 5)]
					OverweightServiced {
						overweight_index: ::core::primitive::u64,
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 6)]
					MaxMessagesExhausted { message_id: [::core::primitive::u8; 32usize] },
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
			pub struct ConfigData {
				pub max_individual: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PageIndexData {
				pub begin_used: ::core::primitive::u32,
				pub end_used: ::core::primitive::u32,
				pub overweight_count: ::core::primitive::u64,
			}
		}
		pub mod cumulus_pallet_parachain_system {
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
					# [codec (index = 0)] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] authorize_upgrade { code_hash : :: subxt :: utils :: H256 , } , # [codec (index = 3)] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
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
					OverlappingUpgrades,
					#[codec(index = 1)]
					ProhibitedByPolkadot,
					#[codec(index = 2)]
					TooBig,
					#[codec(index = 3)]
					ValidationDataNotAvailable,
					#[codec(index = 4)]
					HostConfigurationNotAvailable,
					#[codec(index = 5)]
					NotScheduled,
					#[codec(index = 6)]
					NothingAuthorized,
					#[codec(index = 7)]
					Unauthorized,
				}
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
					ValidationFunctionStored,
					#[codec(index = 1)]
					ValidationFunctionApplied { relay_chain_block_num: ::core::primitive::u32 },
					#[codec(index = 2)]
					ValidationFunctionDiscarded,
					#[codec(index = 3)]
					UpgradeAuthorized { code_hash: ::subxt::utils::H256 },
					#[codec(index = 4)]
					DownwardMessagesReceived { count: ::core::primitive::u32 },
					#[codec(index = 5)]
					DownwardMessagesProcessed {
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
						dmq_head: ::subxt::utils::H256,
					},
					#[codec(index = 6)]
					UpwardMessageSent {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			pub mod relay_state_snapshot {
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
				pub struct MessagingStateSnapshot {
					pub dmq_mqc_head: ::subxt::utils::H256,
					pub relay_dispatch_queue_size: (::core::primitive::u32, ::core::primitive::u32),
					pub ingress_channels: ::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::polkadot_primitives::v2::AbridgedHrmpChannel,
					)>,
					pub egress_channels: ::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::polkadot_primitives::v2::AbridgedHrmpChannel,
					)>,
				}
			}
		}
		pub mod cumulus_pallet_xcm {
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
				pub enum Error {}
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
					InvalidFormat([::core::primitive::u8; 32usize]),
					#[codec(index = 1)]
					UnsupportedVersion([::core::primitive::u8; 32usize]),
					#[codec(index = 2)]
					ExecutedDownward(
						[::core::primitive::u8; 32usize],
						runtime_types::xcm::v3::traits::Outcome,
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
					Relay,
					#[codec(index = 1)]
					SiblingParachain(runtime_types::polkadot_parachain::primitives::Id),
				}
			}
		}
		pub mod cumulus_pallet_xcmp_queue {
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
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 1)]
					suspend_xcm_execution,
					#[codec(index = 2)]
					resume_xcm_execution,
					#[codec(index = 3)]
					update_suspend_threshold { new: ::core::primitive::u32 },
					#[codec(index = 4)]
					update_drop_threshold { new: ::core::primitive::u32 },
					#[codec(index = 5)]
					update_resume_threshold { new: ::core::primitive::u32 },
					#[codec(index = 6)]
					update_threshold_weight { new: runtime_types::sp_weights::weight_v2::Weight },
					#[codec(index = 7)]
					update_weight_restrict_decay {
						new: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 8)]
					update_xcmp_max_individual_weight {
						new: runtime_types::sp_weights::weight_v2::Weight,
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
					FailedToSend,
					#[codec(index = 1)]
					BadXcmOrigin,
					#[codec(index = 2)]
					BadXcm,
					#[codec(index = 3)]
					BadOverweightIndex,
					#[codec(index = 4)]
					WeightOverLimit,
				}
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
					Success {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 1)]
					Fail {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						error: runtime_types::xcm::v3::traits::Error,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					BadVersion {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 3)]
					BadFormat {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 4)]
					XcmpMessageSent {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 5)]
					OverweightEnqueued {
						sender: runtime_types::polkadot_parachain::primitives::Id,
						sent_at: ::core::primitive::u32,
						index: ::core::primitive::u64,
						required: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 6)]
					OverweightServiced {
						index: ::core::primitive::u64,
						used: runtime_types::sp_weights::weight_v2::Weight,
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
			pub struct InboundChannelDetails {
				pub sender: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::InboundState,
				pub message_metadata: ::std::vec::Vec<(
					::core::primitive::u32,
					runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
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
			pub enum InboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OutboundChannelDetails {
				pub recipient: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::OutboundState,
				pub signals_exist: ::core::primitive::bool,
				pub first_index: ::core::primitive::u16,
				pub last_index: ::core::primitive::u16,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum OutboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct QueueConfigData {
				pub suspend_threshold: ::core::primitive::u32,
				pub drop_threshold: ::core::primitive::u32,
				pub resume_threshold: ::core::primitive::u32,
				pub threshold_weight: runtime_types::sp_weights::weight_v2::Weight,
				pub weight_restrict_decay: runtime_types::sp_weights::weight_v2::Weight,
				pub xcmp_max_individual_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
		}
		pub mod cumulus_primitives_parachain_inherent {
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
			pub struct MessageQueueChain(pub ::subxt::utils::H256);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ParachainInherentData {
				pub validation_data:
					runtime_types::polkadot_primitives::v2::PersistedValidationData<
						::subxt::utils::H256,
						::core::primitive::u32,
					>,
				pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
				pub downward_messages: ::std::vec::Vec<
					runtime_types::polkadot_core_primitives::InboundDownwardMessage<
						::core::primitive::u32,
					>,
				>,
				pub horizontal_messages: ::subxt::utils::KeyedVec<
					runtime_types::polkadot_parachain::primitives::Id,
					::std::vec::Vec<
						runtime_types::polkadot_core_primitives::InboundHrmpMessage<
							::core::primitive::u32,
						>,
					>,
				>,
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
		pub mod ibc {
			use super::runtime_types;
			pub mod applications {
				use super::runtime_types;
				pub mod transfer {
					use super::runtime_types;
					pub mod denom {
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
						pub struct BaseDenom(pub ::std::string::String);
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub struct PrefixedDenom {
							pub trace_path:
								runtime_types::ibc::applications::transfer::denom::TracePath,
							pub base_denom:
								runtime_types::ibc::applications::transfer::denom::BaseDenom,
						}
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub struct TracePath(
							pub  ::std::vec::Vec<
								runtime_types::ibc::applications::transfer::denom::TracePrefix,
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
						pub struct TracePrefix {
							pub port_id: runtime_types::ibc::core::ics24_host::identifier::PortId,
							pub channel_id:
								runtime_types::ibc::core::ics24_host::identifier::ChannelId,
						}
					}
				}
			}
			pub mod core {
				use super::runtime_types;
				pub mod ics02_client {
					use super::runtime_types;
					pub mod height {
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
						pub struct Height {
							pub revision_number: ::core::primitive::u64,
							pub revision_height: ::core::primitive::u64,
						}
					}
				}
				pub mod ics24_host {
					use super::runtime_types;
					pub mod identifier {
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
						pub struct ChannelId(pub ::std::string::String);
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub struct PortId(pub ::std::string::String);
					}
				}
			}
			pub mod signer {
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
				pub struct Signer(pub ::std::string::String);
			}
		}
		pub mod ibc_primitives {
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
			pub enum Timeout {
				#[codec(index = 0)]
				Offset {
					timestamp: ::core::option::Option<::core::primitive::u64>,
					height: ::core::option::Option<::core::primitive::u64>,
				},
				#[codec(index = 1)]
				Absolute {
					timestamp: ::core::option::Option<::core::primitive::u64>,
					height: ::core::option::Option<::core::primitive::u64>,
				},
			}
		}
		pub mod orml_tokens {
			use super::runtime_types;
			pub mod module {
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
					transfer {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					transfer_all {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					transfer_keep_alive {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					force_transfer {
						source: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					set_balance {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						new_reserved: ::core::primitive::u128,
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
					BalanceTooLow,
					#[codec(index = 1)]
					AmountIntoBalanceFailed,
					#[codec(index = 2)]
					LiquidityRestrictions,
					#[codec(index = 3)]
					MaxLocksExceeded,
					#[codec(index = 4)]
					KeepAlive,
					#[codec(index = 5)]
					ExistentialDeposit,
					#[codec(index = 6)]
					DeadAccount,
					#[codec(index = 7)]
					TooManyReserves,
				}
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
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					DustLost {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					Transfer {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					Reserved {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					Unreserved {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					ReserveRepatriated {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 6)]
					BalanceSet {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						free: ::core::primitive::u128,
						reserved: ::core::primitive::u128,
					},
					#[codec(index = 7)]
					TotalIssuanceSet {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					Withdrawn {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					Slashed {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						free_amount: ::core::primitive::u128,
						reserved_amount: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					Deposited {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					LockSet {
						lock_id: [::core::primitive::u8; 8usize],
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					LockRemoved {
						lock_id: [::core::primitive::u8; 8usize],
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
					},
					#[codec(index = 13)]
					Locked {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 14)]
					Unlocked {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
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
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub frozen: _0,
			}
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
			}
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
		pub mod orml_unknown_tokens {
			use super::runtime_types;
			pub mod module {
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
					BalanceTooLow,
					#[codec(index = 1)]
					BalanceOverflow,
					#[codec(index = 2)]
					UnhandledAsset,
				}
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
					Deposited {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						who: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 1)]
					Withdrawn {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						who: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
				}
			}
		}
		pub mod orml_xtokens {
			use super::runtime_types;
			pub mod module {
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
					transfer {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 1)]
					transfer_multiasset {
						asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 2)]
					transfer_with_fee {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						fee: ::core::primitive::u128,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 3)]
					transfer_multiasset_with_fee {
						asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						fee: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 4)]
					transfer_multicurrencies {
						currencies: ::std::vec::Vec<(
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
						)>,
						fee_item: ::core::primitive::u32,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 5)]
					transfer_multiassets {
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_item: ::core::primitive::u32,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
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
					AssetHasNoReserve,
					#[codec(index = 1)]
					NotCrossChainTransfer,
					#[codec(index = 2)]
					InvalidDest,
					#[codec(index = 3)]
					NotCrossChainTransferableCurrency,
					#[codec(index = 4)]
					UnweighableMessage,
					#[codec(index = 5)]
					XcmExecutionFailed,
					#[codec(index = 6)]
					CannotReanchor,
					#[codec(index = 7)]
					InvalidAncestry,
					#[codec(index = 8)]
					InvalidAsset,
					#[codec(index = 9)]
					DestinationNotInvertible,
					#[codec(index = 10)]
					BadVersion,
					#[codec(index = 11)]
					DistinctReserveForAssetAndFee,
					#[codec(index = 12)]
					ZeroFee,
					#[codec(index = 13)]
					ZeroAmount,
					#[codec(index = 14)]
					TooManyAssetsBeingSent,
					#[codec(index = 15)]
					AssetIndexNonExistent,
					#[codec(index = 16)]
					FeeNotEnough,
					#[codec(index = 17)]
					NotSupportedMultiLocation,
					#[codec(index = 18)]
					MinXcmFeeNotDefined,
				}
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
					TransferredMultiAssets {
						sender: ::subxt::utils::AccountId32,
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						fee: runtime_types::xcm::v3::multiasset::MultiAsset,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
				}
			}
		}
		pub mod pallet_asset_tx_payment {
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
					set_payment_asset {
						payer: ::subxt::utils::AccountId32,
						asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
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
			pub struct ChargeAssetTxPayment {
				#[codec(compact)]
				pub tip: ::core::primitive::u128,
				pub asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
			}
		}
		pub mod pallet_assets {
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
					transfer {
						asset: runtime_types::primitives::currency::CurrencyId,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 1)]
					transfer_native {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					force_transfer {
						asset: runtime_types::primitives::currency::CurrencyId,
						source: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 3)]
					force_transfer_native {
						source: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					transfer_all {
						asset: runtime_types::primitives::currency::CurrencyId,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					transfer_all_native {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 6)]
					mint_initialize {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 7)]
					mint_initialize_with_governance {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						governance_origin: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 8)]
					mint_into {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					burn_from {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
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
					CannotSetNewCurrencyToRegistry,
					#[codec(index = 1)]
					InvalidCurrency,
				}
			}
		}
		pub mod pallet_assets_registry {
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
					register_asset {
						protocol_id: [::core::primitive::u8; 4usize],
						nonce: ::core::primitive::u64,
						location: ::core::option::Option<
							runtime_types::primitives::currency::ForeignAssetId,
						>,
						asset_info: runtime_types::composable_traits::assets::AssetInfo<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 1)]
					update_asset {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						asset_info: runtime_types::composable_traits::assets::AssetInfoUpdate<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 2)]
					set_min_fee {
						target_parachain_id: ::core::primitive::u32,
						foreign_asset_id: runtime_types::primitives::currency::ForeignAssetId,
						amount: ::core::option::Option<::core::primitive::u128>,
					},
					#[codec(index = 3)]
					update_asset_location {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						location: ::core::option::Option<
							runtime_types::primitives::currency::ForeignAssetId,
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
					AssetNotFound,
					#[codec(index = 1)]
					AssetAlreadyRegistered,
					#[codec(index = 2)]
					AssetLocationIsNone,
					#[codec(index = 3)]
					StringExceedsMaxLength,
					#[codec(index = 4)]
					LocationIsUsed,
				}
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
					AssetRegistered {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						location: ::core::option::Option<
							runtime_types::primitives::currency::ForeignAssetId,
						>,
						asset_info: runtime_types::composable_traits::assets::AssetInfo<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 1)]
					AssetUpdated {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						asset_info: runtime_types::composable_traits::assets::AssetInfoUpdate<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 2)]
					AssetLocationUpdated {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						location: runtime_types::primitives::currency::ForeignAssetId,
					},
					#[codec(index = 3)]
					AssetLocationRemoved {
						asset_id: runtime_types::primitives::currency::CurrencyId,
					},
					#[codec(index = 4)]
					MinFeeUpdated {
						target_parachain_id: ::core::primitive::u32,
						foreign_asset_id: runtime_types::primitives::currency::ForeignAssetId,
						amount: ::core::option::Option<::core::primitive::u128>,
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
					transfer {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					set_balance {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						new_reserved: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					force_transfer {
						source: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					transfer_keep_alive {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					transfer_all {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					force_unreserve {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						amount: ::core::primitive::u128,
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
					KeepAlive,
					#[codec(index = 5)]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					DeadAccount,
					#[codec(index = 7)]
					TooManyReserves,
				}
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
					BalanceSet {
						who: ::subxt::utils::AccountId32,
						free: ::core::primitive::u128,
						reserved: ::core::primitive::u128,
					},
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
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub misc_frozen: _0,
				pub fee_frozen: _0,
			}
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
				pub reasons: runtime_types::pallet_balances::Reasons,
			}
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
		pub mod pallet_call_filter {
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
					disable {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					},
					#[codec(index = 1)]
					enable {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
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
					CannotDisable,
					#[codec(index = 1)]
					InvalidString,
				}
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
					Disabled {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					},
					#[codec(index = 1)]
					Enabled {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
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
				pub struct CallFilterEntry<_0> {
					pub pallet_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub function_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_0>,
				}
			}
		}
		pub mod pallet_collator_selection {
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
					set_invulnerables { new: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 1)]
					set_desired_candidates { max: ::core::primitive::u32 },
					#[codec(index = 2)]
					set_candidacy_bond { bond: ::core::primitive::u128 },
					#[codec(index = 3)]
					register_as_candidate,
					#[codec(index = 4)]
					leave_intent,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CandidateInfo<_0, _1> {
					pub who: _0,
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
				pub enum Error {
					#[codec(index = 0)]
					TooManyCandidates,
					#[codec(index = 1)]
					TooFewCandidates,
					#[codec(index = 2)]
					Unknown,
					#[codec(index = 3)]
					Permission,
					#[codec(index = 4)]
					AlreadyCandidate,
					#[codec(index = 5)]
					NotCandidate,
					#[codec(index = 6)]
					TooManyInvulnerables,
					#[codec(index = 7)]
					AlreadyInvulnerable,
					#[codec(index = 8)]
					NoAssociatedValidatorId,
					#[codec(index = 9)]
					ValidatorNotRegistered,
				}
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
					NewInvulnerables { invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 1)]
					NewDesiredCandidates { desired_candidates: ::core::primitive::u32 },
					#[codec(index = 2)]
					NewCandidacyBond { bond_amount: ::core::primitive::u128 },
					#[codec(index = 3)]
					CandidateAdded {
						account_id: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					CandidateRemoved { account_id: ::subxt::utils::AccountId32 },
				}
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
						proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					propose {
						#[codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
					#[codec(index = 4)]
					close_old_weight {
						proposal_hash: ::subxt::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						#[codec(compact)]
						proposal_weight_bound: runtime_types::sp_weights::OldWeight,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
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
						to: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						conviction: runtime_types::pallet_conviction_voting::conviction::Conviction,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					undelegate { class: ::core::primitive::u16 },
					#[codec(index = 3)]
					unlock {
						class: ::core::primitive::u16,
						target: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 4)]
					remove_vote {
						class: ::core::option::Option<::core::primitive::u16>,
						index: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					remove_other_vote {
						target: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
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
		pub mod pallet_crowdloan_rewards {
			use super::runtime_types;
			pub mod models {
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
				pub enum Proof<_0> {
					#[codec(index = 0)]
					RelayChain(_0, runtime_types::sp_runtime::MultiSignature),
					#[codec(index = 1)]
					Ethereum(runtime_types::composable_support::types::EcdsaSignature),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum RemoteAccount<_0> {
					#[codec(index = 0)]
					RelayChain(_0),
					#[codec(index = 1)]
					Ethereum(runtime_types::composable_support::types::EthereumAddress),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Reward<_0, _1> {
					pub total: _0,
					pub claimed: _0,
					pub vesting_period: _1,
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
					initialize,
					#[codec(index = 1)]
					initialize_at { at: ::core::primitive::u64 },
					#[codec(index = 2)]
					populate {
						rewards: ::std::vec::Vec<(
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
							::core::primitive::u128,
							::core::primitive::u64,
						)>,
					},
					#[codec(index = 3)]
					associate {
						reward_account: ::subxt::utils::AccountId32,
						proof: runtime_types::pallet_crowdloan_rewards::models::Proof<
							[::core::primitive::u8; 32usize],
						>,
					},
					#[codec(index = 4)]
					claim,
					#[codec(index = 5)]
					unlock_rewards_for {
						reward_accounts: ::std::vec::Vec<::subxt::utils::AccountId32>,
					},
					#[codec(index = 6)]
					add {
						additions: ::std::vec::Vec<(
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
							::core::primitive::u128,
							::core::primitive::u64,
						)>,
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
					NotInitialized,
					#[codec(index = 1)]
					AlreadyInitialized,
					#[codec(index = 2)]
					BackToTheFuture,
					#[codec(index = 3)]
					RewardsNotFunded,
					#[codec(index = 4)]
					InvalidProof,
					#[codec(index = 5)]
					InvalidClaim,
					#[codec(index = 6)]
					NothingToClaim,
					#[codec(index = 7)]
					NotAssociated,
					#[codec(index = 8)]
					AlreadyAssociated,
					#[codec(index = 9)]
					NotClaimableYet,
					#[codec(index = 10)]
					UnexpectedRewardAmount,
				}
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
					Initialized { at: ::core::primitive::u64 },
					#[codec(index = 1)]
					Claimed {
						remote_account:
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
						reward_account: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					Associated {
						remote_account:
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
						reward_account: ::subxt::utils::AccountId32,
					},
					#[codec(index = 3)]
					OverFunded { excess_funds: ::core::primitive::u128 },
					#[codec(index = 4)]
					RewardsUnlocked { at: ::core::primitive::u64 },
					#[codec(index = 5)]
					RewardsAdded {
						additions: ::std::vec::Vec<(
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
							::core::primitive::u128,
							::core::primitive::u64,
						)>,
					},
					#[codec(index = 6)]
					RewardsDeleted {
						deletions: ::std::vec::Vec<
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
						>,
					},
				}
			}
		}
		pub mod pallet_currency_factory {
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
					add_range { length: ::core::primitive::u64 },
					#[codec(index = 1)]
					set_metadata {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						metadata: runtime_types::composable_traits::assets::BasicAssetMetadata,
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
					AssetNotFound,
				}
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
					RangeCreated {
						range: runtime_types::pallet_currency_factory::ranges::Range<
							runtime_types::primitives::currency::CurrencyId,
						>,
					},
				}
			}
			pub mod ranges {
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
				pub struct Range<_0> {
					pub current: _0,
					pub end: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Ranges<_0> {
					pub ranges: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_currency_factory::ranges::Range<_0>,
					>,
				}
			}
		}
		pub mod pallet_custom_origins {
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
					WhitelistedCaller,
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
							runtime_types::composable_runtime::RuntimeCall,
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
							runtime_types::composable_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 5)]
					external_propose_majority {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::composable_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 6)]
					external_propose_default {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::composable_runtime::RuntimeCall,
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
						to: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					undelegate,
					#[codec(index = 12)]
					clear_public_proposals,
					#[codec(index = 13)]
					unlock {
						target: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 14)]
					remove_vote { index: ::core::primitive::u32 },
					#[codec(index = 15)]
					remove_other_vote {
						target: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
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
		pub mod pallet_ibc {
			use super::runtime_types;
			pub mod errors {
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
				pub enum IbcError {
					#[codec(index = 0)]
					Ics02Client { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					Ics03Connection { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 2)]
					Ics04Channel { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 3)]
					Ics20FungibleTokenTransfer { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					UnknownMessageTypeUrl { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 5)]
					MalformedMessageBytes { message: ::std::vec::Vec<::core::primitive::u8> },
				}
			}
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
				pub enum IbcEvent {
					#[codec(index = 0)]
					NewBlock {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
					},
					#[codec(index = 1)]
					CreateClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 2)]
					UpdateClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 3)]
					UpgradeClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					ClientMisbehaviour {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 5)]
					OpenInitConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 6)]
					OpenConfirmConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 7)]
					OpenTryConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 8)]
					OpenAckConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 9)]
					OpenInitChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 10)]
					OpenConfirmChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 11)]
					OpenTryChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 12)]
					OpenAckChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 13)]
					CloseInitChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 14)]
					CloseConfirmChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 15)]
					ReceivePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 16)]
					SendPacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 17)]
					AcknowledgePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 18)]
					WriteAcknowledgement {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					TimeoutPacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 20)]
					TimeoutOnClosePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 21)]
					Empty,
					#[codec(index = 22)]
					ChainError,
					#[codec(index = 23)]
					AppModule {
						kind: ::std::vec::Vec<::core::primitive::u8>,
						module_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 24)]
					PushWasmCode { wasm_code_id: ::std::vec::Vec<::core::primitive::u8> },
				}
			}
			pub mod ics20_fee {
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
						set_charge { charge: runtime_types::sp_arithmetic::per_things::Perbill },
						#[codec(index = 1)]
						add_channels_to_feeless_channel_list {
							source_channel: ::core::primitive::u64,
							destination_channel: ::core::primitive::u64,
						},
						#[codec(index = 2)]
						remove_channels_from_feeless_channel_list {
							source_channel: ::core::primitive::u64,
							destination_channel: ::core::primitive::u64,
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
					pub enum Event {
						#[codec(index = 0)]
						IbcTransferFeeCollected {
							amount: ::core::primitive::u128,
							asset_id: runtime_types::primitives::currency::CurrencyId,
						},
						#[codec(index = 1)]
						FeeLessChannelIdsAdded {
							source_channel: ::core::primitive::u64,
							destination_channel: ::core::primitive::u64,
						},
						#[codec(index = 2)]
						FeeLessChannelIdsRemoved {
							source_channel: ::core::primitive::u64,
							destination_channel: ::core::primitive::u64,
						},
					}
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
					deliver { messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any> },
					#[codec(index = 1)]
					transfer {
						params:
							runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						memo: ::core::option::Option<::std::string::String>,
					},
					#[codec(index = 3)]
					upgrade_client { params: runtime_types::pallet_ibc::UpgradeParams },
					#[codec(index = 4)]
					freeze_client {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						height: ::core::primitive::u64,
					},
					#[codec(index = 5)]
					increase_counters,
					#[codec(index = 6)]
					add_channels_to_feeless_channel_list {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec(index = 7)]
					remove_channels_from_feeless_channel_list {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec(index = 8)]
					set_child_storage {
						key: ::std::vec::Vec<::core::primitive::u8>,
						value: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 9)]
					substitute_client_state {
						client_id: ::std::string::String,
						height: runtime_types::ibc::core::ics02_client::height::Height,
						client_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
						consensus_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
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
					ProcessingError,
					#[codec(index = 1)]
					DecodingError,
					#[codec(index = 2)]
					EncodingError,
					#[codec(index = 3)]
					ProofGenerationError,
					#[codec(index = 4)]
					ConsensusStateNotFound,
					#[codec(index = 5)]
					ChannelNotFound,
					#[codec(index = 6)]
					ClientStateNotFound,
					#[codec(index = 7)]
					ConnectionNotFound,
					#[codec(index = 8)]
					PacketCommitmentNotFound,
					#[codec(index = 9)]
					PacketReceiptNotFound,
					#[codec(index = 10)]
					PacketAcknowledgmentNotFound,
					#[codec(index = 11)]
					SendPacketError,
					#[codec(index = 12)]
					InvalidChannelId,
					#[codec(index = 13)]
					InvalidPortId,
					#[codec(index = 14)]
					Other,
					#[codec(index = 15)]
					InvalidRoute,
					#[codec(index = 16)]
					InvalidMessageType,
					#[codec(index = 17)]
					TransferInternals,
					#[codec(index = 18)]
					TransferSerde,
					#[codec(index = 19)]
					TransferOther,
					#[codec(index = 20)]
					TransferProtocol,
					#[codec(index = 21)]
					TransferSend,
					#[codec(index = 22)]
					Utf8Error,
					#[codec(index = 23)]
					InvalidAssetId,
					#[codec(index = 24)]
					PrefixedDenomParse,
					#[codec(index = 25)]
					InvalidAmount,
					#[codec(index = 26)]
					InvalidTimestamp,
					#[codec(index = 27)]
					FailedToGetRevisionNumber,
					#[codec(index = 28)]
					InvalidParams,
					#[codec(index = 29)]
					ChannelInitError,
					#[codec(index = 30)]
					TimestampAndHeightNotFound,
					#[codec(index = 31)]
					ChannelEscrowAddress,
					#[codec(index = 32)]
					WriteAckError,
					#[codec(index = 33)]
					ClientUpdateNotFound,
					#[codec(index = 34)]
					ClientFreezeFailed,
					#[codec(index = 35)]
					AccessDenied,
					#[codec(index = 36)]
					RateLimiter,
					#[codec(index = 37)]
					FailedSendFeeToAccount,
					#[codec(index = 38)]
					OriginAddress,
				}
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
					Events {
						events: ::std::vec::Vec<
							::core::result::Result<
								runtime_types::pallet_ibc::events::IbcEvent,
								runtime_types::pallet_ibc::errors::IbcError,
							>,
						>,
					},
					#[codec(index = 1)]
					TokenTransferInitiated {
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 2)]
					ChannelOpened {
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 3)]
					ParamsUpdated {
						send_enabled: ::core::primitive::bool,
						receive_enabled: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					TokenTransferCompleted {
						from: runtime_types::ibc::signer::Signer,
						to: runtime_types::ibc::signer::Signer,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 5)]
					TokenReceived {
						from: runtime_types::ibc::signer::Signer,
						to: runtime_types::ibc::signer::Signer,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
						amount: ::core::primitive::u128,
						is_receiver_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 6)]
					TokenTransferFailed {
						from: runtime_types::ibc::signer::Signer,
						to: runtime_types::ibc::signer::Signer,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 7)]
					TokenTransferTimeout {
						from: runtime_types::ibc::signer::Signer,
						to: runtime_types::ibc::signer::Signer,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 8)]
					OnRecvPacketError { msg: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 9)]
					ClientUpgradeSet,
					#[codec(index = 10)]
					ClientFrozen {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
					},
					#[codec(index = 11)]
					AssetAdminUpdated { admin_account: ::subxt::utils::AccountId32 },
					#[codec(index = 12)]
					FeeLessChannelIdsAdded {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					FeeLessChannelIdsRemoved {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec(index = 14)]
					ChargingFeeOnTransferInitiated {
						sequence: ::core::primitive::u64,
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
						amount: ::core::primitive::u128,
						is_flat_fee: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 15)]
					ChargingFeeConfirmed { sequence: ::core::primitive::u64 },
					#[codec(index = 16)]
					ChargingFeeTimeout { sequence: ::core::primitive::u64 },
					#[codec(index = 17)]
					ChargingFeeFailedAcknowledgement { sequence: ::core::primitive::u64 },
					#[codec(index = 18)]
					ChildStateUpdated,
					#[codec(index = 19)]
					ClientStateSubstituted {
						client_id: ::std::string::String,
						height: runtime_types::ibc::core::ics02_client::height::Height,
					},
					#[codec(index = 20)]
					ExecuteMemoStarted {
						account_id: ::subxt::utils::AccountId32,
						memo: ::core::option::Option<::std::string::String>,
					},
					#[codec(index = 21)]
					ExecuteMemoIbcTokenTransferSuccess {
						from: ::subxt::utils::AccountId32,
						to: ::std::vec::Vec<::core::primitive::u8>,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						channel: ::core::primitive::u64,
						next_memo: ::core::option::Option<::std::string::String>,
					},
					#[codec(index = 22)]
					ExecuteMemoIbcTokenTransferFailedWithReason {
						from: ::subxt::utils::AccountId32,
						memo: ::std::string::String,
						reason: ::core::primitive::u8,
					},
					#[codec(index = 23)]
					ExecuteMemoIbcTokenTransferFailed {
						from: ::subxt::utils::AccountId32,
						to: ::std::vec::Vec<::core::primitive::u8>,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						channel: ::core::primitive::u64,
						next_memo: ::core::option::Option<::std::string::String>,
					},
					#[codec(index = 24)]
					ExecuteMemoXcmSuccess {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						para_id: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 25)]
					ExecuteMemoXcmFailed {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						para_id: ::core::option::Option<::core::primitive::u32>,
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
			pub struct Any {
				pub type_url: ::std::string::String,
				pub value: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum LightClientProtocol {
				#[codec(index = 0)]
				Beefy,
				#[codec(index = 1)]
				Grandpa,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum MultiAddress<_0> {
				#[codec(index = 0)]
				Id(_0),
				#[codec(index = 1)]
				Raw(::std::vec::Vec<::core::primitive::u8>),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferParams<_0> {
				pub to: runtime_types::pallet_ibc::MultiAddress<_0>,
				pub source_channel: ::core::primitive::u64,
				pub timeout: runtime_types::ibc_primitives::Timeout,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpgradeParams {
				pub client_state: ::std::vec::Vec<::core::primitive::u8>,
				pub consensus_state: ::std::vec::Vec<::core::primitive::u8>,
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
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					free { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					force_transfer {
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
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
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					remove_member {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 2)]
					swap_member {
						remove: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						add: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					reset_members { members: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 4)]
					change_key {
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 5)]
					set_prime {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
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
		pub mod pallet_multihop_xcm_ibc {
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
					add_route {
						route_id: ::core::primitive::u128,
						route: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							runtime_types::composable_traits::xcm::memo::ChainInfo,
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						)>,
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
					IncorrectAddress { chain_id: ::core::primitive::u8 },
					#[codec(index = 1)]
					IncorrectChainName { chain_id: ::core::primitive::u8 },
					#[codec(index = 2)]
					FailedToEncodeBech32Address { chain_id: ::core::primitive::u8 },
					#[codec(index = 3)]
					IncorrectMultiLocation,
					#[codec(index = 4)]
					XcmDepositFailed,
					#[codec(index = 5)]
					MultiHopRouteDoesNotExist,
					#[codec(index = 6)]
					DoesNotSupportNonFungible,
					#[codec(index = 7)]
					IncorrectCountOfAddresses,
					#[codec(index = 8)]
					FailedToConstructMemo,
					#[codec(index = 9)]
					FailedToDecodeAccountId,
				}
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
					SuccessXcmToIbc {
						origin_address: ::subxt::utils::AccountId32,
						to: [::core::primitive::u8; 32usize],
						amount: ::core::primitive::u128,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						memo: ::core::option::Option<::std::string::String>,
					},
					#[codec(index = 1)]
					FailedXcmToIbc {
						origin_address: ::subxt::utils::AccountId32,
						to: [::core::primitive::u8; 32usize],
						amount: ::core::primitive::u128,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						memo: ::core::option::Option<::std::string::String>,
					},
					#[codec(index = 2)]
					FailedCallback {
						origin_address: [::core::primitive::u8; 32usize],
						route_id: ::core::primitive::u128,
						reason: runtime_types::pallet_multihop_xcm_ibc::pallet::MultihopEventReason,
					},
					#[codec(index = 3)]
					MultihopXcmMemo {
						reason: runtime_types::pallet_multihop_xcm_ibc::pallet::MultihopEventReason,
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						asset_id: ::core::primitive::u128,
						is_error: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					FailedMatchLocation,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum MultihopEventReason {
					#[codec(index = 0)]
					FailedToConvertAddressToBytes,
					#[codec(index = 1)]
					XcmTransferInitiated,
					#[codec(index = 2)]
					IncorrectPalletId,
					#[codec(index = 3)]
					MultiHopRouteDoesNotExist,
					#[codec(index = 4)]
					MultiHopRouteExistButNotConfigured,
					#[codec(index = 5)]
					IncorrectCountOfAddresses,
					#[codec(index = 6)]
					FailedToDeriveCosmosAddressFromBytes,
					#[codec(index = 7)]
					FailedToDeriveChainNameFromUtf8,
					#[codec(index = 8)]
					FailedToEncodeBech32Address,
					#[codec(index = 9)]
					FailedToDecodeDestAccountId,
					#[codec(index = 10)]
					FailedToDecodeSenderAccountId,
					#[codec(index = 11)]
					DoesNotSupportNonFungible,
					#[codec(index = 12)]
					FailedCreateMemo,
					#[codec(index = 13)]
					FailedToConvertMemoIntoPalletIbcMemoMessageType,
				}
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
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					as_multi {
						threshold: ::core::primitive::u16,
						other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
						maybe_timepoint: ::core::option::Option<
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						>,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
						real: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						force_proxy_type: ::core::option::Option<
							runtime_types::composable_traits::account_proxy::ProxyType,
						>,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					add_proxy {
						delegate: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					remove_proxy {
						delegate: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					remove_proxies,
					#[codec(index = 4)]
					create_pure {
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
						index: ::core::primitive::u16,
					},
					#[codec(index = 5)]
					kill_pure {
						spawner: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						index: ::core::primitive::u16,
						#[codec(compact)]
						height: ::core::primitive::u32,
						#[codec(compact)]
						ext_index: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					announce {
						real: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						call_hash: ::subxt::utils::H256,
					},
					#[codec(index = 7)]
					remove_announcement {
						real: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						call_hash: ::subxt::utils::H256,
					},
					#[codec(index = 8)]
					reject_announcement {
						delegate: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						call_hash: ::subxt::utils::H256,
					},
					#[codec(index = 9)]
					proxy_announced {
						delegate: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						real: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						force_proxy_type: ::core::option::Option<
							runtime_types::composable_traits::account_proxy::ProxyType,
						>,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
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
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					ProxyRemoved {
						delegator: ::subxt::utils::AccountId32,
						delegatee: ::subxt::utils::AccountId32,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
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
							::std::boxed::Box<runtime_types::composable_runtime::OriginCaller>,
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::composable_runtime::RuntimeCall,
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
							runtime_types::composable_runtime::RuntimeCall,
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
							runtime_types::composable_runtime::RuntimeCall,
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
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
						keys: runtime_types::composable_runtime::opaque::SessionKeys,
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
		pub mod pallet_sudo {
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
					sudo { call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall> },
					#[codec(index = 1)]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					set_key {
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					sudo_as {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
					RequireSudo,
				}
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
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					KeyChanged { old_sudoer: ::core::option::Option<::subxt::utils::AccountId32> },
					#[codec(index = 2)]
					SudoAsDone {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
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
						beneficiary: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
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
						beneficiary: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
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
					batch { calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall> },
					#[codec(index = 1)]
					as_derivative {
						index: ::core::primitive::u16,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 2)]
					batch_all {
						calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 3)]
					dispatch_as {
						as_origin:
							::std::boxed::Box<runtime_types::composable_runtime::OriginCaller>,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 4)]
					force_batch {
						calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 5)]
					with_weight {
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
				}
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
				pub struct RemoteLockedFungibleRecord {
					pub amount: ::core::primitive::u128,
					pub owner: runtime_types::xcm::VersionedMultiLocation,
					pub locker: runtime_types::xcm::VersionedMultiLocation,
					pub users: ::core::primitive::u32,
				}
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
		pub mod parachain_info {
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
				pub enum XcmpMessageFormat {
					#[codec(index = 0)]
					ConcatenatedVersionedXcm,
					#[codec(index = 1)]
					ConcatenatedEncodedBlob,
					#[codec(index = 2)]
					Signals,
				}
			}
		}
		pub mod polkadot_primitives {
			use super::runtime_types;
			pub mod v2 {
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
				pub struct AbridgedHostConfiguration {
					pub max_code_size: ::core::primitive::u32,
					pub max_head_data_size: ::core::primitive::u32,
					pub max_upward_queue_count: ::core::primitive::u32,
					pub max_upward_queue_size: ::core::primitive::u32,
					pub max_upward_message_size: ::core::primitive::u32,
					pub max_upward_message_num_per_candidate: ::core::primitive::u32,
					pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
					pub validation_upgrade_cooldown: ::core::primitive::u32,
					pub validation_upgrade_delay: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AbridgedHrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<::subxt::utils::H256>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PersistedValidationData<_0, _1> {
					pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
					pub relay_parent_number: _1,
					pub relay_parent_storage_root: _0,
					pub max_pov_size: _1,
				}
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
			}
		}
		pub mod primitives {
			use super::runtime_types;
			pub mod currency {
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
				pub struct CurrencyId(pub ::core::primitive::u128);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum ForeignAssetId {
					#[codec(index = 0)]
					Xcm(runtime_types::primitives::currency::VersionedMultiLocation),
					#[codec(index = 1)]
					IbcIcs20(runtime_types::primitives::currency::PrefixedDenom),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PrefixedDenom(
					pub runtime_types::ibc::applications::transfer::denom::PrefixedDenom,
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
				pub enum VersionedMultiLocation {
					#[codec(index = 3)]
					V3(runtime_types::xcm::v3::multilocation::MultiLocation),
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
		pub mod sp_consensus_aura {
			use super::runtime_types;
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
				}
			}
		}
		pub mod sp_consensus_slots {
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
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod sr25519 {
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
			}
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
			pub enum TokenError {
				#[codec(index = 0)]
				NoFunds,
				#[codec(index = 1)]
				WouldDie,
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
			}
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
		pub mod sp_trie {
			use super::runtime_types;
			pub mod storage_proof {
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
				pub struct StorageProof {
					pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
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
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OldWeight(pub ::core::primitive::u64);
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
			pub enum VersionedMultiAsset {
				#[codec(index = 1)]
				V2(runtime_types::xcm::v2::multiasset::MultiAsset),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::MultiAsset),
			}
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
