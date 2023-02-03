#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
pub mod api {
	use super::api as root_mod;
	pub static PALLETS: [&str; 20usize] = [
		"System",
		"Timestamp",
		"ParachainSystem",
		"ParachainInfo",
		"Balances",
		"TransactionPayment",
		"Authorship",
		"CollatorSelection",
		"Session",
		"Aura",
		"AuraExt",
		"XcmpQueue",
		"PolkadotXcm",
		"CumulusXcm",
		"DmpQueue",
		"Sudo",
		"IbcPing",
		"Assets",
		"AssetRegistry",
		"Ibc",
	];
	#[derive(:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug)]
	pub enum Event {
		#[codec(index = 0)]
		System(system::Event),
		#[codec(index = 2)]
		ParachainSystem(parachain_system::Event),
		#[codec(index = 10)]
		Balances(balances::Event),
		#[codec(index = 11)]
		TransactionPayment(transaction_payment::Event),
		#[codec(index = 21)]
		CollatorSelection(collator_selection::Event),
		#[codec(index = 22)]
		Session(session::Event),
		#[codec(index = 30)]
		XcmpQueue(xcmp_queue::Event),
		#[codec(index = 31)]
		PolkadotXcm(polkadot_xcm::Event),
		#[codec(index = 32)]
		CumulusXcm(cumulus_xcm::Event),
		#[codec(index = 33)]
		DmpQueue(dmp_queue::Event),
		#[codec(index = 35)]
		Sudo(sudo::Event),
		#[codec(index = 36)]
		IbcPing(ibc_ping::Event),
		#[codec(index = 37)]
		Assets(assets::Event),
		#[codec(index = 38)]
		AssetRegistry(asset_registry::Event),
		#[codec(index = 255)]
		Ibc(ibc::Event),
	}
	pub mod system {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct FillBlock {
				pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Remark {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHeapPages {
				pub pages: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetCode {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetCodeWithoutChecks {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetStorage {
				pub items: ::std::vec::Vec<(
					::std::vec::Vec<::core::primitive::u8>,
					::std::vec::Vec<::core::primitive::u8>,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct KillStorage {
				pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct KillPrefix {
				pub prefix: ::std::vec::Vec<::core::primitive::u8>,
				pub subkeys: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemarkWithEvent {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "A dispatch that will fill the block weight up to the given ratio."]
				pub fn fill_block(
					&self,
					ratio: runtime_types::sp_arithmetic::per_things::Perbill,
				) -> ::subxt::tx::StaticTxPayload<FillBlock> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"fill_block",
						FillBlock { ratio },
						[
							48u8, 18u8, 205u8, 90u8, 222u8, 4u8, 20u8, 251u8, 173u8, 76u8, 167u8,
							4u8, 83u8, 203u8, 160u8, 89u8, 132u8, 218u8, 191u8, 145u8, 130u8,
							245u8, 177u8, 201u8, 169u8, 129u8, 173u8, 105u8, 88u8, 45u8, 136u8,
							191u8,
						],
					)
				}
				#[doc = "Make some on-chain remark."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`"]
				#[doc = "# </weight>"]
				pub fn remark(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<Remark> {
					::subxt::tx::StaticTxPayload::new(
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
				#[doc = "Set the number of pages in the WebAssembly environment's heap."]
				pub fn set_heap_pages(
					&self,
					pages: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<SetHeapPages> {
					::subxt::tx::StaticTxPayload::new(
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
				#[doc = "Set the new runtime code."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
				#[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
				#[doc = "  expensive)."]
				#[doc = "- 1 storage write (codec `O(C)`)."]
				#[doc = "- 1 digest item."]
				#[doc = "- 1 event."]
				#[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
				#[doc = "expensive. We will treat this as a full block."]
				#[doc = "# </weight>"]
				pub fn set_code(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SetCode> {
					::subxt::tx::StaticTxPayload::new(
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
				#[doc = "Set the new runtime code without doing any checks of the given `code`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(C)` where `C` length of `code`"]
				#[doc = "- 1 storage write (codec `O(C)`)."]
				#[doc = "- 1 digest item."]
				#[doc = "- 1 event."]
				#[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
				#[doc = "block. # </weight>"]
				pub fn set_code_without_checks(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SetCodeWithoutChecks> {
					::subxt::tx::StaticTxPayload::new(
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
				#[doc = "Set some items of storage."]
				pub fn set_storage(
					&self,
					items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				) -> ::subxt::tx::StaticTxPayload<SetStorage> {
					::subxt::tx::StaticTxPayload::new(
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
				#[doc = "Kill some items from storage."]
				pub fn kill_storage(
					&self,
					keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				) -> ::subxt::tx::StaticTxPayload<KillStorage> {
					::subxt::tx::StaticTxPayload::new(
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
				#[doc = "Kill all storage items with a key that starts with the given prefix."]
				#[doc = ""]
				#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
				#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
				pub fn kill_prefix(
					&self,
					prefix: ::std::vec::Vec<::core::primitive::u8>,
					subkeys: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<KillPrefix> {
					::subxt::tx::StaticTxPayload::new(
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
				#[doc = "Make some on-chain remark and emit event."]
				pub fn remark_with_event(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<RemarkWithEvent> {
					::subxt::tx::StaticTxPayload::new(
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
		#[doc = "Event for the System pallet."]
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An extrinsic completed successfully."]
			pub struct ExtrinsicSuccess {
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An extrinsic failed."]
			pub struct ExtrinsicFailed {
				pub dispatch_error: runtime_types::sp_runtime::DispatchError,
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "`:code` was updated."]
			pub struct CodeUpdated;
			impl ::subxt::events::StaticEvent for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A new account was created."]
			pub struct NewAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account was reaped."]
			pub struct KilledAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "On on-chain remark happened."]
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
				#[doc = " The full account information for a particular account ID."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::AccountInfo<
							::core::primitive::u32,
							runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Account",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
							114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
							42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
						],
					)
				}
				#[doc = " The full account information for a particular account ID."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::AccountInfo<
							::core::primitive::u32,
							runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Total extrinsics count for the current block."]
				pub fn extrinsic_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " The current weight for the block."]
				pub fn block_weight(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::dispatch::PerDispatchClass<
							runtime_types::sp_weights::weight_v2::Weight,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
				pub fn all_extrinsics_len(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"BlockHash",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
							21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
							80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::H256>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExtrinsicData",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
							254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
							59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " The current block number being processed. Set by `execute_block`."]
				pub fn number(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Hash of the previous block."]
				pub fn parent_hash(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Digest of the current block, also part of the block header."]
				pub fn digest(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::generic::digest::Digest,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Events deposited for the current block."]
				#[doc = ""]
				#[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
				#[doc = " It could otherwise inflate the PoV size of a block."]
				#[doc = ""]
				#[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
				#[doc = " just in case someone still reads them from within the runtime."]
				pub fn events(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::frame_system::EventRecord<
								runtime_types::parachain_runtime::RuntimeEvent,
								::subxt::utils::H256,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Events",
						vec![],
						[
							75u8, 183u8, 245u8, 228u8, 94u8, 80u8, 145u8, 50u8, 37u8, 2u8, 167u8,
							131u8, 74u8, 255u8, 236u8, 116u8, 51u8, 56u8, 126u8, 45u8, 15u8, 72u8,
							144u8, 231u8, 37u8, 96u8, 176u8, 158u8, 163u8, 16u8, 30u8, 154u8,
						],
					)
				}
				#[doc = " The number of events in the `Events<T>` list."]
				pub fn event_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"EventTopics",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
							63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
							111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
							38u8,
						],
					)
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
				pub fn last_runtime_upgrade(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::LastRuntimeUpgradeInfo,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
				pub fn upgraded_to_u32_ref_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
				#[doc = " (default) if not."]
				pub fn upgraded_to_triple_ref_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " The execution phase of the block."]
				pub fn execution_phase(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::frame_system::Phase>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Block & extrinsics weights: base values and limits."]
				pub fn block_weights(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::limits::BlockWeights,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
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
				#[doc = " The maximum length of a block (in bytes)."]
				pub fn block_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::limits::BlockLength,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
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
				#[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
				pub fn block_hash_count(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
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
				#[doc = " The weight of runtime database operations the runtime can invoke."]
				pub fn db_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_weights::RuntimeDbWeight>,
				> {
					::subxt::constants::StaticConstantAddress::new(
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
				#[doc = " Get the chain's current version."]
				pub fn version(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_version::RuntimeVersion>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"Version",
						[
							93u8, 98u8, 57u8, 243u8, 229u8, 8u8, 234u8, 231u8, 72u8, 230u8, 139u8,
							47u8, 63u8, 181u8, 17u8, 2u8, 220u8, 231u8, 104u8, 237u8, 185u8, 143u8,
							165u8, 253u8, 188u8, 76u8, 147u8, 12u8, 170u8, 26u8, 74u8, 200u8,
						],
					)
				}
				#[doc = " The designated SS58 prefix of this chain."]
				#[doc = ""]
				#[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
				#[doc = " that the runtime should know about the prefix in order to make use of it as"]
				#[doc = " an identifier of the chain."]
				pub fn ss58_prefix(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
				> {
					::subxt::constants::StaticConstantAddress::new(
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
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Set {
				#[codec(compact)]
				pub now: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the current time."]
				#[doc = ""]
				#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase, if this call hasn't been invoked by that time."]
				#[doc = ""]
				#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
				#[doc = "`MinimumPeriod`."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Inherent`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
				#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
				#[doc = "  `on_finalize`)"]
				#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
				#[doc = "# </weight>"]
				pub fn set(
					&self,
					now: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<Set> {
					::subxt::tx::StaticTxPayload::new(
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
				#[doc = " Current time for the current block."]
				pub fn now(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Did the timestamp get updated in this block?"]
				pub fn did_update(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
				#[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
				#[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
				#[doc = " double this period on default settings."]
				pub fn minimum_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
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
	pub mod parachain_system {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetValidationData {
				pub data:
					runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoSendUpwardMessage {
				pub message: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AuthorizeUpgrade {
				pub code_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct EnactAuthorizedUpgrade {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the current validation data."]
				#[doc = ""]
				#[doc = "This should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase if the call was not invoked."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Inherent`"]
				#[doc = ""]
				#[doc = "As a side effect, this function upgrades the current validation function"]
				#[doc = "if the appropriate time has come."]
				pub fn set_validation_data(
					&self,
					data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
				) -> ::subxt::tx::StaticTxPayload<SetValidationData> {
					::subxt::tx::StaticTxPayload::new(
						"ParachainSystem",
						"set_validation_data",
						SetValidationData { data },
						[
							200u8, 80u8, 163u8, 177u8, 184u8, 117u8, 61u8, 203u8, 244u8, 214u8,
							106u8, 151u8, 128u8, 131u8, 254u8, 120u8, 254u8, 76u8, 104u8, 39u8,
							215u8, 227u8, 233u8, 254u8, 26u8, 62u8, 17u8, 42u8, 19u8, 127u8, 108u8,
							242u8,
						],
					)
				}
				pub fn sudo_send_upward_message(
					&self,
					message: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SudoSendUpwardMessage> {
					::subxt::tx::StaticTxPayload::new(
						"ParachainSystem",
						"sudo_send_upward_message",
						SudoSendUpwardMessage { message },
						[
							127u8, 79u8, 45u8, 183u8, 190u8, 205u8, 184u8, 169u8, 255u8, 191u8,
							86u8, 154u8, 134u8, 25u8, 249u8, 63u8, 47u8, 194u8, 108u8, 62u8, 60u8,
							170u8, 81u8, 240u8, 113u8, 48u8, 181u8, 171u8, 95u8, 63u8, 26u8, 222u8,
						],
					)
				}
				pub fn authorize_upgrade(
					&self,
					code_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<AuthorizeUpgrade> {
					::subxt::tx::StaticTxPayload::new(
						"ParachainSystem",
						"authorize_upgrade",
						AuthorizeUpgrade { code_hash },
						[
							52u8, 152u8, 69u8, 207u8, 143u8, 113u8, 163u8, 11u8, 181u8, 182u8,
							124u8, 101u8, 207u8, 19u8, 59u8, 81u8, 129u8, 29u8, 79u8, 115u8, 90u8,
							83u8, 225u8, 124u8, 21u8, 108u8, 99u8, 194u8, 78u8, 83u8, 252u8, 163u8,
						],
					)
				}
				pub fn enact_authorized_upgrade(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<EnactAuthorizedUpgrade> {
					::subxt::tx::StaticTxPayload::new(
						"ParachainSystem",
						"enact_authorized_upgrade",
						EnactAuthorizedUpgrade { code },
						[
							43u8, 157u8, 1u8, 230u8, 134u8, 72u8, 230u8, 35u8, 159u8, 13u8, 201u8,
							134u8, 184u8, 94u8, 167u8, 13u8, 108u8, 157u8, 145u8, 166u8, 119u8,
							37u8, 51u8, 121u8, 252u8, 255u8, 48u8, 251u8, 126u8, 152u8, 247u8, 5u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The validation function has been scheduled to apply."]
			pub struct ValidationFunctionStored;
			impl ::subxt::events::StaticEvent for ValidationFunctionStored {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionStored";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "The validation function was applied as of the contained relay chain block number."]
			pub struct ValidationFunctionApplied {
				pub relay_chain_block_num: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ValidationFunctionApplied {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionApplied";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The relay-chain aborted the upgrade process."]
			pub struct ValidationFunctionDiscarded;
			impl ::subxt::events::StaticEvent for ValidationFunctionDiscarded {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionDiscarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An upgrade has been authorized."]
			pub struct UpgradeAuthorized {
				pub code_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for UpgradeAuthorized {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "UpgradeAuthorized";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "Some downward messages have been received and will be processed."]
			pub struct DownwardMessagesReceived {
				pub count: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesReceived {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesReceived";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward messages were processed using the given weight."]
			pub struct DownwardMessagesProcessed {
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
				pub dmq_head: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesProcessed {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesProcessed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " In case of a scheduled upgrade, this storage field contains the validation code to be applied."]
				#[doc = ""]
				#[doc = " As soon as the relay chain gives us the go-ahead signal, we will overwrite the [`:code`][well_known_keys::CODE]"]
				#[doc = " which will result the next block process with the new validation code. This concludes the upgrade process."]
				#[doc = ""]
				#[doc = " [well_known_keys::CODE]: sp_core::storage::well_known_keys::CODE"]
				pub fn pending_validation_code(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"PendingValidationCode",
						vec![],
						[
							162u8, 35u8, 108u8, 76u8, 160u8, 93u8, 215u8, 84u8, 20u8, 249u8, 57u8,
							187u8, 88u8, 161u8, 15u8, 131u8, 213u8, 89u8, 140u8, 20u8, 227u8,
							204u8, 79u8, 176u8, 114u8, 119u8, 8u8, 7u8, 64u8, 15u8, 90u8, 92u8,
						],
					)
				}
				#[doc = " Validation code that is set by the parachain and is to be communicated to collator and"]
				#[doc = " consequently the relay-chain."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block if no other pallet already set"]
				#[doc = " the value."]
				pub fn new_validation_code(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"NewValidationCode",
						vec![],
						[
							224u8, 174u8, 53u8, 106u8, 240u8, 49u8, 48u8, 79u8, 219u8, 74u8, 142u8,
							166u8, 92u8, 204u8, 244u8, 200u8, 43u8, 169u8, 177u8, 207u8, 190u8,
							106u8, 180u8, 65u8, 245u8, 131u8, 134u8, 4u8, 53u8, 45u8, 76u8, 3u8,
						],
					)
				}
				#[doc = " The [`PersistedValidationData`] set for this block."]
				#[doc = " This value is expected to be set only once per block and it's never stored"]
				#[doc = " in the trie."]
				pub fn validation_data(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::PersistedValidationData<
							::subxt::utils::H256,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"ValidationData",
						vec![],
						[
							112u8, 58u8, 240u8, 81u8, 219u8, 110u8, 244u8, 186u8, 251u8, 90u8,
							195u8, 217u8, 229u8, 102u8, 233u8, 24u8, 109u8, 96u8, 219u8, 72u8,
							139u8, 93u8, 58u8, 140u8, 40u8, 110u8, 167u8, 98u8, 199u8, 12u8, 138u8,
							131u8,
						],
					)
				}
				#[doc = " Were the validation data set to notify the relay chain?"]
				pub fn did_set_validation_code(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"DidSetValidationCode",
						vec![],
						[
							89u8, 83u8, 74u8, 174u8, 234u8, 188u8, 149u8, 78u8, 140u8, 17u8, 92u8,
							165u8, 243u8, 87u8, 59u8, 97u8, 135u8, 81u8, 192u8, 86u8, 193u8, 187u8,
							113u8, 22u8, 108u8, 83u8, 242u8, 208u8, 174u8, 40u8, 49u8, 245u8,
						],
					)
				}
				#[doc = " The relay chain block number associated with the last parachain block."]
				pub fn last_relay_chain_block_number(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"LastRelayChainBlockNumber",
						vec![],
						[
							68u8, 121u8, 6u8, 159u8, 181u8, 94u8, 151u8, 215u8, 225u8, 244u8, 4u8,
							158u8, 216u8, 85u8, 55u8, 228u8, 197u8, 35u8, 200u8, 33u8, 29u8, 182u8,
							17u8, 83u8, 59u8, 63u8, 25u8, 180u8, 132u8, 23u8, 97u8, 252u8,
						],
					)
				}
				#[doc = " An option which indicates if the relay-chain restricts signalling a validation code upgrade."]
				#[doc = " In other words, if this is `Some` and [`NewValidationCode`] is `Some` then the produced"]
				#[doc = " candidate will be invalid."]
				#[doc = ""]
				#[doc = " This storage item is a mirror of the corresponding value for the current parachain from the"]
				#[doc = " relay-chain. This value is ephemeral which means it doesn't hit the storage. This value is"]
				#[doc = " set after the inherent."]
				pub fn upgrade_restriction_signal(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::core::option::Option<
							runtime_types::polkadot_primitives::v2::UpgradeRestriction,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"UpgradeRestrictionSignal",
						vec![],
						[
							61u8, 3u8, 26u8, 6u8, 88u8, 114u8, 109u8, 63u8, 7u8, 115u8, 245u8,
							198u8, 73u8, 234u8, 28u8, 228u8, 126u8, 27u8, 151u8, 18u8, 133u8, 54u8,
							144u8, 149u8, 246u8, 43u8, 83u8, 47u8, 77u8, 238u8, 10u8, 196u8,
						],
					)
				}
				#[doc = " The state proof for the last relay parent block."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]
				pub fn relay_state_proof(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_trie::storage_proof::StorageProof,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"RelayStateProof",
						vec![],
						[
							35u8, 124u8, 167u8, 221u8, 162u8, 145u8, 158u8, 186u8, 57u8, 154u8,
							225u8, 6u8, 176u8, 13u8, 178u8, 195u8, 209u8, 122u8, 221u8, 26u8,
							155u8, 126u8, 153u8, 246u8, 101u8, 221u8, 61u8, 145u8, 211u8, 236u8,
							48u8, 130u8,
						],
					)
				}
				#[doc = " The snapshot of some state related to messaging relevant to the current parachain as per"]
				#[doc = " the relay parent."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]				pub fn relevant_messaging_state (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot > , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"RelevantMessagingState",
						vec![],
						[
							68u8, 241u8, 114u8, 83u8, 200u8, 99u8, 8u8, 244u8, 110u8, 134u8, 106u8,
							153u8, 17u8, 90u8, 184u8, 157u8, 100u8, 140u8, 157u8, 83u8, 25u8,
							166u8, 173u8, 31u8, 221u8, 24u8, 236u8, 85u8, 176u8, 223u8, 237u8,
							65u8,
						],
					)
				}
				#[doc = " The parachain host configuration that was obtained from the relay parent."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]
				pub fn host_configuration(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::AbridgedHostConfiguration,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"HostConfiguration",
						vec![],
						[
							104u8, 200u8, 30u8, 202u8, 119u8, 204u8, 233u8, 20u8, 67u8, 199u8,
							47u8, 166u8, 254u8, 152u8, 10u8, 187u8, 240u8, 255u8, 148u8, 201u8,
							134u8, 41u8, 130u8, 201u8, 112u8, 65u8, 68u8, 103u8, 56u8, 123u8,
							178u8, 113u8,
						],
					)
				}
				#[doc = " The last downward message queue chain head we have observed."]
				#[doc = ""]
				#[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
				#[doc = " by the system inherent."]
				pub fn last_dmq_mqc_head(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"LastDmqMqcHead",
						vec![],
						[
							176u8, 255u8, 246u8, 125u8, 36u8, 120u8, 24u8, 44u8, 26u8, 64u8, 236u8,
							210u8, 189u8, 237u8, 50u8, 78u8, 45u8, 139u8, 58u8, 141u8, 112u8,
							253u8, 178u8, 198u8, 87u8, 71u8, 77u8, 248u8, 21u8, 145u8, 187u8, 52u8,
						],
					)
				}
				#[doc = " The message queue chain heads we have observed per each channel incoming channel."]
				#[doc = ""]
				#[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
				#[doc = " by the system inherent."]
				pub fn last_hrmp_mqc_heads(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::subxt::utils::KeyedVec<
							runtime_types::polkadot_parachain::primitives::Id,
							runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"LastHrmpMqcHeads",
						vec![],
						[
							55u8, 179u8, 35u8, 16u8, 173u8, 0u8, 122u8, 179u8, 236u8, 98u8, 9u8,
							112u8, 11u8, 219u8, 241u8, 89u8, 131u8, 198u8, 64u8, 139u8, 103u8,
							158u8, 77u8, 107u8, 83u8, 236u8, 255u8, 208u8, 47u8, 61u8, 219u8,
							240u8,
						],
					)
				}
				#[doc = " Number of downward messages processed in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn processed_downward_messages(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"ProcessedDownwardMessages",
						vec![],
						[
							48u8, 177u8, 84u8, 228u8, 101u8, 235u8, 181u8, 27u8, 66u8, 55u8, 50u8,
							146u8, 245u8, 223u8, 77u8, 132u8, 178u8, 80u8, 74u8, 90u8, 166u8, 81u8,
							109u8, 25u8, 91u8, 69u8, 5u8, 69u8, 123u8, 197u8, 160u8, 146u8,
						],
					)
				}
				#[doc = " HRMP watermark that was set in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn hrmp_watermark(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"HrmpWatermark",
						vec![],
						[
							189u8, 59u8, 183u8, 195u8, 69u8, 185u8, 241u8, 226u8, 62u8, 204u8,
							230u8, 77u8, 102u8, 75u8, 86u8, 157u8, 249u8, 140u8, 219u8, 72u8, 94u8,
							64u8, 176u8, 72u8, 34u8, 205u8, 114u8, 103u8, 231u8, 233u8, 206u8,
							111u8,
						],
					)
				}
				#[doc = " HRMP messages that were sent in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn hrmp_outbound_messages(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
								runtime_types::polkadot_parachain::primitives::Id,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"HrmpOutboundMessages",
						vec![],
						[
							74u8, 86u8, 173u8, 248u8, 90u8, 230u8, 71u8, 225u8, 127u8, 164u8,
							221u8, 62u8, 146u8, 13u8, 73u8, 9u8, 98u8, 168u8, 6u8, 14u8, 97u8,
							166u8, 45u8, 70u8, 62u8, 210u8, 9u8, 32u8, 83u8, 18u8, 4u8, 201u8,
						],
					)
				}
				#[doc = " Upward messages that were sent in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn upward_messages(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"UpwardMessages",
						vec![],
						[
							129u8, 208u8, 187u8, 36u8, 48u8, 108u8, 135u8, 56u8, 204u8, 60u8,
							100u8, 158u8, 113u8, 238u8, 46u8, 92u8, 228u8, 41u8, 178u8, 177u8,
							208u8, 195u8, 148u8, 149u8, 127u8, 21u8, 93u8, 92u8, 29u8, 115u8, 10u8,
							248u8,
						],
					)
				}
				#[doc = " Upward messages that are still pending and not yet send to the relay chain."]
				pub fn pending_upward_messages(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"PendingUpwardMessages",
						vec![],
						[
							223u8, 46u8, 224u8, 227u8, 222u8, 119u8, 225u8, 244u8, 59u8, 87u8,
							127u8, 19u8, 217u8, 237u8, 103u8, 61u8, 6u8, 210u8, 107u8, 201u8,
							117u8, 25u8, 85u8, 248u8, 36u8, 231u8, 28u8, 202u8, 41u8, 140u8, 208u8,
							254u8,
						],
					)
				}
				#[doc = " The number of HRMP messages we observed in `on_initialize` and thus used that number for"]
				#[doc = " announcing the weight of `on_initialize` and `on_finalize`."]
				pub fn announced_hrmp_messages_per_candidate(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"AnnouncedHrmpMessagesPerCandidate",
						vec![],
						[
							132u8, 61u8, 162u8, 129u8, 251u8, 243u8, 20u8, 144u8, 162u8, 73u8,
							237u8, 51u8, 248u8, 41u8, 127u8, 171u8, 180u8, 79u8, 137u8, 23u8, 66u8,
							134u8, 106u8, 222u8, 182u8, 154u8, 0u8, 145u8, 184u8, 156u8, 36u8,
							97u8,
						],
					)
				}
				#[doc = " The weight we reserve at the beginning of the block for processing XCMP messages. This"]
				#[doc = " overrides the amount set in the Config trait."]
				pub fn reserved_xcmp_weight_override(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"ReservedXcmpWeightOverride",
						vec![],
						[
							180u8, 90u8, 34u8, 178u8, 1u8, 242u8, 211u8, 97u8, 100u8, 34u8, 39u8,
							42u8, 142u8, 249u8, 236u8, 194u8, 244u8, 164u8, 96u8, 54u8, 98u8, 46u8,
							92u8, 196u8, 185u8, 51u8, 231u8, 234u8, 249u8, 143u8, 244u8, 64u8,
						],
					)
				}
				#[doc = " The weight we reserve at the beginning of the block for processing DMP messages. This"]
				#[doc = " overrides the amount set in the Config trait."]
				pub fn reserved_dmp_weight_override(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"ReservedDmpWeightOverride",
						vec![],
						[
							90u8, 122u8, 168u8, 240u8, 95u8, 195u8, 160u8, 109u8, 175u8, 170u8,
							227u8, 44u8, 139u8, 176u8, 32u8, 161u8, 57u8, 233u8, 56u8, 55u8, 123u8,
							168u8, 174u8, 96u8, 159u8, 62u8, 186u8, 186u8, 17u8, 70u8, 57u8, 246u8,
						],
					)
				}
				#[doc = " The next authorized upgrade, if there is one."]
				pub fn authorized_upgrade(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::H256>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"AuthorizedUpgrade",
						vec![],
						[
							136u8, 238u8, 241u8, 144u8, 252u8, 61u8, 101u8, 171u8, 234u8, 160u8,
							145u8, 210u8, 69u8, 29u8, 204u8, 166u8, 250u8, 101u8, 254u8, 32u8,
							96u8, 197u8, 222u8, 212u8, 50u8, 189u8, 25u8, 7u8, 48u8, 183u8, 234u8,
							95u8,
						],
					)
				}
				#[doc = " A custom head data that should be returned as result of `validate_block`."]
				#[doc = ""]
				#[doc = " See [`Pallet::set_custom_validation_head_data`] for more information."]
				pub fn custom_validation_head_data(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"CustomValidationHeadData",
						vec![],
						[
							189u8, 150u8, 234u8, 128u8, 111u8, 27u8, 173u8, 92u8, 109u8, 4u8, 98u8,
							103u8, 158u8, 19u8, 16u8, 5u8, 107u8, 135u8, 126u8, 170u8, 62u8, 64u8,
							149u8, 80u8, 33u8, 17u8, 83u8, 22u8, 176u8, 118u8, 26u8, 223u8,
						],
					)
				}
			}
		}
	}
	pub mod parachain_info {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn parachain_id(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::Id,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainInfo",
						"ParachainId",
						vec![],
						[
							151u8, 191u8, 241u8, 118u8, 192u8, 47u8, 166u8, 151u8, 217u8, 240u8,
							165u8, 232u8, 51u8, 113u8, 243u8, 1u8, 89u8, 240u8, 11u8, 1u8, 77u8,
							104u8, 12u8, 56u8, 17u8, 135u8, 214u8, 19u8, 114u8, 135u8, 66u8, 76u8,
						],
					)
				}
			}
		}
	}
	pub mod balances {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Transfer {
				pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetBalance {
				pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub new_free: ::core::primitive::u128,
				#[codec(compact)]
				pub new_reserved: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceTransfer {
				pub source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferKeepAlive {
				pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferAll {
				pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceUnreserve {
				pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub amount: ::core::primitive::u128,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "If the sender's account is below the existential deposit as a result"]
				#[doc = "of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
				#[doc = "  types. See related functions below."]
				#[doc = "- It contains a limited number of reads and writes internally and no complex"]
				#[doc = "  computation."]
				#[doc = ""]
				#[doc = "Related functions:"]
				#[doc = ""]
				#[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
				#[doc = "  - Transferring balances to accounts that did not exist before will cause"]
				#[doc = "    `T::OnNewAccount::on_new_account` to be called."]
				#[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
				#[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
				#[doc = "    that the transfer will not kill the origin account."]
				#[doc = "---------------------------------"]
				#[doc = "- Origin account is already in memory, so no DB operations for them."]
				#[doc = "# </weight>"]
				pub fn transfer(
					&self,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer",
						Transfer { dest, value },
						[
							111u8, 222u8, 32u8, 56u8, 171u8, 77u8, 252u8, 29u8, 194u8, 155u8,
							200u8, 192u8, 198u8, 81u8, 23u8, 115u8, 236u8, 91u8, 218u8, 114u8,
							107u8, 141u8, 138u8, 100u8, 237u8, 21u8, 58u8, 172u8, 3u8, 20u8, 216u8,
							38u8,
						],
					)
				}
				#[doc = "Set the balances of a given account."]
				#[doc = ""]
				#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
				#[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
				#[doc = "If the new free or reserved balance is below the existential deposit,"]
				#[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub fn set_balance(
					&self,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<SetBalance> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"set_balance",
						SetBalance { who, new_free, new_reserved },
						[
							234u8, 215u8, 97u8, 98u8, 243u8, 199u8, 57u8, 76u8, 59u8, 161u8, 118u8,
							207u8, 34u8, 197u8, 198u8, 61u8, 231u8, 210u8, 169u8, 235u8, 150u8,
							137u8, 173u8, 49u8, 28u8, 77u8, 84u8, 149u8, 143u8, 210u8, 139u8,
							193u8,
						],
					)
				}
				#[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
				#[doc = "specified."]
				#[doc = "# <weight>"]
				#[doc = "- Same as transfer, but additional read and write because the source account is not"]
				#[doc = "  assumed to be in the overlay."]
				#[doc = "# </weight>"]
				pub fn force_transfer(
					&self,
					source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"force_transfer",
						ForceTransfer { source, dest, value },
						[
							79u8, 174u8, 212u8, 108u8, 184u8, 33u8, 170u8, 29u8, 232u8, 254u8,
							195u8, 218u8, 221u8, 134u8, 57u8, 99u8, 6u8, 70u8, 181u8, 227u8, 56u8,
							239u8, 243u8, 158u8, 157u8, 245u8, 36u8, 162u8, 11u8, 237u8, 147u8,
							15u8,
						],
					)
				}
				#[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
				#[doc = "origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer`] instead."]
				#[doc = ""]
				#[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer_keep_alive",
						TransferKeepAlive { dest, value },
						[
							112u8, 179u8, 75u8, 168u8, 193u8, 221u8, 9u8, 82u8, 190u8, 113u8,
							253u8, 13u8, 130u8, 134u8, 170u8, 216u8, 136u8, 111u8, 242u8, 220u8,
							202u8, 112u8, 47u8, 79u8, 73u8, 244u8, 226u8, 59u8, 240u8, 188u8,
							210u8, 208u8,
						],
					)
				}
				#[doc = "Transfer the entire transferable balance from the caller account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
				#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
				#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
				#[doc = "you might need to prepare the account by removing any reference counters, storage"]
				#[doc = "deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be Signed."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
				#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
				#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true). # <weight>"]
				#[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
				#[doc = "  #</weight>"]
				pub fn transfer_all(
					&self,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<TransferAll> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer_all",
						TransferAll { dest, keep_alive },
						[
							46u8, 129u8, 29u8, 177u8, 221u8, 107u8, 245u8, 69u8, 238u8, 126u8,
							145u8, 26u8, 219u8, 208u8, 14u8, 80u8, 149u8, 1u8, 214u8, 63u8, 67u8,
							201u8, 144u8, 45u8, 129u8, 145u8, 174u8, 71u8, 238u8, 113u8, 208u8,
							34u8,
						],
					)
				}
				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
				pub fn force_unreserve(
					&self,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceUnreserve> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"force_unreserve",
						ForceUnreserve { who, amount },
						[
							160u8, 146u8, 137u8, 76u8, 157u8, 187u8, 66u8, 148u8, 207u8, 76u8,
							32u8, 254u8, 82u8, 215u8, 35u8, 161u8, 213u8, 52u8, 32u8, 98u8, 102u8,
							106u8, 234u8, 123u8, 6u8, 175u8, 184u8, 188u8, 174u8, 106u8, 176u8,
							78u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account was created with some free balance."]
			pub struct Endowed {
				pub account: ::subxt::utils::AccountId32,
				pub free_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
			#[doc = "resulting in an outright loss."]
			pub struct DustLost {
				pub account: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Transfer succeeded."]
			pub struct Transfer {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A balance was set by root."]
			pub struct BalanceSet {
				pub who: ::subxt::utils::AccountId32,
				pub free: ::core::primitive::u128,
				pub reserved: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			pub struct Reserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			pub struct Unreserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balance was moved from the reserve of the first account to the second account."]
			#[doc = "Final argument indicates the destination balance type."]
			pub struct ReserveRepatriated {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
				pub destination_status:
					runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some amount was deposited (e.g. for transaction fees)."]
			pub struct Deposit {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
			pub struct Withdraw {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdraw {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
			pub struct Slashed {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Slashed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The total units issued in the system."]
				pub fn total_issuance(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"TotalIssuance",
						vec![],
						[
							1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8,
							156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8,
							20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Account",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
							40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
							217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Account",
						Vec::new(),
						[
							246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
							40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
							217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Locks",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
							58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
							136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Locks",
						Vec::new(),
						[
							216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
							58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
							136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::ReserveData<
								[::core::primitive::u8; 8usize],
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Reserves",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
							167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
							183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				pub fn reserves_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::ReserveData<
								[::core::primitive::u8; 8usize],
								::core::primitive::u128,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Reserves",
						Vec::new(),
						[
							17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
							167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
							183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
						],
					)
				}
				#[doc = " Storage version of the pallet."]
				#[doc = ""]
				#[doc = " This is set to v2.0.0 for new networks."]
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_balances::Releases>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"StorageVersion",
						vec![],
						[
							135u8, 96u8, 28u8, 234u8, 124u8, 212u8, 56u8, 140u8, 40u8, 101u8,
							235u8, 128u8, 136u8, 221u8, 182u8, 81u8, 17u8, 9u8, 184u8, 228u8,
							174u8, 165u8, 200u8, 162u8, 214u8, 178u8, 227u8, 72u8, 34u8, 5u8,
							173u8, 96u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum amount required to keep an account open."]
				pub fn existential_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Balances",
						"ExistentialDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum number of locks that should exist on an account."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
				pub fn max_locks(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Balances",
						"MaxLocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of named reserves that can exist on an account."]
				pub fn max_reserves(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Balances",
						"MaxReserves",
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
	pub mod transaction_payment {
		use super::{root_mod, runtime_types};
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
			#[doc = "has been paid by `who`."]
			pub struct TransactionFeePaid {
				pub who: ::subxt::utils::AccountId32,
				pub actual_fee: ::core::primitive::u128,
				pub tip: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TransactionFeePaid {
				const PALLET: &'static str = "TransactionPayment";
				const EVENT: &'static str = "TransactionFeePaid";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn next_fee_multiplier(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::fixed_point::FixedU128,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TransactionPayment",
						"NextFeeMultiplier",
						vec![],
						[
							210u8, 0u8, 206u8, 165u8, 183u8, 10u8, 206u8, 52u8, 14u8, 90u8, 218u8,
							197u8, 189u8, 125u8, 113u8, 216u8, 52u8, 161u8, 45u8, 24u8, 245u8,
							237u8, 121u8, 41u8, 106u8, 29u8, 45u8, 129u8, 250u8, 203u8, 206u8,
							180u8,
						],
					)
				}
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_transaction_payment::Releases,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TransactionPayment",
						"StorageVersion",
						vec![],
						[
							219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8, 176u8,
							200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8, 12u8, 198u8,
							58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8, 228u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
				#[doc = " `priority`"]
				#[doc = ""]
				#[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
				#[doc = " added to a tip component in regular `priority` calculations."]
				#[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
				#[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
				#[doc = ""]
				#[doc = " ```rust,ignore"]
				#[doc = " // For `Normal`"]
				#[doc = " let priority = priority_calc(tip);"]
				#[doc = ""]
				#[doc = " // For `Operational`"]
				#[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
				#[doc = " let priority = priority_calc(tip + virtual_tip);"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
				#[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
				#[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
				#[doc = " transactions."]
				pub fn operational_fee_multiplier(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"TransactionPayment",
						"OperationalFeeMultiplier",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
							28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
							114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
							165u8,
						],
					)
				}
			}
		}
	}
	pub mod authorship {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetUncles {
				pub new_uncles: ::std::vec::Vec<
					runtime_types::sp_runtime::generic::header::Header<
						::core::primitive::u32,
						runtime_types::sp_runtime::traits::BlakeTwo256,
					>,
				>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Provide a set of uncles."]
				pub fn set_uncles(
					&self,
					new_uncles: ::std::vec::Vec<
						runtime_types::sp_runtime::generic::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetUncles> {
					::subxt::tx::StaticTxPayload::new(
						"Authorship",
						"set_uncles",
						SetUncles { new_uncles },
						[
							181u8, 70u8, 222u8, 83u8, 154u8, 215u8, 200u8, 64u8, 154u8, 228u8,
							115u8, 247u8, 117u8, 89u8, 229u8, 102u8, 128u8, 189u8, 90u8, 60u8,
							223u8, 19u8, 111u8, 172u8, 5u8, 223u8, 132u8, 37u8, 235u8, 119u8, 42u8,
							64u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Uncles"]
				pub fn uncles(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_authorship::UncleEntryItem<
								::core::primitive::u32,
								::subxt::utils::H256,
								::subxt::utils::AccountId32,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"Uncles",
						vec![],
						[
							193u8, 226u8, 196u8, 151u8, 233u8, 82u8, 60u8, 164u8, 27u8, 156u8,
							231u8, 51u8, 79u8, 134u8, 170u8, 166u8, 71u8, 120u8, 250u8, 255u8,
							52u8, 168u8, 74u8, 199u8, 122u8, 253u8, 248u8, 178u8, 39u8, 233u8,
							132u8, 67u8,
						],
					)
				}
				#[doc = " Author of current block."]
				pub fn author(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"Author",
						vec![],
						[
							149u8, 42u8, 33u8, 147u8, 190u8, 207u8, 174u8, 227u8, 190u8, 110u8,
							25u8, 131u8, 5u8, 167u8, 237u8, 188u8, 188u8, 33u8, 177u8, 126u8,
							181u8, 49u8, 126u8, 118u8, 46u8, 128u8, 154u8, 95u8, 15u8, 91u8, 103u8,
							113u8,
						],
					)
				}
				#[doc = " Whether uncles were already set in this block."]
				pub fn did_set_uncles(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"DidSetUncles",
						vec![],
						[
							64u8, 3u8, 208u8, 187u8, 50u8, 45u8, 37u8, 88u8, 163u8, 226u8, 37u8,
							126u8, 232u8, 107u8, 156u8, 187u8, 29u8, 15u8, 53u8, 46u8, 28u8, 73u8,
							83u8, 123u8, 14u8, 244u8, 243u8, 43u8, 245u8, 143u8, 15u8, 115u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The number of blocks back we should accept uncles."]
				#[doc = " This means that we will deal with uncle-parents that are"]
				#[doc = " `UncleGenerations + 1` before `now`."]
				pub fn uncle_generations(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Authorship",
						"UncleGenerations",
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
	pub mod collator_selection {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetInvulnerables {
				pub new: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetDesiredCandidates {
				pub max: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetCandidacyBond {
				pub bond: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RegisterAsCandidate;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct LeaveIntent;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the list of invulnerable (fixed) collators."]
				pub fn set_invulnerables(
					&self,
					new: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<SetInvulnerables> {
					::subxt::tx::StaticTxPayload::new(
						"CollatorSelection",
						"set_invulnerables",
						SetInvulnerables { new },
						[
							120u8, 177u8, 166u8, 239u8, 2u8, 102u8, 76u8, 143u8, 218u8, 130u8,
							168u8, 152u8, 200u8, 107u8, 221u8, 30u8, 252u8, 18u8, 108u8, 147u8,
							81u8, 251u8, 183u8, 185u8, 0u8, 184u8, 100u8, 251u8, 95u8, 168u8, 26u8,
							142u8,
						],
					)
				}
				#[doc = "Set the ideal number of collators (not including the invulnerables)."]
				#[doc = "If lowering this number, then the number of running collators could be higher than this figure."]
				#[doc = "Aside from that edge case, there should be no other way to have more collators than the desired number."]
				pub fn set_desired_candidates(
					&self,
					max: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetDesiredCandidates> {
					::subxt::tx::StaticTxPayload::new(
						"CollatorSelection",
						"set_desired_candidates",
						SetDesiredCandidates { max },
						[
							181u8, 32u8, 138u8, 37u8, 254u8, 213u8, 197u8, 224u8, 82u8, 26u8, 3u8,
							113u8, 11u8, 146u8, 251u8, 35u8, 250u8, 202u8, 209u8, 2u8, 231u8,
							176u8, 216u8, 124u8, 125u8, 43u8, 52u8, 126u8, 150u8, 140u8, 20u8,
							113u8,
						],
					)
				}
				#[doc = "Set the candidacy bond amount."]
				pub fn set_candidacy_bond(
					&self,
					bond: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<SetCandidacyBond> {
					::subxt::tx::StaticTxPayload::new(
						"CollatorSelection",
						"set_candidacy_bond",
						SetCandidacyBond { bond },
						[
							42u8, 173u8, 79u8, 226u8, 224u8, 202u8, 70u8, 185u8, 125u8, 17u8,
							123u8, 99u8, 107u8, 163u8, 67u8, 75u8, 110u8, 65u8, 248u8, 179u8, 39u8,
							177u8, 135u8, 186u8, 66u8, 237u8, 30u8, 73u8, 163u8, 98u8, 81u8, 152u8,
						],
					)
				}
				#[doc = "Register this account as a collator candidate. The account must (a) already have"]
				#[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
				#[doc = ""]
				#[doc = "This call is not available to `Invulnerable` collators."]
				pub fn register_as_candidate(
					&self,
				) -> ::subxt::tx::StaticTxPayload<RegisterAsCandidate> {
					::subxt::tx::StaticTxPayload::new(
						"CollatorSelection",
						"register_as_candidate",
						RegisterAsCandidate {},
						[
							63u8, 11u8, 114u8, 142u8, 89u8, 78u8, 120u8, 214u8, 22u8, 215u8, 125u8,
							60u8, 203u8, 89u8, 141u8, 126u8, 124u8, 167u8, 70u8, 240u8, 85u8,
							253u8, 34u8, 245u8, 67u8, 46u8, 240u8, 195u8, 57u8, 81u8, 138u8, 69u8,
						],
					)
				}
				#[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
				#[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
				#[doc = ""]
				#[doc = "This call will fail if the total number of candidates would drop below `MinCandidates`."]
				#[doc = ""]
				#[doc = "This call is not available to `Invulnerable` collators."]
				pub fn leave_intent(&self) -> ::subxt::tx::StaticTxPayload<LeaveIntent> {
					::subxt::tx::StaticTxPayload::new(
						"CollatorSelection",
						"leave_intent",
						LeaveIntent {},
						[
							217u8, 3u8, 35u8, 71u8, 152u8, 203u8, 203u8, 212u8, 25u8, 113u8, 158u8,
							124u8, 161u8, 154u8, 32u8, 47u8, 116u8, 134u8, 11u8, 201u8, 154u8,
							40u8, 138u8, 163u8, 184u8, 188u8, 33u8, 237u8, 219u8, 40u8, 63u8,
							221u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct NewInvulnerables {
				pub invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for NewInvulnerables {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewInvulnerables";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct NewDesiredCandidates {
				pub desired_candidates: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewDesiredCandidates {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewDesiredCandidates";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct NewCandidacyBond {
				pub bond_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for NewCandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewCandidacyBond";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CandidateAdded {
				pub account_id: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for CandidateAdded {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CandidateRemoved {
				pub account_id: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for CandidateRemoved {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateRemoved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The invulnerable, fixed collators."]
				pub fn invulnerables(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"Invulnerables",
						vec![],
						[
							215u8, 62u8, 140u8, 81u8, 0u8, 189u8, 182u8, 139u8, 32u8, 42u8, 20u8,
							223u8, 81u8, 212u8, 100u8, 97u8, 146u8, 253u8, 75u8, 123u8, 240u8,
							125u8, 249u8, 62u8, 226u8, 70u8, 57u8, 206u8, 16u8, 74u8, 52u8, 72u8,
						],
					)
				}
				#[doc = " The (community, limited) collation candidates."]
				pub fn candidates(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_collator_selection::pallet::CandidateInfo<
								::subxt::utils::AccountId32,
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"Candidates",
						vec![],
						[
							28u8, 116u8, 232u8, 94u8, 147u8, 216u8, 214u8, 30u8, 26u8, 241u8, 68u8,
							108u8, 165u8, 107u8, 89u8, 136u8, 111u8, 239u8, 150u8, 42u8, 210u8,
							214u8, 192u8, 234u8, 29u8, 41u8, 157u8, 169u8, 120u8, 126u8, 192u8,
							32u8,
						],
					)
				}
				#[doc = " Last block authored by collator."]
				pub fn last_authored_block(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"LastAuthoredBlock",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							53u8, 30u8, 243u8, 31u8, 228u8, 231u8, 175u8, 153u8, 204u8, 241u8,
							76u8, 147u8, 6u8, 202u8, 255u8, 89u8, 30u8, 129u8, 85u8, 92u8, 10u8,
							97u8, 177u8, 129u8, 88u8, 196u8, 7u8, 255u8, 74u8, 52u8, 28u8, 0u8,
						],
					)
				}
				#[doc = " Last block authored by collator."]
				pub fn last_authored_block_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"LastAuthoredBlock",
						Vec::new(),
						[
							53u8, 30u8, 243u8, 31u8, 228u8, 231u8, 175u8, 153u8, 204u8, 241u8,
							76u8, 147u8, 6u8, 202u8, 255u8, 89u8, 30u8, 129u8, 85u8, 92u8, 10u8,
							97u8, 177u8, 129u8, 88u8, 196u8, 7u8, 255u8, 74u8, 52u8, 28u8, 0u8,
						],
					)
				}
				#[doc = " Desired number of candidates."]
				#[doc = ""]
				#[doc = " This should ideally always be less than [`Config::MaxCandidates`] for weights to be correct."]
				pub fn desired_candidates(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"DesiredCandidates",
						vec![],
						[
							161u8, 170u8, 254u8, 76u8, 112u8, 146u8, 144u8, 7u8, 177u8, 152u8,
							146u8, 60u8, 143u8, 237u8, 1u8, 168u8, 176u8, 33u8, 103u8, 35u8, 39u8,
							233u8, 107u8, 253u8, 47u8, 183u8, 11u8, 86u8, 230u8, 13u8, 127u8,
							133u8,
						],
					)
				}
				#[doc = " Fixed amount to deposit to become a collator."]
				#[doc = ""]
				#[doc = " When a collator calls `leave_intent` they immediately receive the deposit back."]
				pub fn candidacy_bond(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"CandidacyBond",
						vec![],
						[
							1u8, 153u8, 211u8, 74u8, 138u8, 178u8, 81u8, 9u8, 205u8, 117u8, 102u8,
							182u8, 56u8, 184u8, 56u8, 62u8, 193u8, 82u8, 224u8, 218u8, 253u8,
							194u8, 250u8, 55u8, 220u8, 107u8, 157u8, 175u8, 62u8, 35u8, 224u8,
							183u8,
						],
					)
				}
			}
		}
	}
	pub mod session {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetKeys {
				pub keys: runtime_types::parachain_runtime::SessionKeys,
				pub proof: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct PurgeKeys;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Sets the session key(s) of the function caller to `keys`."]
				#[doc = "Allows an account to set its session key prior to becoming a validator."]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be signed."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
				#[doc = "  `T::Keys::key_ids()` which is fixed."]
				#[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
				#[doc = "- DbWrites: `origin account`, `NextKeys`"]
				#[doc = "- DbReads per key id: `KeyOwner`"]
				#[doc = "- DbWrites per key id: `KeyOwner`"]
				#[doc = "# </weight>"]
				pub fn set_keys(
					&self,
					keys: runtime_types::parachain_runtime::SessionKeys,
					proof: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SetKeys> {
					::subxt::tx::StaticTxPayload::new(
						"Session",
						"set_keys",
						SetKeys { keys, proof },
						[
							199u8, 56u8, 39u8, 236u8, 44u8, 88u8, 207u8, 0u8, 187u8, 195u8, 218u8,
							94u8, 126u8, 128u8, 37u8, 162u8, 216u8, 223u8, 36u8, 165u8, 18u8, 37u8,
							16u8, 72u8, 136u8, 28u8, 134u8, 230u8, 231u8, 48u8, 230u8, 122u8,
						],
					)
				}
				#[doc = "Removes any session key(s) of the function caller."]
				#[doc = ""]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
				#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
				#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
				#[doc = "usually means being a stash account)."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
				#[doc = "  of `T::Keys::key_ids()` which is fixed."]
				#[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
				#[doc = "- DbWrites: `NextKeys`, `origin account`"]
				#[doc = "- DbWrites per key id: `KeyOwner`"]
				#[doc = "# </weight>"]
				pub fn purge_keys(&self) -> ::subxt::tx::StaticTxPayload<PurgeKeys> {
					::subxt::tx::StaticTxPayload::new(
						"Session",
						"purge_keys",
						PurgeKeys {},
						[
							200u8, 255u8, 4u8, 213u8, 188u8, 92u8, 99u8, 116u8, 163u8, 152u8, 29u8,
							35u8, 133u8, 119u8, 246u8, 44u8, 91u8, 31u8, 145u8, 23u8, 213u8, 64u8,
							71u8, 242u8, 207u8, 239u8, 231u8, 37u8, 61u8, 63u8, 190u8, 35u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "New session has happened. Note that the argument is the session index, not the"]
			#[doc = "block number as the type might suggest."]
			pub struct NewSession {
				pub session_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewSession {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "NewSession";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current set of validators."]
				pub fn validators(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::utils::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"Validators",
						vec![],
						[
							144u8, 235u8, 200u8, 43u8, 151u8, 57u8, 147u8, 172u8, 201u8, 202u8,
							242u8, 96u8, 57u8, 76u8, 124u8, 77u8, 42u8, 113u8, 218u8, 220u8, 230u8,
							32u8, 151u8, 152u8, 172u8, 106u8, 60u8, 227u8, 122u8, 118u8, 137u8,
							68u8,
						],
					)
				}
				#[doc = " Current index of the session."]
				pub fn current_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"CurrentIndex",
						vec![],
						[
							148u8, 179u8, 159u8, 15u8, 197u8, 95u8, 214u8, 30u8, 209u8, 251u8,
							183u8, 231u8, 91u8, 25u8, 181u8, 191u8, 143u8, 252u8, 227u8, 80u8,
							159u8, 66u8, 194u8, 67u8, 113u8, 74u8, 111u8, 91u8, 218u8, 187u8,
							130u8, 40u8,
						],
					)
				}
				#[doc = " True if the underlying economic identities or weighting behind the validators"]
				#[doc = " has changed in the queued validator set."]
				pub fn queued_changed(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"QueuedChanged",
						vec![],
						[
							105u8, 140u8, 235u8, 218u8, 96u8, 100u8, 252u8, 10u8, 58u8, 221u8,
							244u8, 251u8, 67u8, 91u8, 80u8, 202u8, 152u8, 42u8, 50u8, 113u8, 200u8,
							247u8, 59u8, 213u8, 77u8, 195u8, 1u8, 150u8, 220u8, 18u8, 245u8, 46u8,
						],
					)
				}
				#[doc = " The queued keys for the next session. When the next session begins, these keys"]
				#[doc = " will be used to determine the validator's session keys."]
				pub fn queued_keys(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::subxt::utils::AccountId32,
							runtime_types::parachain_runtime::SessionKeys,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"QueuedKeys",
						vec![],
						[
							42u8, 134u8, 252u8, 233u8, 29u8, 69u8, 168u8, 107u8, 77u8, 70u8, 80u8,
							189u8, 149u8, 227u8, 77u8, 74u8, 100u8, 175u8, 10u8, 162u8, 145u8,
							105u8, 85u8, 196u8, 169u8, 195u8, 116u8, 255u8, 112u8, 122u8, 112u8,
							133u8,
						],
					)
				}
				#[doc = " Indices of disabled validators."]
				#[doc = ""]
				#[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
				#[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
				#[doc = " a new set of identities."]
				pub fn disabled_validators(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u32>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"DisabledValidators",
						vec![],
						[
							135u8, 22u8, 22u8, 97u8, 82u8, 217u8, 144u8, 141u8, 121u8, 240u8,
							189u8, 16u8, 176u8, 88u8, 177u8, 31u8, 20u8, 242u8, 73u8, 104u8, 11u8,
							110u8, 214u8, 34u8, 52u8, 217u8, 106u8, 33u8, 174u8, 174u8, 198u8,
							84u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::parachain_runtime::SessionKeys,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"NextKeys",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							21u8, 0u8, 237u8, 42u8, 156u8, 77u8, 229u8, 211u8, 105u8, 8u8, 231u8,
							5u8, 246u8, 188u8, 69u8, 143u8, 202u8, 240u8, 252u8, 253u8, 106u8,
							37u8, 51u8, 244u8, 206u8, 199u8, 249u8, 37u8, 17u8, 102u8, 20u8, 246u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::parachain_runtime::SessionKeys,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"NextKeys",
						Vec::new(),
						[
							21u8, 0u8, 237u8, 42u8, 156u8, 77u8, 229u8, 211u8, 105u8, 8u8, 231u8,
							5u8, 246u8, 188u8, 69u8, 143u8, 202u8, 240u8, 252u8, 253u8, 106u8,
							37u8, 51u8, 244u8, 206u8, 199u8, 249u8, 37u8, 17u8, 102u8, 20u8, 246u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"KeyOwner",
						vec![::subxt::storage::address::StorageMapKey::new(
							&(_0.borrow(), _1.borrow()),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
							199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
							0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"KeyOwner",
						Vec::new(),
						[
							4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
							199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
							0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
						],
					)
				}
			}
		}
	}
	pub mod aura {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current authority set."]
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Aura",
						"Authorities",
						vec![],
						[
							199u8, 89u8, 94u8, 48u8, 249u8, 35u8, 105u8, 90u8, 15u8, 86u8, 218u8,
							85u8, 22u8, 236u8, 228u8, 36u8, 137u8, 64u8, 236u8, 171u8, 242u8,
							217u8, 91u8, 240u8, 205u8, 205u8, 226u8, 16u8, 147u8, 235u8, 181u8,
							41u8,
						],
					)
				}
				#[doc = " The current slot of this block."]
				#[doc = ""]
				#[doc = " This will be set in `on_initialize`."]
				pub fn current_slot(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_consensus_slots::Slot>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Aura",
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
			}
		}
	}
	pub mod aura_ext {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Serves as cache for the authorities."]
				#[doc = ""]
				#[doc = " The authorities in AuRa are overwritten in `on_initialize` when we switch to a new session,"]
				#[doc = " but we require the old authorities to verify the seal when validating a PoV. This will always"]
				#[doc = " be updated to the latest AuRa authorities in `on_finalize`."]
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AuraExt",
						"Authorities",
						vec![],
						[
							199u8, 89u8, 94u8, 48u8, 249u8, 35u8, 105u8, 90u8, 15u8, 86u8, 218u8,
							85u8, 22u8, 236u8, 228u8, 36u8, 137u8, 64u8, 236u8, 171u8, 242u8,
							217u8, 91u8, 240u8, 205u8, 205u8, 226u8, 16u8, 147u8, 235u8, 181u8,
							41u8,
						],
					)
				}
			}
		}
	}
	pub mod xcmp_queue {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ServiceOverweight {
				pub index: ::core::primitive::u64,
				pub weight_limit: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SuspendXcmExecution;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ResumeXcmExecution;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateSuspendThreshold {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateDropThreshold {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateResumeThreshold {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateThresholdWeight {
				pub new: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateWeightRestrictDecay {
				pub new: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateXcmpMaxIndividualWeight {
				pub new: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Services a single overweight XCM."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
				#[doc = "- `index`: The index of the overweight XCM to service"]
				#[doc = "- `weight_limit`: The amount of weight that XCM execution may take."]
				#[doc = ""]
				#[doc = "Errors:"]
				#[doc = "- `BadOverweightIndex`: XCM under `index` is not found in the `Overweight` storage map."]
				#[doc = "- `BadXcm`: XCM under `index` cannot be properly decoded into a valid XCM format."]
				#[doc = "- `WeightOverLimit`: XCM execution may use greater `weight_limit`."]
				#[doc = ""]
				#[doc = "Events:"]
				#[doc = "- `OverweightServiced`: On success."]
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<ServiceOverweight> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"service_overweight",
						ServiceOverweight { index, weight_limit },
						[
							225u8, 41u8, 132u8, 91u8, 28u8, 116u8, 89u8, 197u8, 194u8, 131u8, 28u8,
							217u8, 102u8, 241u8, 122u8, 230u8, 242u8, 75u8, 83u8, 67u8, 104u8,
							55u8, 133u8, 129u8, 91u8, 25u8, 185u8, 131u8, 22u8, 253u8, 84u8, 221u8,
						],
					)
				}
				#[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ControllerOrigin`."]
				pub fn suspend_xcm_execution(
					&self,
				) -> ::subxt::tx::StaticTxPayload<SuspendXcmExecution> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"suspend_xcm_execution",
						SuspendXcmExecution {},
						[
							139u8, 76u8, 166u8, 86u8, 106u8, 144u8, 16u8, 47u8, 105u8, 185u8, 7u8,
							7u8, 63u8, 14u8, 250u8, 236u8, 99u8, 121u8, 101u8, 143u8, 28u8, 175u8,
							108u8, 197u8, 226u8, 43u8, 103u8, 92u8, 186u8, 12u8, 51u8, 153u8,
						],
					)
				}
				#[doc = "Resumes all XCM executions for the XCMP queue."]
				#[doc = ""]
				#[doc = "Note that this function doesn't change the status of the in/out bound channels."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ControllerOrigin`."]
				pub fn resume_xcm_execution(
					&self,
				) -> ::subxt::tx::StaticTxPayload<ResumeXcmExecution> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"resume_xcm_execution",
						ResumeXcmExecution {},
						[
							67u8, 111u8, 47u8, 237u8, 79u8, 42u8, 90u8, 56u8, 245u8, 2u8, 20u8,
							23u8, 33u8, 121u8, 135u8, 50u8, 204u8, 147u8, 195u8, 80u8, 177u8,
							202u8, 8u8, 160u8, 164u8, 138u8, 64u8, 252u8, 178u8, 63u8, 102u8,
							245u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages of messages which must be in the queue for the other side to be told to"]
				#[doc = "suspend their sending."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
				pub fn update_suspend_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<UpdateSuspendThreshold> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_suspend_threshold",
						UpdateSuspendThreshold { new },
						[
							155u8, 120u8, 9u8, 228u8, 110u8, 62u8, 233u8, 36u8, 57u8, 85u8, 19u8,
							67u8, 246u8, 88u8, 81u8, 116u8, 243u8, 236u8, 174u8, 130u8, 8u8, 246u8,
							254u8, 97u8, 155u8, 207u8, 123u8, 60u8, 164u8, 14u8, 196u8, 97u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages of messages which must be in the queue after which we drop any further"]
				#[doc = "messages from the channel."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
				pub fn update_drop_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<UpdateDropThreshold> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_drop_threshold",
						UpdateDropThreshold { new },
						[
							146u8, 177u8, 164u8, 96u8, 247u8, 182u8, 229u8, 175u8, 194u8, 101u8,
							186u8, 168u8, 94u8, 114u8, 172u8, 119u8, 35u8, 222u8, 175u8, 21u8,
							67u8, 61u8, 216u8, 144u8, 194u8, 10u8, 181u8, 62u8, 166u8, 198u8,
							138u8, 243u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages of messages which the queue must be reduced to before it signals that"]
				#[doc = "message sending may recommence after it has been suspended."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
				pub fn update_resume_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<UpdateResumeThreshold> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_resume_threshold",
						UpdateResumeThreshold { new },
						[
							231u8, 128u8, 80u8, 179u8, 61u8, 50u8, 103u8, 209u8, 103u8, 55u8,
							101u8, 113u8, 150u8, 10u8, 202u8, 7u8, 0u8, 77u8, 58u8, 4u8, 227u8,
							17u8, 225u8, 112u8, 121u8, 203u8, 184u8, 113u8, 231u8, 156u8, 174u8,
							154u8,
						],
					)
				}
				#[doc = "Overwrites the amount of remaining weight under which we stop processing messages."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.threshold_weight`"]
				pub fn update_threshold_weight(
					&self,
					new: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<UpdateThresholdWeight> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_threshold_weight",
						UpdateThresholdWeight { new },
						[
							129u8, 208u8, 93u8, 179u8, 45u8, 236u8, 84u8, 209u8, 37u8, 226u8, 88u8,
							123u8, 156u8, 101u8, 93u8, 84u8, 110u8, 61u8, 56u8, 45u8, 14u8, 120u8,
							181u8, 71u8, 174u8, 104u8, 225u8, 36u8, 17u8, 74u8, 94u8, 59u8,
						],
					)
				}
				#[doc = "Overwrites the speed to which the available weight approaches the maximum weight."]
				#[doc = "A lower number results in a faster progression. A value of 1 makes the entire weight available initially."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.weight_restrict_decay`."]
				pub fn update_weight_restrict_decay(
					&self,
					new: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<UpdateWeightRestrictDecay> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_weight_restrict_decay",
						UpdateWeightRestrictDecay { new },
						[
							73u8, 98u8, 189u8, 10u8, 137u8, 162u8, 71u8, 54u8, 24u8, 117u8, 15u8,
							137u8, 251u8, 121u8, 86u8, 5u8, 123u8, 42u8, 151u8, 244u8, 200u8,
							140u8, 104u8, 149u8, 101u8, 14u8, 58u8, 163u8, 208u8, 205u8, 177u8,
							142u8,
						],
					)
				}
				#[doc = "Overwrite the maximum amount of weight any individual message may consume."]
				#[doc = "Messages above this weight go into the overweight queue and may only be serviced explicitly."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.xcmp_max_individual_weight`."]
				pub fn update_xcmp_max_individual_weight(
					&self,
					new: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<UpdateXcmpMaxIndividualWeight> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_xcmp_max_individual_weight",
						UpdateXcmpMaxIndividualWeight { new },
						[
							52u8, 93u8, 25u8, 215u8, 36u8, 235u8, 88u8, 49u8, 142u8, 132u8, 57u8,
							2u8, 204u8, 195u8, 166u8, 254u8, 235u8, 247u8, 142u8, 207u8, 224u8,
							43u8, 7u8, 106u8, 142u8, 3u8, 188u8, 101u8, 9u8, 75u8, 57u8, 39u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some XCM was executed ok."]
			pub struct Success {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for Success {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Success";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some XCM failed."]
			pub struct Fail {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
				pub error: runtime_types::xcm::v2::traits::Error,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for Fail {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Fail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Bad XCM version used."]
			pub struct BadVersion {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
			}
			impl ::subxt::events::StaticEvent for BadVersion {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Bad XCM format used."]
			pub struct BadFormat {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
			}
			impl ::subxt::events::StaticEvent for BadFormat {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An upward message was sent to the relay chain."]
			pub struct UpwardMessageSent {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
			}
			impl ::subxt::events::StaticEvent for UpwardMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "UpwardMessageSent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An HRMP message was sent to a sibling parachain."]
			pub struct XcmpMessageSent {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
			}
			impl ::subxt::events::StaticEvent for XcmpMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "XcmpMessageSent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An XCM exceeded the individual message weight budget."]
			pub struct OverweightEnqueued {
				pub sender: runtime_types::polkadot_parachain::primitives::Id,
				pub sent_at: ::core::primitive::u32,
				pub index: ::core::primitive::u64,
				pub required: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightEnqueued {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An XCM from the overweight queue was executed with the given actual weight used."]
			pub struct OverweightServiced {
				pub index: ::core::primitive::u64,
				pub used: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightServiced {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "OverweightServiced";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Status of the inbound XCMP channels."]
				pub fn inbound_xcmp_status(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::cumulus_pallet_xcmp_queue::InboundChannelDetails,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"InboundXcmpStatus",
						vec![],
						[
							183u8, 198u8, 237u8, 153u8, 132u8, 201u8, 87u8, 182u8, 121u8, 164u8,
							129u8, 241u8, 58u8, 192u8, 115u8, 152u8, 7u8, 33u8, 95u8, 51u8, 2u8,
							176u8, 144u8, 12u8, 125u8, 83u8, 92u8, 198u8, 211u8, 101u8, 28u8, 50u8,
						],
					)
				}
				#[doc = " Inbound aggregate XCMP messages. It can only be one per ParaId/block."]
				pub fn inbound_xcmp_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"InboundXcmpMessages",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							157u8, 232u8, 222u8, 97u8, 218u8, 96u8, 96u8, 90u8, 216u8, 205u8, 39u8,
							130u8, 109u8, 152u8, 127u8, 57u8, 54u8, 63u8, 104u8, 135u8, 33u8,
							175u8, 197u8, 166u8, 238u8, 22u8, 137u8, 162u8, 226u8, 199u8, 87u8,
							25u8,
						],
					)
				}
				#[doc = " Inbound aggregate XCMP messages. It can only be one per ParaId/block."]
				pub fn inbound_xcmp_messages_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"InboundXcmpMessages",
						Vec::new(),
						[
							157u8, 232u8, 222u8, 97u8, 218u8, 96u8, 96u8, 90u8, 216u8, 205u8, 39u8,
							130u8, 109u8, 152u8, 127u8, 57u8, 54u8, 63u8, 104u8, 135u8, 33u8,
							175u8, 197u8, 166u8, 238u8, 22u8, 137u8, 162u8, 226u8, 199u8, 87u8,
							25u8,
						],
					)
				}
				#[doc = " The non-empty XCMP channels in order of becoming non-empty, and the index of the first"]
				#[doc = " and last outbound message. If the two indices are equal, then it indicates an empty"]
				#[doc = " queue and there must be a non-`Ok` `OutboundStatus`. We assume queues grow no greater"]
				#[doc = " than 65535 items. Queue indices for normal messages begin at one; zero is reserved in"]
				#[doc = " case of the need to send a high-priority signal message this block."]
				#[doc = " The bool is true if there is a signal message waiting to be sent."]
				pub fn outbound_xcmp_status(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::cumulus_pallet_xcmp_queue::OutboundChannelDetails,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"OutboundXcmpStatus",
						vec![],
						[
							238u8, 120u8, 185u8, 141u8, 82u8, 159u8, 41u8, 68u8, 204u8, 15u8, 46u8,
							152u8, 144u8, 74u8, 250u8, 83u8, 71u8, 105u8, 54u8, 53u8, 226u8, 87u8,
							14u8, 202u8, 58u8, 160u8, 54u8, 162u8, 239u8, 248u8, 227u8, 116u8,
						],
					)
				}
				#[doc = " The messages outbound in a given XCMP channel."]
				pub fn outbound_xcmp_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u16>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"OutboundXcmpMessages",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							50u8, 182u8, 237u8, 191u8, 106u8, 67u8, 54u8, 1u8, 17u8, 107u8, 70u8,
							90u8, 202u8, 8u8, 63u8, 184u8, 171u8, 111u8, 192u8, 196u8, 7u8, 31u8,
							186u8, 68u8, 31u8, 63u8, 71u8, 61u8, 83u8, 223u8, 79u8, 200u8,
						],
					)
				}
				#[doc = " The messages outbound in a given XCMP channel."]
				pub fn outbound_xcmp_messages_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"OutboundXcmpMessages",
						Vec::new(),
						[
							50u8, 182u8, 237u8, 191u8, 106u8, 67u8, 54u8, 1u8, 17u8, 107u8, 70u8,
							90u8, 202u8, 8u8, 63u8, 184u8, 171u8, 111u8, 192u8, 196u8, 7u8, 31u8,
							186u8, 68u8, 31u8, 63u8, 71u8, 61u8, 83u8, 223u8, 79u8, 200u8,
						],
					)
				}
				#[doc = " Any signal messages waiting to be sent."]
				pub fn signal_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"SignalMessages",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							156u8, 242u8, 186u8, 89u8, 177u8, 195u8, 90u8, 121u8, 94u8, 106u8,
							222u8, 78u8, 19u8, 162u8, 179u8, 96u8, 38u8, 113u8, 209u8, 148u8, 29u8,
							110u8, 106u8, 167u8, 162u8, 96u8, 221u8, 20u8, 33u8, 179u8, 168u8,
							142u8,
						],
					)
				}
				#[doc = " Any signal messages waiting to be sent."]
				pub fn signal_messages_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"SignalMessages",
						Vec::new(),
						[
							156u8, 242u8, 186u8, 89u8, 177u8, 195u8, 90u8, 121u8, 94u8, 106u8,
							222u8, 78u8, 19u8, 162u8, 179u8, 96u8, 38u8, 113u8, 209u8, 148u8, 29u8,
							110u8, 106u8, 167u8, 162u8, 96u8, 221u8, 20u8, 33u8, 179u8, 168u8,
							142u8,
						],
					)
				}
				#[doc = " The configuration which controls the dynamics of the outbound queue."]
				pub fn queue_config(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"QueueConfig",
						vec![],
						[
							154u8, 172u8, 227u8, 208u8, 130u8, 93u8, 173u8, 129u8, 33u8, 75u8,
							180u8, 100u8, 35u8, 154u8, 40u8, 188u8, 86u8, 53u8, 74u8, 118u8, 131u8,
							159u8, 240u8, 159u8, 185u8, 45u8, 165u8, 6u8, 90u8, 125u8, 77u8, 253u8,
						],
					)
				}
				#[doc = " The messages that exceeded max individual message weight budget."]
				#[doc = ""]
				#[doc = " These message stay in this storage map until they are manually dispatched via"]
				#[doc = " `service_overweight`."]
				pub fn overweight(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"Overweight",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							222u8, 249u8, 232u8, 110u8, 117u8, 229u8, 165u8, 164u8, 219u8, 219u8,
							149u8, 204u8, 25u8, 78u8, 204u8, 116u8, 111u8, 114u8, 120u8, 222u8,
							56u8, 77u8, 122u8, 147u8, 108u8, 15u8, 94u8, 161u8, 212u8, 50u8, 7u8,
							7u8,
						],
					)
				}
				#[doc = " The messages that exceeded max individual message weight budget."]
				#[doc = ""]
				#[doc = " These message stay in this storage map until they are manually dispatched via"]
				#[doc = " `service_overweight`."]
				pub fn overweight_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"Overweight",
						Vec::new(),
						[
							222u8, 249u8, 232u8, 110u8, 117u8, 229u8, 165u8, 164u8, 219u8, 219u8,
							149u8, 204u8, 25u8, 78u8, 204u8, 116u8, 111u8, 114u8, 120u8, 222u8,
							56u8, 77u8, 122u8, 147u8, 108u8, 15u8, 94u8, 161u8, 212u8, 50u8, 7u8,
							7u8,
						],
					)
				}
				#[doc = " The number of overweight messages ever recorded in `Overweight`. Also doubles as the next"]
				#[doc = " available free overweight index."]
				pub fn overweight_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"OverweightCount",
						vec![],
						[
							102u8, 180u8, 196u8, 148u8, 115u8, 62u8, 46u8, 238u8, 97u8, 116u8,
							117u8, 42u8, 14u8, 5u8, 72u8, 237u8, 230u8, 46u8, 150u8, 126u8, 89u8,
							64u8, 233u8, 166u8, 180u8, 137u8, 52u8, 233u8, 252u8, 255u8, 36u8,
							20u8,
						],
					)
				}
				#[doc = " Whether or not the XCMP queue is suspended from executing incoming XCMs or not."]
				pub fn queue_suspended(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"QueueSuspended",
						vec![],
						[
							23u8, 37u8, 48u8, 112u8, 222u8, 17u8, 252u8, 65u8, 160u8, 217u8, 218u8,
							30u8, 2u8, 1u8, 204u8, 0u8, 251u8, 17u8, 138u8, 197u8, 164u8, 50u8,
							122u8, 0u8, 31u8, 238u8, 147u8, 213u8, 30u8, 132u8, 184u8, 215u8,
						],
					)
				}
			}
		}
	}
	pub mod polkadot_xcm {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Send {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TeleportAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ReserveTransferAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Execute {
				pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
				pub max_weight: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceXcmVersion {
				pub location:
					::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
				pub xcm_version: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceDefaultXcmVersion {
				pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceSubscribeVersionNotify {
				pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceUnsubscribeVersionNotify {
				pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct LimitedReserveTransferAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct LimitedTeleportAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn send(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					message: runtime_types::xcm::VersionedXcm,
				) -> ::subxt::tx::StaticTxPayload<Send> {
					::subxt::tx::StaticTxPayload::new(
						"PolkadotXcm",
						"send",
						Send {
							dest: ::std::boxed::Box::new(dest),
							message: ::std::boxed::Box::new(message),
						},
						[
							190u8, 88u8, 197u8, 248u8, 111u8, 198u8, 199u8, 206u8, 39u8, 121u8,
							23u8, 121u8, 93u8, 82u8, 22u8, 61u8, 96u8, 210u8, 142u8, 249u8, 195u8,
							78u8, 44u8, 8u8, 118u8, 120u8, 113u8, 168u8, 99u8, 94u8, 232u8, 4u8,
						],
					)
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
				#[doc = "  `dest` side. May not be empty."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub fn teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<TeleportAssets> {
					::subxt::tx::StaticTxPayload::new(
						"PolkadotXcm",
						"teleport_assets",
						TeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							255u8, 5u8, 68u8, 38u8, 44u8, 181u8, 75u8, 221u8, 239u8, 103u8, 88u8,
							47u8, 136u8, 90u8, 253u8, 55u8, 0u8, 122u8, 217u8, 126u8, 13u8, 77u8,
							209u8, 41u8, 7u8, 35u8, 235u8, 171u8, 150u8, 235u8, 202u8, 240u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
				#[doc = "chain and forward a notification XCM."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
				#[doc = "  `dest` side."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub fn reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ReserveTransferAssets> {
					::subxt::tx::StaticTxPayload::new(
						"PolkadotXcm",
						"reserve_transfer_assets",
						ReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							177u8, 160u8, 188u8, 106u8, 153u8, 135u8, 121u8, 12u8, 83u8, 233u8,
							43u8, 161u8, 133u8, 26u8, 104u8, 79u8, 113u8, 8u8, 33u8, 128u8, 82u8,
							62u8, 30u8, 46u8, 203u8, 199u8, 175u8, 193u8, 55u8, 130u8, 206u8, 28u8,
						],
					)
				}
				#[doc = "Execute an XCM message from a local, signed, origin."]
				#[doc = ""]
				#[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
				#[doc = "partially."]
				#[doc = ""]
				#[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than the"]
				#[doc = "maximum amount of weight that the message could take to be executed, then no execution"]
				#[doc = "attempt will be made."]
				#[doc = ""]
				#[doc = "NOTE: A successful return to this does *not* imply that the `msg` was executed successfully"]
				#[doc = "to completion; only that *some* of it was executed."]
				pub fn execute(
					&self,
					message: runtime_types::xcm::VersionedXcm,
					max_weight: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<Execute> {
					::subxt::tx::StaticTxPayload::new(
						"PolkadotXcm",
						"execute",
						Execute { message: ::std::boxed::Box::new(message), max_weight },
						[
							191u8, 177u8, 39u8, 21u8, 1u8, 110u8, 39u8, 58u8, 94u8, 27u8, 44u8,
							18u8, 253u8, 135u8, 100u8, 205u8, 0u8, 231u8, 68u8, 247u8, 5u8, 140u8,
							131u8, 184u8, 251u8, 197u8, 100u8, 113u8, 253u8, 255u8, 120u8, 206u8,
						],
					)
				}
				#[doc = "Extoll that a particular destination can be communicated with through a particular"]
				#[doc = "version of XCM."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `location`: The destination that is being described."]
				#[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
				pub fn force_xcm_version(
					&self,
					location: runtime_types::xcm::v1::multilocation::MultiLocation,
					xcm_version: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ForceXcmVersion> {
					::subxt::tx::StaticTxPayload::new(
						"PolkadotXcm",
						"force_xcm_version",
						ForceXcmVersion { location: ::std::boxed::Box::new(location), xcm_version },
						[
							231u8, 106u8, 60u8, 226u8, 31u8, 25u8, 20u8, 115u8, 107u8, 246u8,
							248u8, 11u8, 71u8, 183u8, 93u8, 3u8, 219u8, 21u8, 97u8, 188u8, 119u8,
							121u8, 239u8, 72u8, 200u8, 81u8, 6u8, 177u8, 111u8, 188u8, 168u8, 86u8,
						],
					)
				}
				#[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
				#[doc = "version a destination can accept is unknown)."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
				pub fn force_default_xcm_version(
					&self,
					maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::StaticTxPayload<ForceDefaultXcmVersion> {
					::subxt::tx::StaticTxPayload::new(
						"PolkadotXcm",
						"force_default_xcm_version",
						ForceDefaultXcmVersion { maybe_xcm_version },
						[
							38u8, 36u8, 59u8, 231u8, 18u8, 79u8, 76u8, 9u8, 200u8, 125u8, 214u8,
							166u8, 37u8, 99u8, 111u8, 161u8, 135u8, 2u8, 133u8, 157u8, 165u8, 18u8,
							152u8, 81u8, 209u8, 255u8, 137u8, 237u8, 28u8, 126u8, 224u8, 141u8,
						],
					)
				}
				#[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
				pub fn force_subscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::tx::StaticTxPayload<ForceSubscribeVersionNotify> {
					::subxt::tx::StaticTxPayload::new(
						"PolkadotXcm",
						"force_subscribe_version_notify",
						ForceSubscribeVersionNotify { location: ::std::boxed::Box::new(location) },
						[
							136u8, 216u8, 207u8, 51u8, 42u8, 153u8, 92u8, 70u8, 140u8, 169u8,
							172u8, 89u8, 69u8, 28u8, 200u8, 100u8, 209u8, 226u8, 194u8, 240u8,
							71u8, 38u8, 18u8, 6u8, 6u8, 83u8, 103u8, 254u8, 248u8, 241u8, 62u8,
							189u8,
						],
					)
				}
				#[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
				#[doc = "version changes."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
				#[doc = "  notifications which we no longer desire."]
				pub fn force_unsubscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::tx::StaticTxPayload<ForceUnsubscribeVersionNotify> {
					::subxt::tx::StaticTxPayload::new(
						"PolkadotXcm",
						"force_unsubscribe_version_notify",
						ForceUnsubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						},
						[
							51u8, 72u8, 5u8, 227u8, 251u8, 243u8, 199u8, 9u8, 8u8, 213u8, 191u8,
							52u8, 21u8, 215u8, 170u8, 6u8, 53u8, 242u8, 225u8, 89u8, 150u8, 142u8,
							104u8, 249u8, 225u8, 209u8, 142u8, 234u8, 161u8, 100u8, 153u8, 120u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
				#[doc = "chain and forward a notification XCM."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
				#[doc = "  `dest` side."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn limited_reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::tx::StaticTxPayload<LimitedReserveTransferAssets> {
					::subxt::tx::StaticTxPayload::new(
						"PolkadotXcm",
						"limited_reserve_transfer_assets",
						LimitedReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							191u8, 81u8, 68u8, 116u8, 196u8, 125u8, 226u8, 154u8, 144u8, 126u8,
							159u8, 149u8, 17u8, 124u8, 205u8, 60u8, 249u8, 106u8, 38u8, 251u8,
							136u8, 128u8, 81u8, 201u8, 164u8, 242u8, 216u8, 80u8, 21u8, 234u8,
							20u8, 70u8,
						],
					)
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
				#[doc = "  `dest` side. May not be empty."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn limited_teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::tx::StaticTxPayload<LimitedTeleportAssets> {
					::subxt::tx::StaticTxPayload::new(
						"PolkadotXcm",
						"limited_teleport_assets",
						LimitedTeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							29u8, 31u8, 229u8, 83u8, 40u8, 60u8, 36u8, 185u8, 169u8, 74u8, 30u8,
							47u8, 118u8, 118u8, 22u8, 15u8, 246u8, 220u8, 169u8, 135u8, 72u8,
							154u8, 109u8, 192u8, 195u8, 58u8, 121u8, 240u8, 166u8, 243u8, 29u8,
							29u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Execution of an XCM message was attempted."]
			#[doc = ""]
			#[doc = "\\[ outcome \\]"]
			pub struct Attempted(pub runtime_types::xcm::v2::traits::Outcome);
			impl ::subxt::events::StaticEvent for Attempted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Attempted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A XCM message was sent."]
			#[doc = ""]
			#[doc = "\\[ origin, destination, message \\]"]
			pub struct Sent(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::v2::Xcm,
			);
			impl ::subxt::events::StaticEvent for Sent {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Sent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response received which does not match a registered query. This may be because a"]
			#[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
			#[doc = "because the query timed out."]
			#[doc = ""]
			#[doc = "\\[ origin location, id \\]"]
			pub struct UnexpectedResponse(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for UnexpectedResponse {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "UnexpectedResponse";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
			#[doc = "no registered notification call."]
			#[doc = ""]
			#[doc = "\\[ id, response \\]"]
			pub struct ResponseReady(
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v2::Response,
			);
			impl ::subxt::events::StaticEvent for ResponseReady {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseReady";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and query is removed. The registered notification has"]
			#[doc = "been dispatched and executed successfully."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index \\]"]
			pub struct Notified(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for Notified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Notified";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and query is removed. The registered notification could"]
			#[doc = "not be dispatched because the dispatch weight is greater than the maximum weight"]
			#[doc = "originally budgeted by this runtime for the query result."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index, actual weight, max budgeted weight \\]"]
			pub struct NotifyOverweight(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
				pub runtime_types::sp_weights::weight_v2::Weight,
				pub runtime_types::sp_weights::weight_v2::Weight,
			);
			impl ::subxt::events::StaticEvent for NotifyOverweight {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyOverweight";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and query is removed. There was a general error with"]
			#[doc = "dispatching the notification call."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index \\]"]
			pub struct NotifyDispatchError(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for NotifyDispatchError {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDispatchError";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
			#[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
			#[doc = "is not `(origin, QueryId, Response)`."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index \\]"]
			pub struct NotifyDecodeFailed(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for NotifyDecodeFailed {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDecodeFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Expected query response has been received but the origin location of the response does"]
			#[doc = "not match that expected. The query remains registered for a later, valid, response to"]
			#[doc = "be received and acted upon."]
			#[doc = ""]
			#[doc = "\\[ origin location, id, expected location \\]"]
			pub struct InvalidResponder(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub ::core::option::Option<runtime_types::xcm::v1::multilocation::MultiLocation>,
			);
			impl ::subxt::events::StaticEvent for InvalidResponder {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponder";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Expected query response has been received but the expected origin location placed in"]
			#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
			#[doc = ""]
			#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
			#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
			#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
			#[doc = "needed."]
			#[doc = ""]
			#[doc = "\\[ origin location, id \\]"]
			pub struct InvalidResponderVersion(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for InvalidResponderVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponderVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "Received query response has been read and removed."]
			#[doc = ""]
			#[doc = "\\[ id \\]"]
			pub struct ResponseTaken(pub ::core::primitive::u64);
			impl ::subxt::events::StaticEvent for ResponseTaken {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseTaken";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some assets have been placed in an asset trap."]
			#[doc = ""]
			#[doc = "\\[ hash, origin, assets \\]"]
			pub struct AssetsTrapped(
				pub ::subxt::utils::H256,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::VersionedMultiAssets,
			);
			impl ::subxt::events::StaticEvent for AssetsTrapped {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AssetsTrapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An XCM version change notification message has been attempted to be sent."]
			#[doc = ""]
			#[doc = "\\[ destination, result \\]"]
			pub struct VersionChangeNotified(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for VersionChangeNotified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionChangeNotified";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The supported version of a location has been changed. This might be through an"]
			#[doc = "automatic notification or a manual intervention."]
			#[doc = ""]
			#[doc = "\\[ location, XCM version \\]"]
			pub struct SupportedVersionChanged(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for SupportedVersionChanged {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "SupportedVersionChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "sending the notification to it."]
			#[doc = ""]
			#[doc = "\\[ location, query ID, error \\]"]
			pub struct NotifyTargetSendFail(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v2::traits::Error,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetSendFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetSendFail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "migrating the location to our new XCM format."]
			#[doc = ""]
			#[doc = "\\[ location, query ID \\]"]
			pub struct NotifyTargetMigrationFail(
				pub runtime_types::xcm::VersionedMultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetMigrationFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetMigrationFail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some assets have been claimed from an asset trap"]
			#[doc = ""]
			#[doc = "\\[ hash, origin, assets \\]"]
			pub struct AssetsClaimed(
				pub ::subxt::utils::H256,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::VersionedMultiAssets,
			);
			impl ::subxt::events::StaticEvent for AssetsClaimed {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AssetsClaimed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The latest available query index."]
				pub fn query_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"QueryCounter",
						vec![],
						[
							137u8, 58u8, 184u8, 88u8, 247u8, 22u8, 151u8, 64u8, 50u8, 77u8, 49u8,
							10u8, 234u8, 84u8, 213u8, 156u8, 26u8, 200u8, 214u8, 225u8, 125u8,
							231u8, 42u8, 93u8, 159u8, 168u8, 86u8, 201u8, 116u8, 153u8, 41u8,
							127u8,
						],
					)
				}
				#[doc = " The ongoing queries."]
				pub fn queries(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"Queries",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							251u8, 97u8, 131u8, 135u8, 93u8, 68u8, 156u8, 25u8, 181u8, 231u8,
							124u8, 93u8, 170u8, 114u8, 250u8, 177u8, 172u8, 51u8, 59u8, 44u8,
							148u8, 189u8, 199u8, 62u8, 118u8, 89u8, 75u8, 29u8, 71u8, 49u8, 248u8,
							48u8,
						],
					)
				}
				#[doc = " The ongoing queries."]
				pub fn queries_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"Queries",
						Vec::new(),
						[
							251u8, 97u8, 131u8, 135u8, 93u8, 68u8, 156u8, 25u8, 181u8, 231u8,
							124u8, 93u8, 170u8, 114u8, 250u8, 177u8, 172u8, 51u8, 59u8, 44u8,
							148u8, 189u8, 199u8, 62u8, 118u8, 89u8, 75u8, 29u8, 71u8, 49u8, 248u8,
							48u8,
						],
					)
				}
				#[doc = " The existing asset traps."]
				#[doc = ""]
				#[doc = " Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of"]
				#[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
				pub fn asset_traps(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"AssetTraps",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							4u8, 185u8, 92u8, 4u8, 7u8, 71u8, 214u8, 1u8, 141u8, 59u8, 87u8, 55u8,
							149u8, 26u8, 125u8, 8u8, 88u8, 31u8, 240u8, 138u8, 133u8, 28u8, 37u8,
							131u8, 107u8, 218u8, 86u8, 152u8, 147u8, 44u8, 19u8, 239u8,
						],
					)
				}
				#[doc = " The existing asset traps."]
				#[doc = ""]
				#[doc = " Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of"]
				#[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
				pub fn asset_traps_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"AssetTraps",
						Vec::new(),
						[
							4u8, 185u8, 92u8, 4u8, 7u8, 71u8, 214u8, 1u8, 141u8, 59u8, 87u8, 55u8,
							149u8, 26u8, 125u8, 8u8, 88u8, 31u8, 240u8, 138u8, 133u8, 28u8, 37u8,
							131u8, 107u8, 218u8, 86u8, 152u8, 147u8, 44u8, 19u8, 239u8,
						],
					)
				}
				#[doc = " Default version to encode XCM when latest version of destination is unknown. If `None`,"]
				#[doc = " then the destinations whose XCM version is unknown are considered unreachable."]
				pub fn safe_xcm_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"SafeXcmVersion",
						vec![],
						[
							1u8, 223u8, 218u8, 204u8, 222u8, 129u8, 137u8, 237u8, 197u8, 142u8,
							233u8, 66u8, 229u8, 153u8, 138u8, 222u8, 113u8, 164u8, 135u8, 213u8,
							233u8, 34u8, 24u8, 23u8, 215u8, 59u8, 40u8, 188u8, 45u8, 244u8, 205u8,
							199u8,
						],
					)
				}
				#[doc = " The Latest versions that we know various locations support."]
				pub fn supported_version(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"SupportedVersion",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							112u8, 34u8, 251u8, 179u8, 217u8, 54u8, 125u8, 242u8, 190u8, 8u8, 44u8,
							14u8, 138u8, 76u8, 241u8, 95u8, 233u8, 96u8, 141u8, 26u8, 151u8, 196u8,
							219u8, 137u8, 165u8, 27u8, 87u8, 128u8, 19u8, 35u8, 222u8, 202u8,
						],
					)
				}
				#[doc = " The Latest versions that we know various locations support."]
				pub fn supported_version_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"SupportedVersion",
						Vec::new(),
						[
							112u8, 34u8, 251u8, 179u8, 217u8, 54u8, 125u8, 242u8, 190u8, 8u8, 44u8,
							14u8, 138u8, 76u8, 241u8, 95u8, 233u8, 96u8, 141u8, 26u8, 151u8, 196u8,
							219u8, 137u8, 165u8, 27u8, 87u8, 128u8, 19u8, 35u8, 222u8, 202u8,
						],
					)
				}
				#[doc = " All locations that we have requested version notifications from."]
				pub fn version_notifiers(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"VersionNotifiers",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							233u8, 217u8, 119u8, 102u8, 41u8, 77u8, 198u8, 24u8, 161u8, 22u8,
							104u8, 149u8, 204u8, 128u8, 123u8, 166u8, 17u8, 36u8, 202u8, 92u8,
							190u8, 44u8, 73u8, 239u8, 88u8, 17u8, 92u8, 41u8, 236u8, 80u8, 154u8,
							10u8,
						],
					)
				}
				#[doc = " All locations that we have requested version notifications from."]
				pub fn version_notifiers_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"VersionNotifiers",
						Vec::new(),
						[
							233u8, 217u8, 119u8, 102u8, 41u8, 77u8, 198u8, 24u8, 161u8, 22u8,
							104u8, 149u8, 204u8, 128u8, 123u8, 166u8, 17u8, 36u8, 202u8, 92u8,
							190u8, 44u8, 73u8, 239u8, 88u8, 17u8, 92u8, 41u8, 236u8, 80u8, 154u8,
							10u8,
						],
					)
				}
				#[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
				#[doc = " of our versions we informed them of."]
				pub fn version_notify_targets(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u64,
						::core::primitive::u64,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"VersionNotifyTargets",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							108u8, 104u8, 137u8, 191u8, 2u8, 2u8, 240u8, 174u8, 32u8, 174u8, 150u8,
							136u8, 33u8, 84u8, 30u8, 74u8, 95u8, 94u8, 20u8, 112u8, 101u8, 204u8,
							15u8, 47u8, 136u8, 56u8, 40u8, 66u8, 1u8, 42u8, 16u8, 247u8,
						],
					)
				}
				#[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
				#[doc = " of our versions we informed them of."]
				pub fn version_notify_targets_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u64,
						::core::primitive::u64,
						::core::primitive::u32,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"VersionNotifyTargets",
						Vec::new(),
						[
							108u8, 104u8, 137u8, 191u8, 2u8, 2u8, 240u8, 174u8, 32u8, 174u8, 150u8,
							136u8, 33u8, 84u8, 30u8, 74u8, 95u8, 94u8, 20u8, 112u8, 101u8, 204u8,
							15u8, 47u8, 136u8, 56u8, 40u8, 66u8, 1u8, 42u8, 16u8, 247u8,
						],
					)
				}
				#[doc = " Destinations whose latest XCM version we would like to know. Duplicates not allowed, and"]
				#[doc = " the `u32` counter is the number of times that a send to the destination has been attempted,"]
				#[doc = " which is used as a prioritization."]
				pub fn version_discovery_queue(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<(
							runtime_types::xcm::VersionedMultiLocation,
							::core::primitive::u32,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"VersionDiscoveryQueue",
						vec![],
						[
							30u8, 163u8, 210u8, 133u8, 30u8, 63u8, 36u8, 9u8, 162u8, 133u8, 99u8,
							170u8, 34u8, 205u8, 27u8, 41u8, 226u8, 141u8, 165u8, 151u8, 46u8,
							140u8, 150u8, 242u8, 178u8, 88u8, 164u8, 12u8, 129u8, 118u8, 25u8,
							79u8,
						],
					)
				}
				#[doc = " The current migration's stage, if any."]
				pub fn current_migration(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_xcm::pallet::VersionMigrationStage,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"PolkadotXcm",
						"CurrentMigration",
						vec![],
						[
							137u8, 144u8, 168u8, 185u8, 158u8, 90u8, 127u8, 243u8, 227u8, 134u8,
							150u8, 73u8, 15u8, 99u8, 23u8, 47u8, 68u8, 18u8, 39u8, 16u8, 24u8,
							43u8, 161u8, 56u8, 66u8, 111u8, 16u8, 7u8, 252u8, 125u8, 100u8, 225u8,
						],
					)
				}
			}
		}
	}
	pub mod cumulus_xcm {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message is invalid XCM."]
			#[doc = "\\[ id \\]"]
			pub struct InvalidFormat(pub [::core::primitive::u8; 8usize]);
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message is unsupported version of XCM."]
			#[doc = "\\[ id \\]"]
			pub struct UnsupportedVersion(pub [::core::primitive::u8; 8usize]);
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message executed with the given outcome."]
			#[doc = "\\[ id, outcome \\]"]
			pub struct ExecutedDownward(
				pub [::core::primitive::u8; 8usize],
				pub runtime_types::xcm::v2::traits::Outcome,
			);
			impl ::subxt::events::StaticEvent for ExecutedDownward {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "ExecutedDownward";
			}
		}
	}
	pub mod dmp_queue {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ServiceOverweight {
				pub index: ::core::primitive::u64,
				pub weight_limit: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Service a single overweight message."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
				#[doc = "- `index`: The index of the overweight message to service."]
				#[doc = "- `weight_limit`: The amount of weight that message execution may take."]
				#[doc = ""]
				#[doc = "Errors:"]
				#[doc = "- `Unknown`: Message of `index` is unknown."]
				#[doc = "- `OverLimit`: Message execution may use greater than `weight_limit`."]
				#[doc = ""]
				#[doc = "Events:"]
				#[doc = "- `OverweightServiced`: On success."]
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<ServiceOverweight> {
					::subxt::tx::StaticTxPayload::new(
						"DmpQueue",
						"service_overweight",
						ServiceOverweight { index, weight_limit },
						[
							225u8, 41u8, 132u8, 91u8, 28u8, 116u8, 89u8, 197u8, 194u8, 131u8, 28u8,
							217u8, 102u8, 241u8, 122u8, 230u8, 242u8, 75u8, 83u8, 67u8, 104u8,
							55u8, 133u8, 129u8, 91u8, 25u8, 185u8, 131u8, 22u8, 253u8, 84u8, 221u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::cumulus_pallet_dmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message is invalid XCM."]
			pub struct InvalidFormat {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message is unsupported version of XCM."]
			pub struct UnsupportedVersion {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message executed with the given outcome."]
			pub struct ExecutedDownward {
				pub message_id: [::core::primitive::u8; 32usize],
				pub outcome: runtime_types::xcm::v2::traits::Outcome,
			}
			impl ::subxt::events::StaticEvent for ExecutedDownward {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "ExecutedDownward";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The weight limit for handling downward messages was reached."]
			pub struct WeightExhausted {
				pub message_id: [::core::primitive::u8; 32usize],
				pub remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
				pub required_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for WeightExhausted {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "WeightExhausted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message is overweight and was placed in the overweight queue."]
			pub struct OverweightEnqueued {
				pub message_id: [::core::primitive::u8; 32usize],
				pub overweight_index: ::core::primitive::u64,
				pub required_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightEnqueued {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message from the overweight queue was executed."]
			pub struct OverweightServiced {
				pub overweight_index: ::core::primitive::u64,
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightServiced {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightServiced";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The configuration."]
				pub fn configuration(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::cumulus_pallet_dmp_queue::ConfigData,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"Configuration",
						vec![],
						[
							133u8, 113u8, 115u8, 164u8, 128u8, 145u8, 234u8, 106u8, 150u8, 54u8,
							247u8, 135u8, 181u8, 197u8, 178u8, 30u8, 204u8, 46u8, 6u8, 137u8, 82u8,
							1u8, 75u8, 171u8, 7u8, 157u8, 3u8, 19u8, 92u8, 10u8, 234u8, 66u8,
						],
					)
				}
				#[doc = " The page index."]
				pub fn page_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::cumulus_pallet_dmp_queue::PageIndexData,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"PageIndex",
						vec![],
						[
							94u8, 132u8, 34u8, 67u8, 10u8, 22u8, 235u8, 96u8, 168u8, 26u8, 57u8,
							200u8, 130u8, 218u8, 37u8, 71u8, 28u8, 119u8, 78u8, 107u8, 209u8,
							120u8, 190u8, 2u8, 101u8, 215u8, 122u8, 187u8, 94u8, 38u8, 255u8,
							234u8,
						],
					)
				}
				#[doc = " The queue pages."]
				pub fn pages(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::core::primitive::u32,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"Pages",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							228u8, 86u8, 33u8, 107u8, 248u8, 4u8, 223u8, 175u8, 222u8, 25u8, 204u8,
							42u8, 235u8, 21u8, 215u8, 91u8, 167u8, 14u8, 133u8, 151u8, 190u8, 57u8,
							138u8, 208u8, 79u8, 244u8, 132u8, 14u8, 48u8, 247u8, 171u8, 108u8,
						],
					)
				}
				#[doc = " The queue pages."]
				pub fn pages_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::core::primitive::u32,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"Pages",
						Vec::new(),
						[
							228u8, 86u8, 33u8, 107u8, 248u8, 4u8, 223u8, 175u8, 222u8, 25u8, 204u8,
							42u8, 235u8, 21u8, 215u8, 91u8, 167u8, 14u8, 133u8, 151u8, 190u8, 57u8,
							138u8, 208u8, 79u8, 244u8, 132u8, 14u8, 48u8, 247u8, 171u8, 108u8,
						],
					)
				}
				#[doc = " The overweight messages."]
				pub fn overweight(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"Overweight",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							222u8, 85u8, 143u8, 49u8, 42u8, 248u8, 138u8, 163u8, 46u8, 199u8,
							188u8, 61u8, 137u8, 135u8, 127u8, 146u8, 210u8, 254u8, 121u8, 42u8,
							112u8, 114u8, 22u8, 228u8, 207u8, 207u8, 245u8, 175u8, 152u8, 140u8,
							225u8, 237u8,
						],
					)
				}
				#[doc = " The overweight messages."]
				pub fn overweight_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"Overweight",
						Vec::new(),
						[
							222u8, 85u8, 143u8, 49u8, 42u8, 248u8, 138u8, 163u8, 46u8, 199u8,
							188u8, 61u8, 137u8, 135u8, 127u8, 146u8, 210u8, 254u8, 121u8, 42u8,
							112u8, 114u8, 22u8, 228u8, 207u8, 207u8, 245u8, 175u8, 152u8, 140u8,
							225u8, 237u8,
						],
					)
				}
			}
		}
	}
	pub mod sudo {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Sudo {
				pub call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoUncheckedWeight {
				pub call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetKey {
				pub new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoAs {
				pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB write (event)."]
				#[doc = "- Weight of derivative `call` execution + 10,000."]
				#[doc = "# </weight>"]
				pub fn sudo(
					&self,
					call: runtime_types::parachain_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<Sudo> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo",
						Sudo { call: ::std::boxed::Box::new(call) },
						[
							74u8, 109u8, 45u8, 177u8, 181u8, 81u8, 67u8, 54u8, 55u8, 134u8, 243u8,
							226u8, 228u8, 143u8, 115u8, 223u8, 220u8, 144u8, 99u8, 37u8, 125u8,
							2u8, 236u8, 183u8, 120u8, 149u8, 44u8, 114u8, 150u8, 196u8, 131u8,
							185u8,
						],
					)
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = "This function does not check the weight of the call, and instead allows the"]
				#[doc = "Sudo user to specify the weight of the call."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- The weight of this call is defined by the caller."]
				#[doc = "# </weight>"]
				pub fn sudo_unchecked_weight(
					&self,
					call: runtime_types::parachain_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::StaticTxPayload<SudoUncheckedWeight> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo_unchecked_weight",
						SudoUncheckedWeight { call: ::std::boxed::Box::new(call), weight },
						[
							0u8, 28u8, 79u8, 218u8, 178u8, 83u8, 193u8, 37u8, 69u8, 167u8, 192u8,
							163u8, 159u8, 63u8, 230u8, 115u8, 49u8, 223u8, 222u8, 160u8, 148u8,
							219u8, 200u8, 27u8, 102u8, 181u8, 139u8, 71u8, 215u8, 35u8, 136u8,
							101u8,
						],
					)
				}
				#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
				#[doc = "key."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB change."]
				#[doc = "# </weight>"]
				pub fn set_key(
					&self,
					new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::StaticTxPayload<SetKey> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"set_key",
						SetKey { new },
						[
							23u8, 224u8, 218u8, 169u8, 8u8, 28u8, 111u8, 199u8, 26u8, 88u8, 225u8,
							105u8, 17u8, 19u8, 87u8, 156u8, 97u8, 67u8, 89u8, 173u8, 70u8, 0u8,
							5u8, 246u8, 198u8, 135u8, 182u8, 180u8, 44u8, 9u8, 212u8, 95u8,
						],
					)
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
				#[doc = "a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB write (event)."]
				#[doc = "- Weight of derivative `call` execution + 10,000."]
				#[doc = "# </weight>"]
				pub fn sudo_as(
					&self,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					call: runtime_types::parachain_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<SudoAs> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo_as",
						SudoAs { who, call: ::std::boxed::Box::new(call) },
						[
							130u8, 201u8, 232u8, 173u8, 142u8, 48u8, 192u8, 21u8, 237u8, 185u8,
							164u8, 222u8, 29u8, 226u8, 164u8, 207u8, 132u8, 163u8, 151u8, 2u8,
							81u8, 197u8, 76u8, 73u8, 126u8, 73u8, 221u8, 53u8, 20u8, 58u8, 226u8,
							218u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A sudo just took place. \\[result\\]"]
			pub struct Sudid {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
			pub struct KeyChanged {
				pub old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A sudo just took place. \\[result\\]"]
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
				#[doc = " The `AccountId` of the sudo key."]
				pub fn key(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
	pub mod ibc_ping {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SendPing {
				pub params: runtime_types::pallet_ibc_ping::SendPingParams,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn send_ping(
					&self,
					params: runtime_types::pallet_ibc_ping::SendPingParams,
				) -> ::subxt::tx::StaticTxPayload<SendPing> {
					::subxt::tx::StaticTxPayload::new(
						"IbcPing",
						"send_ping",
						SendPing { params },
						[
							220u8, 39u8, 88u8, 154u8, 82u8, 81u8, 40u8, 100u8, 229u8, 27u8, 172u8,
							50u8, 71u8, 157u8, 163u8, 188u8, 184u8, 2u8, 180u8, 93u8, 135u8, 166u8,
							112u8, 206u8, 165u8, 238u8, 89u8, 26u8, 163u8, 33u8, 212u8, 144u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_ibc_ping::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A send packet has been registered"]
			pub struct PacketSent;
			impl ::subxt::events::StaticEvent for PacketSent {
				const PALLET: &'static str = "IbcPing";
				const EVENT: &'static str = "PacketSent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A channel has been opened"]
			pub struct ChannelOpened {
				pub channel_id: ::std::vec::Vec<::core::primitive::u8>,
				pub port_id: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ChannelOpened {
				const PALLET: &'static str = "IbcPing";
				const EVENT: &'static str = "ChannelOpened";
			}
		}
	}
	pub mod assets {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Create {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub min_balance: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceCreate {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub is_sufficient: ::core::primitive::bool,
				#[codec(compact)]
				pub min_balance: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Destroy {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub witness: runtime_types::pallet_assets::types::DestroyWitness,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Mint {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Burn {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Transfer {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferKeepAlive {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceTransfer {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Freeze {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Thaw {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct FreezeAsset {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ThawAsset {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferOwnership {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetTeam {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetMetadata {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub name: ::std::vec::Vec<::core::primitive::u8>,
				pub symbol: ::std::vec::Vec<::core::primitive::u8>,
				pub decimals: ::core::primitive::u8,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ClearMetadata {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceSetMetadata {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub name: ::std::vec::Vec<::core::primitive::u8>,
				pub symbol: ::std::vec::Vec<::core::primitive::u8>,
				pub decimals: ::core::primitive::u8,
				pub is_frozen: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceClearMetadata {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceAssetStatus {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub min_balance: ::core::primitive::u128,
				pub is_sufficient: ::core::primitive::bool,
				pub is_frozen: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ApproveTransfer {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CancelApproval {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceCancelApproval {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferApproved {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				pub destination: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Touch {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Refund {
				#[codec(compact)]
				pub id: ::core::primitive::u128,
				pub allow_burn: ::core::primitive::bool,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Issue a new class of fungible assets from a public origin."]
				#[doc = ""]
				#[doc = "This new asset class has no assets initially and its owner is the origin."]
				#[doc = ""]
				#[doc = "The origin must conform to the configured `CreateOrigin` and have sufficient funds free."]
				#[doc = ""]
				#[doc = "Funds of sender are reserved by `AssetDeposit`."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
				#[doc = "an existing asset."]
				#[doc = "- `admin`: The admin of this class of assets. The admin is the initial address of each"]
				#[doc = "member of the asset class's admin team."]
				#[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
				#[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
				#[doc = ""]
				#[doc = "Emits `Created` event when successful."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn create(
					&self,
					id: ::core::primitive::u128,
					admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					min_balance: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Create> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"create",
						Create { id, admin, min_balance },
						[
							142u8, 26u8, 211u8, 242u8, 115u8, 25u8, 192u8, 8u8, 229u8, 194u8, 33u8,
							252u8, 218u8, 217u8, 216u8, 233u8, 134u8, 31u8, 96u8, 151u8, 163u8,
							126u8, 25u8, 19u8, 43u8, 78u8, 249u8, 85u8, 220u8, 88u8, 8u8, 11u8,
						],
					)
				}
				#[doc = "Issue a new class of fungible assets from a privileged origin."]
				#[doc = ""]
				#[doc = "This new asset class has no assets initially."]
				#[doc = ""]
				#[doc = "The origin must conform to `ForceOrigin`."]
				#[doc = ""]
				#[doc = "Unlike `create`, no funds are reserved."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
				#[doc = "an existing asset."]
				#[doc = "- `owner`: The owner of this class of assets. The owner has full superuser permissions"]
				#[doc = "over this asset, but may later change and configure the permissions using"]
				#[doc = "`transfer_ownership` and `set_team`."]
				#[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
				#[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
				#[doc = ""]
				#[doc = "Emits `ForceCreated` event when successful."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn force_create(
					&self,
					id: ::core::primitive::u128,
					owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					is_sufficient: ::core::primitive::bool,
					min_balance: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceCreate> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"force_create",
						ForceCreate { id, owner, is_sufficient, min_balance },
						[
							72u8, 99u8, 108u8, 213u8, 36u8, 43u8, 110u8, 164u8, 234u8, 58u8, 96u8,
							155u8, 55u8, 117u8, 254u8, 33u8, 191u8, 246u8, 51u8, 50u8, 21u8, 169u8,
							27u8, 153u8, 39u8, 177u8, 255u8, 239u8, 58u8, 59u8, 158u8, 220u8,
						],
					)
				}
				#[doc = "Destroy a class of fungible assets."]
				#[doc = ""]
				#[doc = "The origin must conform to `ForceOrigin` or must be Signed and the sender must be the"]
				#[doc = "owner of the asset `id`."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
				#[doc = "asset."]
				#[doc = ""]
				#[doc = "Emits `Destroyed` event when successful."]
				#[doc = ""]
				#[doc = "NOTE: It can be helpful to first freeze an asset before destroying it so that you"]
				#[doc = "can provide accurate witness information and prevent users from manipulating state"]
				#[doc = "in a way that can make it harder to destroy."]
				#[doc = ""]
				#[doc = "Weight: `O(c + p + a)` where:"]
				#[doc = "- `c = (witness.accounts - witness.sufficients)`"]
				#[doc = "- `s = witness.sufficients`"]
				#[doc = "- `a = witness.approvals`"]
				pub fn destroy(
					&self,
					id: ::core::primitive::u128,
					witness: runtime_types::pallet_assets::types::DestroyWitness,
				) -> ::subxt::tx::StaticTxPayload<Destroy> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"destroy",
						Destroy { id, witness },
						[
							183u8, 52u8, 56u8, 40u8, 161u8, 252u8, 91u8, 170u8, 109u8, 241u8,
							158u8, 69u8, 216u8, 170u8, 106u8, 182u8, 82u8, 0u8, 178u8, 26u8, 88u8,
							86u8, 70u8, 127u8, 104u8, 122u8, 253u8, 107u8, 100u8, 81u8, 67u8,
							246u8,
						],
					)
				}
				#[doc = "Mint assets of a particular class."]
				#[doc = ""]
				#[doc = "The origin must be Signed and the sender must be the Issuer of the asset `id`."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to have some amount minted."]
				#[doc = "- `beneficiary`: The account to be credited with the minted assets."]
				#[doc = "- `amount`: The amount of the asset to be minted."]
				#[doc = ""]
				#[doc = "Emits `Issued` event when successful."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				#[doc = "Modes: Pre-existing balance of `beneficiary`; Account pre-existence of `beneficiary`."]
				pub fn mint(
					&self,
					id: ::core::primitive::u128,
					beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Mint> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"mint",
						Mint { id, beneficiary, amount },
						[
							229u8, 200u8, 12u8, 126u8, 52u8, 12u8, 171u8, 100u8, 152u8, 113u8,
							114u8, 83u8, 178u8, 15u8, 80u8, 12u8, 230u8, 141u8, 18u8, 155u8, 124u8,
							255u8, 53u8, 92u8, 65u8, 37u8, 173u8, 224u8, 122u8, 102u8, 109u8, 15u8,
						],
					)
				}
				#[doc = "Reduce the balance of `who` by as much as possible up to `amount` assets of `id`."]
				#[doc = ""]
				#[doc = "Origin must be Signed and the sender should be the Manager of the asset `id`."]
				#[doc = ""]
				#[doc = "Bails with `NoAccount` if the `who` is already dead."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to have some amount burned."]
				#[doc = "- `who`: The account to be debited from."]
				#[doc = "- `amount`: The maximum amount by which `who`'s balance should be reduced."]
				#[doc = ""]
				#[doc = "Emits `Burned` with the actual amount burned. If this takes the balance to below the"]
				#[doc = "minimum for the asset, then the amount burned is increased to take it to zero."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				#[doc = "Modes: Post-existence of `who`; Pre & post Zombie-status of `who`."]
				pub fn burn(
					&self,
					id: ::core::primitive::u128,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Burn> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"burn",
						Burn { id, who, amount },
						[
							215u8, 92u8, 78u8, 189u8, 185u8, 221u8, 48u8, 193u8, 12u8, 185u8,
							114u8, 164u8, 162u8, 157u8, 90u8, 253u8, 129u8, 104u8, 252u8, 227u8,
							30u8, 29u8, 215u8, 186u8, 235u8, 85u8, 167u8, 41u8, 131u8, 220u8,
							184u8, 242u8,
						],
					)
				}
				#[doc = "Move some assets from the sender account to another."]
				#[doc = ""]
				#[doc = "Origin must be Signed."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to have some amount transferred."]
				#[doc = "- `target`: The account to be credited."]
				#[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
				#[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
				#[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
				#[doc = "the minimum balance. Must be greater than zero."]
				#[doc = ""]
				#[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
				#[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
				#[doc = "to zero."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				#[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
				#[doc = "`target`."]
				pub fn transfer(
					&self,
					id: ::core::primitive::u128,
					target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"transfer",
						Transfer { id, target, amount },
						[
							162u8, 30u8, 88u8, 36u8, 26u8, 87u8, 130u8, 201u8, 234u8, 92u8, 65u8,
							216u8, 139u8, 145u8, 43u8, 231u8, 221u8, 57u8, 251u8, 73u8, 112u8,
							172u8, 39u8, 196u8, 19u8, 198u8, 140u8, 29u8, 61u8, 228u8, 106u8,
							238u8,
						],
					)
				}
				#[doc = "Move some assets from the sender account to another, keeping the sender account alive."]
				#[doc = ""]
				#[doc = "Origin must be Signed."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to have some amount transferred."]
				#[doc = "- `target`: The account to be credited."]
				#[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
				#[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
				#[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
				#[doc = "the minimum balance. Must be greater than zero."]
				#[doc = ""]
				#[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
				#[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
				#[doc = "to zero."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				#[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
				#[doc = "`target`."]
				pub fn transfer_keep_alive(
					&self,
					id: ::core::primitive::u128,
					target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"transfer_keep_alive",
						TransferKeepAlive { id, target, amount },
						[
							248u8, 6u8, 41u8, 68u8, 23u8, 229u8, 171u8, 235u8, 213u8, 184u8, 61u8,
							141u8, 130u8, 132u8, 207u8, 177u8, 188u8, 89u8, 188u8, 156u8, 114u8,
							200u8, 223u8, 97u8, 83u8, 19u8, 135u8, 27u8, 123u8, 10u8, 235u8, 76u8,
						],
					)
				}
				#[doc = "Move some assets from one account to another."]
				#[doc = ""]
				#[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to have some amount transferred."]
				#[doc = "- `source`: The account to be debited."]
				#[doc = "- `dest`: The account to be credited."]
				#[doc = "- `amount`: The amount by which the `source`'s balance of assets should be reduced and"]
				#[doc = "`dest`'s balance increased. The amount actually transferred may be slightly greater in"]
				#[doc = "the case that the transfer would otherwise take the `source` balance above zero but"]
				#[doc = "below the minimum balance. Must be greater than zero."]
				#[doc = ""]
				#[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
				#[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
				#[doc = "to zero."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				#[doc = "Modes: Pre-existence of `dest`; Post-existence of `source`; Account pre-existence of"]
				#[doc = "`dest`."]
				pub fn force_transfer(
					&self,
					id: ::core::primitive::u128,
					source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"force_transfer",
						ForceTransfer { id, source, dest, amount },
						[
							64u8, 71u8, 236u8, 12u8, 10u8, 20u8, 5u8, 227u8, 104u8, 175u8, 97u8,
							55u8, 52u8, 195u8, 52u8, 211u8, 29u8, 113u8, 109u8, 192u8, 46u8, 72u8,
							140u8, 70u8, 163u8, 52u8, 196u8, 145u8, 113u8, 156u8, 186u8, 55u8,
						],
					)
				}
				#[doc = "Disallow further unprivileged transfers from an account."]
				#[doc = ""]
				#[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to be frozen."]
				#[doc = "- `who`: The account to be frozen."]
				#[doc = ""]
				#[doc = "Emits `Frozen`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn freeze(
					&self,
					id: ::core::primitive::u128,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::StaticTxPayload<Freeze> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"freeze",
						Freeze { id, who },
						[
							238u8, 4u8, 95u8, 112u8, 162u8, 153u8, 53u8, 91u8, 146u8, 254u8, 209u8,
							12u8, 222u8, 51u8, 214u8, 238u8, 178u8, 188u8, 206u8, 204u8, 118u8,
							83u8, 76u8, 15u8, 62u8, 87u8, 185u8, 226u8, 107u8, 14u8, 53u8, 176u8,
						],
					)
				}
				#[doc = "Allow unprivileged transfers from an account again."]
				#[doc = ""]
				#[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to be frozen."]
				#[doc = "- `who`: The account to be unfrozen."]
				#[doc = ""]
				#[doc = "Emits `Thawed`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn thaw(
					&self,
					id: ::core::primitive::u128,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::StaticTxPayload<Thaw> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"thaw",
						Thaw { id, who },
						[
							135u8, 135u8, 90u8, 45u8, 229u8, 30u8, 115u8, 177u8, 204u8, 143u8,
							144u8, 116u8, 243u8, 186u8, 173u8, 200u8, 52u8, 40u8, 120u8, 33u8,
							249u8, 224u8, 174u8, 247u8, 223u8, 210u8, 73u8, 215u8, 186u8, 209u8,
							157u8, 91u8,
						],
					)
				}
				#[doc = "Disallow further unprivileged transfers for the asset class."]
				#[doc = ""]
				#[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to be frozen."]
				#[doc = ""]
				#[doc = "Emits `Frozen`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn freeze_asset(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<FreezeAsset> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"freeze_asset",
						FreezeAsset { id },
						[
							143u8, 236u8, 41u8, 9u8, 222u8, 193u8, 21u8, 39u8, 2u8, 191u8, 102u8,
							224u8, 203u8, 101u8, 40u8, 138u8, 128u8, 226u8, 197u8, 143u8, 216u8,
							63u8, 151u8, 49u8, 155u8, 221u8, 207u8, 161u8, 92u8, 236u8, 150u8,
							221u8,
						],
					)
				}
				#[doc = "Allow unprivileged transfers for the asset again."]
				#[doc = ""]
				#[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to be thawed."]
				#[doc = ""]
				#[doc = "Emits `Thawed`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn thaw_asset(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ThawAsset> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"thaw_asset",
						ThawAsset { id },
						[
							76u8, 108u8, 33u8, 107u8, 82u8, 243u8, 182u8, 166u8, 220u8, 76u8, 4u8,
							20u8, 233u8, 183u8, 19u8, 198u8, 79u8, 79u8, 230u8, 34u8, 231u8, 112u8,
							239u8, 199u8, 242u8, 140u8, 115u8, 230u8, 11u8, 3u8, 176u8, 63u8,
						],
					)
				}
				#[doc = "Change the Owner of an asset."]
				#[doc = ""]
				#[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset."]
				#[doc = "- `owner`: The new Owner of this asset."]
				#[doc = ""]
				#[doc = "Emits `OwnerChanged`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn transfer_ownership(
					&self,
					id: ::core::primitive::u128,
					owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::StaticTxPayload<TransferOwnership> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"transfer_ownership",
						TransferOwnership { id, owner },
						[
							178u8, 53u8, 191u8, 144u8, 167u8, 148u8, 182u8, 247u8, 205u8, 78u8,
							253u8, 71u8, 0u8, 161u8, 42u8, 144u8, 109u8, 147u8, 195u8, 180u8,
							118u8, 208u8, 64u8, 138u8, 229u8, 139u8, 220u8, 66u8, 132u8, 46u8,
							129u8, 208u8,
						],
					)
				}
				#[doc = "Change the Issuer, Admin and Freezer of an asset."]
				#[doc = ""]
				#[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to be frozen."]
				#[doc = "- `issuer`: The new Issuer of this asset."]
				#[doc = "- `admin`: The new Admin of this asset."]
				#[doc = "- `freezer`: The new Freezer of this asset."]
				#[doc = ""]
				#[doc = "Emits `TeamChanged`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn set_team(
					&self,
					id: ::core::primitive::u128,
					issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::StaticTxPayload<SetTeam> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"set_team",
						SetTeam { id, issuer, admin, freezer },
						[
							158u8, 0u8, 167u8, 98u8, 22u8, 185u8, 117u8, 9u8, 59u8, 67u8, 122u8,
							71u8, 82u8, 172u8, 184u8, 121u8, 115u8, 135u8, 73u8, 152u8, 229u8,
							151u8, 10u8, 136u8, 82u8, 196u8, 142u8, 244u8, 17u8, 113u8, 154u8,
							72u8,
						],
					)
				}
				#[doc = "Set the metadata for an asset."]
				#[doc = ""]
				#[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
				#[doc = ""]
				#[doc = "Funds of sender are reserved according to the formula:"]
				#[doc = "`MetadataDepositBase + MetadataDepositPerByte * (name.len + symbol.len)` taking into"]
				#[doc = "account any already reserved funds."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to update."]
				#[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
				#[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
				#[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
				#[doc = ""]
				#[doc = "Emits `MetadataSet`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn set_metadata(
					&self,
					id: ::core::primitive::u128,
					name: ::std::vec::Vec<::core::primitive::u8>,
					symbol: ::std::vec::Vec<::core::primitive::u8>,
					decimals: ::core::primitive::u8,
				) -> ::subxt::tx::StaticTxPayload<SetMetadata> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"set_metadata",
						SetMetadata { id, name, symbol, decimals },
						[
							2u8, 98u8, 101u8, 38u8, 45u8, 19u8, 137u8, 54u8, 16u8, 23u8, 1u8,
							182u8, 149u8, 208u8, 216u8, 170u8, 147u8, 128u8, 198u8, 169u8, 244u8,
							121u8, 139u8, 66u8, 225u8, 146u8, 162u8, 14u8, 44u8, 250u8, 16u8,
							137u8,
						],
					)
				}
				#[doc = "Clear the metadata for an asset."]
				#[doc = ""]
				#[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
				#[doc = ""]
				#[doc = "Any deposit is freed for the asset owner."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to clear."]
				#[doc = ""]
				#[doc = "Emits `MetadataCleared`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn clear_metadata(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ClearMetadata> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"clear_metadata",
						ClearMetadata { id },
						[
							203u8, 43u8, 102u8, 26u8, 145u8, 21u8, 241u8, 238u8, 108u8, 74u8, 71u8,
							14u8, 69u8, 132u8, 237u8, 223u8, 78u8, 160u8, 86u8, 1u8, 200u8, 168u8,
							238u8, 49u8, 105u8, 176u8, 76u8, 63u8, 126u8, 71u8, 189u8, 192u8,
						],
					)
				}
				#[doc = "Force the metadata for an asset to some value."]
				#[doc = ""]
				#[doc = "Origin must be ForceOrigin."]
				#[doc = ""]
				#[doc = "Any deposit is left alone."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to update."]
				#[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
				#[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
				#[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
				#[doc = ""]
				#[doc = "Emits `MetadataSet`."]
				#[doc = ""]
				#[doc = "Weight: `O(N + S)` where N and S are the length of the name and symbol respectively."]
				pub fn force_set_metadata(
					&self,
					id: ::core::primitive::u128,
					name: ::std::vec::Vec<::core::primitive::u8>,
					symbol: ::std::vec::Vec<::core::primitive::u8>,
					decimals: ::core::primitive::u8,
					is_frozen: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<ForceSetMetadata> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"force_set_metadata",
						ForceSetMetadata { id, name, symbol, decimals, is_frozen },
						[
							98u8, 119u8, 248u8, 186u8, 149u8, 217u8, 63u8, 146u8, 247u8, 15u8,
							132u8, 152u8, 239u8, 65u8, 226u8, 204u8, 198u8, 55u8, 188u8, 239u8,
							136u8, 78u8, 20u8, 83u8, 209u8, 168u8, 215u8, 156u8, 57u8, 147u8, 28u8,
							109u8,
						],
					)
				}
				#[doc = "Clear the metadata for an asset."]
				#[doc = ""]
				#[doc = "Origin must be ForceOrigin."]
				#[doc = ""]
				#[doc = "Any deposit is returned."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset to clear."]
				#[doc = ""]
				#[doc = "Emits `MetadataCleared`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn force_clear_metadata(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceClearMetadata> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"force_clear_metadata",
						ForceClearMetadata { id },
						[
							48u8, 251u8, 205u8, 159u8, 163u8, 213u8, 97u8, 254u8, 20u8, 98u8,
							246u8, 182u8, 77u8, 226u8, 24u8, 34u8, 3u8, 213u8, 131u8, 232u8, 75u8,
							124u8, 96u8, 187u8, 116u8, 143u8, 13u8, 17u8, 220u8, 102u8, 244u8,
							144u8,
						],
					)
				}
				#[doc = "Alter the attributes of a given asset."]
				#[doc = ""]
				#[doc = "Origin must be `ForceOrigin`."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset."]
				#[doc = "- `owner`: The new Owner of this asset."]
				#[doc = "- `issuer`: The new Issuer of this asset."]
				#[doc = "- `admin`: The new Admin of this asset."]
				#[doc = "- `freezer`: The new Freezer of this asset."]
				#[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
				#[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
				#[doc = "- `is_sufficient`: Whether a non-zero balance of this asset is deposit of sufficient"]
				#[doc = "value to account for the state bloat associated with its balance storage. If set to"]
				#[doc = "`true`, then non-zero balances may be stored without a `consumer` reference (and thus"]
				#[doc = "an ED in the Balances pallet or whatever else is used to control user-account state"]
				#[doc = "growth)."]
				#[doc = "- `is_frozen`: Whether this asset class is frozen except for permissioned/admin"]
				#[doc = "instructions."]
				#[doc = ""]
				#[doc = "Emits `AssetStatusChanged` with the identity of the asset."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn force_asset_status(
					&self,
					id: ::core::primitive::u128,
					owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					min_balance: ::core::primitive::u128,
					is_sufficient: ::core::primitive::bool,
					is_frozen: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<ForceAssetStatus> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"force_asset_status",
						ForceAssetStatus {
							id,
							owner,
							issuer,
							admin,
							freezer,
							min_balance,
							is_sufficient,
							is_frozen,
						},
						[
							244u8, 218u8, 14u8, 227u8, 39u8, 76u8, 62u8, 47u8, 254u8, 74u8, 153u8,
							243u8, 199u8, 24u8, 133u8, 121u8, 103u8, 35u8, 188u8, 164u8, 21u8,
							244u8, 25u8, 74u8, 6u8, 98u8, 0u8, 94u8, 3u8, 163u8, 40u8, 205u8,
						],
					)
				}
				#[doc = "Approve an amount of asset for transfer by a delegated third-party account."]
				#[doc = ""]
				#[doc = "Origin must be Signed."]
				#[doc = ""]
				#[doc = "Ensures that `ApprovalDeposit` worth of `Currency` is reserved from signing account"]
				#[doc = "for the purpose of holding the approval. If some non-zero amount of assets is already"]
				#[doc = "approved from signing account to `delegate`, then it is topped up or unreserved to"]
				#[doc = "meet the right value."]
				#[doc = ""]
				#[doc = "NOTE: The signing account does not need to own `amount` of assets at the point of"]
				#[doc = "making this call."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset."]
				#[doc = "- `delegate`: The account to delegate permission to transfer asset."]
				#[doc = "- `amount`: The amount of asset that may be transferred by `delegate`. If there is"]
				#[doc = "already an approval in place, then this acts additively."]
				#[doc = ""]
				#[doc = "Emits `ApprovedTransfer` on success."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn approve_transfer(
					&self,
					id: ::core::primitive::u128,
					delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ApproveTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"approve_transfer",
						ApproveTransfer { id, delegate, amount },
						[
							206u8, 129u8, 0u8, 213u8, 213u8, 23u8, 149u8, 131u8, 57u8, 118u8,
							116u8, 32u8, 149u8, 195u8, 59u8, 255u8, 129u8, 38u8, 69u8, 174u8, 78u8,
							28u8, 99u8, 234u8, 237u8, 213u8, 254u8, 59u8, 155u8, 4u8, 139u8, 139u8,
						],
					)
				}
				#[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
				#[doc = ""]
				#[doc = "Origin must be Signed and there must be an approval in place between signer and"]
				#[doc = "`delegate`."]
				#[doc = ""]
				#[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset."]
				#[doc = "- `delegate`: The account delegated permission to transfer asset."]
				#[doc = ""]
				#[doc = "Emits `ApprovalCancelled` on success."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn cancel_approval(
					&self,
					id: ::core::primitive::u128,
					delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::StaticTxPayload<CancelApproval> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"cancel_approval",
						CancelApproval { id, delegate },
						[
							0u8, 158u8, 247u8, 196u8, 56u8, 159u8, 21u8, 4u8, 113u8, 252u8, 212u8,
							154u8, 94u8, 72u8, 5u8, 233u8, 195u8, 252u8, 30u8, 25u8, 148u8, 218u8,
							236u8, 82u8, 166u8, 58u8, 74u8, 17u8, 232u8, 179u8, 126u8, 234u8,
						],
					)
				}
				#[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
				#[doc = ""]
				#[doc = "Origin must be either ForceOrigin or Signed origin with the signer being the Admin"]
				#[doc = "account of the asset `id`."]
				#[doc = ""]
				#[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset."]
				#[doc = "- `delegate`: The account delegated permission to transfer asset."]
				#[doc = ""]
				#[doc = "Emits `ApprovalCancelled` on success."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn force_cancel_approval(
					&self,
					id: ::core::primitive::u128,
					owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::StaticTxPayload<ForceCancelApproval> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"force_cancel_approval",
						ForceCancelApproval { id, owner, delegate },
						[
							18u8, 58u8, 156u8, 26u8, 40u8, 68u8, 208u8, 28u8, 239u8, 67u8, 207u8,
							184u8, 6u8, 178u8, 187u8, 86u8, 203u8, 177u8, 139u8, 37u8, 136u8, 49u8,
							129u8, 165u8, 51u8, 7u8, 157u8, 120u8, 129u8, 14u8, 177u8, 128u8,
						],
					)
				}
				#[doc = "Transfer some asset balance from a previously delegated account to some third-party"]
				#[doc = "account."]
				#[doc = ""]
				#[doc = "Origin must be Signed and there must be an approval in place by the `owner` to the"]
				#[doc = "signer."]
				#[doc = ""]
				#[doc = "If the entire amount approved for transfer is transferred, then any deposit previously"]
				#[doc = "reserved by `approve_transfer` is unreserved."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset."]
				#[doc = "- `owner`: The account which previously approved for a transfer of at least `amount` and"]
				#[doc = "from which the asset balance will be withdrawn."]
				#[doc = "- `destination`: The account to which the asset balance of `amount` will be transferred."]
				#[doc = "- `amount`: The amount of assets to transfer."]
				#[doc = ""]
				#[doc = "Emits `TransferredApproved` on success."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn transfer_approved(
					&self,
					id: ::core::primitive::u128,
					owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					destination: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<TransferApproved> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"transfer_approved",
						TransferApproved { id, owner, destination, amount },
						[
							217u8, 22u8, 40u8, 177u8, 232u8, 27u8, 123u8, 82u8, 142u8, 124u8,
							183u8, 246u8, 188u8, 228u8, 206u8, 217u8, 0u8, 184u8, 44u8, 205u8,
							50u8, 100u8, 194u8, 47u8, 53u8, 248u8, 205u8, 21u8, 133u8, 220u8,
							181u8, 200u8,
						],
					)
				}
				#[doc = "Create an asset account for non-provider assets."]
				#[doc = ""]
				#[doc = "A deposit will be taken from the signer account."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Signed; the signer account must have sufficient funds for a deposit"]
				#[doc = "  to be taken."]
				#[doc = "- `id`: The identifier of the asset for the account to be created."]
				#[doc = ""]
				#[doc = "Emits `Touched` event when successful."]
				pub fn touch(
					&self,
					id: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Touch> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"touch",
						Touch { id },
						[
							204u8, 178u8, 116u8, 113u8, 68u8, 63u8, 58u8, 179u8, 73u8, 226u8,
							224u8, 63u8, 182u8, 77u8, 43u8, 186u8, 247u8, 26u8, 247u8, 78u8, 228u8,
							149u8, 162u8, 119u8, 188u8, 215u8, 122u8, 49u8, 19u8, 41u8, 251u8,
							218u8,
						],
					)
				}
				#[doc = "Return the deposit (if any) of an asset account."]
				#[doc = ""]
				#[doc = "The origin must be Signed."]
				#[doc = ""]
				#[doc = "- `id`: The identifier of the asset for the account to be created."]
				#[doc = "- `allow_burn`: If `true` then assets may be destroyed in order to complete the refund."]
				#[doc = ""]
				#[doc = "Emits `Refunded` event when successful."]
				pub fn refund(
					&self,
					id: ::core::primitive::u128,
					allow_burn: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<Refund> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"refund",
						Refund { id, allow_burn },
						[
							162u8, 52u8, 208u8, 78u8, 108u8, 88u8, 200u8, 116u8, 179u8, 110u8,
							219u8, 41u8, 53u8, 70u8, 108u8, 190u8, 146u8, 194u8, 46u8, 117u8,
							135u8, 78u8, 57u8, 163u8, 108u8, 202u8, 168u8, 204u8, 1u8, 231u8, 42u8,
							94u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_assets::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some asset class was created."]
			pub struct Created {
				pub asset_id: ::core::primitive::u128,
				pub creator: ::subxt::utils::AccountId32,
				pub owner: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Created {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Created";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some assets were issued."]
			pub struct Issued {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
				pub total_supply: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Issued {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Issued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some assets were transferred."]
			pub struct Transferred {
				pub asset_id: ::core::primitive::u128,
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transferred {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Transferred";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some assets were destroyed."]
			pub struct Burned {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
				pub balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Burned {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Burned";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The management team changed."]
			pub struct TeamChanged {
				pub asset_id: ::core::primitive::u128,
				pub issuer: ::subxt::utils::AccountId32,
				pub admin: ::subxt::utils::AccountId32,
				pub freezer: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for TeamChanged {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "TeamChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The owner changed."]
			pub struct OwnerChanged {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for OwnerChanged {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "OwnerChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some account `who` was frozen."]
			pub struct Frozen {
				pub asset_id: ::core::primitive::u128,
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Frozen {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Frozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some account `who` was thawed."]
			pub struct Thawed {
				pub asset_id: ::core::primitive::u128,
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Thawed {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Thawed";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "Some asset `asset_id` was frozen."]
			pub struct AssetFrozen {
				pub asset_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for AssetFrozen {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "AssetFrozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "Some asset `asset_id` was thawed."]
			pub struct AssetThawed {
				pub asset_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for AssetThawed {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "AssetThawed";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "An asset class was destroyed."]
			pub struct Destroyed {
				pub asset_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Destroyed {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "Destroyed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some asset class was force-created."]
			pub struct ForceCreated {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for ForceCreated {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "ForceCreated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "New metadata has been set for an asset."]
			pub struct MetadataSet {
				pub asset_id: ::core::primitive::u128,
				pub name: ::std::vec::Vec<::core::primitive::u8>,
				pub symbol: ::std::vec::Vec<::core::primitive::u8>,
				pub decimals: ::core::primitive::u8,
				pub is_frozen: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for MetadataSet {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "MetadataSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "Metadata has been cleared for an asset."]
			pub struct MetadataCleared {
				pub asset_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for MetadataCleared {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "MetadataCleared";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "(Additional) funds have been approved for transfer to a destination account."]
			pub struct ApprovedTransfer {
				pub asset_id: ::core::primitive::u128,
				pub source: ::subxt::utils::AccountId32,
				pub delegate: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for ApprovedTransfer {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "ApprovedTransfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An approval for account `delegate` was cancelled by `owner`."]
			pub struct ApprovalCancelled {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
				pub delegate: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for ApprovalCancelled {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "ApprovalCancelled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An `amount` was transferred in its entirety from `owner` to `destination` by"]
			#[doc = "the approved `delegate`."]
			pub struct TransferredApproved {
				pub asset_id: ::core::primitive::u128,
				pub owner: ::subxt::utils::AccountId32,
				pub delegate: ::subxt::utils::AccountId32,
				pub destination: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TransferredApproved {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "TransferredApproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "An asset has had its attributes changed by the `Force` origin."]
			pub struct AssetStatusChanged {
				pub asset_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for AssetStatusChanged {
				const PALLET: &'static str = "Assets";
				const EVENT: &'static str = "AssetStatusChanged";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Details of an asset."]
				pub fn asset(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_assets::types::AssetDetails<
							::core::primitive::u128,
							::subxt::utils::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Assets",
						"Asset",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							254u8, 219u8, 24u8, 7u8, 169u8, 16u8, 29u8, 63u8, 4u8, 199u8, 85u8,
							89u8, 208u8, 119u8, 245u8, 187u8, 48u8, 57u8, 112u8, 0u8, 53u8, 71u8,
							112u8, 127u8, 148u8, 237u8, 94u8, 56u8, 252u8, 84u8, 184u8, 133u8,
						],
					)
				}
				#[doc = " Details of an asset."]
				pub fn asset_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_assets::types::AssetDetails<
							::core::primitive::u128,
							::subxt::utils::AccountId32,
							::core::primitive::u128,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Assets",
						"Asset",
						Vec::new(),
						[
							254u8, 219u8, 24u8, 7u8, 169u8, 16u8, 29u8, 63u8, 4u8, 199u8, 85u8,
							89u8, 208u8, 119u8, 245u8, 187u8, 48u8, 57u8, 112u8, 0u8, 53u8, 71u8,
							112u8, 127u8, 148u8, 237u8, 94u8, 56u8, 252u8, 84u8, 184u8, 133u8,
						],
					)
				}
				#[doc = " The holdings of a specific account for a specific asset."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_assets::types::AssetAccount<
							::core::primitive::u128,
							::core::primitive::u128,
							(),
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Assets",
						"Account",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							67u8, 0u8, 63u8, 201u8, 208u8, 191u8, 42u8, 3u8, 219u8, 238u8, 187u8,
							35u8, 152u8, 230u8, 213u8, 81u8, 169u8, 97u8, 206u8, 44u8, 135u8,
							229u8, 164u8, 45u8, 37u8, 142u8, 2u8, 225u8, 104u8, 28u8, 6u8, 185u8,
						],
					)
				}
				#[doc = " The holdings of a specific account for a specific asset."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_assets::types::AssetAccount<
							::core::primitive::u128,
							::core::primitive::u128,
							(),
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Assets",
						"Account",
						Vec::new(),
						[
							67u8, 0u8, 63u8, 201u8, 208u8, 191u8, 42u8, 3u8, 219u8, 238u8, 187u8,
							35u8, 152u8, 230u8, 213u8, 81u8, 169u8, 97u8, 206u8, 44u8, 135u8,
							229u8, 164u8, 45u8, 37u8, 142u8, 2u8, 225u8, 104u8, 28u8, 6u8, 185u8,
						],
					)
				}
				#[doc = " Approved balance transfers. First balance is the amount approved for transfer. Second"]
				#[doc = " is the amount of `T::Currency` reserved for storing this."]
				#[doc = " First key is the asset ID, second key is the owner and third key is the delegate."]
				pub fn approvals(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
					_2: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_assets::types::Approval<
							::core::primitive::u128,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Assets",
						"Approvals",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_2.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							137u8, 65u8, 132u8, 181u8, 243u8, 60u8, 35u8, 185u8, 141u8, 134u8,
							92u8, 197u8, 141u8, 11u8, 213u8, 109u8, 168u8, 204u8, 163u8, 202u8,
							192u8, 68u8, 111u8, 151u8, 117u8, 204u8, 176u8, 159u8, 195u8, 103u8,
							89u8, 197u8,
						],
					)
				}
				#[doc = " Approved balance transfers. First balance is the amount approved for transfer. Second"]
				#[doc = " is the amount of `T::Currency` reserved for storing this."]
				#[doc = " First key is the asset ID, second key is the owner and third key is the delegate."]
				pub fn approvals_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_assets::types::Approval<
							::core::primitive::u128,
							::core::primitive::u128,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Assets",
						"Approvals",
						Vec::new(),
						[
							137u8, 65u8, 132u8, 181u8, 243u8, 60u8, 35u8, 185u8, 141u8, 134u8,
							92u8, 197u8, 141u8, 11u8, 213u8, 109u8, 168u8, 204u8, 163u8, 202u8,
							192u8, 68u8, 111u8, 151u8, 117u8, 204u8, 176u8, 159u8, 195u8, 103u8,
							89u8, 197u8,
						],
					)
				}
				#[doc = " Metadata of an asset."]
				pub fn metadata(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_assets::types::AssetMetadata<
							::core::primitive::u128,
							runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Assets",
						"Metadata",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							145u8, 150u8, 61u8, 31u8, 56u8, 121u8, 18u8, 219u8, 177u8, 160u8, 5u8,
							255u8, 20u8, 182u8, 156u8, 122u8, 36u8, 156u8, 68u8, 80u8, 43u8, 196u8,
							228u8, 119u8, 15u8, 20u8, 176u8, 186u8, 255u8, 85u8, 179u8, 251u8,
						],
					)
				}
				#[doc = " Metadata of an asset."]
				pub fn metadata_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_assets::types::AssetMetadata<
							::core::primitive::u128,
							runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Assets",
						"Metadata",
						Vec::new(),
						[
							145u8, 150u8, 61u8, 31u8, 56u8, 121u8, 18u8, 219u8, 177u8, 160u8, 5u8,
							255u8, 20u8, 182u8, 156u8, 122u8, 36u8, 156u8, 68u8, 80u8, 43u8, 196u8,
							228u8, 119u8, 15u8, 20u8, 176u8, 186u8, 255u8, 85u8, 179u8, 251u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The basic amount of funds that must be reserved for an asset."]
				pub fn asset_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Assets",
						"AssetDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The amount of funds that must be reserved for a non-provider asset account to be"]
				#[doc = " maintained."]
				pub fn asset_account_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Assets",
						"AssetAccountDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The basic amount of funds that must be reserved when adding metadata to your asset."]
				pub fn metadata_deposit_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Assets",
						"MetadataDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The additional funds that must be reserved for the number of bytes you store in your"]
				#[doc = " metadata."]
				pub fn metadata_deposit_per_byte(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Assets",
						"MetadataDepositPerByte",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The amount of funds that must be reserved when creating a new approval."]
				pub fn approval_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Assets",
						"ApprovalDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum length of a name or symbol stored on-chain."]
				pub fn string_limit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Assets",
						"StringLimit",
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
	pub mod asset_registry {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RegisterAsset {
				pub metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
					::core::primitive::u128,
					(),
				>,
				pub asset_id: ::core::option::Option<::core::primitive::u128>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct UpdateAsset {
				pub asset_id: ::core::primitive::u128,
				pub decimals: ::core::option::Option<::core::primitive::u32>,
				pub name: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
				pub symbol: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
				pub existential_deposit: ::core::option::Option<::core::primitive::u128>,
				pub location: ::core::option::Option<
					::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
				>,
				pub additional: ::core::option::Option<()>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn register_asset(
					&self,
					metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
						::core::primitive::u128,
						(),
					>,
					asset_id: ::core::option::Option<::core::primitive::u128>,
				) -> ::subxt::tx::StaticTxPayload<RegisterAsset> {
					::subxt::tx::StaticTxPayload::new(
						"AssetRegistry",
						"register_asset",
						RegisterAsset { metadata, asset_id },
						[
							73u8, 10u8, 1u8, 123u8, 28u8, 1u8, 221u8, 71u8, 183u8, 53u8, 99u8,
							29u8, 200u8, 45u8, 235u8, 49u8, 176u8, 249u8, 15u8, 146u8, 165u8,
							117u8, 198u8, 62u8, 148u8, 5u8, 147u8, 236u8, 16u8, 127u8, 122u8, 64u8,
						],
					)
				}
				pub fn update_asset(
					&self,
					asset_id: ::core::primitive::u128,
					decimals: ::core::option::Option<::core::primitive::u32>,
					name: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					symbol: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					existential_deposit: ::core::option::Option<::core::primitive::u128>,
					location: ::core::option::Option<
						::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
					>,
					additional: ::core::option::Option<()>,
				) -> ::subxt::tx::StaticTxPayload<UpdateAsset> {
					::subxt::tx::StaticTxPayload::new(
						"AssetRegistry",
						"update_asset",
						UpdateAsset {
							asset_id,
							decimals,
							name,
							symbol,
							existential_deposit,
							location,
							additional,
						},
						[
							254u8, 253u8, 26u8, 215u8, 167u8, 34u8, 52u8, 15u8, 45u8, 152u8, 52u8,
							111u8, 123u8, 45u8, 106u8, 103u8, 124u8, 11u8, 236u8, 106u8, 249u8,
							248u8, 177u8, 42u8, 196u8, 120u8, 30u8, 188u8, 45u8, 73u8, 132u8, 81u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::orml_asset_registry::module::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RegisteredAsset {
				pub asset_id: ::core::primitive::u128,
				pub metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
					::core::primitive::u128,
					(),
				>,
			}
			impl ::subxt::events::StaticEvent for RegisteredAsset {
				const PALLET: &'static str = "AssetRegistry";
				const EVENT: &'static str = "RegisteredAsset";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct UpdatedAsset {
				pub asset_id: ::core::primitive::u128,
				pub metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
					::core::primitive::u128,
					(),
				>,
			}
			impl ::subxt::events::StaticEvent for UpdatedAsset {
				const PALLET: &'static str = "AssetRegistry";
				const EVENT: &'static str = "UpdatedAsset";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The metadata of an asset, indexed by asset id."]
				pub fn metadata(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::orml_traits::asset_registry::AssetMetadata<
							::core::primitive::u128,
							(),
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AssetRegistry",
						"Metadata",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							201u8, 109u8, 42u8, 68u8, 153u8, 115u8, 26u8, 61u8, 122u8, 19u8, 241u8,
							99u8, 106u8, 118u8, 92u8, 246u8, 199u8, 186u8, 7u8, 76u8, 32u8, 195u8,
							42u8, 131u8, 196u8, 89u8, 64u8, 157u8, 76u8, 82u8, 105u8, 24u8,
						],
					)
				}
				#[doc = " The metadata of an asset, indexed by asset id."]
				pub fn metadata_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::orml_traits::asset_registry::AssetMetadata<
							::core::primitive::u128,
							(),
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AssetRegistry",
						"Metadata",
						Vec::new(),
						[
							201u8, 109u8, 42u8, 68u8, 153u8, 115u8, 26u8, 61u8, 122u8, 19u8, 241u8,
							99u8, 106u8, 118u8, 92u8, 246u8, 199u8, 186u8, 7u8, 76u8, 32u8, 195u8,
							42u8, 131u8, 196u8, 89u8, 64u8, 157u8, 76u8, 82u8, 105u8, 24u8,
						],
					)
				}
				#[doc = " Maps a multilocation to an asset id - useful when processing xcm"]
				#[doc = " messages."]
				pub fn location_to_asset_id(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::xcm::v1::multilocation::MultiLocation>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AssetRegistry",
						"LocationToAssetId",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							163u8, 239u8, 235u8, 35u8, 77u8, 89u8, 26u8, 231u8, 88u8, 136u8, 240u8,
							194u8, 148u8, 60u8, 186u8, 101u8, 254u8, 3u8, 11u8, 156u8, 71u8, 120u8,
							50u8, 44u8, 175u8, 96u8, 64u8, 57u8, 203u8, 64u8, 211u8, 106u8,
						],
					)
				}
				#[doc = " Maps a multilocation to an asset id - useful when processing xcm"]
				#[doc = " messages."]
				pub fn location_to_asset_id_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AssetRegistry",
						"LocationToAssetId",
						Vec::new(),
						[
							163u8, 239u8, 235u8, 35u8, 77u8, 89u8, 26u8, 231u8, 88u8, 136u8, 240u8,
							194u8, 148u8, 60u8, 186u8, 101u8, 254u8, 3u8, 11u8, 156u8, 71u8, 120u8,
							50u8, 44u8, 175u8, 96u8, 64u8, 57u8, 203u8, 64u8, 211u8, 106u8,
						],
					)
				}
				#[doc = " The last processed asset id - used when assigning a sequential id."]
				pub fn last_asset_id(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AssetRegistry",
						"LastAssetId",
						vec![],
						[
							11u8, 29u8, 228u8, 168u8, 123u8, 15u8, 150u8, 10u8, 158u8, 5u8, 103u8,
							205u8, 152u8, 218u8, 91u8, 82u8, 199u8, 45u8, 162u8, 207u8, 24u8, 55u8,
							40u8, 228u8, 202u8, 10u8, 132u8, 74u8, 82u8, 25u8, 10u8, 238u8,
						],
					)
				}
			}
		}
	}
	pub mod ibc {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Deliver {
				pub messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Transfer {
				pub params: runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
				pub asset_id: ::core::primitive::u128,
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetParams {
				pub params: runtime_types::pallet_ibc::PalletParams,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct UpgradeClient {
				pub params: runtime_types::pallet_ibc::UpgradeParams,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct FreezeClient {
				pub client_id: ::std::vec::Vec<::core::primitive::u8>,
				pub height: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn deliver(
					&self,
					messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
				) -> ::subxt::tx::StaticTxPayload<Deliver> {
					::subxt::tx::StaticTxPayload::new(
						"Ibc",
						"deliver",
						Deliver { messages },
						[
							179u8, 205u8, 83u8, 66u8, 171u8, 103u8, 175u8, 57u8, 35u8, 60u8, 170u8,
							172u8, 60u8, 57u8, 56u8, 226u8, 130u8, 222u8, 121u8, 25u8, 230u8,
							143u8, 253u8, 77u8, 111u8, 152u8, 89u8, 150u8, 129u8, 239u8, 141u8,
							61u8,
						],
					)
				}
				pub fn transfer(
					&self,
					params: runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
					asset_id: ::core::primitive::u128,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Ibc",
						"transfer",
						Transfer { params, asset_id, amount },
						[
							126u8, 30u8, 168u8, 114u8, 3u8, 125u8, 185u8, 145u8, 187u8, 173u8,
							20u8, 241u8, 233u8, 215u8, 52u8, 12u8, 150u8, 174u8, 242u8, 58u8,
							116u8, 11u8, 102u8, 61u8, 221u8, 30u8, 117u8, 47u8, 73u8, 230u8, 8u8,
							0u8,
						],
					)
				}
				pub fn set_params(
					&self,
					params: runtime_types::pallet_ibc::PalletParams,
				) -> ::subxt::tx::StaticTxPayload<SetParams> {
					::subxt::tx::StaticTxPayload::new(
						"Ibc",
						"set_params",
						SetParams { params },
						[
							116u8, 243u8, 44u8, 94u8, 198u8, 240u8, 175u8, 200u8, 234u8, 175u8,
							193u8, 228u8, 45u8, 51u8, 89u8, 123u8, 211u8, 209u8, 214u8, 0u8, 124u8,
							86u8, 142u8, 43u8, 104u8, 198u8, 156u8, 224u8, 51u8, 82u8, 220u8,
							165u8,
						],
					)
				}
				#[doc = "We write the consensus & client state under these predefined paths so that"]
				#[doc = "we can produce state proofs of the values to connected chains"]
				#[doc = "in order to execute client upgrades."]
				pub fn upgrade_client(
					&self,
					params: runtime_types::pallet_ibc::UpgradeParams,
				) -> ::subxt::tx::StaticTxPayload<UpgradeClient> {
					::subxt::tx::StaticTxPayload::new(
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
				#[doc = "Freeze a client at a specific height"]
				pub fn freeze_client(
					&self,
					client_id: ::std::vec::Vec<::core::primitive::u8>,
					height: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<FreezeClient> {
					::subxt::tx::StaticTxPayload::new(
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
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_ibc::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Events emitted by the ibc subsystem"]
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
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An Ibc token transfer has been started"]
			pub struct TokenTransferInitiated {
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id: ::core::option::Option<::core::primitive::u128>,
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
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A channel has been opened"]
			pub struct ChannelOpened {
				pub channel_id: ::std::vec::Vec<::core::primitive::u8>,
				pub port_id: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ChannelOpened {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChannelOpened";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Pallet params updated"]
			pub struct ParamsUpdated {
				pub send_enabled: ::core::primitive::bool,
				pub receive_enabled: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for ParamsUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ParamsUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An outgoing Ibc token transfer has been completed and burnt"]
			pub struct TokenTransferCompleted {
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id: ::core::option::Option<::core::primitive::u128>,
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
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Ibc tokens have been received and minted"]
			pub struct TokenReceived {
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id: ::core::option::Option<::core::primitive::u128>,
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
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Ibc transfer failed, received an acknowledgement error, tokens have been refunded"]
			pub struct TokenTransferFailed {
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id: ::core::option::Option<::core::primitive::u128>,
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
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "On recv packet was not processed successfully processes"]
			pub struct OnRecvPacketError {
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for OnRecvPacketError {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "OnRecvPacketError";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Client upgrade path has been set"]
			pub struct ClientUpgradeSet;
			impl ::subxt::events::StaticEvent for ClientUpgradeSet {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientUpgradeSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Client has been frozen"]
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
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Asset Admin Account Updated"]
			pub struct AssetAdminUpdated {
				pub admin_account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for AssetAdminUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "AssetAdminUpdated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " client_id , Height => Height"]
				pub fn client_update_height(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ClientUpdateHeight",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							169u8, 6u8, 192u8, 186u8, 79u8, 156u8, 202u8, 105u8, 213u8, 28u8,
							186u8, 112u8, 216u8, 170u8, 8u8, 166u8, 181u8, 179u8, 111u8, 212u8,
							35u8, 121u8, 7u8, 86u8, 212u8, 69u8, 66u8, 3u8, 19u8, 220u8, 114u8,
							167u8,
						],
					)
				}
				#[doc = " client_id , Height => Height"]
				pub fn client_update_height_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " client_id , Height => Timestamp"]
				pub fn client_update_time(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ClientUpdateTime",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							98u8, 194u8, 46u8, 221u8, 34u8, 111u8, 178u8, 66u8, 21u8, 234u8, 174u8,
							27u8, 188u8, 45u8, 219u8, 211u8, 68u8, 207u8, 23u8, 228u8, 175u8,
							165u8, 179u8, 18u8, 219u8, 248u8, 34u8, 60u8, 202u8, 106u8, 171u8,
							68u8,
						],
					)
				}
				#[doc = " client_id , Height => Timestamp"]
				pub fn client_update_time_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " connection_identifier => Vec<(port_id, channel_id)>"]
				pub fn channels_connection(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ChannelsConnection",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							175u8, 74u8, 214u8, 39u8, 82u8, 72u8, 28u8, 110u8, 105u8, 136u8, 218u8,
							218u8, 110u8, 111u8, 182u8, 21u8, 180u8, 80u8, 66u8, 44u8, 85u8, 138u8,
							56u8, 102u8, 121u8, 201u8, 111u8, 240u8, 73u8, 7u8, 8u8, 115u8,
						],
					)
				}
				#[doc = " connection_identifier => Vec<(port_id, channel_id)>"]
				pub fn channels_connection_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " counter for clients"]
				pub fn client_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " counter for clients"]
				pub fn connection_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " counter for acknowledgments"]
				pub fn acknowledgement_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " counter for packet receipts"]
				pub fn packet_receipt_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " client_id => Vec<Connection_id>"]
				pub fn connection_client(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ConnectionClient",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							134u8, 166u8, 43u8, 43u8, 142u8, 200u8, 83u8, 81u8, 252u8, 1u8, 153u8,
							167u8, 197u8, 170u8, 154u8, 242u8, 241u8, 178u8, 166u8, 147u8, 223u8,
							188u8, 118u8, 48u8, 40u8, 203u8, 29u8, 17u8, 120u8, 250u8, 79u8, 111u8,
						],
					)
				}
				#[doc = " client_id => Vec<Connection_id>"]
				pub fn connection_client_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Pallet Params used to disable sending or receipt of ibc tokens"]
				pub fn params(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_ibc::PalletParams>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"Params",
						vec![],
						[
							53u8, 220u8, 56u8, 9u8, 21u8, 121u8, 177u8, 62u8, 240u8, 196u8, 215u8,
							157u8, 220u8, 38u8, 85u8, 220u8, 196u8, 38u8, 44u8, 236u8, 64u8, 11u8,
							242u8, 82u8, 230u8, 33u8, 60u8, 148u8, 35u8, 176u8, 81u8, 188u8,
						],
					)
				}
				#[doc = " Map of asset id to ibc denom pairs (T::AssetId, Vec<u8>)"]
				#[doc = " ibc denoms represented as utf8 string bytes"]
				pub fn ibc_asset_ids(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"IbcAssetIds",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							157u8, 241u8, 230u8, 128u8, 83u8, 82u8, 121u8, 80u8, 104u8, 15u8, 12u8,
							250u8, 31u8, 92u8, 230u8, 208u8, 240u8, 144u8, 155u8, 167u8, 210u8,
							60u8, 148u8, 169u8, 176u8, 61u8, 38u8, 249u8, 219u8, 90u8, 159u8,
							123u8,
						],
					)
				}
				#[doc = " Map of asset id to ibc denom pairs (T::AssetId, Vec<u8>)"]
				#[doc = " ibc denoms represented as utf8 string bytes"]
				pub fn ibc_asset_ids_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"IbcAssetIds",
						Vec::new(),
						[
							157u8, 241u8, 230u8, 128u8, 83u8, 82u8, 121u8, 80u8, 104u8, 15u8, 12u8,
							250u8, 31u8, 92u8, 230u8, 208u8, 240u8, 144u8, 155u8, 167u8, 210u8,
							60u8, 148u8, 169u8, 176u8, 61u8, 38u8, 249u8, 219u8, 90u8, 159u8,
							123u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_ibc_asset_ids(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Map of asset id to ibc denom pairs (Vec<u8>, T::AssetId)"]
				#[doc = " ibc denoms represented as utf8 string bytes"]
				pub fn ibc_denoms(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"IbcDenoms",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							128u8, 121u8, 172u8, 103u8, 195u8, 191u8, 151u8, 20u8, 152u8, 114u8,
							90u8, 85u8, 237u8, 10u8, 88u8, 113u8, 97u8, 63u8, 185u8, 102u8, 207u8,
							198u8, 35u8, 106u8, 71u8, 151u8, 13u8, 140u8, 84u8, 37u8, 203u8, 165u8,
						],
					)
				}
				#[doc = " Map of asset id to ibc denom pairs (Vec<u8>, T::AssetId)"]
				#[doc = " ibc denoms represented as utf8 string bytes"]
				pub fn ibc_denoms_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"IbcDenoms",
						Vec::new(),
						[
							128u8, 121u8, 172u8, 103u8, 195u8, 191u8, 151u8, 20u8, 152u8, 114u8,
							90u8, 85u8, 237u8, 10u8, 88u8, 113u8, 97u8, 63u8, 185u8, 102u8, 207u8,
							198u8, 35u8, 106u8, 71u8, 151u8, 13u8, 140u8, 84u8, 37u8, 203u8, 165u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_ibc_denoms(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " ChannelIds open from this module"]
				pub fn channel_ids(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Active Escrow addresses"]
				pub fn escrow_addresses(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::utils::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
				#[doc = " Consensus heights"]
				#[doc = " Stored as a tuple of (revision_number, revision_height)"]
				pub fn consensus_heights(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_btree_set::BoundedBTreeSet<
							runtime_types::ibc::core::ics02_client::height::Height,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ConsensusHeights",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							238u8, 213u8, 231u8, 8u8, 158u8, 84u8, 100u8, 101u8, 78u8, 142u8,
							125u8, 133u8, 128u8, 92u8, 138u8, 184u8, 144u8, 221u8, 58u8, 101u8,
							206u8, 217u8, 140u8, 30u8, 206u8, 26u8, 242u8, 223u8, 113u8, 46u8,
							227u8, 240u8,
						],
					)
				}
				#[doc = " Consensus heights"]
				#[doc = " Stored as a tuple of (revision_number, revision_height)"]
				pub fn consensus_heights_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_btree_set::BoundedBTreeSet<
							runtime_types::ibc::core::ics02_client::height::Height,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
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
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The native asset id, this will use the `NativeCurrency` for all operations."]
				pub fn native_asset_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Ibc",
						"NativeAssetId",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Expected block time in milliseconds"]
				pub fn expected_block_time(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
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
				#[doc = " Minimum connection delay period in seconds for ibc connections that can be created or"]
				#[doc = " accepted. Ensure that this is non-zero in production as it's a critical vulnerability."]
				pub fn minimum_connection_delay(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
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
				#[doc = " Amount to be reserved for client and connection creation"]
				pub fn spam_protection_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Ibc",
						"SpamProtectionDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
			}
		}
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod cumulus_pallet_dmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Service a single overweight message."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
					#[doc = "- `index`: The index of the overweight message to service."]
					#[doc = "- `weight_limit`: The amount of weight that message execution may take."]
					#[doc = ""]
					#[doc = "Errors:"]
					#[doc = "- `Unknown`: Message of `index` is unknown."]
					#[doc = "- `OverLimit`: Message execution may use greater than `weight_limit`."]
					#[doc = ""]
					#[doc = "Events:"]
					#[doc = "- `OverweightServiced`: On success."]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: ::core::primitive::u64,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The message index given is unknown."]
					Unknown,
					#[codec(index = 1)]
					#[doc = "The amount of weight given is possibly not enough for executing the message."]
					OverLimit,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Downward message is invalid XCM."]
					InvalidFormat { message_id: [::core::primitive::u8; 32usize] },
					#[codec(index = 1)]
					#[doc = "Downward message is unsupported version of XCM."]
					UnsupportedVersion { message_id: [::core::primitive::u8; 32usize] },
					#[codec(index = 2)]
					#[doc = "Downward message executed with the given outcome."]
					ExecutedDownward {
						message_id: [::core::primitive::u8; 32usize],
						outcome: runtime_types::xcm::v2::traits::Outcome,
					},
					#[codec(index = 3)]
					#[doc = "The weight limit for handling downward messages was reached."]
					WeightExhausted {
						message_id: [::core::primitive::u8; 32usize],
						remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 4)]
					#[doc = "Downward message is overweight and was placed in the overweight queue."]
					OverweightEnqueued {
						message_id: [::core::primitive::u8; 32usize],
						overweight_index: ::core::primitive::u64,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 5)]
					#[doc = "Downward message from the overweight queue was executed."]
					OverweightServiced {
						overweight_index: ::core::primitive::u64,
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ConfigData {
				pub max_individual: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					# [codec (index = 0)] # [doc = "Set the current validation data."] # [doc = ""] # [doc = "This should be invoked exactly once per block. It will panic at the finalization"] # [doc = "phase if the call was not invoked."] # [doc = ""] # [doc = "The dispatch origin for this call must be `Inherent`"] # [doc = ""] # [doc = "As a side effect, this function upgrades the current validation function"] # [doc = "if the appropriate time has come."] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] authorize_upgrade { code_hash : :: subxt :: utils :: H256 , } , # [codec (index = 3)] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Attempt to upgrade validation function while existing upgrade pending"]
					OverlappingUpgrades,
					#[codec(index = 1)]
					#[doc = "Polkadot currently prohibits this parachain from upgrading its validation function"]
					ProhibitedByPolkadot,
					#[codec(index = 2)]
					#[doc = "The supplied validation function has compiled into a blob larger than Polkadot is"]
					#[doc = "willing to run"]
					TooBig,
					#[codec(index = 3)]
					#[doc = "The inherent which supplies the validation data did not run this block"]
					ValidationDataNotAvailable,
					#[codec(index = 4)]
					#[doc = "The inherent which supplies the host configuration did not run this block"]
					HostConfigurationNotAvailable,
					#[codec(index = 5)]
					#[doc = "No validation function upgrade is currently scheduled."]
					NotScheduled,
					#[codec(index = 6)]
					#[doc = "No code upgrade has been authorized."]
					NothingAuthorized,
					#[codec(index = 7)]
					#[doc = "The given code upgrade has not been authorized."]
					Unauthorized,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "The validation function has been scheduled to apply."]
					ValidationFunctionStored,
					#[codec(index = 1)]
					#[doc = "The validation function was applied as of the contained relay chain block number."]
					ValidationFunctionApplied { relay_chain_block_num: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "The relay-chain aborted the upgrade process."]
					ValidationFunctionDiscarded,
					#[codec(index = 3)]
					#[doc = "An upgrade has been authorized."]
					UpgradeAuthorized { code_hash: ::subxt::utils::H256 },
					#[codec(index = 4)]
					#[doc = "Some downward messages have been received and will be processed."]
					DownwardMessagesReceived { count: ::core::primitive::u32 },
					#[codec(index = 5)]
					#[doc = "Downward messages were processed using the given weight."]
					DownwardMessagesProcessed {
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
						dmq_head: ::subxt::utils::H256,
					},
				}
			}
			pub mod relay_state_snapshot {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Downward message is invalid XCM."]
					#[doc = "\\[ id \\]"]
					InvalidFormat([::core::primitive::u8; 8usize]),
					#[codec(index = 1)]
					#[doc = "Downward message is unsupported version of XCM."]
					#[doc = "\\[ id \\]"]
					UnsupportedVersion([::core::primitive::u8; 8usize]),
					#[codec(index = 2)]
					#[doc = "Downward message executed with the given outcome."]
					#[doc = "\\[ id, outcome \\]"]
					ExecutedDownward(
						[::core::primitive::u8; 8usize],
						runtime_types::xcm::v2::traits::Outcome,
					),
				}
			}
		}
		pub mod cumulus_pallet_xcmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Services a single overweight XCM."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
					#[doc = "- `index`: The index of the overweight XCM to service"]
					#[doc = "- `weight_limit`: The amount of weight that XCM execution may take."]
					#[doc = ""]
					#[doc = "Errors:"]
					#[doc = "- `BadOverweightIndex`: XCM under `index` is not found in the `Overweight` storage map."]
					#[doc = "- `BadXcm`: XCM under `index` cannot be properly decoded into a valid XCM format."]
					#[doc = "- `WeightOverLimit`: XCM execution may use greater `weight_limit`."]
					#[doc = ""]
					#[doc = "Events:"]
					#[doc = "- `OverweightServiced`: On success."]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: ::core::primitive::u64,
					},
					#[codec(index = 1)]
					#[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ControllerOrigin`."]
					suspend_xcm_execution,
					#[codec(index = 2)]
					#[doc = "Resumes all XCM executions for the XCMP queue."]
					#[doc = ""]
					#[doc = "Note that this function doesn't change the status of the in/out bound channels."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ControllerOrigin`."]
					resume_xcm_execution,
					#[codec(index = 3)]
					#[doc = "Overwrites the number of pages of messages which must be in the queue for the other side to be told to"]
					#[doc = "suspend their sending."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
					update_suspend_threshold { new: ::core::primitive::u32 },
					#[codec(index = 4)]
					#[doc = "Overwrites the number of pages of messages which must be in the queue after which we drop any further"]
					#[doc = "messages from the channel."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
					update_drop_threshold { new: ::core::primitive::u32 },
					#[codec(index = 5)]
					#[doc = "Overwrites the number of pages of messages which the queue must be reduced to before it signals that"]
					#[doc = "message sending may recommence after it has been suspended."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
					update_resume_threshold { new: ::core::primitive::u32 },
					#[codec(index = 6)]
					#[doc = "Overwrites the amount of remaining weight under which we stop processing messages."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.threshold_weight`"]
					update_threshold_weight { new: ::core::primitive::u64 },
					#[codec(index = 7)]
					#[doc = "Overwrites the speed to which the available weight approaches the maximum weight."]
					#[doc = "A lower number results in a faster progression. A value of 1 makes the entire weight available initially."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.weight_restrict_decay`."]
					update_weight_restrict_decay { new: ::core::primitive::u64 },
					#[codec(index = 8)]
					#[doc = "Overwrite the maximum amount of weight any individual message may consume."]
					#[doc = "Messages above this weight go into the overweight queue and may only be serviced explicitly."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.xcmp_max_individual_weight`."]
					update_xcmp_max_individual_weight { new: ::core::primitive::u64 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Failed to send XCM message."]
					FailedToSend,
					#[codec(index = 1)]
					#[doc = "Bad XCM origin."]
					BadXcmOrigin,
					#[codec(index = 2)]
					#[doc = "Bad XCM data."]
					BadXcm,
					#[codec(index = 3)]
					#[doc = "Bad overweight index."]
					BadOverweightIndex,
					#[codec(index = 4)]
					#[doc = "Provided weight is possibly not enough to execute the message."]
					WeightOverLimit,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Some XCM was executed ok."]
					Success {
						message_hash: ::core::option::Option<::subxt::utils::H256>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 1)]
					#[doc = "Some XCM failed."]
					Fail {
						message_hash: ::core::option::Option<::subxt::utils::H256>,
						error: runtime_types::xcm::v2::traits::Error,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					#[doc = "Bad XCM version used."]
					BadVersion { message_hash: ::core::option::Option<::subxt::utils::H256> },
					#[codec(index = 3)]
					#[doc = "Bad XCM format used."]
					BadFormat { message_hash: ::core::option::Option<::subxt::utils::H256> },
					#[codec(index = 4)]
					#[doc = "An upward message was sent to the relay chain."]
					UpwardMessageSent { message_hash: ::core::option::Option<::subxt::utils::H256> },
					#[codec(index = 5)]
					#[doc = "An HRMP message was sent to a sibling parachain."]
					XcmpMessageSent { message_hash: ::core::option::Option<::subxt::utils::H256> },
					#[codec(index = 6)]
					#[doc = "An XCM exceeded the individual message weight budget."]
					OverweightEnqueued {
						sender: runtime_types::polkadot_parachain::primitives::Id,
						sent_at: ::core::primitive::u32,
						index: ::core::primitive::u64,
						required: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 7)]
					#[doc = "An XCM from the overweight queue was executed with the given actual weight used."]
					OverweightServiced {
						index: ::core::primitive::u64,
						used: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct InboundChannelDetails {
				pub sender: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::InboundState,
				pub message_metadata: ::std::vec::Vec<(
					::core::primitive::u32,
					runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum InboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct OutboundChannelDetails {
				pub recipient: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::OutboundState,
				pub signals_exist: ::core::primitive::bool,
				pub first_index: ::core::primitive::u16,
				pub last_index: ::core::primitive::u16,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum OutboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct MessageQueueChain(pub ::subxt::utils::H256);
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct DispatchInfo {
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							Debug,
						)]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
					}
				}
			}
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
						Debug,
					)]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
						::core::primitive::u32,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BlockWeights {
					pub base_block: runtime_types::sp_weights::weight_v2::Weight,
					pub max_block: runtime_types::sp_weights::weight_v2::Weight,
					pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "A dispatch that will fill the block weight up to the given ratio."]
					fill_block { ratio: runtime_types::sp_arithmetic::per_things::Perbill },
					#[codec(index = 1)]
					#[doc = "Make some on-chain remark."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`"]
					#[doc = "# </weight>"]
					remark { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 2)]
					#[doc = "Set the number of pages in the WebAssembly environment's heap."]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 3)]
					#[doc = "Set the new runtime code."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
					#[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
					#[doc = "  expensive)."]
					#[doc = "- 1 storage write (codec `O(C)`)."]
					#[doc = "- 1 digest item."]
					#[doc = "- 1 event."]
					#[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
					#[doc = "expensive. We will treat this as a full block."]
					#[doc = "# </weight>"]
					set_code { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					#[doc = "Set the new runtime code without doing any checks of the given `code`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(C)` where `C` length of `code`"]
					#[doc = "- 1 storage write (codec `O(C)`)."]
					#[doc = "- 1 digest item."]
					#[doc = "- 1 event."]
					#[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
					#[doc = "block. # </weight>"]
					set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 5)]
					#[doc = "Set some items of storage."]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 6)]
					#[doc = "Kill some items from storage."]
					kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
					#[codec(index = 7)]
					#[doc = "Kill all storage items with a key that starts with the given prefix."]
					#[doc = ""]
					#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
					#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					#[doc = "Make some on-chain remark and emit event."]
					remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Error for the System pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The name of specification does not match between the current runtime"]
					#[doc = "and the new runtime."]
					InvalidSpecName,
					#[codec(index = 1)]
					#[doc = "The specification version is not allowed to decrease between the current runtime"]
					#[doc = "and the new runtime."]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					#[doc = "Failed to extract the runtime version from the new runtime."]
					#[doc = ""]
					#[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					#[doc = "Suicide called when the account has non-default composite data."]
					NonDefaultComposite,
					#[codec(index = 4)]
					#[doc = "There is a non-zero reference count preventing the account from being purged."]
					NonZeroRefCount,
					#[codec(index = 5)]
					#[doc = "The origin filter prevent the call to be dispatched."]
					CallFiltered,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Event for the System pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An extrinsic completed successfully."]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 1)]
					#[doc = "An extrinsic failed."]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 2)]
					#[doc = "`:code` was updated."]
					CodeUpdated,
					#[codec(index = 3)]
					#[doc = "A new account was created."]
					NewAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 4)]
					#[doc = "An account was reaped."]
					KilledAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 5)]
					#[doc = "On on-chain remark happened."]
					Remarked { sender: ::subxt::utils::AccountId32, hash: ::subxt::utils::H256 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: _0,
				pub providers: _0,
				pub sufficients: _0,
				pub data: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
			pub mod core {
				use super::runtime_types;
				pub mod ics02_client {
					use super::runtime_types;
					pub mod height {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							Debug,
						)]
						pub struct Height {
							pub revision_number: ::core::primitive::u64,
							pub revision_height: ::core::primitive::u64,
						}
					}
				}
			}
		}
		pub mod ibc_primitives {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
		pub mod orml_asset_registry {
			use super::runtime_types;
			pub mod module {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					register_asset {
						metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
							::core::primitive::u128,
							(),
						>,
						asset_id: ::core::option::Option<::core::primitive::u128>,
					},
					#[codec(index = 1)]
					update_asset {
						asset_id: ::core::primitive::u128,
						decimals: ::core::option::Option<::core::primitive::u32>,
						name: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						symbol: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						existential_deposit: ::core::option::Option<::core::primitive::u128>,
						location: ::core::option::Option<
							::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
						>,
						additional: ::core::option::Option<()>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Asset was not found."]
					AssetNotFound,
					#[codec(index = 1)]
					#[doc = "The version of the `VersionedMultiLocation` value used is not able"]
					#[doc = "to be interpreted."]
					BadVersion,
					#[codec(index = 2)]
					#[doc = "The asset id is invalid."]
					InvalidAssetId,
					#[codec(index = 3)]
					#[doc = "Another asset was already register with this location."]
					ConflictingLocation,
					#[codec(index = 4)]
					#[doc = "Another asset was already register with this asset id."]
					ConflictingAssetId,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					RegisteredAsset {
						asset_id: ::core::primitive::u128,
						metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
							::core::primitive::u128,
							(),
						>,
					},
					#[codec(index = 1)]
					UpdatedAsset {
						asset_id: ::core::primitive::u128,
						metadata: runtime_types::orml_traits::asset_registry::AssetMetadata<
							::core::primitive::u128,
							(),
						>,
					},
				}
			}
		}
		pub mod orml_traits {
			use super::runtime_types;
			pub mod asset_registry {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct AssetMetadata<_0, _1> {
					pub decimals: ::core::primitive::u32,
					pub name: ::std::vec::Vec<::core::primitive::u8>,
					pub symbol: ::std::vec::Vec<::core::primitive::u8>,
					pub existential_deposit: _0,
					pub location:
						::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
					pub additional: _1,
				}
			}
		}
		pub mod pallet_assets {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Issue a new class of fungible assets from a public origin."]
					#[doc = ""]
					#[doc = "This new asset class has no assets initially and its owner is the origin."]
					#[doc = ""]
					#[doc = "The origin must conform to the configured `CreateOrigin` and have sufficient funds free."]
					#[doc = ""]
					#[doc = "Funds of sender are reserved by `AssetDeposit`."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
					#[doc = "an existing asset."]
					#[doc = "- `admin`: The admin of this class of assets. The admin is the initial address of each"]
					#[doc = "member of the asset class's admin team."]
					#[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
					#[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
					#[doc = ""]
					#[doc = "Emits `Created` event when successful."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					create {
						#[codec(compact)]
						id: ::core::primitive::u128,
						admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						min_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "Issue a new class of fungible assets from a privileged origin."]
					#[doc = ""]
					#[doc = "This new asset class has no assets initially."]
					#[doc = ""]
					#[doc = "The origin must conform to `ForceOrigin`."]
					#[doc = ""]
					#[doc = "Unlike `create`, no funds are reserved."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
					#[doc = "an existing asset."]
					#[doc = "- `owner`: The owner of this class of assets. The owner has full superuser permissions"]
					#[doc = "over this asset, but may later change and configure the permissions using"]
					#[doc = "`transfer_ownership` and `set_team`."]
					#[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
					#[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
					#[doc = ""]
					#[doc = "Emits `ForceCreated` event when successful."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					force_create {
						#[codec(compact)]
						id: ::core::primitive::u128,
						owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						is_sufficient: ::core::primitive::bool,
						#[codec(compact)]
						min_balance: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Destroy a class of fungible assets."]
					#[doc = ""]
					#[doc = "The origin must conform to `ForceOrigin` or must be Signed and the sender must be the"]
					#[doc = "owner of the asset `id`."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
					#[doc = "asset."]
					#[doc = ""]
					#[doc = "Emits `Destroyed` event when successful."]
					#[doc = ""]
					#[doc = "NOTE: It can be helpful to first freeze an asset before destroying it so that you"]
					#[doc = "can provide accurate witness information and prevent users from manipulating state"]
					#[doc = "in a way that can make it harder to destroy."]
					#[doc = ""]
					#[doc = "Weight: `O(c + p + a)` where:"]
					#[doc = "- `c = (witness.accounts - witness.sufficients)`"]
					#[doc = "- `s = witness.sufficients`"]
					#[doc = "- `a = witness.approvals`"]
					destroy {
						#[codec(compact)]
						id: ::core::primitive::u128,
						witness: runtime_types::pallet_assets::types::DestroyWitness,
					},
					#[codec(index = 3)]
					#[doc = "Mint assets of a particular class."]
					#[doc = ""]
					#[doc = "The origin must be Signed and the sender must be the Issuer of the asset `id`."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to have some amount minted."]
					#[doc = "- `beneficiary`: The account to be credited with the minted assets."]
					#[doc = "- `amount`: The amount of the asset to be minted."]
					#[doc = ""]
					#[doc = "Emits `Issued` event when successful."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					#[doc = "Modes: Pre-existing balance of `beneficiary`; Account pre-existence of `beneficiary`."]
					mint {
						#[codec(compact)]
						id: ::core::primitive::u128,
						beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Reduce the balance of `who` by as much as possible up to `amount` assets of `id`."]
					#[doc = ""]
					#[doc = "Origin must be Signed and the sender should be the Manager of the asset `id`."]
					#[doc = ""]
					#[doc = "Bails with `NoAccount` if the `who` is already dead."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to have some amount burned."]
					#[doc = "- `who`: The account to be debited from."]
					#[doc = "- `amount`: The maximum amount by which `who`'s balance should be reduced."]
					#[doc = ""]
					#[doc = "Emits `Burned` with the actual amount burned. If this takes the balance to below the"]
					#[doc = "minimum for the asset, then the amount burned is increased to take it to zero."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					#[doc = "Modes: Post-existence of `who`; Pre & post Zombie-status of `who`."]
					burn {
						#[codec(compact)]
						id: ::core::primitive::u128,
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "Move some assets from the sender account to another."]
					#[doc = ""]
					#[doc = "Origin must be Signed."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to have some amount transferred."]
					#[doc = "- `target`: The account to be credited."]
					#[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
					#[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
					#[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
					#[doc = "the minimum balance. Must be greater than zero."]
					#[doc = ""]
					#[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
					#[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
					#[doc = "to zero."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					#[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
					#[doc = "`target`."]
					transfer {
						#[codec(compact)]
						id: ::core::primitive::u128,
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Move some assets from the sender account to another, keeping the sender account alive."]
					#[doc = ""]
					#[doc = "Origin must be Signed."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to have some amount transferred."]
					#[doc = "- `target`: The account to be credited."]
					#[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
					#[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
					#[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
					#[doc = "the minimum balance. Must be greater than zero."]
					#[doc = ""]
					#[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
					#[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
					#[doc = "to zero."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					#[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
					#[doc = "`target`."]
					transfer_keep_alive {
						#[codec(compact)]
						id: ::core::primitive::u128,
						target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 7)]
					#[doc = "Move some assets from one account to another."]
					#[doc = ""]
					#[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to have some amount transferred."]
					#[doc = "- `source`: The account to be debited."]
					#[doc = "- `dest`: The account to be credited."]
					#[doc = "- `amount`: The amount by which the `source`'s balance of assets should be reduced and"]
					#[doc = "`dest`'s balance increased. The amount actually transferred may be slightly greater in"]
					#[doc = "the case that the transfer would otherwise take the `source` balance above zero but"]
					#[doc = "below the minimum balance. Must be greater than zero."]
					#[doc = ""]
					#[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
					#[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
					#[doc = "to zero."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					#[doc = "Modes: Pre-existence of `dest`; Post-existence of `source`; Account pre-existence of"]
					#[doc = "`dest`."]
					force_transfer {
						#[codec(compact)]
						id: ::core::primitive::u128,
						source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "Disallow further unprivileged transfers from an account."]
					#[doc = ""]
					#[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to be frozen."]
					#[doc = "- `who`: The account to be frozen."]
					#[doc = ""]
					#[doc = "Emits `Frozen`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					freeze {
						#[codec(compact)]
						id: ::core::primitive::u128,
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 9)]
					#[doc = "Allow unprivileged transfers from an account again."]
					#[doc = ""]
					#[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to be frozen."]
					#[doc = "- `who`: The account to be unfrozen."]
					#[doc = ""]
					#[doc = "Emits `Thawed`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					thaw {
						#[codec(compact)]
						id: ::core::primitive::u128,
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 10)]
					#[doc = "Disallow further unprivileged transfers for the asset class."]
					#[doc = ""]
					#[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to be frozen."]
					#[doc = ""]
					#[doc = "Emits `Frozen`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					freeze_asset {
						#[codec(compact)]
						id: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					#[doc = "Allow unprivileged transfers for the asset again."]
					#[doc = ""]
					#[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to be thawed."]
					#[doc = ""]
					#[doc = "Emits `Thawed`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					thaw_asset {
						#[codec(compact)]
						id: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					#[doc = "Change the Owner of an asset."]
					#[doc = ""]
					#[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset."]
					#[doc = "- `owner`: The new Owner of this asset."]
					#[doc = ""]
					#[doc = "Emits `OwnerChanged`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					transfer_ownership {
						#[codec(compact)]
						id: ::core::primitive::u128,
						owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 13)]
					#[doc = "Change the Issuer, Admin and Freezer of an asset."]
					#[doc = ""]
					#[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to be frozen."]
					#[doc = "- `issuer`: The new Issuer of this asset."]
					#[doc = "- `admin`: The new Admin of this asset."]
					#[doc = "- `freezer`: The new Freezer of this asset."]
					#[doc = ""]
					#[doc = "Emits `TeamChanged`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					set_team {
						#[codec(compact)]
						id: ::core::primitive::u128,
						issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 14)]
					#[doc = "Set the metadata for an asset."]
					#[doc = ""]
					#[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
					#[doc = ""]
					#[doc = "Funds of sender are reserved according to the formula:"]
					#[doc = "`MetadataDepositBase + MetadataDepositPerByte * (name.len + symbol.len)` taking into"]
					#[doc = "account any already reserved funds."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to update."]
					#[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
					#[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
					#[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
					#[doc = ""]
					#[doc = "Emits `MetadataSet`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					set_metadata {
						#[codec(compact)]
						id: ::core::primitive::u128,
						name: ::std::vec::Vec<::core::primitive::u8>,
						symbol: ::std::vec::Vec<::core::primitive::u8>,
						decimals: ::core::primitive::u8,
					},
					#[codec(index = 15)]
					#[doc = "Clear the metadata for an asset."]
					#[doc = ""]
					#[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
					#[doc = ""]
					#[doc = "Any deposit is freed for the asset owner."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to clear."]
					#[doc = ""]
					#[doc = "Emits `MetadataCleared`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					clear_metadata {
						#[codec(compact)]
						id: ::core::primitive::u128,
					},
					#[codec(index = 16)]
					#[doc = "Force the metadata for an asset to some value."]
					#[doc = ""]
					#[doc = "Origin must be ForceOrigin."]
					#[doc = ""]
					#[doc = "Any deposit is left alone."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to update."]
					#[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
					#[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
					#[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
					#[doc = ""]
					#[doc = "Emits `MetadataSet`."]
					#[doc = ""]
					#[doc = "Weight: `O(N + S)` where N and S are the length of the name and symbol respectively."]
					force_set_metadata {
						#[codec(compact)]
						id: ::core::primitive::u128,
						name: ::std::vec::Vec<::core::primitive::u8>,
						symbol: ::std::vec::Vec<::core::primitive::u8>,
						decimals: ::core::primitive::u8,
						is_frozen: ::core::primitive::bool,
					},
					#[codec(index = 17)]
					#[doc = "Clear the metadata for an asset."]
					#[doc = ""]
					#[doc = "Origin must be ForceOrigin."]
					#[doc = ""]
					#[doc = "Any deposit is returned."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset to clear."]
					#[doc = ""]
					#[doc = "Emits `MetadataCleared`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					force_clear_metadata {
						#[codec(compact)]
						id: ::core::primitive::u128,
					},
					#[codec(index = 18)]
					#[doc = "Alter the attributes of a given asset."]
					#[doc = ""]
					#[doc = "Origin must be `ForceOrigin`."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset."]
					#[doc = "- `owner`: The new Owner of this asset."]
					#[doc = "- `issuer`: The new Issuer of this asset."]
					#[doc = "- `admin`: The new Admin of this asset."]
					#[doc = "- `freezer`: The new Freezer of this asset."]
					#[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
					#[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
					#[doc = "- `is_sufficient`: Whether a non-zero balance of this asset is deposit of sufficient"]
					#[doc = "value to account for the state bloat associated with its balance storage. If set to"]
					#[doc = "`true`, then non-zero balances may be stored without a `consumer` reference (and thus"]
					#[doc = "an ED in the Balances pallet or whatever else is used to control user-account state"]
					#[doc = "growth)."]
					#[doc = "- `is_frozen`: Whether this asset class is frozen except for permissioned/admin"]
					#[doc = "instructions."]
					#[doc = ""]
					#[doc = "Emits `AssetStatusChanged` with the identity of the asset."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					force_asset_status {
						#[codec(compact)]
						id: ::core::primitive::u128,
						owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						min_balance: ::core::primitive::u128,
						is_sufficient: ::core::primitive::bool,
						is_frozen: ::core::primitive::bool,
					},
					#[codec(index = 19)]
					#[doc = "Approve an amount of asset for transfer by a delegated third-party account."]
					#[doc = ""]
					#[doc = "Origin must be Signed."]
					#[doc = ""]
					#[doc = "Ensures that `ApprovalDeposit` worth of `Currency` is reserved from signing account"]
					#[doc = "for the purpose of holding the approval. If some non-zero amount of assets is already"]
					#[doc = "approved from signing account to `delegate`, then it is topped up or unreserved to"]
					#[doc = "meet the right value."]
					#[doc = ""]
					#[doc = "NOTE: The signing account does not need to own `amount` of assets at the point of"]
					#[doc = "making this call."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset."]
					#[doc = "- `delegate`: The account to delegate permission to transfer asset."]
					#[doc = "- `amount`: The amount of asset that may be transferred by `delegate`. If there is"]
					#[doc = "already an approval in place, then this acts additively."]
					#[doc = ""]
					#[doc = "Emits `ApprovedTransfer` on success."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					approve_transfer {
						#[codec(compact)]
						id: ::core::primitive::u128,
						delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 20)]
					#[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
					#[doc = ""]
					#[doc = "Origin must be Signed and there must be an approval in place between signer and"]
					#[doc = "`delegate`."]
					#[doc = ""]
					#[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset."]
					#[doc = "- `delegate`: The account delegated permission to transfer asset."]
					#[doc = ""]
					#[doc = "Emits `ApprovalCancelled` on success."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					cancel_approval {
						#[codec(compact)]
						id: ::core::primitive::u128,
						delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 21)]
					#[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
					#[doc = ""]
					#[doc = "Origin must be either ForceOrigin or Signed origin with the signer being the Admin"]
					#[doc = "account of the asset `id`."]
					#[doc = ""]
					#[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset."]
					#[doc = "- `delegate`: The account delegated permission to transfer asset."]
					#[doc = ""]
					#[doc = "Emits `ApprovalCancelled` on success."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					force_cancel_approval {
						#[codec(compact)]
						id: ::core::primitive::u128,
						owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					},
					#[codec(index = 22)]
					#[doc = "Transfer some asset balance from a previously delegated account to some third-party"]
					#[doc = "account."]
					#[doc = ""]
					#[doc = "Origin must be Signed and there must be an approval in place by the `owner` to the"]
					#[doc = "signer."]
					#[doc = ""]
					#[doc = "If the entire amount approved for transfer is transferred, then any deposit previously"]
					#[doc = "reserved by `approve_transfer` is unreserved."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset."]
					#[doc = "- `owner`: The account which previously approved for a transfer of at least `amount` and"]
					#[doc = "from which the asset balance will be withdrawn."]
					#[doc = "- `destination`: The account to which the asset balance of `amount` will be transferred."]
					#[doc = "- `amount`: The amount of assets to transfer."]
					#[doc = ""]
					#[doc = "Emits `TransferredApproved` on success."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					transfer_approved {
						#[codec(compact)]
						id: ::core::primitive::u128,
						owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						destination: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 23)]
					#[doc = "Create an asset account for non-provider assets."]
					#[doc = ""]
					#[doc = "A deposit will be taken from the signer account."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Signed; the signer account must have sufficient funds for a deposit"]
					#[doc = "  to be taken."]
					#[doc = "- `id`: The identifier of the asset for the account to be created."]
					#[doc = ""]
					#[doc = "Emits `Touched` event when successful."]
					touch {
						#[codec(compact)]
						id: ::core::primitive::u128,
					},
					#[codec(index = 24)]
					#[doc = "Return the deposit (if any) of an asset account."]
					#[doc = ""]
					#[doc = "The origin must be Signed."]
					#[doc = ""]
					#[doc = "- `id`: The identifier of the asset for the account to be created."]
					#[doc = "- `allow_burn`: If `true` then assets may be destroyed in order to complete the refund."]
					#[doc = ""]
					#[doc = "Emits `Refunded` event when successful."]
					refund {
						#[codec(compact)]
						id: ::core::primitive::u128,
						allow_burn: ::core::primitive::bool,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Account balance must be greater than or equal to the transfer amount."]
					BalanceLow,
					#[codec(index = 1)]
					#[doc = "The account to alter does not exist."]
					NoAccount,
					#[codec(index = 2)]
					#[doc = "The signing account has no permission to do the operation."]
					NoPermission,
					#[codec(index = 3)]
					#[doc = "The given asset ID is unknown."]
					Unknown,
					#[codec(index = 4)]
					#[doc = "The origin account is frozen."]
					Frozen,
					#[codec(index = 5)]
					#[doc = "The asset ID is already taken."]
					InUse,
					#[codec(index = 6)]
					#[doc = "Invalid witness data given."]
					BadWitness,
					#[codec(index = 7)]
					#[doc = "Minimum balance should be non-zero."]
					MinBalanceZero,
					#[codec(index = 8)]
					#[doc = "Unable to increment the consumer reference counters on the account. Either no provider"]
					#[doc = "reference exists to allow a non-zero balance of a non-self-sufficient asset, or the"]
					#[doc = "maximum number of consumers has been reached."]
					NoProvider,
					#[codec(index = 9)]
					#[doc = "Invalid metadata given."]
					BadMetadata,
					#[codec(index = 10)]
					#[doc = "No approval exists that would allow the transfer."]
					Unapproved,
					#[codec(index = 11)]
					#[doc = "The source account would not survive the transfer and it needs to stay alive."]
					WouldDie,
					#[codec(index = 12)]
					#[doc = "The asset-account already exists."]
					AlreadyExists,
					#[codec(index = 13)]
					#[doc = "The asset-account doesn't have an associated deposit."]
					NoDeposit,
					#[codec(index = 14)]
					#[doc = "The operation would result in funds being burned."]
					WouldBurn,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Some asset class was created."]
					Created {
						asset_id: ::core::primitive::u128,
						creator: ::subxt::utils::AccountId32,
						owner: ::subxt::utils::AccountId32,
					},
					#[codec(index = 1)]
					#[doc = "Some assets were issued."]
					Issued {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
						total_supply: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Some assets were transferred."]
					Transferred {
						asset_id: ::core::primitive::u128,
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "Some assets were destroyed."]
					Burned {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "The management team changed."]
					TeamChanged {
						asset_id: ::core::primitive::u128,
						issuer: ::subxt::utils::AccountId32,
						admin: ::subxt::utils::AccountId32,
						freezer: ::subxt::utils::AccountId32,
					},
					#[codec(index = 5)]
					#[doc = "The owner changed."]
					OwnerChanged {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
					},
					#[codec(index = 6)]
					#[doc = "Some account `who` was frozen."]
					Frozen { asset_id: ::core::primitive::u128, who: ::subxt::utils::AccountId32 },
					#[codec(index = 7)]
					#[doc = "Some account `who` was thawed."]
					Thawed { asset_id: ::core::primitive::u128, who: ::subxt::utils::AccountId32 },
					#[codec(index = 8)]
					#[doc = "Some asset `asset_id` was frozen."]
					AssetFrozen { asset_id: ::core::primitive::u128 },
					#[codec(index = 9)]
					#[doc = "Some asset `asset_id` was thawed."]
					AssetThawed { asset_id: ::core::primitive::u128 },
					#[codec(index = 10)]
					#[doc = "An asset class was destroyed."]
					Destroyed { asset_id: ::core::primitive::u128 },
					#[codec(index = 11)]
					#[doc = "Some asset class was force-created."]
					ForceCreated {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
					},
					#[codec(index = 12)]
					#[doc = "New metadata has been set for an asset."]
					MetadataSet {
						asset_id: ::core::primitive::u128,
						name: ::std::vec::Vec<::core::primitive::u8>,
						symbol: ::std::vec::Vec<::core::primitive::u8>,
						decimals: ::core::primitive::u8,
						is_frozen: ::core::primitive::bool,
					},
					#[codec(index = 13)]
					#[doc = "Metadata has been cleared for an asset."]
					MetadataCleared { asset_id: ::core::primitive::u128 },
					#[codec(index = 14)]
					#[doc = "(Additional) funds have been approved for transfer to a destination account."]
					ApprovedTransfer {
						asset_id: ::core::primitive::u128,
						source: ::subxt::utils::AccountId32,
						delegate: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 15)]
					#[doc = "An approval for account `delegate` was cancelled by `owner`."]
					ApprovalCancelled {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
						delegate: ::subxt::utils::AccountId32,
					},
					#[codec(index = 16)]
					#[doc = "An `amount` was transferred in its entirety from `owner` to `destination` by"]
					#[doc = "the approved `delegate`."]
					TransferredApproved {
						asset_id: ::core::primitive::u128,
						owner: ::subxt::utils::AccountId32,
						delegate: ::subxt::utils::AccountId32,
						destination: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 17)]
					#[doc = "An asset has had its attributes changed by the `Force` origin."]
					AssetStatusChanged { asset_id: ::core::primitive::u128 },
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Approval<_0, _1> {
					pub amount: _0,
					pub deposit: _0,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct AssetAccount<_0, _1, _2> {
					pub balance: _0,
					pub is_frozen: ::core::primitive::bool,
					pub reason: runtime_types::pallet_assets::types::ExistenceReason<_0>,
					pub extra: _2,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct AssetDetails<_0, _1, _2> {
					pub owner: _1,
					pub issuer: _1,
					pub admin: _1,
					pub freezer: _1,
					pub supply: _0,
					pub deposit: _0,
					pub min_balance: _0,
					pub is_sufficient: ::core::primitive::bool,
					pub accounts: ::core::primitive::u32,
					pub sufficients: ::core::primitive::u32,
					pub approvals: ::core::primitive::u32,
					pub is_frozen: ::core::primitive::bool,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct AssetMetadata<_0, _1> {
					pub deposit: _0,
					pub name: _1,
					pub symbol: _1,
					pub decimals: ::core::primitive::u8,
					pub is_frozen: ::core::primitive::bool,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct DestroyWitness {
					#[codec(compact)]
					pub accounts: ::core::primitive::u32,
					#[codec(compact)]
					pub sufficients: ::core::primitive::u32,
					#[codec(compact)]
					pub approvals: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum ExistenceReason<_0> {
					#[codec(index = 0)]
					Consumer,
					#[codec(index = 1)]
					Sufficient,
					#[codec(index = 2)]
					DepositHeld(_0),
					#[codec(index = 3)]
					DepositRefunded,
				}
			}
		}
		pub mod pallet_authorship {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Provide a set of uncles."]
					set_uncles {
						new_uncles: ::std::vec::Vec<
							runtime_types::sp_runtime::generic::header::Header<
								::core::primitive::u32,
								runtime_types::sp_runtime::traits::BlakeTwo256,
							>,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The uncle parent not in the chain."]
					InvalidUncleParent,
					#[codec(index = 1)]
					#[doc = "Uncles already set in the block."]
					UnclesAlreadySet,
					#[codec(index = 2)]
					#[doc = "Too many uncles."]
					TooManyUncles,
					#[codec(index = 3)]
					#[doc = "The uncle is genesis."]
					GenesisUncle,
					#[codec(index = 4)]
					#[doc = "The uncle is too high in chain."]
					TooHighUncle,
					#[codec(index = 5)]
					#[doc = "The uncle is already included."]
					UncleAlreadyIncluded,
					#[codec(index = 6)]
					#[doc = "The uncle isn't recent enough to be included."]
					OldUncle,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum UncleEntryItem<_0, _1, _2> {
				#[codec(index = 0)]
				InclusionHeight(_0),
				#[codec(index = 1)]
				Uncle(_1, ::core::option::Option<_2>),
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer some liquid free balance to another account."]
					#[doc = ""]
					#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
					#[doc = "If the sender's account is below the existential deposit as a result"]
					#[doc = "of the transfer, the account will be reaped."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
					#[doc = "  types. See related functions below."]
					#[doc = "- It contains a limited number of reads and writes internally and no complex"]
					#[doc = "  computation."]
					#[doc = ""]
					#[doc = "Related functions:"]
					#[doc = ""]
					#[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
					#[doc = "  - Transferring balances to accounts that did not exist before will cause"]
					#[doc = "    `T::OnNewAccount::on_new_account` to be called."]
					#[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
					#[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
					#[doc = "    that the transfer will not kill the origin account."]
					#[doc = "---------------------------------"]
					#[doc = "- Origin account is already in memory, so no DB operations for them."]
					#[doc = "# </weight>"]
					transfer {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "Set the balances of a given account."]
					#[doc = ""]
					#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
					#[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
					#[doc = "If the new free or reserved balance is below the existential deposit,"]
					#[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call is `root`."]
					set_balance {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						new_reserved: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
					#[doc = "specified."]
					#[doc = "# <weight>"]
					#[doc = "- Same as transfer, but additional read and write because the source account is not"]
					#[doc = "  assumed to be in the overlay."]
					#[doc = "# </weight>"]
					force_transfer {
						source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
					#[doc = "origin account."]
					#[doc = ""]
					#[doc = "99% of the time you want [`transfer`] instead."]
					#[doc = ""]
					#[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
					transfer_keep_alive {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Transfer the entire transferable balance from the caller account."]
					#[doc = ""]
					#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
					#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
					#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
					#[doc = "you might need to prepare the account by removing any reference counters, storage"]
					#[doc = "deposits, etc..."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be Signed."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
					#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
					#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
					#[doc = "  keep the sender account alive (true). # <weight>"]
					#[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
					#[doc = "  #</weight>"]
					transfer_all {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "Unreserve some balance from a user by force."]
					#[doc = ""]
					#[doc = "Can only be called by ROOT."]
					force_unreserve {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						amount: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Vesting balance too high to send value"]
					VestingBalance,
					#[codec(index = 1)]
					#[doc = "Account liquidity restrictions prevent withdrawal"]
					LiquidityRestrictions,
					#[codec(index = 2)]
					#[doc = "Balance too low to send value."]
					InsufficientBalance,
					#[codec(index = 3)]
					#[doc = "Value too low to create account due to existential deposit"]
					ExistentialDeposit,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account"]
					KeepAlive,
					#[codec(index = 5)]
					#[doc = "A vesting schedule already exists for this account"]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist"]
					DeadAccount,
					#[codec(index = 7)]
					#[doc = "Number of named reserves exceed MaxReserves"]
					TooManyReserves,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance."]
					Endowed {
						account: ::subxt::utils::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
					#[doc = "resulting in an outright loss."]
					DustLost {
						account: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Transfer succeeded."]
					Transfer {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A balance was set by root."]
					BalanceSet {
						who: ::subxt::utils::AccountId32,
						free: ::core::primitive::u128,
						reserved: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					Reserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 5)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					Unreserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 6)]
					#[doc = "Some balance was moved from the reserve of the first account to the second account."]
					#[doc = "Final argument indicates the destination balance type."]
					ReserveRepatriated {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					#[doc = "Some amount was deposited (e.g. for transaction fees)."]
					Deposit { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 8)]
					#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
					Withdraw { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 9)]
					#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
					Slashed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub misc_frozen: _0,
				pub fee_frozen: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct BalanceLock<_0> {
				pub id: [::core::primitive::u8; 8usize],
				pub amount: _0,
				pub reasons: runtime_types::pallet_balances::Reasons,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Reasons {
				#[codec(index = 0)]
				Fee,
				#[codec(index = 1)]
				Misc,
				#[codec(index = 2)]
				All,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Releases {
				#[codec(index = 0)]
				V1_0_0,
				#[codec(index = 1)]
				V2_0_0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ReserveData<_0, _1> {
				pub id: _0,
				pub amount: _1,
			}
		}
		pub mod pallet_collator_selection {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the list of invulnerable (fixed) collators."]
					set_invulnerables { new: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 1)]
					#[doc = "Set the ideal number of collators (not including the invulnerables)."]
					#[doc = "If lowering this number, then the number of running collators could be higher than this figure."]
					#[doc = "Aside from that edge case, there should be no other way to have more collators than the desired number."]
					set_desired_candidates { max: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "Set the candidacy bond amount."]
					set_candidacy_bond { bond: ::core::primitive::u128 },
					#[codec(index = 3)]
					#[doc = "Register this account as a collator candidate. The account must (a) already have"]
					#[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
					#[doc = ""]
					#[doc = "This call is not available to `Invulnerable` collators."]
					register_as_candidate,
					#[codec(index = 4)]
					#[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
					#[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
					#[doc = ""]
					#[doc = "This call will fail if the total number of candidates would drop below `MinCandidates`."]
					#[doc = ""]
					#[doc = "This call is not available to `Invulnerable` collators."]
					leave_intent,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct CandidateInfo<_0, _1> {
					pub who: _0,
					pub deposit: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Too many candidates"]
					TooManyCandidates,
					#[codec(index = 1)]
					#[doc = "Too few candidates"]
					TooFewCandidates,
					#[codec(index = 2)]
					#[doc = "Unknown error"]
					Unknown,
					#[codec(index = 3)]
					#[doc = "Permission issue"]
					Permission,
					#[codec(index = 4)]
					#[doc = "User is already a candidate"]
					AlreadyCandidate,
					#[codec(index = 5)]
					#[doc = "User is not a candidate"]
					NotCandidate,
					#[codec(index = 6)]
					#[doc = "Too many invulnerables"]
					TooManyInvulnerables,
					#[codec(index = 7)]
					#[doc = "User is already an Invulnerable"]
					AlreadyInvulnerable,
					#[codec(index = 8)]
					#[doc = "Account has no associated validator ID"]
					NoAssociatedValidatorId,
					#[codec(index = 9)]
					#[doc = "Validator ID is not yet registered"]
					ValidatorNotRegistered,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
		pub mod pallet_ibc {
			use super::runtime_types;
			pub mod errors {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					deliver { messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any> },
					#[codec(index = 1)]
					transfer {
						params:
							runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
						asset_id: ::core::primitive::u128,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					set_params { params: runtime_types::pallet_ibc::PalletParams },
					#[codec(index = 3)]
					#[doc = "We write the consensus & client state under these predefined paths so that"]
					#[doc = "we can produce state proofs of the values to connected chains"]
					#[doc = "in order to execute client upgrades."]
					upgrade_client { params: runtime_types::pallet_ibc::UpgradeParams },
					#[codec(index = 4)]
					#[doc = "Freeze a client at a specific height"]
					freeze_client {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						height: ::core::primitive::u64,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Errors inform users that something went wrong."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Error processing ibc messages"]
					ProcessingError,
					#[codec(index = 1)]
					#[doc = "Error decoding some type"]
					DecodingError,
					#[codec(index = 2)]
					#[doc = "Error encoding some type"]
					EncodingError,
					#[codec(index = 3)]
					#[doc = "Error generating trie proof"]
					ProofGenerationError,
					#[codec(index = 4)]
					#[doc = "Client consensus state not found for height"]
					ConsensusStateNotFound,
					#[codec(index = 5)]
					#[doc = "Channel not found"]
					ChannelNotFound,
					#[codec(index = 6)]
					#[doc = "Client state not found"]
					ClientStateNotFound,
					#[codec(index = 7)]
					#[doc = "Connection not found"]
					ConnectionNotFound,
					#[codec(index = 8)]
					#[doc = "Packet commitment wasn't found"]
					PacketCommitmentNotFound,
					#[codec(index = 9)]
					#[doc = "Packet receipt wasn't found"]
					PacketReceiptNotFound,
					#[codec(index = 10)]
					#[doc = "Packet Acknowledgment wasn't found"]
					PacketAcknowledgmentNotFound,
					#[codec(index = 11)]
					#[doc = "Error constructing packet"]
					SendPacketError,
					#[codec(index = 12)]
					#[doc = "Invalid channel id"]
					InvalidChannelId,
					#[codec(index = 13)]
					#[doc = "Invalid port id"]
					InvalidPortId,
					#[codec(index = 14)]
					#[doc = "Other forms of errors"]
					Other,
					#[codec(index = 15)]
					#[doc = "Invalid route"]
					InvalidRoute,
					#[codec(index = 16)]
					#[doc = "Invalid message for extrinsic"]
					InvalidMessageType,
					#[codec(index = 17)]
					#[doc = "The interchain token transfer was not successfully initiated"]
					TransferFailed,
					#[codec(index = 18)]
					#[doc = "Error Decoding utf8 bytes"]
					Utf8Error,
					#[codec(index = 19)]
					#[doc = "Invalid asset id"]
					InvalidAssetId,
					#[codec(index = 20)]
					#[doc = "Invalid Ibc denom"]
					InvalidIbcDenom,
					#[codec(index = 21)]
					#[doc = "Invalid amount"]
					InvalidAmount,
					#[codec(index = 22)]
					#[doc = "Invalid timestamp"]
					InvalidTimestamp,
					#[codec(index = 23)]
					#[doc = "Unable to get client revision number"]
					FailedToGetRevisionNumber,
					#[codec(index = 24)]
					#[doc = "Invalid params passed"]
					InvalidParams,
					#[codec(index = 25)]
					#[doc = "Error opening channel"]
					ChannelInitError,
					#[codec(index = 26)]
					#[doc = "Latest height and timestamp for a client not found"]
					TimestampAndHeightNotFound,
					#[codec(index = 27)]
					#[doc = "Failed to derive channel escrow address"]
					ChannelEscrowAddress,
					#[codec(index = 28)]
					#[doc = "Error writing acknowledgement to storage"]
					WriteAckError,
					#[codec(index = 29)]
					#[doc = "Client update time and height not found"]
					ClientUpdateNotFound,
					#[codec(index = 30)]
					#[doc = "Error Freezing client"]
					ClientFreezeFailed,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Events emitted by the ibc subsystem"]
					Events {
						events: ::std::vec::Vec<
							::core::result::Result<
								runtime_types::pallet_ibc::events::IbcEvent,
								runtime_types::pallet_ibc::errors::IbcError,
							>,
						>,
					},
					#[codec(index = 1)]
					#[doc = "An Ibc token transfer has been started"]
					TokenTransferInitiated {
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id: ::core::option::Option<::core::primitive::u128>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 2)]
					#[doc = "A channel has been opened"]
					ChannelOpened {
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 3)]
					#[doc = "Pallet params updated"]
					ParamsUpdated {
						send_enabled: ::core::primitive::bool,
						receive_enabled: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					#[doc = "An outgoing Ibc token transfer has been completed and burnt"]
					TokenTransferCompleted {
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id: ::core::option::Option<::core::primitive::u128>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 5)]
					#[doc = "Ibc tokens have been received and minted"]
					TokenReceived {
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id: ::core::option::Option<::core::primitive::u128>,
						amount: ::core::primitive::u128,
						is_receiver_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 6)]
					#[doc = "Ibc transfer failed, received an acknowledgement error, tokens have been refunded"]
					TokenTransferFailed {
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id: ::core::option::Option<::core::primitive::u128>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 7)]
					#[doc = "On recv packet was not processed successfully processes"]
					OnRecvPacketError { msg: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 8)]
					#[doc = "Client upgrade path has been set"]
					ClientUpgradeSet,
					#[codec(index = 9)]
					#[doc = "Client has been frozen"]
					ClientFrozen {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
					},
					#[codec(index = 10)]
					#[doc = "Asset Admin Account Updated"]
					AssetAdminUpdated { admin_account: ::subxt::utils::AccountId32 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Any {
				pub type_url: ::std::vec::Vec<::core::primitive::u8>,
				pub value: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum MultiAddress<_0> {
				#[codec(index = 0)]
				Id(_0),
				#[codec(index = 1)]
				Raw(::std::vec::Vec<::core::primitive::u8>),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct PalletParams {
				pub send_enabled: ::core::primitive::bool,
				pub receive_enabled: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferParams<_0> {
				pub to: runtime_types::pallet_ibc::MultiAddress<_0>,
				pub source_channel: ::core::primitive::u64,
				pub timeout: runtime_types::ibc_primitives::Timeout,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct UpgradeParams {
				pub client_state: ::std::vec::Vec<::core::primitive::u8>,
				pub consensus_state: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod pallet_ibc_ping {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					send_ping { params: runtime_types::pallet_ibc_ping::SendPingParams },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Invalid params passed"]
					InvalidParams,
					#[codec(index = 1)]
					#[doc = "Error opening channel"]
					ChannelInitError,
					#[codec(index = 2)]
					#[doc = "Error registering packet"]
					PacketSendError,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A send packet has been registered"]
					PacketSent,
					#[codec(index = 1)]
					#[doc = "A channel has been opened"]
					ChannelOpened {
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SendPingParams {
				pub data: ::std::vec::Vec<::core::primitive::u8>,
				pub timeout_height_offset: ::core::primitive::u64,
				pub timeout_timestamp_offset: ::core::primitive::u64,
				pub channel_id: ::core::primitive::u64,
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Sets the session key(s) of the function caller to `keys`."]
					#[doc = "Allows an account to set its session key prior to becoming a validator."]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be signed."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
					#[doc = "  `T::Keys::key_ids()` which is fixed."]
					#[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
					#[doc = "- DbWrites: `origin account`, `NextKeys`"]
					#[doc = "- DbReads per key id: `KeyOwner`"]
					#[doc = "- DbWrites per key id: `KeyOwner`"]
					#[doc = "# </weight>"]
					set_keys {
						keys: runtime_types::parachain_runtime::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "Removes any session key(s) of the function caller."]
					#[doc = ""]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
					#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
					#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
					#[doc = "usually means being a stash account)."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
					#[doc = "  of `T::Keys::key_ids()` which is fixed."]
					#[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
					#[doc = "- DbWrites: `NextKeys`, `origin account`"]
					#[doc = "- DbWrites per key id: `KeyOwner`"]
					#[doc = "# </weight>"]
					purge_keys,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Error for the session pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Invalid ownership proof."]
					InvalidProof,
					#[codec(index = 1)]
					#[doc = "No associated validator ID for account."]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					#[doc = "Registered duplicate key."]
					DuplicatedKey,
					#[codec(index = 3)]
					#[doc = "No keys are associated with this account."]
					NoKeys,
					#[codec(index = 4)]
					#[doc = "Key setting account is not live, so it's impossible to associate keys."]
					NoAccount,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New session has happened. Note that the argument is the session index, not the"]
					#[doc = "block number as the type might suggest."]
					NewSession { session_index: ::core::primitive::u32 },
				}
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB write (event)."]
					#[doc = "- Weight of derivative `call` execution + 10,000."]
					#[doc = "# </weight>"]
					sudo { call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall> },
					#[codec(index = 1)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					#[doc = "This function does not check the weight of the call, and instead allows the"]
					#[doc = "Sudo user to specify the weight of the call."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- The weight of this call is defined by the caller."]
					#[doc = "# </weight>"]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
					#[doc = "key."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB change."]
					#[doc = "# </weight>"]
					set_key { new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> },
					#[codec(index = 3)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
					#[doc = "a given account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB write (event)."]
					#[doc = "- Weight of derivative `call` execution + 10,000."]
					#[doc = "# </weight>"]
					sudo_as {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Error for the Sudo pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Sender must be the Sudo account"]
					RequireSudo,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A sudo just took place. \\[result\\]"]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
					KeyChanged { old_sudoer: ::core::option::Option<::subxt::utils::AccountId32> },
					#[codec(index = 2)]
					#[doc = "A sudo just took place. \\[result\\]"]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the current time."]
					#[doc = ""]
					#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
					#[doc = "phase, if this call hasn't been invoked by that time."]
					#[doc = ""]
					#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
					#[doc = "`MinimumPeriod`."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Inherent`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
					#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
					#[doc = "  `on_finalize`)"]
					#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
					#[doc = "# </weight>"]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
					#[doc = "has been paid by `who`."]
					TransactionFeePaid {
						who: ::subxt::utils::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Releases {
				#[codec(index = 0)]
				V1Ancient,
				#[codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					send {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
					},
					#[codec(index = 1)]
					#[doc = "Teleport some assets from the local chain to some destination chain."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
					#[doc = "with all fees taken as needed from the asset."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
					#[doc = "  `dest` side. May not be empty."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
					#[doc = "chain and forward a notification XCM."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
					#[doc = "with all fees taken as needed from the asset."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
					#[doc = "  `dest` side."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Execute an XCM message from a local, signed, origin."]
					#[doc = ""]
					#[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
					#[doc = "partially."]
					#[doc = ""]
					#[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than the"]
					#[doc = "maximum amount of weight that the message could take to be executed, then no execution"]
					#[doc = "attempt will be made."]
					#[doc = ""]
					#[doc = "NOTE: A successful return to this does *not* imply that the `msg` was executed successfully"]
					#[doc = "to completion; only that *some* of it was executed."]
					execute {
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					#[doc = "Extoll that a particular destination can be communicated with through a particular"]
					#[doc = "version of XCM."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `location`: The destination that is being described."]
					#[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
					force_xcm_version {
						location:
							::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
						xcm_version: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
					#[doc = "version a destination can accept is unknown)."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
					force_default_xcm_version {
						maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 6)]
					#[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
					force_subscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 7)]
					#[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
					#[doc = "version changes."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
					#[doc = "  notifications which we no longer desire."]
					force_unsubscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 8)]
					#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
					#[doc = "chain and forward a notification XCM."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
					#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
					#[doc = "at risk."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
					#[doc = "  `dest` side."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
					limited_reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 9)]
					#[doc = "Teleport some assets from the local chain to some destination chain."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
					#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
					#[doc = "at risk."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
					#[doc = "  `dest` side. May not be empty."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
					limited_teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The desired destination was unreachable, generally because there is a no way of routing"]
					#[doc = "to it."]
					Unreachable,
					#[codec(index = 1)]
					#[doc = "There was some other issue (i.e. not to do with routing) in sending the message. Perhaps"]
					#[doc = "a lack of space for buffering the message."]
					SendFailure,
					#[codec(index = 2)]
					#[doc = "The message execution fails the filter."]
					Filtered,
					#[codec(index = 3)]
					#[doc = "The message's weight could not be determined."]
					UnweighableMessage,
					#[codec(index = 4)]
					#[doc = "The destination `MultiLocation` provided cannot be inverted."]
					DestinationNotInvertible,
					#[codec(index = 5)]
					#[doc = "The assets to be sent are empty."]
					Empty,
					#[codec(index = 6)]
					#[doc = "Could not re-anchor the assets to declare the fees for the destination chain."]
					CannotReanchor,
					#[codec(index = 7)]
					#[doc = "Too many assets have been attempted for transfer."]
					TooManyAssets,
					#[codec(index = 8)]
					#[doc = "Origin is invalid for sending."]
					InvalidOrigin,
					#[codec(index = 9)]
					#[doc = "The version of the `Versioned` value used is not able to be interpreted."]
					BadVersion,
					#[codec(index = 10)]
					#[doc = "The given location could not be used (e.g. because it cannot be expressed in the"]
					#[doc = "desired version of XCM)."]
					BadLocation,
					#[codec(index = 11)]
					#[doc = "The referenced subscription could not be found."]
					NoSubscription,
					#[codec(index = 12)]
					#[doc = "The location is invalid since it already has a subscription from us."]
					AlreadySubscribed,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Execution of an XCM message was attempted."]
					#[doc = ""]
					#[doc = "\\[ outcome \\]"]
					Attempted(runtime_types::xcm::v2::traits::Outcome),
					#[codec(index = 1)]
					#[doc = "A XCM message was sent."]
					#[doc = ""]
					#[doc = "\\[ origin, destination, message \\]"]
					Sent(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::v2::Xcm,
					),
					#[codec(index = 2)]
					#[doc = "Query response received which does not match a registered query. This may be because a"]
					#[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
					#[doc = "because the query timed out."]
					#[doc = ""]
					#[doc = "\\[ origin location, id \\]"]
					UnexpectedResponse(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 3)]
					#[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
					#[doc = "no registered notification call."]
					#[doc = ""]
					#[doc = "\\[ id, response \\]"]
					ResponseReady(::core::primitive::u64, runtime_types::xcm::v2::Response),
					#[codec(index = 4)]
					#[doc = "Query response has been received and query is removed. The registered notification has"]
					#[doc = "been dispatched and executed successfully."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index \\]"]
					Notified(::core::primitive::u64, ::core::primitive::u8, ::core::primitive::u8),
					#[codec(index = 5)]
					#[doc = "Query response has been received and query is removed. The registered notification could"]
					#[doc = "not be dispatched because the dispatch weight is greater than the maximum weight"]
					#[doc = "originally budgeted by this runtime for the query result."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index, actual weight, max budgeted weight \\]"]
					NotifyOverweight(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
						runtime_types::sp_weights::weight_v2::Weight,
						runtime_types::sp_weights::weight_v2::Weight,
					),
					#[codec(index = 6)]
					#[doc = "Query response has been received and query is removed. There was a general error with"]
					#[doc = "dispatching the notification call."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index \\]"]
					NotifyDispatchError(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 7)]
					#[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
					#[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
					#[doc = "is not `(origin, QueryId, Response)`."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index \\]"]
					NotifyDecodeFailed(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 8)]
					#[doc = "Expected query response has been received but the origin location of the response does"]
					#[doc = "not match that expected. The query remains registered for a later, valid, response to"]
					#[doc = "be received and acted upon."]
					#[doc = ""]
					#[doc = "\\[ origin location, id, expected location \\]"]
					InvalidResponder(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
						::core::option::Option<
							runtime_types::xcm::v1::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 9)]
					#[doc = "Expected query response has been received but the expected origin location placed in"]
					#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
					#[doc = ""]
					#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
					#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
					#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
					#[doc = "needed."]
					#[doc = ""]
					#[doc = "\\[ origin location, id \\]"]
					InvalidResponderVersion(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 10)]
					#[doc = "Received query response has been read and removed."]
					#[doc = ""]
					#[doc = "\\[ id \\]"]
					ResponseTaken(::core::primitive::u64),
					#[codec(index = 11)]
					#[doc = "Some assets have been placed in an asset trap."]
					#[doc = ""]
					#[doc = "\\[ hash, origin, assets \\]"]
					AssetsTrapped(
						::subxt::utils::H256,
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
					#[codec(index = 12)]
					#[doc = "An XCM version change notification message has been attempted to be sent."]
					#[doc = ""]
					#[doc = "\\[ destination, result \\]"]
					VersionChangeNotified(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec(index = 13)]
					#[doc = "The supported version of a location has been changed. This might be through an"]
					#[doc = "automatic notification or a manual intervention."]
					#[doc = ""]
					#[doc = "\\[ location, XCM version \\]"]
					SupportedVersionChanged(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec(index = 14)]
					#[doc = "A given location which had a version change subscription was dropped owing to an error"]
					#[doc = "sending the notification to it."]
					#[doc = ""]
					#[doc = "\\[ location, query ID, error \\]"]
					NotifyTargetSendFail(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v2::traits::Error,
					),
					#[codec(index = 15)]
					#[doc = "A given location which had a version change subscription was dropped owing to an error"]
					#[doc = "migrating the location to our new XCM format."]
					#[doc = ""]
					#[doc = "\\[ location, query ID \\]"]
					NotifyTargetMigrationFail(
						runtime_types::xcm::VersionedMultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 16)]
					#[doc = "Some assets have been claimed from an asset trap"]
					#[doc = ""]
					#[doc = "\\[ hash, origin, assets \\]"]
					AssetsClaimed(
						::subxt::utils::H256,
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum QueryStatus<_0> {
					#[codec(index = 0)]
					Pending {
						responder: runtime_types::xcm::VersionedMultiLocation,
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {}
			}
		}
		pub mod parachain_runtime {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Runtime;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum RuntimeCall {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 2)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
				#[codec(index = 3)]
				ParachainInfo(runtime_types::parachain_info::pallet::Call),
				#[codec(index = 10)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 20)]
				Authorship(runtime_types::pallet_authorship::pallet::Call),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 30)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
				#[codec(index = 31)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
				#[codec(index = 32)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
				#[codec(index = 33)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
				#[codec(index = 35)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
				#[codec(index = 36)]
				IbcPing(runtime_types::pallet_ibc_ping::pallet::Call),
				#[codec(index = 37)]
				Assets(runtime_types::pallet_assets::pallet::Call),
				#[codec(index = 38)]
				AssetRegistry(runtime_types::orml_asset_registry::module::Call),
				#[codec(index = 255)]
				Ibc(runtime_types::pallet_ibc::pallet::Call),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum RuntimeEvent {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 2)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
				#[codec(index = 10)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 11)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 30)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
				#[codec(index = 31)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
				#[codec(index = 32)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
				#[codec(index = 33)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
				#[codec(index = 35)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
				#[codec(index = 36)]
				IbcPing(runtime_types::pallet_ibc_ping::pallet::Event),
				#[codec(index = 37)]
				Assets(runtime_types::pallet_assets::pallet::Event),
				#[codec(index = 38)]
				AssetRegistry(runtime_types::orml_asset_registry::module::Event),
				#[codec(index = 255)]
				Ibc(runtime_types::pallet_ibc::pallet::Event),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SessionKeys {
				pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
			}
		}
		pub mod polkadot_core_primitives {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct Id(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct AbridgedHrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<::subxt::utils::H256>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct PersistedValidationData<_0, _1> {
					pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
					pub relay_parent_number: _1,
					pub relay_parent_storage_root: _0,
					pub max_pov_size: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum UpgradeRestriction {
					#[codec(index = 0)]
					Present,
				}
			}
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct Perbill(pub ::core::primitive::u32);
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
						Debug,
					)]
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
				Debug,
			)]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod bounded {
				use super::runtime_types;
				pub mod bounded_btree_set {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct BoundedBTreeSet<_0>(pub ::std::vec::Vec<_0>);
				}
				pub mod bounded_vec {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
				}
				pub mod weak_bounded_vec {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
				}
			}
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod sr25519 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
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
						Debug,
					)]
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
						Debug,
					)]
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
						Debug,
					)]
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
						Debug,
					)]
					pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
						pub ::std::vec::Vec<::core::primitive::u8>,
						#[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>,
					);
				}
			}
			pub mod traits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BlakeTwo256;
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				Arithmetic(runtime_types::sp_runtime::ArithmeticError),
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
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Signature),
				#[codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Signature),
				#[codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Signature),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct StorageProof {
					pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Weight {
					#[codec(compact)]
					pub ref_time: ::core::primitive::u64,
					#[codec(compact)]
					pub proof_size: ::core::primitive::u64,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct DoubleEncoded {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v0 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum BodyId {
						#[codec(index = 0)]
						Unit,
						#[codec(index = 1)]
						Named(
							runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
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
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
						Debug,
					)]
					pub enum Junction {
						#[codec(index = 0)]
						Parent,
						#[codec(index = 1)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 2)]
						AccountId32 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 3)]
						AccountIndex64 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 4)]
						AccountKey20 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 5)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 6)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 7)]
						GeneralKey(
							runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 8)]
						OnlyChild,
						#[codec(index = 9)]
						Plurality {
							id: runtime_types::xcm::v0::junction::BodyId,
							part: runtime_types::xcm::v0::junction::BodyPart,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum NetworkId {
						#[codec(index = 0)]
						Any,
						#[codec(index = 1)]
						Named(
							runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
					}
				}
				pub mod multi_asset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum MultiAsset {
						#[codec(index = 0)]
						None,
						#[codec(index = 1)]
						All,
						#[codec(index = 2)]
						AllFungible,
						#[codec(index = 3)]
						AllNonFungible,
						#[codec(index = 4)]
						AllAbstractFungible { id: ::std::vec::Vec<::core::primitive::u8> },
						#[codec(index = 5)]
						AllAbstractNonFungible { class: ::std::vec::Vec<::core::primitive::u8> },
						#[codec(index = 6)]
						AllConcreteFungible {
							id: runtime_types::xcm::v0::multi_location::MultiLocation,
						},
						#[codec(index = 7)]
						AllConcreteNonFungible {
							class: runtime_types::xcm::v0::multi_location::MultiLocation,
						},
						#[codec(index = 8)]
						AbstractFungible {
							id: ::std::vec::Vec<::core::primitive::u8>,
							#[codec(compact)]
							amount: ::core::primitive::u128,
						},
						#[codec(index = 9)]
						AbstractNonFungible {
							class: ::std::vec::Vec<::core::primitive::u8>,
							instance: runtime_types::xcm::v1::multiasset::AssetInstance,
						},
						#[codec(index = 10)]
						ConcreteFungible {
							id: runtime_types::xcm::v0::multi_location::MultiLocation,
							#[codec(compact)]
							amount: ::core::primitive::u128,
						},
						#[codec(index = 11)]
						ConcreteNonFungible {
							class: runtime_types::xcm::v0::multi_location::MultiLocation,
							instance: runtime_types::xcm::v1::multiasset::AssetInstance,
						},
					}
				}
				pub mod multi_location {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum MultiLocation {
						#[codec(index = 0)]
						Null,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v0::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
					}
				}
				pub mod order {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Order {
						#[codec(index = 0)]
						Null,
						#[codec(index = 1)]
						DepositAsset {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
						},
						#[codec(index = 2)]
						DepositReserveAsset {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
						},
						#[codec(index = 3)]
						ExchangeAsset {
							give: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							receive:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						},
						#[codec(index = 4)]
						InitiateReserveWithdraw {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							reserve: runtime_types::xcm::v0::multi_location::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
						},
						#[codec(index = 5)]
						InitiateTeleport {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
						},
						#[codec(index = 6)]
						QueryHolding {
							#[codec(compact)]
							query_id: ::core::primitive::u64,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						},
						#[codec(index = 7)]
						BuyExecution {
							fees: runtime_types::xcm::v0::multi_asset::MultiAsset,
							weight: ::core::primitive::u64,
							debt: ::core::primitive::u64,
							halt_on_error: ::core::primitive::bool,
							xcm: ::std::vec::Vec<runtime_types::xcm::v0::Xcm>,
						},
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Response {
					#[codec(index = 0)]
					Assets(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Xcm {
					#[codec(index = 0)]
					WithdrawAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 1)]
					ReserveAssetDeposit {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 2)]
					TeleportAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v0::Response,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						dest: runtime_types::xcm::v0::multi_location::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						dest: runtime_types::xcm::v0::multi_location::MultiLocation,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v0::OriginKind,
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
					RelayedFrom {
						who: runtime_types::xcm::v0::multi_location::MultiLocation,
						message: ::std::boxed::Box<runtime_types::xcm::v0::Xcm>,
					},
				}
			}
			pub mod v1 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey(
							runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v0::junction::BodyId,
							part: runtime_types::xcm::v0::junction::BodyPart,
						},
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v1::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
						Debug,
					)]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v1::multiasset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v1::multiasset::AssetId,
						pub fun: runtime_types::xcm::v1::multiasset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v1::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v1::multiasset::WildMultiAsset),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v1::multiasset::MultiAsset>,
					);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v1::multiasset::AssetId,
							fun: runtime_types::xcm::v1::multiasset::WildFungibility,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v1::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v1::multilocation::Junctions,
					}
				}
				pub mod order {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Order {
						#[codec(index = 0)]
						Noop,
						#[codec(index = 1)]
						DepositAsset {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							max_assets: ::core::primitive::u32,
							beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
						},
						#[codec(index = 2)]
						DepositReserveAsset {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							max_assets: ::core::primitive::u32,
							dest: runtime_types::xcm::v1::multilocation::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
						},
						#[codec(index = 3)]
						ExchangeAsset {
							give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							receive: runtime_types::xcm::v1::multiasset::MultiAssets,
						},
						#[codec(index = 4)]
						InitiateReserveWithdraw {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
						},
						#[codec(index = 5)]
						InitiateTeleport {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							dest: runtime_types::xcm::v1::multilocation::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
						},
						#[codec(index = 6)]
						QueryHolding {
							#[codec(compact)]
							query_id: ::core::primitive::u64,
							dest: runtime_types::xcm::v1::multilocation::MultiLocation,
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						},
						#[codec(index = 7)]
						BuyExecution {
							fees: runtime_types::xcm::v1::multiasset::MultiAsset,
							weight: ::core::primitive::u64,
							debt: ::core::primitive::u64,
							halt_on_error: ::core::primitive::bool,
							instructions: ::std::vec::Vec<runtime_types::xcm::v1::Xcm>,
						},
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Response {
					#[codec(index = 0)]
					Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 1)]
					Version(::core::primitive::u32),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Xcm {
					#[codec(index = 0)]
					WithdrawAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 1)]
					ReserveAssetDeposited {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 2)]
					ReceiveTeleportedAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v1::Response,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v0::OriginKind,
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
					RelayedFrom {
						who: runtime_types::xcm::v1::multilocation::Junctions,
						message: ::std::boxed::Box<runtime_types::xcm::v1::Xcm>,
					},
					#[codec(index = 11)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 12)]
					UnsubscribeVersion,
				}
			}
			pub mod v2 {
				use super::runtime_types;
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Outcome {
						#[codec(index = 0)]
						Complete(::core::primitive::u64),
						#[codec(index = 1)]
						Incomplete(::core::primitive::u64, runtime_types::xcm::v2::traits::Error),
						#[codec(index = 2)]
						Error(runtime_types::xcm::v2::traits::Error),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
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
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v0::OriginKind,
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
					DescendOrigin(runtime_types::xcm::v1::multilocation::Junctions),
					#[codec(index = 12)]
					ReportError {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						receive: runtime_types::xcm::v1::multiasset::MultiAssets,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 18)]
					QueryHolding {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v1::multiasset::MultiAsset,
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
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v1::multilocation::MultiLocation,
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
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
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(#[codec(compact)] ::core::primitive::u64),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedMultiAssets {
				#[codec(index = 0)]
				V0(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::multiasset::MultiAssets),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedMultiLocation {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::multi_location::MultiLocation),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::multilocation::MultiLocation),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedResponse {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::Response),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::Response),
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Response),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedXcm {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::Xcm),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::Xcm),
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Xcm),
			}
		}
	}
	#[doc = r" The default error type returned when there is a runtime issue,"]
	#[doc = r" exposed here for ease of use."]
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
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
		pub fn balances(&self) -> balances::constants::ConstantsApi {
			balances::constants::ConstantsApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
			transaction_payment::constants::ConstantsApi
		}
		pub fn authorship(&self) -> authorship::constants::ConstantsApi {
			authorship::constants::ConstantsApi
		}
		pub fn assets(&self) -> assets::constants::ConstantsApi {
			assets::constants::ConstantsApi
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
		pub fn parachain_system(&self) -> parachain_system::storage::StorageApi {
			parachain_system::storage::StorageApi
		}
		pub fn parachain_info(&self) -> parachain_info::storage::StorageApi {
			parachain_info::storage::StorageApi
		}
		pub fn balances(&self) -> balances::storage::StorageApi {
			balances::storage::StorageApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
			transaction_payment::storage::StorageApi
		}
		pub fn authorship(&self) -> authorship::storage::StorageApi {
			authorship::storage::StorageApi
		}
		pub fn collator_selection(&self) -> collator_selection::storage::StorageApi {
			collator_selection::storage::StorageApi
		}
		pub fn session(&self) -> session::storage::StorageApi {
			session::storage::StorageApi
		}
		pub fn aura(&self) -> aura::storage::StorageApi {
			aura::storage::StorageApi
		}
		pub fn aura_ext(&self) -> aura_ext::storage::StorageApi {
			aura_ext::storage::StorageApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::storage::StorageApi {
			xcmp_queue::storage::StorageApi
		}
		pub fn polkadot_xcm(&self) -> polkadot_xcm::storage::StorageApi {
			polkadot_xcm::storage::StorageApi
		}
		pub fn dmp_queue(&self) -> dmp_queue::storage::StorageApi {
			dmp_queue::storage::StorageApi
		}
		pub fn sudo(&self) -> sudo::storage::StorageApi {
			sudo::storage::StorageApi
		}
		pub fn assets(&self) -> assets::storage::StorageApi {
			assets::storage::StorageApi
		}
		pub fn asset_registry(&self) -> asset_registry::storage::StorageApi {
			asset_registry::storage::StorageApi
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
		pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi {
			parachain_system::calls::TransactionApi
		}
		pub fn parachain_info(&self) -> parachain_info::calls::TransactionApi {
			parachain_info::calls::TransactionApi
		}
		pub fn balances(&self) -> balances::calls::TransactionApi {
			balances::calls::TransactionApi
		}
		pub fn authorship(&self) -> authorship::calls::TransactionApi {
			authorship::calls::TransactionApi
		}
		pub fn collator_selection(&self) -> collator_selection::calls::TransactionApi {
			collator_selection::calls::TransactionApi
		}
		pub fn session(&self) -> session::calls::TransactionApi {
			session::calls::TransactionApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::calls::TransactionApi {
			xcmp_queue::calls::TransactionApi
		}
		pub fn polkadot_xcm(&self) -> polkadot_xcm::calls::TransactionApi {
			polkadot_xcm::calls::TransactionApi
		}
		pub fn cumulus_xcm(&self) -> cumulus_xcm::calls::TransactionApi {
			cumulus_xcm::calls::TransactionApi
		}
		pub fn dmp_queue(&self) -> dmp_queue::calls::TransactionApi {
			dmp_queue::calls::TransactionApi
		}
		pub fn sudo(&self) -> sudo::calls::TransactionApi {
			sudo::calls::TransactionApi
		}
		pub fn ibc_ping(&self) -> ibc_ping::calls::TransactionApi {
			ibc_ping::calls::TransactionApi
		}
		pub fn assets(&self) -> assets::calls::TransactionApi {
			assets::calls::TransactionApi
		}
		pub fn asset_registry(&self) -> asset_registry::calls::TransactionApi {
			asset_registry::calls::TransactionApi
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
				105u8, 174u8, 59u8, 212u8, 90u8, 228u8, 78u8, 23u8, 80u8, 88u8, 127u8, 13u8, 180u8,
				80u8, 93u8, 193u8, 157u8, 237u8, 214u8, 13u8, 122u8, 129u8, 150u8, 178u8, 229u8,
				145u8, 181u8, 231u8, 90u8, 169u8, 204u8, 246u8,
			] {
			Err(::subxt::error::MetadataError::IncompatibleMetadata)
		} else {
			Ok(())
		}
	}
}
