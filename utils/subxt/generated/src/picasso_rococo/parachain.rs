#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]

pub mod api {
	#[allow(unused_imports)]
	mod root_mod {
		pub use super::*;
	}
	pub static PALLETS: [&str; 50usize] = [
		"System",
		"Timestamp",
		"Sudo",
		"TransactionPayment",
		"AssetTxPayment",
		"Indices",
		"Balances",
		"Identity",
		"Multisig",
		"ParachainSystem",
		"ParachainInfo",
		"Authorship",
		"CollatorSelection",
		"Session",
		"Aura",
		"AuraExt",
		"Council",
		"CouncilMembership",
		"Treasury",
		"Democracy",
		"TechnicalCommittee",
		"TechnicalCommitteeMembership",
		"ReleaseCommittee",
		"ReleaseMembership",
		"Scheduler",
		"Utility",
		"Preimage",
		"Proxy",
		"XcmpQueue",
		"PolkadotXcm",
		"CumulusXcm",
		"DmpQueue",
		"XTokens",
		"UnknownTokens",
		"Tokens",
		"CurrencyFactory",
		"CrowdloanRewards",
		"Vesting",
		"BondedFinance",
		"AssetsRegistry",
		"Pablo",
		"Oracle",
		"AssetsTransactorRouter",
		"FarmingRewards",
		"Farming",
		"CallFilter",
		"Cosmwasm",
		"Ibc",
		"Ics20Fee",
		"PalletMultihopXcmIbc",
	];
	pub static RUNTIME_APIS: [&str; 0usize] = [];
	#[doc = r" The error type returned when there is a runtime issue."]
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
	#[doc = r" The outer event enum."]
	pub type Event = runtime_types::picasso_runtime::RuntimeEvent;
	#[doc = r" The outer extrinsic enum."]
	pub type Call = runtime_types::picasso_runtime::RuntimeCall;
	#[doc = r" The outer error enum representing the DispatchError's Module variant."]
	pub type Error = runtime_types::picasso_runtime::RuntimeError;
	pub fn constants() -> ConstantsApi {
		ConstantsApi
	}
	pub fn storage() -> StorageApi {
		StorageApi
	}
	pub fn tx() -> TransactionApi {
		TransactionApi
	}
	pub fn apis() -> runtime_apis::RuntimeApi {
		runtime_apis::RuntimeApi
	}
	pub mod runtime_apis {
		use super::{root_mod, runtime_types};
		use ::subxt::ext::codec::Encode;
		pub struct RuntimeApi;
		impl RuntimeApi {}
	}
	pub struct ConstantsApi;
	impl ConstantsApi {
		pub fn system(&self) -> system::constants::ConstantsApi {
			system::constants::ConstantsApi
		}
		pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
			timestamp::constants::ConstantsApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
			transaction_payment::constants::ConstantsApi
		}
		pub fn asset_tx_payment(&self) -> asset_tx_payment::constants::ConstantsApi {
			asset_tx_payment::constants::ConstantsApi
		}
		pub fn indices(&self) -> indices::constants::ConstantsApi {
			indices::constants::ConstantsApi
		}
		pub fn balances(&self) -> balances::constants::ConstantsApi {
			balances::constants::ConstantsApi
		}
		pub fn identity(&self) -> identity::constants::ConstantsApi {
			identity::constants::ConstantsApi
		}
		pub fn multisig(&self) -> multisig::constants::ConstantsApi {
			multisig::constants::ConstantsApi
		}
		pub fn treasury(&self) -> treasury::constants::ConstantsApi {
			treasury::constants::ConstantsApi
		}
		pub fn democracy(&self) -> democracy::constants::ConstantsApi {
			democracy::constants::ConstantsApi
		}
		pub fn scheduler(&self) -> scheduler::constants::ConstantsApi {
			scheduler::constants::ConstantsApi
		}
		pub fn utility(&self) -> utility::constants::ConstantsApi {
			utility::constants::ConstantsApi
		}
		pub fn proxy(&self) -> proxy::constants::ConstantsApi {
			proxy::constants::ConstantsApi
		}
		pub fn x_tokens(&self) -> x_tokens::constants::ConstantsApi {
			x_tokens::constants::ConstantsApi
		}
		pub fn tokens(&self) -> tokens::constants::ConstantsApi {
			tokens::constants::ConstantsApi
		}
		pub fn crowdloan_rewards(&self) -> crowdloan_rewards::constants::ConstantsApi {
			crowdloan_rewards::constants::ConstantsApi
		}
		pub fn vesting(&self) -> vesting::constants::ConstantsApi {
			vesting::constants::ConstantsApi
		}
		pub fn bonded_finance(&self) -> bonded_finance::constants::ConstantsApi {
			bonded_finance::constants::ConstantsApi
		}
		pub fn assets_registry(&self) -> assets_registry::constants::ConstantsApi {
			assets_registry::constants::ConstantsApi
		}
		pub fn pablo(&self) -> pablo::constants::ConstantsApi {
			pablo::constants::ConstantsApi
		}
		pub fn oracle(&self) -> oracle::constants::ConstantsApi {
			oracle::constants::ConstantsApi
		}
		pub fn assets_transactor_router(
			&self,
		) -> assets_transactor_router::constants::ConstantsApi {
			assets_transactor_router::constants::ConstantsApi
		}
		pub fn farming_rewards(&self) -> farming_rewards::constants::ConstantsApi {
			farming_rewards::constants::ConstantsApi
		}
		pub fn farming(&self) -> farming::constants::ConstantsApi {
			farming::constants::ConstantsApi
		}
		pub fn call_filter(&self) -> call_filter::constants::ConstantsApi {
			call_filter::constants::ConstantsApi
		}
		pub fn cosmwasm(&self) -> cosmwasm::constants::ConstantsApi {
			cosmwasm::constants::ConstantsApi
		}
		pub fn ibc(&self) -> ibc::constants::ConstantsApi {
			ibc::constants::ConstantsApi
		}
		pub fn ics20_fee(&self) -> ics20_fee::constants::ConstantsApi {
			ics20_fee::constants::ConstantsApi
		}
		pub fn pallet_multihop_xcm_ibc(&self) -> pallet_multihop_xcm_ibc::constants::ConstantsApi {
			pallet_multihop_xcm_ibc::constants::ConstantsApi
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
		pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
			transaction_payment::storage::StorageApi
		}
		pub fn asset_tx_payment(&self) -> asset_tx_payment::storage::StorageApi {
			asset_tx_payment::storage::StorageApi
		}
		pub fn indices(&self) -> indices::storage::StorageApi {
			indices::storage::StorageApi
		}
		pub fn balances(&self) -> balances::storage::StorageApi {
			balances::storage::StorageApi
		}
		pub fn identity(&self) -> identity::storage::StorageApi {
			identity::storage::StorageApi
		}
		pub fn multisig(&self) -> multisig::storage::StorageApi {
			multisig::storage::StorageApi
		}
		pub fn parachain_system(&self) -> parachain_system::storage::StorageApi {
			parachain_system::storage::StorageApi
		}
		pub fn parachain_info(&self) -> parachain_info::storage::StorageApi {
			parachain_info::storage::StorageApi
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
		pub fn council(&self) -> council::storage::StorageApi {
			council::storage::StorageApi
		}
		pub fn council_membership(&self) -> council_membership::storage::StorageApi {
			council_membership::storage::StorageApi
		}
		pub fn treasury(&self) -> treasury::storage::StorageApi {
			treasury::storage::StorageApi
		}
		pub fn democracy(&self) -> democracy::storage::StorageApi {
			democracy::storage::StorageApi
		}
		pub fn technical_committee(&self) -> technical_committee::storage::StorageApi {
			technical_committee::storage::StorageApi
		}
		pub fn technical_committee_membership(
			&self,
		) -> technical_committee_membership::storage::StorageApi {
			technical_committee_membership::storage::StorageApi
		}
		pub fn release_committee(&self) -> release_committee::storage::StorageApi {
			release_committee::storage::StorageApi
		}
		pub fn release_membership(&self) -> release_membership::storage::StorageApi {
			release_membership::storage::StorageApi
		}
		pub fn scheduler(&self) -> scheduler::storage::StorageApi {
			scheduler::storage::StorageApi
		}
		pub fn preimage(&self) -> preimage::storage::StorageApi {
			preimage::storage::StorageApi
		}
		pub fn proxy(&self) -> proxy::storage::StorageApi {
			proxy::storage::StorageApi
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
		pub fn unknown_tokens(&self) -> unknown_tokens::storage::StorageApi {
			unknown_tokens::storage::StorageApi
		}
		pub fn tokens(&self) -> tokens::storage::StorageApi {
			tokens::storage::StorageApi
		}
		pub fn currency_factory(&self) -> currency_factory::storage::StorageApi {
			currency_factory::storage::StorageApi
		}
		pub fn crowdloan_rewards(&self) -> crowdloan_rewards::storage::StorageApi {
			crowdloan_rewards::storage::StorageApi
		}
		pub fn vesting(&self) -> vesting::storage::StorageApi {
			vesting::storage::StorageApi
		}
		pub fn bonded_finance(&self) -> bonded_finance::storage::StorageApi {
			bonded_finance::storage::StorageApi
		}
		pub fn assets_registry(&self) -> assets_registry::storage::StorageApi {
			assets_registry::storage::StorageApi
		}
		pub fn pablo(&self) -> pablo::storage::StorageApi {
			pablo::storage::StorageApi
		}
		pub fn oracle(&self) -> oracle::storage::StorageApi {
			oracle::storage::StorageApi
		}
		pub fn farming_rewards(&self) -> farming_rewards::storage::StorageApi {
			farming_rewards::storage::StorageApi
		}
		pub fn farming(&self) -> farming::storage::StorageApi {
			farming::storage::StorageApi
		}
		pub fn call_filter(&self) -> call_filter::storage::StorageApi {
			call_filter::storage::StorageApi
		}
		pub fn cosmwasm(&self) -> cosmwasm::storage::StorageApi {
			cosmwasm::storage::StorageApi
		}
		pub fn ibc(&self) -> ibc::storage::StorageApi {
			ibc::storage::StorageApi
		}
		pub fn ics20_fee(&self) -> ics20_fee::storage::StorageApi {
			ics20_fee::storage::StorageApi
		}
		pub fn pallet_multihop_xcm_ibc(&self) -> pallet_multihop_xcm_ibc::storage::StorageApi {
			pallet_multihop_xcm_ibc::storage::StorageApi
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
		pub fn asset_tx_payment(&self) -> asset_tx_payment::calls::TransactionApi {
			asset_tx_payment::calls::TransactionApi
		}
		pub fn indices(&self) -> indices::calls::TransactionApi {
			indices::calls::TransactionApi
		}
		pub fn balances(&self) -> balances::calls::TransactionApi {
			balances::calls::TransactionApi
		}
		pub fn identity(&self) -> identity::calls::TransactionApi {
			identity::calls::TransactionApi
		}
		pub fn multisig(&self) -> multisig::calls::TransactionApi {
			multisig::calls::TransactionApi
		}
		pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi {
			parachain_system::calls::TransactionApi
		}
		pub fn parachain_info(&self) -> parachain_info::calls::TransactionApi {
			parachain_info::calls::TransactionApi
		}
		pub fn collator_selection(&self) -> collator_selection::calls::TransactionApi {
			collator_selection::calls::TransactionApi
		}
		pub fn session(&self) -> session::calls::TransactionApi {
			session::calls::TransactionApi
		}
		pub fn council(&self) -> council::calls::TransactionApi {
			council::calls::TransactionApi
		}
		pub fn council_membership(&self) -> council_membership::calls::TransactionApi {
			council_membership::calls::TransactionApi
		}
		pub fn treasury(&self) -> treasury::calls::TransactionApi {
			treasury::calls::TransactionApi
		}
		pub fn democracy(&self) -> democracy::calls::TransactionApi {
			democracy::calls::TransactionApi
		}
		pub fn technical_committee(&self) -> technical_committee::calls::TransactionApi {
			technical_committee::calls::TransactionApi
		}
		pub fn technical_committee_membership(
			&self,
		) -> technical_committee_membership::calls::TransactionApi {
			technical_committee_membership::calls::TransactionApi
		}
		pub fn release_committee(&self) -> release_committee::calls::TransactionApi {
			release_committee::calls::TransactionApi
		}
		pub fn release_membership(&self) -> release_membership::calls::TransactionApi {
			release_membership::calls::TransactionApi
		}
		pub fn scheduler(&self) -> scheduler::calls::TransactionApi {
			scheduler::calls::TransactionApi
		}
		pub fn utility(&self) -> utility::calls::TransactionApi {
			utility::calls::TransactionApi
		}
		pub fn preimage(&self) -> preimage::calls::TransactionApi {
			preimage::calls::TransactionApi
		}
		pub fn proxy(&self) -> proxy::calls::TransactionApi {
			proxy::calls::TransactionApi
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
		pub fn x_tokens(&self) -> x_tokens::calls::TransactionApi {
			x_tokens::calls::TransactionApi
		}
		pub fn unknown_tokens(&self) -> unknown_tokens::calls::TransactionApi {
			unknown_tokens::calls::TransactionApi
		}
		pub fn tokens(&self) -> tokens::calls::TransactionApi {
			tokens::calls::TransactionApi
		}
		pub fn currency_factory(&self) -> currency_factory::calls::TransactionApi {
			currency_factory::calls::TransactionApi
		}
		pub fn crowdloan_rewards(&self) -> crowdloan_rewards::calls::TransactionApi {
			crowdloan_rewards::calls::TransactionApi
		}
		pub fn vesting(&self) -> vesting::calls::TransactionApi {
			vesting::calls::TransactionApi
		}
		pub fn bonded_finance(&self) -> bonded_finance::calls::TransactionApi {
			bonded_finance::calls::TransactionApi
		}
		pub fn assets_registry(&self) -> assets_registry::calls::TransactionApi {
			assets_registry::calls::TransactionApi
		}
		pub fn pablo(&self) -> pablo::calls::TransactionApi {
			pablo::calls::TransactionApi
		}
		pub fn oracle(&self) -> oracle::calls::TransactionApi {
			oracle::calls::TransactionApi
		}
		pub fn assets_transactor_router(&self) -> assets_transactor_router::calls::TransactionApi {
			assets_transactor_router::calls::TransactionApi
		}
		pub fn farming_rewards(&self) -> farming_rewards::calls::TransactionApi {
			farming_rewards::calls::TransactionApi
		}
		pub fn farming(&self) -> farming::calls::TransactionApi {
			farming::calls::TransactionApi
		}
		pub fn call_filter(&self) -> call_filter::calls::TransactionApi {
			call_filter::calls::TransactionApi
		}
		pub fn cosmwasm(&self) -> cosmwasm::calls::TransactionApi {
			cosmwasm::calls::TransactionApi
		}
		pub fn ibc(&self) -> ibc::calls::TransactionApi {
			ibc::calls::TransactionApi
		}
		pub fn ics20_fee(&self) -> ics20_fee::calls::TransactionApi {
			ics20_fee::calls::TransactionApi
		}
		pub fn pallet_multihop_xcm_ibc(&self) -> pallet_multihop_xcm_ibc::calls::TransactionApi {
			pallet_multihop_xcm_ibc::calls::TransactionApi
		}
	}
	#[doc = r" check whether the metadata provided is aligned with this statically generated code."]
	pub fn is_codegen_valid_for(metadata: &::subxt::Metadata) -> bool {
		let runtime_metadata_hash = metadata
			.hasher()
			.only_these_pallets(&PALLETS)
			.only_these_runtime_apis(&RUNTIME_APIS)
			.hash();
		runtime_metadata_hash ==
			[
				238u8, 114u8, 24u8, 163u8, 243u8, 209u8, 12u8, 57u8, 153u8, 175u8, 131u8, 7u8,
				98u8, 94u8, 125u8, 159u8, 170u8, 193u8, 16u8, 66u8, 254u8, 43u8, 239u8, 67u8,
				173u8, 94u8, 107u8, 103u8, 155u8, 83u8, 24u8, 200u8,
			]
	}
	pub mod system {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::frame_system::pallet::Error;
		pub type Call = runtime_types::frame_system::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Remark {
					pub remark: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Remark {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct SetHeapPages {
					pub pages: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetHeapPages {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_heap_pages";
				}
				pub struct SetCode {
					pub code: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCode {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code";
				}
				pub struct SetCodeWithoutChecks {
					pub code: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCodeWithoutChecks {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code_without_checks";
				}
				pub struct SetStorage {
					pub items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_storage";
				}
				pub struct KillStorage {
					pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				}
				impl ::subxt::blocks::StaticExtrinsic for KillStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_storage";
				}
				pub struct KillPrefix {
					pub prefix: ::std::vec::Vec<::core::primitive::u8>,
					pub subkeys: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for KillPrefix {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_prefix";
				}
				pub struct RemarkWithEvent {
					pub remark: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemarkWithEvent {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark_with_event";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn remark(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::Remark> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark",
						types::Remark { remark },
						[
							43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
							216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
							250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
							13u8,
						],
					)
				}
				pub fn set_heap_pages(
					&self,
					pages: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::SetHeapPages> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_heap_pages",
						types::SetHeapPages { pages },
						[
							188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
							215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
							134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
							57u8, 147u8,
						],
					)
				}
				pub fn set_code(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetCode> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code",
						types::SetCode { code },
						[
							233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
							203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
							27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
						],
					)
				}
				pub fn set_code_without_checks(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetCodeWithoutChecks> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code_without_checks",
						types::SetCodeWithoutChecks { code },
						[
							82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
							157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
							147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
							115u8,
						],
					)
				}
				pub fn set_storage(
					&self,
					items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				) -> ::subxt::tx::Payload<types::SetStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_storage",
						types::SetStorage { items },
						[
							141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
							163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
							150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
							234u8, 43u8,
						],
					)
				}
				pub fn kill_storage(
					&self,
					keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				) -> ::subxt::tx::Payload<types::KillStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_storage",
						types::KillStorage { keys },
						[
							73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
							234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
							156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
							35u8,
						],
					)
				}
				pub fn kill_prefix(
					&self,
					prefix: ::std::vec::Vec<::core::primitive::u8>,
					subkeys: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::KillPrefix> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_prefix",
						types::KillPrefix { prefix, subkeys },
						[
							184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
							175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
							67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
							85u8,
						],
					)
				}
				pub fn remark_with_event(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::RemarkWithEvent> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark_with_event",
						types::RemarkWithEvent { remark },
						[
							120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
							228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
							147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct ExtrinsicSuccess {
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			pub struct ExtrinsicFailed {
				pub dispatch_error: runtime_types::sp_runtime::DispatchError,
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			pub struct CodeUpdated;
			impl ::subxt::events::StaticEvent for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			pub struct NewAccount {
				pub account: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			pub struct KilledAccount {
				pub account: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			pub struct Remarked {
				pub sender: runtime_types::sp_core::crypto::AccountId32,
				pub hash: runtime_types::primitive_types::H256,
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
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
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
							137u8, 212u8, 91u8, 4u8, 157u8, 162u8, 11u8, 45u8, 224u8, 30u8, 223u8,
							139u8, 23u8, 40u8, 51u8, 204u8, 68u8, 22u8, 134u8, 34u8, 183u8, 19u8,
							4u8, 188u8, 80u8, 84u8, 227u8, 36u8, 165u8, 100u8, 40u8, 145u8,
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
							137u8, 212u8, 91u8, 4u8, 157u8, 162u8, 11u8, 45u8, 224u8, 30u8, 223u8,
							139u8, 23u8, 40u8, 51u8, 204u8, 68u8, 22u8, 134u8, 34u8, 183u8, 19u8,
							4u8, 188u8, 80u8, 84u8, 227u8, 36u8, 165u8, 100u8, 40u8, 145u8,
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
							102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
							153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
							120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
							159u8, 79u8,
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
							158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
							62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
							229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
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
							117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
							243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
							101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
							242u8, 65u8,
						],
					)
				}
				pub fn block_hash(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitive_types::H256,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
							103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
							164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
							202u8, 118u8,
						],
					)
				}
				pub fn block_hash_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitive_types::H256,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						Vec::new(),
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
							103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
							164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
							202u8, 118u8,
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
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
							220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
							128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
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
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
							220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
							128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
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
							30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
							9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
							200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
						],
					)
				}
				pub fn parent_hash(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitive_types::H256,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ParentHash",
						vec![],
						[
							26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
							192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
							71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
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
							61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
							91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
							58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
						],
					)
				}
				pub fn events(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::frame_system::EventRecord<
							runtime_types::picasso_runtime::RuntimeEvent,
							runtime_types::primitive_types::H256,
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
							182u8, 225u8, 145u8, 143u8, 95u8, 11u8, 137u8, 204u8, 152u8, 47u8,
							120u8, 47u8, 222u8, 211u8, 122u8, 41u8, 31u8, 200u8, 109u8, 173u8,
							25u8, 66u8, 54u8, 146u8, 63u8, 204u8, 82u8, 10u8, 56u8, 41u8, 135u8,
							217u8,
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
							175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
							151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
							254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
							133u8,
						],
					)
				}
				pub fn event_topics(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
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
							40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
							133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
							120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
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
							40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
							133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
							120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
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
							137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
							148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
							194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
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
							229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
							130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
							107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
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
							97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
							101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
							167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
							151u8,
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
							191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
							0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
							35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
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
							176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
							190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
							163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
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
							23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
							229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
							96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
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
							42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
							200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
							183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
							177u8,
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
							219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
							228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
							72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
							165u8,
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
		use codec::{Compact, Decode, Encode, HasCompact};
		pub type Call = runtime_types::pallet_timestamp::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Set {
					#[codec::codec(compact)]
					pub now: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for Set {
					const PALLET: &'static str = "Timestamp";
					const CALL: &'static str = "set";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set(&self, now: ::core::primitive::u64) -> ::subxt::tx::Payload<types::Set> {
					::subxt::tx::Payload::new_static(
						"Timestamp",
						"set",
						types::Set { now },
						[
							37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
							199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
							200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
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
							44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
							92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
							141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
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
							229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
							205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
							248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
							214u8, 140u8,
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
		pub type Error = runtime_types::pallet_sudo::pallet::Error;
		pub type Call = runtime_types::pallet_sudo::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Sudo {
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Sudo {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo";
				}
				pub struct SudoUncheckedWeight {
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoUncheckedWeight {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_unchecked_weight";
				}
				pub struct SetKey {
					pub new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetKey {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "set_key";
				}
				pub struct SudoAs {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoAs {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_as";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn sudo(
					&self,
					call: runtime_types::picasso_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::Sudo> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo",
						types::Sudo { call: ::std::boxed::Box::new(call) },
						[
							69u8, 254u8, 159u8, 251u8, 135u8, 85u8, 97u8, 239u8, 89u8, 48u8, 244u8,
							121u8, 205u8, 182u8, 142u8, 147u8, 91u8, 74u8, 41u8, 213u8, 41u8,
							234u8, 112u8, 153u8, 19u8, 187u8, 165u8, 155u8, 93u8, 38u8, 255u8,
							26u8,
						],
					)
				}
				pub fn sudo_unchecked_weight(
					&self,
					call: runtime_types::picasso_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::SudoUncheckedWeight> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_unchecked_weight",
						types::SudoUncheckedWeight { call: ::std::boxed::Box::new(call), weight },
						[
							167u8, 183u8, 157u8, 7u8, 251u8, 162u8, 243u8, 97u8, 83u8, 198u8,
							232u8, 211u8, 53u8, 189u8, 73u8, 164u8, 170u8, 41u8, 194u8, 90u8,
							203u8, 103u8, 128u8, 251u8, 33u8, 13u8, 49u8, 132u8, 49u8, 21u8, 70u8,
							100u8,
						],
					)
				}
				pub fn set_key(
					&self,
					new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SetKey> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"set_key",
						types::SetKey { new },
						[
							247u8, 148u8, 125u8, 134u8, 90u8, 138u8, 216u8, 117u8, 246u8, 75u8,
							85u8, 242u8, 229u8, 203u8, 227u8, 242u8, 166u8, 28u8, 247u8, 215u8,
							35u8, 13u8, 243u8, 13u8, 4u8, 48u8, 175u8, 192u8, 63u8, 142u8, 13u8,
							94u8,
						],
					)
				}
				pub fn sudo_as(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					call: runtime_types::picasso_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::SudoAs> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_as",
						types::SudoAs { who, call: ::std::boxed::Box::new(call) },
						[
							249u8, 95u8, 30u8, 134u8, 35u8, 1u8, 147u8, 64u8, 27u8, 239u8, 219u8,
							248u8, 186u8, 175u8, 138u8, 100u8, 186u8, 84u8, 93u8, 139u8, 219u8,
							162u8, 197u8, 254u8, 209u8, 10u8, 208u8, 122u8, 125u8, 8u8, 20u8,
							239u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Sudid {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			pub struct KeyChanged {
				pub old_sudoer: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
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
					runtime_types::sp_core::crypto::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Sudo",
						"Key",
						vec![],
						[
							72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
							31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
							36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
						],
					)
				}
			}
		}
	}
	pub mod transaction_payment {
		use super::{root_mod, runtime_types};
		pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct TransactionFeePaid {
				pub who: runtime_types::sp_core::crypto::AccountId32,
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
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TransactionPayment",
						"NextFeeMultiplier",
						vec![],
						[
							247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
							147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
							159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
							197u8,
						],
					)
				}
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_transaction_payment::Releases,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TransactionPayment",
						"StorageVersion",
						vec![],
						[
							105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
							178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
							251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
							144u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn operational_fee_multiplier(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u8> {
					::subxt::constants::Address::new_static(
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
	pub mod asset_tx_payment {
		use super::{root_mod, runtime_types};
		pub type Call = runtime_types::pallet_asset_tx_payment::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct SetPaymentAsset {
					pub payer: runtime_types::sp_core::crypto::AccountId32,
					pub asset_id:
						::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetPaymentAsset {
					const PALLET: &'static str = "AssetTxPayment";
					const CALL: &'static str = "set_payment_asset";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_payment_asset(
					&self,
					payer: runtime_types::sp_core::crypto::AccountId32,
					asset_id: ::core::option::Option<
						runtime_types::primitives::currency::CurrencyId,
					>,
				) -> ::subxt::tx::Payload<types::SetPaymentAsset> {
					::subxt::tx::Payload::new_static(
						"AssetTxPayment",
						"set_payment_asset",
						types::SetPaymentAsset { payer, asset_id },
						[
							95u8, 250u8, 148u8, 0u8, 111u8, 232u8, 249u8, 9u8, 69u8, 59u8, 37u8,
							240u8, 255u8, 61u8, 96u8, 85u8, 23u8, 193u8, 214u8, 2u8, 127u8, 154u8,
							243u8, 104u8, 46u8, 117u8, 36u8, 43u8, 206u8, 147u8, 173u8, 39u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn payment_assets(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(runtime_types::primitives::currency::CurrencyId, ::core::primitive::u128),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetTxPayment",
						"PaymentAssets",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							58u8, 117u8, 64u8, 244u8, 69u8, 225u8, 149u8, 16u8, 119u8, 125u8,
							229u8, 78u8, 76u8, 185u8, 70u8, 254u8, 111u8, 100u8, 154u8, 184u8,
							248u8, 14u8, 127u8, 233u8, 84u8, 185u8, 194u8, 110u8, 173u8, 241u8,
							125u8, 12u8,
						],
					)
				}
				pub fn payment_assets_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(runtime_types::primitives::currency::CurrencyId, ::core::primitive::u128),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"AssetTxPayment",
						"PaymentAssets",
						Vec::new(),
						[
							58u8, 117u8, 64u8, 244u8, 69u8, 225u8, 149u8, 16u8, 119u8, 125u8,
							229u8, 78u8, 76u8, 185u8, 70u8, 254u8, 111u8, 100u8, 154u8, 184u8,
							248u8, 14u8, 127u8, 233u8, 84u8, 185u8, 194u8, 110u8, 173u8, 241u8,
							125u8, 12u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn use_user_configuration(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::bool> {
					::subxt::constants::Address::new_static(
						"AssetTxPayment",
						"UseUserConfiguration",
						[
							165u8, 28u8, 112u8, 190u8, 18u8, 129u8, 182u8, 206u8, 237u8, 1u8, 68u8,
							252u8, 125u8, 234u8, 185u8, 50u8, 149u8, 164u8, 47u8, 126u8, 134u8,
							100u8, 14u8, 86u8, 209u8, 39u8, 20u8, 4u8, 233u8, 115u8, 102u8, 131u8,
						],
					)
				}
			}
		}
	}
	pub mod indices {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_indices::pallet::Error;
		pub type Call = runtime_types::pallet_indices::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct Claim {
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Claim {
					const PALLET: &'static str = "Indices";
					const CALL: &'static str = "claim";
				}
				pub struct Transfer {
					pub new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Transfer {
					const PALLET: &'static str = "Indices";
					const CALL: &'static str = "transfer";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct Free {
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Free {
					const PALLET: &'static str = "Indices";
					const CALL: &'static str = "free";
				}
				pub struct ForceTransfer {
					pub new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub index: ::core::primitive::u32,
					pub freeze: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "Indices";
					const CALL: &'static str = "force_transfer";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct Freeze {
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Freeze {
					const PALLET: &'static str = "Indices";
					const CALL: &'static str = "freeze";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn claim(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Claim> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"claim",
						types::Claim { index },
						[
							146u8, 58u8, 246u8, 135u8, 59u8, 90u8, 3u8, 5u8, 140u8, 169u8, 232u8,
							195u8, 11u8, 107u8, 36u8, 141u8, 118u8, 174u8, 160u8, 160u8, 19u8,
							205u8, 177u8, 193u8, 18u8, 102u8, 115u8, 31u8, 72u8, 29u8, 91u8, 235u8,
						],
					)
				}
				pub fn transfer(
					&self,
					new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Transfer> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"transfer",
						types::Transfer { new, index },
						[
							253u8, 209u8, 123u8, 236u8, 91u8, 71u8, 183u8, 49u8, 84u8, 13u8, 130u8,
							208u8, 181u8, 218u8, 219u8, 178u8, 71u8, 76u8, 228u8, 249u8, 197u8,
							243u8, 136u8, 122u8, 150u8, 179u8, 249u8, 187u8, 150u8, 158u8, 201u8,
							134u8,
						],
					)
				}
				pub fn free(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Free> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"free",
						types::Free { index },
						[
							241u8, 211u8, 234u8, 102u8, 189u8, 22u8, 209u8, 27u8, 8u8, 229u8, 80u8,
							227u8, 138u8, 252u8, 222u8, 111u8, 77u8, 201u8, 235u8, 51u8, 163u8,
							247u8, 13u8, 126u8, 216u8, 136u8, 57u8, 222u8, 56u8, 66u8, 215u8,
							244u8,
						],
					)
				}
				pub fn force_transfer(
					&self,
					new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
					freeze: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"force_transfer",
						types::ForceTransfer { new, index, freeze },
						[
							61u8, 7u8, 111u8, 227u8, 228u8, 62u8, 178u8, 225u8, 195u8, 185u8,
							243u8, 161u8, 156u8, 53u8, 165u8, 178u8, 238u8, 146u8, 66u8, 165u8,
							7u8, 137u8, 36u8, 7u8, 118u8, 84u8, 203u8, 3u8, 143u8, 95u8, 99u8,
							192u8,
						],
					)
				}
				pub fn freeze(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Freeze> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"freeze",
						types::Freeze { index },
						[
							238u8, 215u8, 108u8, 156u8, 84u8, 240u8, 130u8, 229u8, 27u8, 132u8,
							93u8, 78u8, 2u8, 251u8, 43u8, 203u8, 2u8, 142u8, 147u8, 48u8, 92u8,
							101u8, 207u8, 24u8, 51u8, 16u8, 36u8, 229u8, 188u8, 129u8, 160u8,
							117u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_indices::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct IndexAssigned {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexAssigned {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexAssigned";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct IndexFreed {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexFreed {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFreed";
			}
			pub struct IndexFrozen {
				pub index: ::core::primitive::u32,
				pub who: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for IndexFrozen {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFrozen";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn accounts(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Indices",
						"Accounts",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							48u8, 189u8, 43u8, 119u8, 32u8, 168u8, 28u8, 12u8, 245u8, 81u8, 119u8,
							182u8, 23u8, 201u8, 33u8, 147u8, 128u8, 171u8, 155u8, 134u8, 71u8,
							87u8, 100u8, 248u8, 107u8, 129u8, 36u8, 197u8, 220u8, 90u8, 11u8,
							238u8,
						],
					)
				}
				pub fn accounts_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Indices",
						"Accounts",
						Vec::new(),
						[
							48u8, 189u8, 43u8, 119u8, 32u8, 168u8, 28u8, 12u8, 245u8, 81u8, 119u8,
							182u8, 23u8, 201u8, 33u8, 147u8, 128u8, 171u8, 155u8, 134u8, 71u8,
							87u8, 100u8, 248u8, 107u8, 129u8, 36u8, 197u8, 220u8, 90u8, 11u8,
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
				pub fn deposit(&self) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Indices",
						"Deposit",
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
	pub mod balances {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_balances::pallet::Error;
		pub type Call = runtime_types::pallet_balances::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Transfer {
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					#[codec::codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Transfer {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer";
				}
				pub struct SetBalance {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					#[codec::codec(compact)]
					pub new_free: ::core::primitive::u128,
					#[codec::codec(compact)]
					pub new_reserved: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetBalance {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "set_balance";
				}
				pub struct ForceTransfer {
					pub source: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					#[codec::codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_transfer";
				}
				pub struct TransferKeepAlive {
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					#[codec::codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_keep_alive";
				}
				pub struct TransferAll {
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAll {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_all";
				}
				pub struct ForceUnreserve {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_unreserve";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn transfer(
					&self,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Transfer> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer",
						types::Transfer { dest, value },
						[
							228u8, 253u8, 44u8, 208u8, 33u8, 44u8, 33u8, 42u8, 114u8, 57u8, 107u8,
							6u8, 127u8, 116u8, 15u8, 205u8, 122u8, 172u8, 64u8, 108u8, 169u8,
							241u8, 190u8, 221u8, 248u8, 171u8, 236u8, 129u8, 120u8, 147u8, 49u8,
							95u8,
						],
					)
				}
				pub fn set_balance(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::SetBalance> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"set_balance",
						types::SetBalance { who, new_free, new_reserved },
						[
							234u8, 35u8, 192u8, 235u8, 238u8, 153u8, 65u8, 43u8, 218u8, 79u8, 26u8,
							131u8, 190u8, 59u8, 105u8, 82u8, 167u8, 48u8, 193u8, 247u8, 7u8, 79u8,
							85u8, 48u8, 205u8, 66u8, 145u8, 26u8, 101u8, 184u8, 65u8, 173u8,
						],
					)
				}
				pub fn force_transfer(
					&self,
					source: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_transfer",
						types::ForceTransfer { source, dest, value },
						[
							23u8, 7u8, 44u8, 138u8, 180u8, 140u8, 216u8, 52u8, 198u8, 3u8, 225u8,
							116u8, 47u8, 26u8, 61u8, 163u8, 55u8, 64u8, 113u8, 250u8, 192u8, 16u8,
							228u8, 228u8, 85u8, 255u8, 100u8, 128u8, 245u8, 132u8, 84u8, 186u8,
						],
					)
				}
				pub fn transfer_keep_alive(
					&self,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_keep_alive",
						types::TransferKeepAlive { dest, value },
						[
							196u8, 51u8, 121u8, 239u8, 68u8, 97u8, 174u8, 26u8, 21u8, 9u8, 111u8,
							224u8, 189u8, 35u8, 106u8, 30u8, 83u8, 184u8, 234u8, 174u8, 27u8,
							197u8, 40u8, 126u8, 197u8, 92u8, 201u8, 253u8, 144u8, 175u8, 8u8,
							215u8,
						],
					)
				}
				pub fn transfer_all(
					&self,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::TransferAll> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_all",
						types::TransferAll { dest, keep_alive },
						[
							13u8, 46u8, 127u8, 231u8, 179u8, 61u8, 45u8, 188u8, 195u8, 251u8,
							146u8, 25u8, 138u8, 19u8, 52u8, 112u8, 148u8, 241u8, 134u8, 145u8,
							97u8, 9u8, 199u8, 172u8, 229u8, 239u8, 67u8, 185u8, 128u8, 36u8, 134u8,
							122u8,
						],
					)
				}
				pub fn force_unreserve(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceUnreserve> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_unreserve",
						types::ForceUnreserve { who, amount },
						[
							176u8, 105u8, 20u8, 111u8, 49u8, 253u8, 22u8, 225u8, 0u8, 81u8, 221u8,
							39u8, 62u8, 22u8, 95u8, 12u8, 21u8, 251u8, 179u8, 31u8, 104u8, 23u8,
							34u8, 216u8, 119u8, 205u8, 133u8, 196u8, 182u8, 113u8, 36u8, 93u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Endowed {
				pub account: runtime_types::sp_core::crypto::AccountId32,
				pub free_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Endowed";
			}
			pub struct DustLost {
				pub account: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "DustLost";
			}
			pub struct Transfer {
				pub from: runtime_types::sp_core::crypto::AccountId32,
				pub to: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Transfer";
			}
			pub struct BalanceSet {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub free: ::core::primitive::u128,
				pub reserved: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BalanceSet";
			}
			pub struct Reserved {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Reserved";
			}
			pub struct Unreserved {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unreserved";
			}
			pub struct ReserveRepatriated {
				pub from: runtime_types::sp_core::crypto::AccountId32,
				pub to: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
				pub destination_status:
					runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			pub struct Deposit {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Deposit";
			}
			pub struct Withdraw {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdraw {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Withdraw";
			}
			pub struct Slashed {
				pub who: runtime_types::sp_core::crypto::AccountId32,
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
				pub fn total_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"TotalIssuance",
						vec![],
						[
							116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
							171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
							255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
							185u8,
						],
					)
				}
				pub fn inactive_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"InactiveIssuance",
						vec![],
						[
							212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
							249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
							30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
						],
					)
				}
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Account",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							174u8, 194u8, 127u8, 179u8, 128u8, 15u8, 98u8, 227u8, 46u8, 40u8,
							242u8, 228u8, 90u8, 81u8, 159u8, 180u8, 63u8, 172u8, 212u8, 42u8, 10u8,
							219u8, 230u8, 129u8, 24u8, 151u8, 84u8, 242u8, 117u8, 254u8, 202u8,
							190u8,
						],
					)
				}
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Account",
						Vec::new(),
						[
							174u8, 194u8, 127u8, 179u8, 128u8, 15u8, 98u8, 227u8, 46u8, 40u8,
							242u8, 228u8, 90u8, 81u8, 159u8, 180u8, 63u8, 172u8, 212u8, 42u8, 10u8,
							219u8, 230u8, 129u8, 24u8, 151u8, 84u8, 242u8, 117u8, 254u8, 202u8,
							190u8,
						],
					)
				}
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Locks",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
							167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
							13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
						],
					)
				}
				pub fn locks_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Locks",
						Vec::new(),
						[
							10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
							167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
							13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
						],
					)
				}
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Reserves",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
							140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
							106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
						],
					)
				}
				pub fn reserves_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Reserves",
						Vec::new(),
						[
							112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
							140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
							106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn existential_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Balances",
						"ExistentialDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
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
				pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
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
	pub mod identity {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_identity::pallet::Error;
		pub type Call = runtime_types::pallet_identity::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct AddRegistrar {
					pub account: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddRegistrar {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "add_registrar";
				}
				pub struct SetIdentity {
					pub info:
						::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetIdentity {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "set_identity";
				}
				pub struct SetSubs {
					pub subs: ::std::vec::Vec<(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::pallet_identity::types::Data,
					)>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetSubs {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "set_subs";
				}
				pub struct ClearIdentity;
				impl ::subxt::blocks::StaticExtrinsic for ClearIdentity {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "clear_identity";
				}
				pub struct RequestJudgement {
					#[codec::codec(compact)]
					pub reg_index: ::core::primitive::u32,
					#[codec::codec(compact)]
					pub max_fee: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for RequestJudgement {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "request_judgement";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct CancelRequest {
					pub reg_index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for CancelRequest {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "cancel_request";
				}
				pub struct SetFee {
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					#[codec::codec(compact)]
					pub fee: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetFee {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "set_fee";
				}
				pub struct SetAccountId {
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					pub new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetAccountId {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "set_account_id";
				}
				pub struct SetFields {
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					pub fields: runtime_types::pallet_identity::types::BitFlags<
						runtime_types::pallet_identity::types::IdentityField,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetFields {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "set_fields";
				}
				pub struct ProvideJudgement {
					#[codec::codec(compact)]
					pub reg_index: ::core::primitive::u32,
					pub target: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub judgement:
						runtime_types::pallet_identity::types::Judgement<::core::primitive::u128>,
					pub identity: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for ProvideJudgement {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "provide_judgement";
				}
				pub struct KillIdentity {
					pub target: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for KillIdentity {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "kill_identity";
				}
				pub struct AddSub {
					pub sub: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub data: runtime_types::pallet_identity::types::Data,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddSub {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "add_sub";
				}
				pub struct RenameSub {
					pub sub: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub data: runtime_types::pallet_identity::types::Data,
				}
				impl ::subxt::blocks::StaticExtrinsic for RenameSub {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "rename_sub";
				}
				pub struct RemoveSub {
					pub sub: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveSub {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "remove_sub";
				}
				pub struct QuitSub;
				impl ::subxt::blocks::StaticExtrinsic for QuitSub {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "quit_sub";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_registrar(
					&self,
					account: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::AddRegistrar> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"add_registrar",
						types::AddRegistrar { account },
						[
							206u8, 30u8, 240u8, 168u8, 67u8, 228u8, 17u8, 74u8, 26u8, 222u8, 61u8,
							15u8, 100u8, 25u8, 162u8, 159u8, 83u8, 110u8, 30u8, 52u8, 201u8, 49u8,
							115u8, 152u8, 142u8, 76u8, 14u8, 239u8, 184u8, 136u8, 195u8, 39u8,
						],
					)
				}
				pub fn set_identity(
					&self,
					info: runtime_types::pallet_identity::types::IdentityInfo,
				) -> ::subxt::tx::Payload<types::SetIdentity> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"set_identity",
						types::SetIdentity { info: ::std::boxed::Box::new(info) },
						[
							18u8, 86u8, 67u8, 10u8, 116u8, 254u8, 94u8, 95u8, 166u8, 30u8, 204u8,
							189u8, 174u8, 70u8, 191u8, 255u8, 149u8, 93u8, 156u8, 120u8, 105u8,
							138u8, 199u8, 181u8, 43u8, 150u8, 143u8, 254u8, 182u8, 81u8, 86u8,
							45u8,
						],
					)
				}
				pub fn set_subs(
					&self,
					subs: ::std::vec::Vec<(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::pallet_identity::types::Data,
					)>,
				) -> ::subxt::tx::Payload<types::SetSubs> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"set_subs",
						types::SetSubs { subs },
						[
							34u8, 184u8, 18u8, 155u8, 112u8, 247u8, 235u8, 75u8, 209u8, 236u8,
							21u8, 238u8, 43u8, 237u8, 223u8, 147u8, 48u8, 6u8, 39u8, 231u8, 174u8,
							164u8, 243u8, 184u8, 220u8, 151u8, 165u8, 69u8, 219u8, 122u8, 234u8,
							100u8,
						],
					)
				}
				pub fn clear_identity(&self) -> ::subxt::tx::Payload<types::ClearIdentity> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"clear_identity",
						types::ClearIdentity {},
						[
							43u8, 115u8, 205u8, 44u8, 24u8, 130u8, 220u8, 69u8, 247u8, 176u8,
							200u8, 175u8, 67u8, 183u8, 36u8, 200u8, 162u8, 132u8, 242u8, 25u8,
							21u8, 106u8, 197u8, 219u8, 141u8, 51u8, 204u8, 13u8, 191u8, 201u8,
							31u8, 31u8,
						],
					)
				}
				pub fn request_judgement(
					&self,
					reg_index: ::core::primitive::u32,
					max_fee: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::RequestJudgement> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"request_judgement",
						types::RequestJudgement { reg_index, max_fee },
						[
							83u8, 85u8, 55u8, 184u8, 14u8, 54u8, 49u8, 212u8, 26u8, 148u8, 33u8,
							147u8, 182u8, 54u8, 180u8, 12u8, 61u8, 179u8, 216u8, 157u8, 103u8,
							52u8, 120u8, 252u8, 83u8, 203u8, 144u8, 65u8, 15u8, 3u8, 21u8, 33u8,
						],
					)
				}
				pub fn cancel_request(
					&self,
					reg_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::CancelRequest> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"cancel_request",
						types::CancelRequest { reg_index },
						[
							81u8, 14u8, 133u8, 219u8, 43u8, 84u8, 163u8, 208u8, 21u8, 185u8, 75u8,
							117u8, 126u8, 33u8, 210u8, 106u8, 122u8, 210u8, 35u8, 207u8, 104u8,
							206u8, 41u8, 117u8, 247u8, 108u8, 56u8, 23u8, 123u8, 169u8, 169u8,
							61u8,
						],
					)
				}
				pub fn set_fee(
					&self,
					index: ::core::primitive::u32,
					fee: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::SetFee> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"set_fee",
						types::SetFee { index, fee },
						[
							131u8, 20u8, 17u8, 127u8, 180u8, 65u8, 225u8, 144u8, 193u8, 60u8,
							131u8, 241u8, 30u8, 149u8, 8u8, 76u8, 29u8, 52u8, 102u8, 108u8, 127u8,
							130u8, 70u8, 18u8, 94u8, 145u8, 179u8, 109u8, 252u8, 219u8, 58u8,
							163u8,
						],
					)
				}
				pub fn set_account_id(
					&self,
					index: ::core::primitive::u32,
					new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SetAccountId> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"set_account_id",
						types::SetAccountId { index, new },
						[
							40u8, 151u8, 216u8, 253u8, 241u8, 117u8, 210u8, 208u8, 98u8, 94u8,
							228u8, 208u8, 122u8, 100u8, 86u8, 237u8, 240u8, 89u8, 90u8, 109u8,
							23u8, 255u8, 121u8, 176u8, 146u8, 10u8, 190u8, 175u8, 148u8, 228u8,
							176u8, 43u8,
						],
					)
				}
				pub fn set_fields(
					&self,
					index: ::core::primitive::u32,
					fields: runtime_types::pallet_identity::types::BitFlags<
						runtime_types::pallet_identity::types::IdentityField,
					>,
				) -> ::subxt::tx::Payload<types::SetFields> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"set_fields",
						types::SetFields { index, fields },
						[
							25u8, 129u8, 119u8, 232u8, 18u8, 32u8, 77u8, 23u8, 185u8, 56u8, 32u8,
							199u8, 74u8, 174u8, 104u8, 203u8, 171u8, 253u8, 19u8, 225u8, 101u8,
							239u8, 14u8, 242u8, 157u8, 51u8, 203u8, 74u8, 1u8, 65u8, 165u8, 205u8,
						],
					)
				}
				pub fn provide_judgement(
					&self,
					reg_index: ::core::primitive::u32,
					target: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					judgement: runtime_types::pallet_identity::types::Judgement<
						::core::primitive::u128,
					>,
					identity: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::ProvideJudgement> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"provide_judgement",
						types::ProvideJudgement { reg_index, target, judgement, identity },
						[
							224u8, 108u8, 183u8, 113u8, 45u8, 239u8, 165u8, 94u8, 110u8, 181u8,
							66u8, 213u8, 45u8, 9u8, 132u8, 203u8, 55u8, 96u8, 19u8, 129u8, 0u8,
							240u8, 138u8, 193u8, 191u8, 188u8, 150u8, 5u8, 64u8, 188u8, 163u8,
							231u8,
						],
					)
				}
				pub fn kill_identity(
					&self,
					target: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::KillIdentity> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"kill_identity",
						types::KillIdentity { target },
						[
							41u8, 147u8, 161u8, 132u8, 99u8, 63u8, 42u8, 219u8, 109u8, 209u8, 19u8,
							243u8, 61u8, 122u8, 16u8, 248u8, 110u8, 85u8, 71u8, 170u8, 38u8, 4u8,
							91u8, 173u8, 212u8, 55u8, 227u8, 51u8, 100u8, 5u8, 211u8, 177u8,
						],
					)
				}
				pub fn add_sub(
					&self,
					sub: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					data: runtime_types::pallet_identity::types::Data,
				) -> ::subxt::tx::Payload<types::AddSub> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"add_sub",
						types::AddSub { sub, data },
						[
							92u8, 68u8, 199u8, 2u8, 215u8, 177u8, 19u8, 216u8, 8u8, 79u8, 165u8,
							233u8, 254u8, 85u8, 115u8, 41u8, 103u8, 67u8, 61u8, 93u8, 204u8, 245u8,
							197u8, 120u8, 88u8, 70u8, 37u8, 22u8, 221u8, 5u8, 100u8, 78u8,
						],
					)
				}
				pub fn rename_sub(
					&self,
					sub: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					data: runtime_types::pallet_identity::types::Data,
				) -> ::subxt::tx::Payload<types::RenameSub> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"rename_sub",
						types::RenameSub { sub, data },
						[
							54u8, 76u8, 163u8, 56u8, 90u8, 60u8, 49u8, 218u8, 100u8, 249u8, 177u8,
							33u8, 174u8, 122u8, 237u8, 205u8, 107u8, 232u8, 168u8, 155u8, 240u8,
							22u8, 97u8, 197u8, 174u8, 250u8, 8u8, 227u8, 10u8, 205u8, 188u8, 30u8,
						],
					)
				}
				pub fn remove_sub(
					&self,
					sub: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::RemoveSub> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"remove_sub",
						types::RemoveSub { sub },
						[
							80u8, 67u8, 217u8, 201u8, 139u8, 178u8, 58u8, 253u8, 137u8, 193u8,
							133u8, 239u8, 21u8, 226u8, 14u8, 160u8, 110u8, 20u8, 35u8, 168u8,
							139u8, 199u8, 92u8, 125u8, 13u8, 52u8, 248u8, 63u8, 54u8, 166u8, 55u8,
							225u8,
						],
					)
				}
				pub fn quit_sub(&self) -> ::subxt::tx::Payload<types::QuitSub> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"quit_sub",
						types::QuitSub {},
						[
							147u8, 131u8, 175u8, 171u8, 187u8, 201u8, 240u8, 26u8, 146u8, 224u8,
							74u8, 166u8, 242u8, 193u8, 204u8, 247u8, 168u8, 93u8, 18u8, 32u8, 27u8,
							208u8, 149u8, 146u8, 179u8, 172u8, 75u8, 112u8, 84u8, 141u8, 233u8,
							223u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_identity::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct IdentitySet {
				pub who: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for IdentitySet {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "IdentitySet";
			}
			pub struct IdentityCleared {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for IdentityCleared {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "IdentityCleared";
			}
			pub struct IdentityKilled {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for IdentityKilled {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "IdentityKilled";
			}
			pub struct JudgementRequested {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for JudgementRequested {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "JudgementRequested";
			}
			pub struct JudgementUnrequested {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for JudgementUnrequested {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "JudgementUnrequested";
			}
			pub struct JudgementGiven {
				pub target: runtime_types::sp_core::crypto::AccountId32,
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for JudgementGiven {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "JudgementGiven";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct RegistrarAdded {
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for RegistrarAdded {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "RegistrarAdded";
			}
			pub struct SubIdentityAdded {
				pub sub: runtime_types::sp_core::crypto::AccountId32,
				pub main: runtime_types::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for SubIdentityAdded {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "SubIdentityAdded";
			}
			pub struct SubIdentityRemoved {
				pub sub: runtime_types::sp_core::crypto::AccountId32,
				pub main: runtime_types::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for SubIdentityRemoved {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "SubIdentityRemoved";
			}
			pub struct SubIdentityRevoked {
				pub sub: runtime_types::sp_core::crypto::AccountId32,
				pub main: runtime_types::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for SubIdentityRevoked {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "SubIdentityRevoked";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn identity_of(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_identity::types::Registration<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"IdentityOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							112u8, 2u8, 209u8, 123u8, 138u8, 171u8, 80u8, 243u8, 226u8, 88u8, 81u8,
							49u8, 59u8, 172u8, 88u8, 180u8, 255u8, 119u8, 57u8, 16u8, 169u8, 149u8,
							77u8, 239u8, 73u8, 182u8, 28u8, 112u8, 150u8, 110u8, 65u8, 139u8,
						],
					)
				}
				pub fn identity_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_identity::types::Registration<::core::primitive::u128>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"IdentityOf",
						Vec::new(),
						[
							112u8, 2u8, 209u8, 123u8, 138u8, 171u8, 80u8, 243u8, 226u8, 88u8, 81u8,
							49u8, 59u8, 172u8, 88u8, 180u8, 255u8, 119u8, 57u8, 16u8, 169u8, 149u8,
							77u8, 239u8, 73u8, 182u8, 28u8, 112u8, 150u8, 110u8, 65u8, 139u8,
						],
					)
				}
				pub fn super_of(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::pallet_identity::types::Data,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"SuperOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							84u8, 72u8, 64u8, 14u8, 56u8, 9u8, 143u8, 100u8, 141u8, 163u8, 36u8,
							55u8, 38u8, 254u8, 164u8, 17u8, 3u8, 110u8, 88u8, 175u8, 161u8, 65u8,
							159u8, 40u8, 46u8, 8u8, 177u8, 81u8, 130u8, 38u8, 193u8, 28u8,
						],
					)
				}
				pub fn super_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::pallet_identity::types::Data,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"SuperOf",
						Vec::new(),
						[
							84u8, 72u8, 64u8, 14u8, 56u8, 9u8, 143u8, 100u8, 141u8, 163u8, 36u8,
							55u8, 38u8, 254u8, 164u8, 17u8, 3u8, 110u8, 88u8, 175u8, 161u8, 65u8,
							159u8, 40u8, 46u8, 8u8, 177u8, 81u8, 130u8, 38u8, 193u8, 28u8,
						],
					)
				}
				pub fn subs_of(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u128,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::sp_core::crypto::AccountId32,
						>,
					),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"SubsOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							164u8, 140u8, 52u8, 123u8, 220u8, 118u8, 147u8, 3u8, 67u8, 22u8, 191u8,
							18u8, 186u8, 21u8, 154u8, 8u8, 205u8, 224u8, 163u8, 173u8, 174u8,
							107u8, 144u8, 215u8, 116u8, 64u8, 159u8, 115u8, 159u8, 205u8, 91u8,
							28u8,
						],
					)
				}
				pub fn subs_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u128,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::sp_core::crypto::AccountId32,
						>,
					),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"SubsOf",
						Vec::new(),
						[
							164u8, 140u8, 52u8, 123u8, 220u8, 118u8, 147u8, 3u8, 67u8, 22u8, 191u8,
							18u8, 186u8, 21u8, 154u8, 8u8, 205u8, 224u8, 163u8, 173u8, 174u8,
							107u8, 144u8, 215u8, 116u8, 64u8, 159u8, 115u8, 159u8, 205u8, 91u8,
							28u8,
						],
					)
				}
				pub fn registrars(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::option::Option<
							runtime_types::pallet_identity::types::RegistrarInfo<
								::core::primitive::u128,
								runtime_types::sp_core::crypto::AccountId32,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"Registrars",
						vec![],
						[
							207u8, 253u8, 229u8, 237u8, 228u8, 85u8, 173u8, 74u8, 164u8, 67u8,
							144u8, 144u8, 5u8, 242u8, 84u8, 187u8, 110u8, 181u8, 2u8, 162u8, 239u8,
							212u8, 72u8, 233u8, 160u8, 196u8, 121u8, 218u8, 100u8, 0u8, 219u8,
							181u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn basic_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Identity",
						"BasicDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn field_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Identity",
						"FieldDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn sub_account_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Identity",
						"SubAccountDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn max_sub_accounts(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Identity",
						"MaxSubAccounts",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_additional_fields(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Identity",
						"MaxAdditionalFields",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_registrars(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Identity",
						"MaxRegistrars",
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
	pub mod multisig {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_multisig::pallet::Error;
		pub type Call = runtime_types::pallet_multisig::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct AsMultiThreshold1 {
					pub other_signatories:
						::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AsMultiThreshold1 {
					const PALLET: &'static str = "Multisig";
					const CALL: &'static str = "as_multi_threshold_1";
				}
				pub struct AsMulti {
					pub threshold: ::core::primitive::u16,
					pub other_signatories:
						::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					pub maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for AsMulti {
					const PALLET: &'static str = "Multisig";
					const CALL: &'static str = "as_multi";
				}
				pub struct ApproveAsMulti {
					pub threshold: ::core::primitive::u16,
					pub other_signatories:
						::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					pub maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					pub call_hash: [::core::primitive::u8; 32usize],
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for ApproveAsMulti {
					const PALLET: &'static str = "Multisig";
					const CALL: &'static str = "approve_as_multi";
				}
				pub struct CancelAsMulti {
					pub threshold: ::core::primitive::u16,
					pub other_signatories:
						::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					pub timepoint:
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					pub call_hash: [::core::primitive::u8; 32usize],
				}
				impl ::subxt::blocks::StaticExtrinsic for CancelAsMulti {
					const PALLET: &'static str = "Multisig";
					const CALL: &'static str = "cancel_as_multi";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn as_multi_threshold_1(
					&self,
					other_signatories: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					call: runtime_types::picasso_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::AsMultiThreshold1> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"as_multi_threshold_1",
						types::AsMultiThreshold1 {
							other_signatories,
							call: ::std::boxed::Box::new(call),
						},
						[
							136u8, 42u8, 191u8, 74u8, 16u8, 64u8, 250u8, 162u8, 238u8, 94u8, 193u8,
							102u8, 22u8, 196u8, 119u8, 22u8, 58u8, 70u8, 127u8, 16u8, 62u8, 91u8,
							9u8, 190u8, 42u8, 157u8, 77u8, 197u8, 143u8, 35u8, 34u8, 38u8,
						],
					)
				}
				pub fn as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					call: runtime_types::picasso_runtime::RuntimeCall,
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::AsMulti> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"as_multi",
						types::AsMulti {
							threshold,
							other_signatories,
							maybe_timepoint,
							call: ::std::boxed::Box::new(call),
							max_weight,
						},
						[
							181u8, 241u8, 212u8, 214u8, 16u8, 177u8, 4u8, 100u8, 229u8, 185u8,
							53u8, 224u8, 55u8, 4u8, 18u8, 23u8, 140u8, 173u8, 251u8, 17u8, 144u8,
							13u8, 252u8, 206u8, 53u8, 58u8, 94u8, 247u8, 186u8, 143u8, 143u8,
							106u8,
						],
					)
				}
				pub fn approve_as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					call_hash: [::core::primitive::u8; 32usize],
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::ApproveAsMulti> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"approve_as_multi",
						types::ApproveAsMulti {
							threshold,
							other_signatories,
							maybe_timepoint,
							call_hash,
							max_weight,
						},
						[
							248u8, 46u8, 131u8, 35u8, 204u8, 12u8, 218u8, 150u8, 88u8, 131u8, 89u8,
							13u8, 95u8, 122u8, 87u8, 107u8, 136u8, 154u8, 92u8, 199u8, 108u8, 92u8,
							207u8, 171u8, 113u8, 8u8, 47u8, 248u8, 65u8, 26u8, 203u8, 135u8,
						],
					)
				}
				pub fn cancel_as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					call_hash: [::core::primitive::u8; 32usize],
				) -> ::subxt::tx::Payload<types::CancelAsMulti> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"cancel_as_multi",
						types::CancelAsMulti { threshold, other_signatories, timepoint, call_hash },
						[
							212u8, 179u8, 123u8, 40u8, 209u8, 228u8, 181u8, 0u8, 109u8, 28u8, 27u8,
							48u8, 15u8, 47u8, 203u8, 54u8, 106u8, 114u8, 28u8, 118u8, 101u8, 201u8,
							95u8, 187u8, 46u8, 182u8, 4u8, 30u8, 227u8, 105u8, 14u8, 81u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_multisig::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct NewMultisig {
				pub approving: runtime_types::sp_core::crypto::AccountId32,
				pub multisig: runtime_types::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for NewMultisig {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "NewMultisig";
			}
			pub struct MultisigApproval {
				pub approving: runtime_types::sp_core::crypto::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: runtime_types::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for MultisigApproval {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "MultisigApproval";
			}
			pub struct MultisigExecuted {
				pub approving: runtime_types::sp_core::crypto::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: runtime_types::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MultisigExecuted {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "MultisigExecuted";
			}
			pub struct MultisigCancelled {
				pub cancelling: runtime_types::sp_core::crypto::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: runtime_types::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for MultisigCancelled {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "MultisigCancelled";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn multisigs(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_multisig::Multisig<
						::core::primitive::u32,
						::core::primitive::u128,
						runtime_types::sp_core::crypto::AccountId32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Multisig",
						"Multisigs",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							154u8, 109u8, 45u8, 18u8, 155u8, 151u8, 81u8, 28u8, 86u8, 127u8, 189u8,
							151u8, 49u8, 61u8, 12u8, 149u8, 84u8, 61u8, 110u8, 197u8, 200u8, 140u8,
							37u8, 100u8, 14u8, 162u8, 158u8, 161u8, 48u8, 117u8, 102u8, 61u8,
						],
					)
				}
				pub fn multisigs_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_multisig::Multisig<
						::core::primitive::u32,
						::core::primitive::u128,
						runtime_types::sp_core::crypto::AccountId32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Multisig",
						"Multisigs",
						Vec::new(),
						[
							154u8, 109u8, 45u8, 18u8, 155u8, 151u8, 81u8, 28u8, 86u8, 127u8, 189u8,
							151u8, 49u8, 61u8, 12u8, 149u8, 84u8, 61u8, 110u8, 197u8, 200u8, 140u8,
							37u8, 100u8, 14u8, 162u8, 158u8, 161u8, 48u8, 117u8, 102u8, 61u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn deposit_base(&self) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Multisig",
						"DepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn deposit_factor(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Multisig",
						"DepositFactor",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn max_signatories(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Multisig",
						"MaxSignatories",
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
	pub mod parachain_system {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::cumulus_pallet_parachain_system::pallet::Error;
		pub type Call = runtime_types::cumulus_pallet_parachain_system::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct SetValidationData {
					pub data:
						runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetValidationData {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "set_validation_data";
				}
				pub struct SudoSendUpwardMessage {
					pub message: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoSendUpwardMessage {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "sudo_send_upward_message";
				}
				pub struct AuthorizeUpgrade {
					pub code_hash: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for AuthorizeUpgrade {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "authorize_upgrade";
				}
				pub struct EnactAuthorizedUpgrade {
					pub code: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for EnactAuthorizedUpgrade {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "enact_authorized_upgrade";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_validation_data(
					&self,
					data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
				) -> ::subxt::tx::Payload<types::SetValidationData> {
					::subxt::tx::Payload::new_static(
						"ParachainSystem",
						"set_validation_data",
						types::SetValidationData { data },
						[
							167u8, 126u8, 75u8, 137u8, 220u8, 60u8, 106u8, 214u8, 92u8, 170u8,
							136u8, 176u8, 98u8, 0u8, 234u8, 217u8, 146u8, 113u8, 149u8, 88u8,
							114u8, 141u8, 228u8, 105u8, 136u8, 71u8, 233u8, 18u8, 70u8, 36u8, 24u8,
							249u8,
						],
					)
				}
				pub fn sudo_send_upward_message(
					&self,
					message: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SudoSendUpwardMessage> {
					::subxt::tx::Payload::new_static(
						"ParachainSystem",
						"sudo_send_upward_message",
						types::SudoSendUpwardMessage { message },
						[
							1u8, 231u8, 11u8, 78u8, 127u8, 117u8, 248u8, 67u8, 230u8, 199u8, 126u8,
							47u8, 20u8, 62u8, 252u8, 138u8, 199u8, 48u8, 41u8, 21u8, 28u8, 157u8,
							218u8, 143u8, 4u8, 253u8, 62u8, 192u8, 94u8, 252u8, 92u8, 180u8,
						],
					)
				}
				pub fn authorize_upgrade(
					&self,
					code_hash: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::AuthorizeUpgrade> {
					::subxt::tx::Payload::new_static(
						"ParachainSystem",
						"authorize_upgrade",
						types::AuthorizeUpgrade { code_hash },
						[
							4u8, 14u8, 76u8, 107u8, 209u8, 129u8, 9u8, 39u8, 193u8, 17u8, 84u8,
							254u8, 170u8, 214u8, 24u8, 155u8, 29u8, 184u8, 249u8, 241u8, 109u8,
							58u8, 145u8, 131u8, 109u8, 63u8, 38u8, 165u8, 107u8, 215u8, 217u8,
							172u8,
						],
					)
				}
				pub fn enact_authorized_upgrade(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::EnactAuthorizedUpgrade> {
					::subxt::tx::Payload::new_static(
						"ParachainSystem",
						"enact_authorized_upgrade",
						types::EnactAuthorizedUpgrade { code },
						[
							232u8, 135u8, 114u8, 87u8, 196u8, 146u8, 244u8, 19u8, 106u8, 73u8,
							88u8, 193u8, 48u8, 14u8, 72u8, 133u8, 247u8, 147u8, 50u8, 95u8, 252u8,
							213u8, 192u8, 47u8, 244u8, 102u8, 195u8, 120u8, 179u8, 87u8, 94u8, 8u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct ValidationFunctionStored;
			impl ::subxt::events::StaticEvent for ValidationFunctionStored {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionStored";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct ValidationFunctionApplied {
				pub relay_chain_block_num: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ValidationFunctionApplied {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionApplied";
			}
			pub struct ValidationFunctionDiscarded;
			impl ::subxt::events::StaticEvent for ValidationFunctionDiscarded {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionDiscarded";
			}
			pub struct UpgradeAuthorized {
				pub code_hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for UpgradeAuthorized {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "UpgradeAuthorized";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct DownwardMessagesReceived {
				pub count: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesReceived {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesReceived";
			}
			pub struct DownwardMessagesProcessed {
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
				pub dmq_head: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesProcessed {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesProcessed";
			}
			pub struct UpwardMessageSent {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for UpwardMessageSent {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "UpwardMessageSent";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn pending_validation_code(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"PendingValidationCode",
						vec![],
						[
							78u8, 159u8, 219u8, 211u8, 177u8, 80u8, 102u8, 93u8, 83u8, 146u8, 90u8,
							233u8, 232u8, 11u8, 104u8, 172u8, 93u8, 68u8, 44u8, 228u8, 99u8, 197u8,
							254u8, 28u8, 181u8, 215u8, 247u8, 238u8, 49u8, 49u8, 195u8, 249u8,
						],
					)
				}
				pub fn new_validation_code(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"NewValidationCode",
						vec![],
						[
							185u8, 123u8, 152u8, 122u8, 230u8, 136u8, 79u8, 73u8, 206u8, 19u8,
							59u8, 57u8, 75u8, 250u8, 83u8, 185u8, 29u8, 76u8, 89u8, 137u8, 77u8,
							163u8, 25u8, 125u8, 182u8, 67u8, 2u8, 180u8, 48u8, 237u8, 49u8, 171u8,
						],
					)
				}
				pub fn validation_data(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_primitives::v2::PersistedValidationData<
						runtime_types::primitive_types::H256,
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ValidationData",
						vec![],
						[
							193u8, 240u8, 25u8, 56u8, 103u8, 173u8, 56u8, 56u8, 229u8, 243u8, 91u8,
							25u8, 249u8, 95u8, 122u8, 93u8, 37u8, 181u8, 54u8, 244u8, 217u8, 200u8,
							62u8, 136u8, 80u8, 148u8, 16u8, 177u8, 124u8, 211u8, 95u8, 24u8,
						],
					)
				}
				pub fn did_set_validation_code(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"DidSetValidationCode",
						vec![],
						[
							233u8, 228u8, 48u8, 111u8, 200u8, 35u8, 30u8, 139u8, 251u8, 77u8,
							196u8, 252u8, 35u8, 222u8, 129u8, 235u8, 7u8, 19u8, 156u8, 82u8, 126u8,
							173u8, 29u8, 62u8, 20u8, 67u8, 166u8, 116u8, 108u8, 182u8, 57u8, 246u8,
						],
					)
				}
				pub fn last_relay_chain_block_number(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"LastRelayChainBlockNumber",
						vec![],
						[
							17u8, 65u8, 131u8, 169u8, 195u8, 243u8, 195u8, 93u8, 220u8, 174u8,
							75u8, 216u8, 214u8, 227u8, 96u8, 40u8, 8u8, 153u8, 116u8, 160u8, 79u8,
							255u8, 35u8, 232u8, 242u8, 42u8, 100u8, 150u8, 208u8, 210u8, 142u8,
							186u8,
						],
					)
				}
				pub fn upgrade_restriction_signal(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::option::Option<
						runtime_types::polkadot_primitives::v2::UpgradeRestriction,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"UpgradeRestrictionSignal",
						vec![],
						[
							235u8, 240u8, 37u8, 44u8, 181u8, 52u8, 7u8, 216u8, 20u8, 139u8, 69u8,
							124u8, 21u8, 173u8, 237u8, 64u8, 105u8, 88u8, 49u8, 69u8, 123u8, 55u8,
							181u8, 167u8, 112u8, 183u8, 190u8, 231u8, 231u8, 127u8, 77u8, 148u8,
						],
					)
				}
				pub fn relay_state_proof(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_trie::storage_proof::StorageProof,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"RelayStateProof",
						vec![],
						[
							46u8, 115u8, 163u8, 190u8, 246u8, 47u8, 200u8, 159u8, 206u8, 204u8,
							94u8, 250u8, 127u8, 112u8, 109u8, 111u8, 210u8, 195u8, 244u8, 41u8,
							36u8, 187u8, 71u8, 150u8, 149u8, 253u8, 143u8, 33u8, 83u8, 189u8,
							182u8, 238u8,
						],
					)
				}				pub fn relevant_messaging_state (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"RelevantMessagingState",
						vec![],
						[
							155u8, 67u8, 173u8, 168u8, 56u8, 190u8, 228u8, 128u8, 168u8, 197u8,
							127u8, 187u8, 79u8, 48u8, 157u8, 247u8, 193u8, 75u8, 86u8, 12u8, 45u8,
							38u8, 73u8, 187u8, 107u8, 49u8, 125u8, 145u8, 102u8, 40u8, 41u8, 128u8,
						],
					)
				}
				pub fn host_configuration(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_primitives::v2::AbridgedHostConfiguration,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"HostConfiguration",
						vec![],
						[
							224u8, 81u8, 104u8, 216u8, 84u8, 180u8, 220u8, 34u8, 251u8, 192u8,
							110u8, 151u8, 172u8, 254u8, 133u8, 68u8, 16u8, 230u8, 99u8, 164u8,
							162u8, 159u8, 189u8, 125u8, 249u8, 187u8, 148u8, 253u8, 71u8, 64u8,
							89u8, 88u8,
						],
					)
				}
				pub fn last_dmq_mqc_head(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"LastDmqMqcHead",
						vec![],
						[
							1u8, 70u8, 140u8, 40u8, 51u8, 127u8, 75u8, 80u8, 5u8, 49u8, 196u8,
							31u8, 30u8, 61u8, 54u8, 252u8, 0u8, 0u8, 100u8, 115u8, 177u8, 250u8,
							138u8, 48u8, 107u8, 41u8, 93u8, 87u8, 195u8, 107u8, 206u8, 227u8,
						],
					)
				}
				pub fn last_hrmp_mqc_heads(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::collections::BTreeMap<
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"LastHrmpMqcHeads",
						vec![],
						[
							131u8, 170u8, 142u8, 30u8, 101u8, 113u8, 131u8, 81u8, 38u8, 168u8,
							98u8, 3u8, 9u8, 109u8, 96u8, 179u8, 115u8, 177u8, 128u8, 11u8, 238u8,
							54u8, 81u8, 60u8, 97u8, 112u8, 224u8, 175u8, 86u8, 133u8, 182u8, 76u8,
						],
					)
				}
				pub fn processed_downward_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ProcessedDownwardMessages",
						vec![],
						[
							151u8, 234u8, 196u8, 87u8, 130u8, 79u8, 4u8, 102u8, 47u8, 10u8, 33u8,
							132u8, 149u8, 118u8, 61u8, 141u8, 5u8, 1u8, 30u8, 120u8, 220u8, 156u8,
							16u8, 11u8, 14u8, 52u8, 126u8, 151u8, 244u8, 149u8, 197u8, 51u8,
						],
					)
				}
				pub fn hrmp_watermark(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"HrmpWatermark",
						vec![],
						[
							77u8, 62u8, 59u8, 220u8, 7u8, 125u8, 98u8, 249u8, 108u8, 212u8, 223u8,
							99u8, 152u8, 13u8, 29u8, 80u8, 166u8, 65u8, 232u8, 113u8, 145u8, 128u8,
							123u8, 35u8, 238u8, 31u8, 113u8, 156u8, 220u8, 104u8, 217u8, 165u8,
						],
					)
				}
				pub fn hrmp_outbound_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
							runtime_types::polkadot_parachain::primitives::Id,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"HrmpOutboundMessages",
						vec![],
						[
							42u8, 9u8, 96u8, 217u8, 25u8, 101u8, 129u8, 147u8, 150u8, 20u8, 164u8,
							186u8, 217u8, 178u8, 15u8, 201u8, 233u8, 104u8, 92u8, 120u8, 29u8,
							245u8, 196u8, 13u8, 141u8, 210u8, 102u8, 62u8, 216u8, 80u8, 246u8,
							145u8,
						],
					)
				}
				pub fn upward_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"UpwardMessages",
						vec![],
						[
							179u8, 127u8, 8u8, 94u8, 194u8, 246u8, 53u8, 79u8, 80u8, 22u8, 18u8,
							75u8, 116u8, 163u8, 90u8, 161u8, 30u8, 140u8, 57u8, 126u8, 60u8, 91u8,
							23u8, 30u8, 120u8, 245u8, 125u8, 96u8, 152u8, 25u8, 248u8, 85u8,
						],
					)
				}
				pub fn pending_upward_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"PendingUpwardMessages",
						vec![],
						[
							239u8, 45u8, 18u8, 173u8, 148u8, 150u8, 55u8, 176u8, 173u8, 156u8,
							246u8, 226u8, 198u8, 214u8, 104u8, 187u8, 186u8, 13u8, 83u8, 194u8,
							153u8, 29u8, 228u8, 109u8, 26u8, 18u8, 212u8, 151u8, 246u8, 24u8,
							133u8, 216u8,
						],
					)
				}
				pub fn announced_hrmp_messages_per_candidate(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"AnnouncedHrmpMessagesPerCandidate",
						vec![],
						[
							93u8, 11u8, 229u8, 172u8, 73u8, 87u8, 13u8, 149u8, 15u8, 94u8, 163u8,
							107u8, 156u8, 22u8, 131u8, 177u8, 96u8, 247u8, 213u8, 224u8, 41u8,
							126u8, 157u8, 33u8, 154u8, 194u8, 95u8, 234u8, 65u8, 19u8, 58u8, 161u8,
						],
					)
				}
				pub fn reserved_xcmp_weight_override(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_weights::weight_v2::Weight,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ReservedXcmpWeightOverride",
						vec![],
						[
							176u8, 93u8, 203u8, 74u8, 18u8, 170u8, 246u8, 203u8, 109u8, 89u8, 86u8,
							77u8, 96u8, 66u8, 189u8, 79u8, 184u8, 253u8, 11u8, 230u8, 87u8, 120u8,
							1u8, 254u8, 215u8, 41u8, 210u8, 86u8, 239u8, 206u8, 60u8, 2u8,
						],
					)
				}
				pub fn reserved_dmp_weight_override(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_weights::weight_v2::Weight,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ReservedDmpWeightOverride",
						vec![],
						[
							205u8, 124u8, 9u8, 156u8, 255u8, 207u8, 208u8, 23u8, 179u8, 132u8,
							254u8, 157u8, 237u8, 240u8, 167u8, 203u8, 253u8, 111u8, 136u8, 32u8,
							100u8, 152u8, 16u8, 19u8, 175u8, 14u8, 108u8, 61u8, 59u8, 231u8, 70u8,
							112u8,
						],
					)
				}
				pub fn authorized_upgrade(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitive_types::H256,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"AuthorizedUpgrade",
						vec![],
						[
							157u8, 239u8, 49u8, 172u8, 3u8, 215u8, 193u8, 4u8, 49u8, 10u8, 156u8,
							140u8, 52u8, 125u8, 218u8, 92u8, 229u8, 94u8, 238u8, 226u8, 4u8, 136u8,
							73u8, 163u8, 178u8, 240u8, 100u8, 150u8, 246u8, 235u8, 113u8, 73u8,
						],
					)
				}
				pub fn custom_validation_head_data(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"CustomValidationHeadData",
						vec![],
						[
							52u8, 186u8, 187u8, 57u8, 245u8, 171u8, 202u8, 23u8, 92u8, 80u8, 118u8,
							66u8, 251u8, 156u8, 175u8, 254u8, 141u8, 185u8, 115u8, 209u8, 170u8,
							165u8, 1u8, 242u8, 120u8, 234u8, 162u8, 24u8, 135u8, 105u8, 8u8, 177u8,
						],
					)
				}
			}
		}
	}
	pub mod parachain_info {
		use super::{root_mod, runtime_types};
		pub type Call = runtime_types::parachain_info::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn parachain_id(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain::primitives::Id,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainInfo",
						"ParachainId",
						vec![],
						[
							160u8, 130u8, 74u8, 181u8, 231u8, 180u8, 246u8, 152u8, 204u8, 44u8,
							245u8, 91u8, 113u8, 246u8, 218u8, 50u8, 254u8, 248u8, 35u8, 219u8,
							83u8, 144u8, 228u8, 245u8, 122u8, 53u8, 194u8, 172u8, 222u8, 118u8,
							202u8, 91u8,
						],
					)
				}
			}
		}
	}
	pub mod authorship {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn author(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Authorship",
						"Author",
						vec![],
						[
							247u8, 192u8, 118u8, 227u8, 47u8, 20u8, 203u8, 199u8, 216u8, 87u8,
							220u8, 50u8, 166u8, 61u8, 168u8, 213u8, 253u8, 62u8, 202u8, 199u8,
							61u8, 192u8, 237u8, 53u8, 22u8, 148u8, 164u8, 245u8, 99u8, 24u8, 146u8,
							18u8,
						],
					)
				}
			}
		}
	}
	pub mod collator_selection {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_collator_selection::pallet::Error;
		pub type Call = runtime_types::pallet_collator_selection::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct SetInvulnerables {
					pub new: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetInvulnerables {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_invulnerables";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct SetDesiredCandidates {
					pub max: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetDesiredCandidates {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_desired_candidates";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct SetCandidacyBond {
					pub bond: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCandidacyBond {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_candidacy_bond";
				}
				pub struct RegisterAsCandidate;
				impl ::subxt::blocks::StaticExtrinsic for RegisterAsCandidate {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "register_as_candidate";
				}
				pub struct LeaveIntent;
				impl ::subxt::blocks::StaticExtrinsic for LeaveIntent {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "leave_intent";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_invulnerables(
					&self,
					new: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::Payload<types::SetInvulnerables> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"set_invulnerables",
						types::SetInvulnerables { new },
						[
							113u8, 217u8, 14u8, 48u8, 6u8, 198u8, 8u8, 170u8, 8u8, 237u8, 230u8,
							184u8, 17u8, 181u8, 15u8, 126u8, 117u8, 3u8, 208u8, 215u8, 40u8, 16u8,
							150u8, 162u8, 37u8, 196u8, 235u8, 36u8, 247u8, 24u8, 187u8, 17u8,
						],
					)
				}
				pub fn set_desired_candidates(
					&self,
					max: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::SetDesiredCandidates> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"set_desired_candidates",
						types::SetDesiredCandidates { max },
						[
							174u8, 44u8, 232u8, 155u8, 228u8, 219u8, 239u8, 75u8, 86u8, 150u8,
							135u8, 214u8, 58u8, 9u8, 25u8, 133u8, 245u8, 101u8, 85u8, 246u8, 15u8,
							248u8, 165u8, 87u8, 88u8, 28u8, 10u8, 196u8, 86u8, 89u8, 28u8, 165u8,
						],
					)
				}
				pub fn set_candidacy_bond(
					&self,
					bond: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::SetCandidacyBond> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"set_candidacy_bond",
						types::SetCandidacyBond { bond },
						[
							250u8, 4u8, 185u8, 228u8, 101u8, 223u8, 49u8, 44u8, 172u8, 148u8,
							216u8, 242u8, 192u8, 88u8, 228u8, 59u8, 225u8, 222u8, 171u8, 40u8,
							23u8, 1u8, 46u8, 183u8, 189u8, 191u8, 156u8, 12u8, 218u8, 116u8, 76u8,
							59u8,
						],
					)
				}
				pub fn register_as_candidate(
					&self,
				) -> ::subxt::tx::Payload<types::RegisterAsCandidate> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"register_as_candidate",
						types::RegisterAsCandidate {},
						[
							69u8, 222u8, 214u8, 106u8, 105u8, 168u8, 82u8, 239u8, 158u8, 117u8,
							224u8, 89u8, 228u8, 51u8, 221u8, 244u8, 88u8, 63u8, 72u8, 119u8, 224u8,
							111u8, 93u8, 39u8, 18u8, 66u8, 72u8, 105u8, 70u8, 66u8, 178u8, 173u8,
						],
					)
				}
				pub fn leave_intent(&self) -> ::subxt::tx::Payload<types::LeaveIntent> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"leave_intent",
						types::LeaveIntent {},
						[
							126u8, 57u8, 10u8, 67u8, 120u8, 229u8, 70u8, 23u8, 154u8, 215u8, 226u8,
							178u8, 203u8, 152u8, 195u8, 177u8, 157u8, 158u8, 40u8, 17u8, 93u8,
							225u8, 253u8, 217u8, 48u8, 165u8, 55u8, 79u8, 43u8, 123u8, 193u8,
							147u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct NewInvulnerables {
				pub invulnerables: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for NewInvulnerables {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewInvulnerables";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct NewDesiredCandidates {
				pub desired_candidates: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewDesiredCandidates {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewDesiredCandidates";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct NewCandidacyBond {
				pub bond_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for NewCandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewCandidacyBond";
			}
			pub struct CandidateAdded {
				pub account_id: runtime_types::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for CandidateAdded {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateAdded";
			}
			pub struct CandidateRemoved {
				pub account_id: runtime_types::sp_core::crypto::AccountId32,
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
				pub fn invulnerables(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_core::crypto::AccountId32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"Invulnerables",
						vec![],
						[
							109u8, 180u8, 25u8, 41u8, 152u8, 158u8, 186u8, 214u8, 89u8, 222u8,
							103u8, 14u8, 91u8, 3u8, 65u8, 6u8, 255u8, 62u8, 47u8, 255u8, 132u8,
							164u8, 217u8, 200u8, 130u8, 29u8, 168u8, 23u8, 81u8, 217u8, 35u8,
							123u8,
						],
					)
				}
				pub fn candidates(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_collator_selection::pallet::CandidateInfo<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"Candidates",
						vec![],
						[
							95u8, 142u8, 119u8, 195u8, 123u8, 1u8, 212u8, 104u8, 23u8, 112u8,
							215u8, 11u8, 254u8, 30u8, 40u8, 19u8, 86u8, 187u8, 3u8, 179u8, 34u8,
							255u8, 215u8, 181u8, 162u8, 57u8, 23u8, 220u8, 223u8, 55u8, 180u8,
							88u8,
						],
					)
				}
				pub fn last_authored_block(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"LastAuthoredBlock",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							176u8, 170u8, 165u8, 244u8, 101u8, 126u8, 24u8, 132u8, 228u8, 138u8,
							72u8, 241u8, 144u8, 100u8, 79u8, 112u8, 9u8, 46u8, 210u8, 80u8, 12u8,
							126u8, 32u8, 214u8, 26u8, 171u8, 155u8, 3u8, 233u8, 22u8, 164u8, 25u8,
						],
					)
				}
				pub fn last_authored_block_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"LastAuthoredBlock",
						Vec::new(),
						[
							176u8, 170u8, 165u8, 244u8, 101u8, 126u8, 24u8, 132u8, 228u8, 138u8,
							72u8, 241u8, 144u8, 100u8, 79u8, 112u8, 9u8, 46u8, 210u8, 80u8, 12u8,
							126u8, 32u8, 214u8, 26u8, 171u8, 155u8, 3u8, 233u8, 22u8, 164u8, 25u8,
						],
					)
				}
				pub fn desired_candidates(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"DesiredCandidates",
						vec![],
						[
							69u8, 199u8, 130u8, 132u8, 10u8, 127u8, 204u8, 220u8, 59u8, 107u8,
							96u8, 180u8, 42u8, 235u8, 14u8, 126u8, 231u8, 242u8, 162u8, 126u8,
							63u8, 223u8, 15u8, 250u8, 22u8, 210u8, 54u8, 34u8, 235u8, 191u8, 250u8,
							21u8,
						],
					)
				}
				pub fn candidacy_bond(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"CandidacyBond",
						vec![],
						[
							71u8, 134u8, 156u8, 102u8, 201u8, 83u8, 240u8, 251u8, 189u8, 213u8,
							211u8, 182u8, 126u8, 122u8, 41u8, 174u8, 105u8, 29u8, 216u8, 23u8,
							255u8, 55u8, 245u8, 187u8, 234u8, 234u8, 178u8, 155u8, 145u8, 49u8,
							196u8, 214u8,
						],
					)
				}
			}
		}
	}
	pub mod session {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_session::pallet::Error;
		pub type Call = runtime_types::pallet_session::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct SetKeys {
					pub keys: runtime_types::picasso_runtime::opaque::SessionKeys,
					pub proof: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "set_keys";
				}
				pub struct PurgeKeys;
				impl ::subxt::blocks::StaticExtrinsic for PurgeKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "purge_keys";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_keys(
					&self,
					keys: runtime_types::picasso_runtime::opaque::SessionKeys,
					proof: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetKeys> {
					::subxt::tx::Payload::new_static(
						"Session",
						"set_keys",
						types::SetKeys { keys, proof },
						[
							10u8, 183u8, 202u8, 82u8, 236u8, 202u8, 212u8, 220u8, 51u8, 217u8,
							229u8, 169u8, 238u8, 141u8, 129u8, 231u8, 203u8, 176u8, 97u8, 148u8,
							240u8, 87u8, 177u8, 245u8, 33u8, 109u8, 243u8, 52u8, 46u8, 118u8,
							164u8, 35u8,
						],
					)
				}
				pub fn purge_keys(&self) -> ::subxt::tx::Payload<types::PurgeKeys> {
					::subxt::tx::Payload::new_static(
						"Session",
						"purge_keys",
						types::PurgeKeys {},
						[
							215u8, 204u8, 146u8, 236u8, 32u8, 78u8, 198u8, 79u8, 85u8, 214u8, 15u8,
							151u8, 158u8, 31u8, 146u8, 119u8, 119u8, 204u8, 151u8, 169u8, 226u8,
							67u8, 217u8, 39u8, 241u8, 245u8, 203u8, 240u8, 203u8, 172u8, 16u8,
							209u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
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
				pub fn validators(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"Validators",
						vec![],
						[
							50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
							133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
							115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
							86u8,
						],
					)
				}
				pub fn current_index(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"CurrentIndex",
						vec![],
						[
							167u8, 151u8, 125u8, 150u8, 159u8, 21u8, 78u8, 217u8, 237u8, 183u8,
							135u8, 65u8, 187u8, 114u8, 188u8, 206u8, 16u8, 32u8, 69u8, 208u8,
							134u8, 159u8, 232u8, 224u8, 243u8, 27u8, 31u8, 166u8, 145u8, 44u8,
							221u8, 230u8,
						],
					)
				}
				pub fn queued_changed(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"QueuedChanged",
						vec![],
						[
							184u8, 137u8, 224u8, 137u8, 31u8, 236u8, 95u8, 164u8, 102u8, 225u8,
							198u8, 227u8, 140u8, 37u8, 113u8, 57u8, 59u8, 4u8, 202u8, 102u8, 117u8,
							36u8, 226u8, 64u8, 113u8, 141u8, 199u8, 111u8, 99u8, 144u8, 198u8,
							153u8,
						],
					)
				}
				pub fn queued_keys(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::picasso_runtime::opaque::SessionKeys,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"QueuedKeys",
						vec![],
						[
							3u8, 214u8, 191u8, 168u8, 90u8, 94u8, 107u8, 111u8, 170u8, 31u8, 78u8,
							61u8, 240u8, 184u8, 170u8, 104u8, 178u8, 229u8, 159u8, 89u8, 207u8,
							37u8, 49u8, 209u8, 131u8, 165u8, 14u8, 169u8, 13u8, 68u8, 151u8, 144u8,
						],
					)
				}
				pub fn disabled_validators(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"DisabledValidators",
						vec![],
						[
							213u8, 19u8, 168u8, 234u8, 187u8, 200u8, 180u8, 97u8, 234u8, 189u8,
							36u8, 233u8, 158u8, 184u8, 45u8, 35u8, 129u8, 213u8, 133u8, 8u8, 104u8,
							183u8, 46u8, 68u8, 154u8, 240u8, 132u8, 22u8, 247u8, 11u8, 54u8, 221u8,
						],
					)
				}
				pub fn next_keys(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::picasso_runtime::opaque::SessionKeys,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"NextKeys",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							193u8, 216u8, 53u8, 103u8, 143u8, 241u8, 201u8, 54u8, 108u8, 149u8,
							241u8, 42u8, 3u8, 151u8, 223u8, 246u8, 30u8, 6u8, 239u8, 206u8, 27u8,
							172u8, 43u8, 226u8, 177u8, 111u8, 203u8, 78u8, 49u8, 34u8, 200u8, 6u8,
						],
					)
				}
				pub fn next_keys_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::picasso_runtime::opaque::SessionKeys,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"NextKeys",
						Vec::new(),
						[
							193u8, 216u8, 53u8, 103u8, 143u8, 241u8, 201u8, 54u8, 108u8, 149u8,
							241u8, 42u8, 3u8, 151u8, 223u8, 246u8, 30u8, 6u8, 239u8, 206u8, 27u8,
							172u8, 43u8, 226u8, 177u8, 111u8, 203u8, 78u8, 49u8, 34u8, 200u8, 6u8,
						],
					)
				}
				pub fn key_owner(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"KeyOwner",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
							253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
							253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
							206u8,
						],
					)
				}
				pub fn key_owner_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"KeyOwner",
						Vec::new(),
						[
							217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
							253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
							253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
							206u8,
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
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Aura",
						"Authorities",
						vec![],
						[
							232u8, 129u8, 167u8, 104u8, 47u8, 188u8, 238u8, 164u8, 6u8, 29u8,
							129u8, 45u8, 64u8, 182u8, 194u8, 47u8, 0u8, 73u8, 63u8, 102u8, 204u8,
							94u8, 111u8, 96u8, 137u8, 7u8, 141u8, 110u8, 180u8, 80u8, 228u8, 16u8,
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
						"Aura",
						"CurrentSlot",
						vec![],
						[
							112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8,
							236u8, 167u8, 219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8,
							201u8, 169u8, 211u8, 133u8, 135u8, 213u8, 150u8, 105u8, 60u8, 252u8,
							43u8, 57u8,
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
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"AuraExt",
						"Authorities",
						vec![],
						[
							232u8, 129u8, 167u8, 104u8, 47u8, 188u8, 238u8, 164u8, 6u8, 29u8,
							129u8, 45u8, 64u8, 182u8, 194u8, 47u8, 0u8, 73u8, 63u8, 102u8, 204u8,
							94u8, 111u8, 96u8, 137u8, 7u8, 141u8, 110u8, 180u8, 80u8, 228u8, 16u8,
						],
					)
				}
			}
		}
	}
	pub mod council {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_collective::pallet::Error;
		pub type Call = runtime_types::pallet_collective::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct SetMembers {
					pub new_members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					pub prime: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					pub old_count: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMembers {
					const PALLET: &'static str = "Council";
					const CALL: &'static str = "set_members";
				}
				pub struct Execute {
					pub proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Execute {
					const PALLET: &'static str = "Council";
					const CALL: &'static str = "execute";
				}
				pub struct Propose {
					#[codec::codec(compact)]
					pub threshold: ::core::primitive::u32,
					pub proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Propose {
					const PALLET: &'static str = "Council";
					const CALL: &'static str = "propose";
				}
				pub struct Vote {
					pub proposal: runtime_types::primitive_types::H256,
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					pub approve: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for Vote {
					const PALLET: &'static str = "Council";
					const CALL: &'static str = "vote";
				}
				pub struct CloseOldWeight {
					pub proposal_hash: runtime_types::primitive_types::H256,
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					#[codec::codec(compact)]
					pub proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for CloseOldWeight {
					const PALLET: &'static str = "Council";
					const CALL: &'static str = "close_old_weight";
				}
				pub struct DisapproveProposal {
					pub proposal_hash: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for DisapproveProposal {
					const PALLET: &'static str = "Council";
					const CALL: &'static str = "disapprove_proposal";
				}
				pub struct Close {
					pub proposal_hash: runtime_types::primitive_types::H256,
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Close {
					const PALLET: &'static str = "Council";
					const CALL: &'static str = "close";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					prime: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::SetMembers> {
					::subxt::tx::Payload::new_static(
						"Council",
						"set_members",
						types::SetMembers { new_members, prime, old_count },
						[
							66u8, 224u8, 186u8, 178u8, 41u8, 208u8, 67u8, 192u8, 57u8, 242u8,
							141u8, 31u8, 216u8, 118u8, 192u8, 43u8, 125u8, 213u8, 226u8, 85u8,
							142u8, 225u8, 131u8, 45u8, 172u8, 142u8, 12u8, 9u8, 73u8, 7u8, 218u8,
							61u8,
						],
					)
				}
				pub fn execute(
					&self,
					proposal: runtime_types::picasso_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Execute> {
					::subxt::tx::Payload::new_static(
						"Council",
						"execute",
						types::Execute { proposal: ::std::boxed::Box::new(proposal), length_bound },
						[
							8u8, 62u8, 67u8, 160u8, 212u8, 186u8, 251u8, 31u8, 81u8, 184u8, 27u8,
							240u8, 151u8, 199u8, 6u8, 120u8, 138u8, 68u8, 228u8, 161u8, 217u8,
							59u8, 237u8, 202u8, 39u8, 158u8, 245u8, 17u8, 66u8, 117u8, 68u8, 180u8,
						],
					)
				}
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::picasso_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Propose> {
					::subxt::tx::Payload::new_static(
						"Council",
						"propose",
						types::Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							44u8, 9u8, 29u8, 46u8, 210u8, 210u8, 159u8, 203u8, 193u8, 217u8, 106u8,
							125u8, 22u8, 220u8, 157u8, 88u8, 202u8, 66u8, 94u8, 194u8, 234u8, 44u8,
							1u8, 98u8, 160u8, 0u8, 140u8, 94u8, 223u8, 129u8, 30u8, 132u8,
						],
					)
				}
				pub fn vote(
					&self,
					proposal: runtime_types::primitive_types::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::Vote> {
					::subxt::tx::Payload::new_static(
						"Council",
						"vote",
						types::Vote { proposal, index, approve },
						[
							110u8, 141u8, 24u8, 33u8, 91u8, 7u8, 89u8, 198u8, 54u8, 10u8, 76u8,
							129u8, 45u8, 20u8, 216u8, 104u8, 231u8, 246u8, 174u8, 205u8, 190u8,
							176u8, 171u8, 113u8, 33u8, 37u8, 155u8, 203u8, 251u8, 34u8, 25u8,
							120u8,
						],
					)
				}
				pub fn close_old_weight(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::CloseOldWeight> {
					::subxt::tx::Payload::new_static(
						"Council",
						"close_old_weight",
						types::CloseOldWeight {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							121u8, 45u8, 63u8, 33u8, 230u8, 94u8, 125u8, 81u8, 89u8, 96u8, 247u8,
							228u8, 148u8, 166u8, 46u8, 135u8, 232u8, 107u8, 44u8, 77u8, 10u8,
							180u8, 121u8, 208u8, 127u8, 111u8, 249u8, 92u8, 108u8, 119u8, 156u8,
							220u8,
						],
					)
				}
				pub fn disapprove_proposal(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::DisapproveProposal> {
					::subxt::tx::Payload::new_static(
						"Council",
						"disapprove_proposal",
						types::DisapproveProposal { proposal_hash },
						[
							26u8, 140u8, 111u8, 193u8, 229u8, 59u8, 53u8, 196u8, 230u8, 60u8, 7u8,
							155u8, 168u8, 7u8, 201u8, 177u8, 70u8, 103u8, 190u8, 57u8, 244u8,
							156u8, 67u8, 101u8, 228u8, 6u8, 213u8, 83u8, 225u8, 95u8, 148u8, 96u8,
						],
					)
				}
				pub fn close(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Close> {
					::subxt::tx::Payload::new_static(
						"Council",
						"close",
						types::Close { proposal_hash, index, proposal_weight_bound, length_bound },
						[
							136u8, 48u8, 243u8, 34u8, 60u8, 109u8, 186u8, 158u8, 72u8, 48u8, 62u8,
							34u8, 167u8, 46u8, 33u8, 142u8, 239u8, 43u8, 238u8, 125u8, 94u8, 80u8,
							157u8, 245u8, 220u8, 126u8, 58u8, 244u8, 186u8, 195u8, 30u8, 127u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_collective::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Proposed {
				pub account: runtime_types::sp_core::crypto::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Proposed";
			}
			pub struct Voted {
				pub account: runtime_types::sp_core::crypto::AccountId32,
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Voted";
			}
			pub struct Approved {
				pub proposal_hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Approved";
			}
			pub struct Disapproved {
				pub proposal_hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Disapproved";
			}
			pub struct Executed {
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Executed";
			}
			pub struct MemberExecuted {
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "MemberExecuted";
			}
			pub struct Closed {
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Closed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Closed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn proposals(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::primitive_types::H256,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"Proposals",
						vec![],
						[
							210u8, 234u8, 7u8, 29u8, 231u8, 80u8, 17u8, 36u8, 189u8, 34u8, 175u8,
							147u8, 56u8, 92u8, 201u8, 104u8, 207u8, 150u8, 58u8, 110u8, 90u8, 28u8,
							198u8, 79u8, 236u8, 245u8, 19u8, 38u8, 68u8, 59u8, 215u8, 74u8,
						],
					)
				}
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::picasso_runtime::RuntimeCall,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"ProposalOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							109u8, 178u8, 223u8, 96u8, 191u8, 147u8, 141u8, 122u8, 119u8, 151u8,
							153u8, 178u8, 150u8, 69u8, 78u8, 72u8, 136u8, 244u8, 222u8, 64u8, 80u8,
							196u8, 76u8, 19u8, 2u8, 143u8, 158u8, 65u8, 59u8, 61u8, 110u8, 255u8,
						],
					)
				}
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::picasso_runtime::RuntimeCall,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"ProposalOf",
						Vec::new(),
						[
							109u8, 178u8, 223u8, 96u8, 191u8, 147u8, 141u8, 122u8, 119u8, 151u8,
							153u8, 178u8, 150u8, 69u8, 78u8, 72u8, 136u8, 244u8, 222u8, 64u8, 80u8,
							196u8, 76u8, 19u8, 2u8, 143u8, 158u8, 65u8, 59u8, 61u8, 110u8, 255u8,
						],
					)
				}
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"Voting",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							109u8, 198u8, 2u8, 13u8, 29u8, 14u8, 241u8, 217u8, 55u8, 147u8, 147u8,
							4u8, 176u8, 69u8, 132u8, 228u8, 158u8, 203u8, 110u8, 239u8, 158u8,
							137u8, 97u8, 46u8, 228u8, 118u8, 251u8, 201u8, 88u8, 208u8, 94u8,
							132u8,
						],
					)
				}
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"Voting",
						Vec::new(),
						[
							109u8, 198u8, 2u8, 13u8, 29u8, 14u8, 241u8, 217u8, 55u8, 147u8, 147u8,
							4u8, 176u8, 69u8, 132u8, 228u8, 158u8, 203u8, 110u8, 239u8, 158u8,
							137u8, 97u8, 46u8, 228u8, 118u8, 251u8, 201u8, 88u8, 208u8, 94u8,
							132u8,
						],
					)
				}
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"ProposalCount",
						vec![],
						[
							91u8, 238u8, 246u8, 106u8, 95u8, 66u8, 83u8, 134u8, 1u8, 225u8, 164u8,
							216u8, 113u8, 101u8, 203u8, 200u8, 113u8, 97u8, 246u8, 228u8, 140u8,
							29u8, 29u8, 48u8, 176u8, 137u8, 93u8, 230u8, 56u8, 75u8, 51u8, 149u8,
						],
					)
				}
				pub fn members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"Members",
						vec![],
						[
							16u8, 29u8, 32u8, 222u8, 175u8, 136u8, 111u8, 101u8, 43u8, 74u8, 209u8,
							81u8, 47u8, 97u8, 129u8, 39u8, 225u8, 243u8, 110u8, 229u8, 237u8, 21u8,
							90u8, 127u8, 80u8, 239u8, 156u8, 32u8, 90u8, 109u8, 179u8, 0u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"Prime",
						vec![],
						[
							72u8, 128u8, 214u8, 72u8, 78u8, 80u8, 100u8, 198u8, 114u8, 215u8, 59u8,
							3u8, 103u8, 14u8, 152u8, 202u8, 12u8, 165u8, 224u8, 10u8, 41u8, 154u8,
							77u8, 95u8, 116u8, 143u8, 250u8, 250u8, 176u8, 92u8, 238u8, 154u8,
						],
					)
				}
			}
		}
	}
	pub mod council_membership {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_membership::pallet::Error;
		pub type Call = runtime_types::pallet_membership::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct AddMember {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddMember {
					const PALLET: &'static str = "CouncilMembership";
					const CALL: &'static str = "add_member";
				}
				pub struct RemoveMember {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveMember {
					const PALLET: &'static str = "CouncilMembership";
					const CALL: &'static str = "remove_member";
				}
				pub struct SwapMember {
					pub remove: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub add: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SwapMember {
					const PALLET: &'static str = "CouncilMembership";
					const CALL: &'static str = "swap_member";
				}
				pub struct ResetMembers {
					pub members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ResetMembers {
					const PALLET: &'static str = "CouncilMembership";
					const CALL: &'static str = "reset_members";
				}
				pub struct ChangeKey {
					pub new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ChangeKey {
					const PALLET: &'static str = "CouncilMembership";
					const CALL: &'static str = "change_key";
				}
				pub struct SetPrime {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetPrime {
					const PALLET: &'static str = "CouncilMembership";
					const CALL: &'static str = "set_prime";
				}
				pub struct ClearPrime;
				impl ::subxt::blocks::StaticExtrinsic for ClearPrime {
					const PALLET: &'static str = "CouncilMembership";
					const CALL: &'static str = "clear_prime";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_member(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::AddMember> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"add_member",
						types::AddMember { who },
						[
							237u8, 212u8, 114u8, 106u8, 42u8, 1u8, 28u8, 244u8, 161u8, 140u8, 80u8,
							251u8, 217u8, 116u8, 9u8, 155u8, 234u8, 94u8, 3u8, 156u8, 5u8, 68u8,
							197u8, 90u8, 47u8, 223u8, 235u8, 200u8, 194u8, 195u8, 90u8, 70u8,
						],
					)
				}
				pub fn remove_member(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::RemoveMember> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"remove_member",
						types::RemoveMember { who },
						[
							182u8, 80u8, 149u8, 27u8, 252u8, 160u8, 231u8, 23u8, 208u8, 93u8,
							116u8, 252u8, 5u8, 28u8, 162u8, 221u8, 14u8, 150u8, 62u8, 65u8, 169u8,
							164u8, 226u8, 53u8, 152u8, 237u8, 37u8, 171u8, 209u8, 52u8, 235u8,
							11u8,
						],
					)
				}
				pub fn swap_member(
					&self,
					remove: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					add: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SwapMember> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"swap_member",
						types::SwapMember { remove, add },
						[
							11u8, 68u8, 57u8, 232u8, 242u8, 45u8, 34u8, 178u8, 250u8, 195u8, 254u8,
							22u8, 13u8, 164u8, 86u8, 193u8, 47u8, 198u8, 164u8, 143u8, 66u8, 245u8,
							222u8, 24u8, 240u8, 97u8, 241u8, 166u8, 198u8, 236u8, 148u8, 195u8,
						],
					)
				}
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::Payload<types::ResetMembers> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"reset_members",
						types::ResetMembers { members },
						[
							212u8, 144u8, 99u8, 156u8, 70u8, 4u8, 219u8, 227u8, 150u8, 25u8, 86u8,
							8u8, 215u8, 128u8, 193u8, 206u8, 33u8, 193u8, 71u8, 15u8, 20u8, 92u8,
							99u8, 89u8, 174u8, 236u8, 102u8, 82u8, 164u8, 234u8, 12u8, 45u8,
						],
					)
				}
				pub fn change_key(
					&self,
					new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::ChangeKey> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"change_key",
						types::ChangeKey { new },
						[
							189u8, 13u8, 214u8, 17u8, 2u8, 171u8, 123u8, 104u8, 227u8, 136u8,
							198u8, 88u8, 216u8, 105u8, 246u8, 70u8, 25u8, 19u8, 244u8, 19u8, 112u8,
							105u8, 14u8, 16u8, 75u8, 183u8, 85u8, 183u8, 131u8, 186u8, 129u8,
							231u8,
						],
					)
				}
				pub fn set_prime(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SetPrime> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"set_prime",
						types::SetPrime { who },
						[
							76u8, 172u8, 20u8, 105u8, 143u8, 253u8, 234u8, 255u8, 138u8, 98u8,
							81u8, 183u8, 147u8, 232u8, 49u8, 234u8, 17u8, 129u8, 20u8, 119u8,
							116u8, 214u8, 131u8, 112u8, 38u8, 21u8, 93u8, 139u8, 161u8, 242u8,
							88u8, 152u8,
						],
					)
				}
				pub fn clear_prime(&self) -> ::subxt::tx::Payload<types::ClearPrime> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"clear_prime",
						types::ClearPrime {},
						[
							71u8, 213u8, 34u8, 23u8, 186u8, 63u8, 240u8, 216u8, 190u8, 251u8, 84u8,
							109u8, 140u8, 137u8, 210u8, 211u8, 242u8, 231u8, 212u8, 133u8, 151u8,
							125u8, 25u8, 46u8, 210u8, 53u8, 133u8, 222u8, 21u8, 107u8, 120u8, 52u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_membership::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MembersReset";
			}
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "KeyChanged";
			}
			pub struct Dummy;
			impl ::subxt::events::StaticEvent for Dummy {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "Dummy";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_core::crypto::AccountId32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CouncilMembership",
						"Members",
						vec![],
						[
							109u8, 100u8, 14u8, 195u8, 213u8, 67u8, 44u8, 218u8, 84u8, 254u8, 76u8,
							80u8, 210u8, 155u8, 155u8, 30u8, 18u8, 169u8, 195u8, 92u8, 208u8,
							223u8, 242u8, 97u8, 147u8, 20u8, 168u8, 145u8, 254u8, 115u8, 225u8,
							193u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CouncilMembership",
						"Prime",
						vec![],
						[
							72u8, 128u8, 214u8, 72u8, 78u8, 80u8, 100u8, 198u8, 114u8, 215u8, 59u8,
							3u8, 103u8, 14u8, 152u8, 202u8, 12u8, 165u8, 224u8, 10u8, 41u8, 154u8,
							77u8, 95u8, 116u8, 143u8, 250u8, 250u8, 176u8, 92u8, 238u8, 154u8,
						],
					)
				}
			}
		}
	}
	pub mod treasury {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_treasury::pallet::Error;
		pub type Call = runtime_types::pallet_treasury::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct ProposeSpend {
					#[codec::codec(compact)]
					pub value: ::core::primitive::u128,
					pub beneficiary: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ProposeSpend {
					const PALLET: &'static str = "Treasury";
					const CALL: &'static str = "propose_spend";
				}
				pub struct RejectProposal {
					#[codec::codec(compact)]
					pub proposal_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for RejectProposal {
					const PALLET: &'static str = "Treasury";
					const CALL: &'static str = "reject_proposal";
				}
				pub struct ApproveProposal {
					#[codec::codec(compact)]
					pub proposal_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ApproveProposal {
					const PALLET: &'static str = "Treasury";
					const CALL: &'static str = "approve_proposal";
				}
				pub struct Spend {
					#[codec::codec(compact)]
					pub amount: ::core::primitive::u128,
					pub beneficiary: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Spend {
					const PALLET: &'static str = "Treasury";
					const CALL: &'static str = "spend";
				}
				pub struct RemoveApproval {
					#[codec::codec(compact)]
					pub proposal_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveApproval {
					const PALLET: &'static str = "Treasury";
					const CALL: &'static str = "remove_approval";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn propose_spend(
					&self,
					value: ::core::primitive::u128,
					beneficiary: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::ProposeSpend> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"propose_spend",
						types::ProposeSpend { value, beneficiary },
						[
							82u8, 0u8, 77u8, 68u8, 172u8, 126u8, 179u8, 217u8, 173u8, 214u8, 69u8,
							227u8, 243u8, 252u8, 100u8, 30u8, 205u8, 80u8, 99u8, 57u8, 63u8, 59u8,
							142u8, 81u8, 38u8, 22u8, 243u8, 165u8, 131u8, 193u8, 135u8, 171u8,
						],
					)
				}
				pub fn reject_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::RejectProposal> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"reject_proposal",
						types::RejectProposal { proposal_id },
						[
							18u8, 166u8, 80u8, 141u8, 222u8, 230u8, 4u8, 36u8, 7u8, 76u8, 12u8,
							40u8, 145u8, 114u8, 12u8, 43u8, 223u8, 78u8, 189u8, 222u8, 120u8, 80u8,
							225u8, 215u8, 119u8, 68u8, 200u8, 15u8, 25u8, 172u8, 192u8, 173u8,
						],
					)
				}
				pub fn approve_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ApproveProposal> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"approve_proposal",
						types::ApproveProposal { proposal_id },
						[
							154u8, 176u8, 152u8, 97u8, 167u8, 177u8, 78u8, 9u8, 235u8, 229u8,
							199u8, 193u8, 214u8, 3u8, 16u8, 30u8, 4u8, 104u8, 27u8, 184u8, 100u8,
							65u8, 179u8, 13u8, 91u8, 62u8, 115u8, 5u8, 219u8, 211u8, 251u8, 153u8,
						],
					)
				}
				pub fn spend(
					&self,
					amount: ::core::primitive::u128,
					beneficiary: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::Spend> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"spend",
						types::Spend { amount, beneficiary },
						[
							127u8, 149u8, 250u8, 91u8, 236u8, 214u8, 148u8, 99u8, 110u8, 217u8,
							128u8, 254u8, 102u8, 8u8, 203u8, 245u8, 101u8, 171u8, 176u8, 191u8,
							167u8, 223u8, 97u8, 235u8, 86u8, 18u8, 40u8, 41u8, 19u8, 116u8, 1u8,
							13u8,
						],
					)
				}
				pub fn remove_approval(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::RemoveApproval> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"remove_approval",
						types::RemoveApproval { proposal_id },
						[
							180u8, 20u8, 39u8, 227u8, 29u8, 228u8, 234u8, 36u8, 155u8, 114u8,
							197u8, 135u8, 185u8, 31u8, 56u8, 247u8, 224u8, 168u8, 254u8, 233u8,
							250u8, 134u8, 186u8, 155u8, 108u8, 84u8, 94u8, 226u8, 207u8, 130u8,
							196u8, 100u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_treasury::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct Proposed {
				pub proposal_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct Spending {
				pub budget_remaining: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Spending {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Spending";
			}
			pub struct Awarded {
				pub proposal_index: ::core::primitive::u32,
				pub award: ::core::primitive::u128,
				pub account: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Awarded {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Awarded";
			}
			pub struct Rejected {
				pub proposal_index: ::core::primitive::u32,
				pub slashed: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rejected {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rejected";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct Burnt {
				pub burnt_funds: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Burnt {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Burnt";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct Rollover {
				pub rollover_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rollover {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rollover";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct Deposit {
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Deposit";
			}
			pub struct SpendApproved {
				pub proposal_index: ::core::primitive::u32,
				pub amount: ::core::primitive::u128,
				pub beneficiary: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for SpendApproved {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "SpendApproved";
			}
			pub struct UpdatedInactive {
				pub reactivated: ::core::primitive::u128,
				pub deactivated: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for UpdatedInactive {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "UpdatedInactive";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Treasury",
						"ProposalCount",
						vec![],
						[
							91u8, 238u8, 246u8, 106u8, 95u8, 66u8, 83u8, 134u8, 1u8, 225u8, 164u8,
							216u8, 113u8, 101u8, 203u8, 200u8, 113u8, 97u8, 246u8, 228u8, 140u8,
							29u8, 29u8, 48u8, 176u8, 137u8, 93u8, 230u8, 56u8, 75u8, 51u8, 149u8,
						],
					)
				}
				pub fn proposals(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_treasury::Proposal<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Treasury",
						"Proposals",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							207u8, 135u8, 145u8, 146u8, 48u8, 10u8, 252u8, 40u8, 20u8, 115u8,
							205u8, 41u8, 173u8, 83u8, 115u8, 46u8, 106u8, 40u8, 130u8, 157u8,
							213u8, 87u8, 45u8, 23u8, 14u8, 167u8, 99u8, 208u8, 153u8, 163u8, 141u8,
							55u8,
						],
					)
				}
				pub fn proposals_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_treasury::Proposal<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Treasury",
						"Proposals",
						Vec::new(),
						[
							207u8, 135u8, 145u8, 146u8, 48u8, 10u8, 252u8, 40u8, 20u8, 115u8,
							205u8, 41u8, 173u8, 83u8, 115u8, 46u8, 106u8, 40u8, 130u8, 157u8,
							213u8, 87u8, 45u8, 23u8, 14u8, 167u8, 99u8, 208u8, 153u8, 163u8, 141u8,
							55u8,
						],
					)
				}
				pub fn deactivated(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Treasury",
						"Deactivated",
						vec![],
						[
							120u8, 221u8, 159u8, 56u8, 161u8, 44u8, 54u8, 233u8, 47u8, 114u8,
							170u8, 150u8, 52u8, 24u8, 137u8, 212u8, 122u8, 247u8, 40u8, 17u8,
							208u8, 130u8, 42u8, 154u8, 33u8, 222u8, 59u8, 116u8, 0u8, 15u8, 79u8,
							123u8,
						],
					)
				}
				pub fn approvals(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Treasury",
						"Approvals",
						vec![],
						[
							78u8, 147u8, 186u8, 235u8, 17u8, 40u8, 247u8, 235u8, 67u8, 222u8, 3u8,
							14u8, 248u8, 17u8, 67u8, 180u8, 93u8, 161u8, 64u8, 35u8, 119u8, 194u8,
							187u8, 226u8, 135u8, 162u8, 147u8, 174u8, 139u8, 72u8, 99u8, 212u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn proposal_bond(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Permill>
				{
					::subxt::constants::Address::new_static(
						"Treasury",
						"ProposalBond",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
				pub fn proposal_bond_minimum(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Treasury",
						"ProposalBondMinimum",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn proposal_bond_maximum(
					&self,
				) -> ::subxt::constants::Address<::core::option::Option<::core::primitive::u128>> {
					::subxt::constants::Address::new_static(
						"Treasury",
						"ProposalBondMaximum",
						[
							198u8, 51u8, 89u8, 159u8, 124u8, 251u8, 51u8, 80u8, 167u8, 193u8, 44u8,
							199u8, 80u8, 36u8, 41u8, 130u8, 137u8, 229u8, 178u8, 208u8, 37u8,
							215u8, 169u8, 183u8, 180u8, 191u8, 140u8, 240u8, 250u8, 61u8, 42u8,
							147u8,
						],
					)
				}
				pub fn spend_period(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Treasury",
						"SpendPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn burn(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Permill>
				{
					::subxt::constants::Address::new_static(
						"Treasury",
						"Burn",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"Treasury",
						"PalletId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
				pub fn max_approvals(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Treasury",
						"MaxApprovals",
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
	pub mod democracy {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_democracy::pallet::Error;
		pub type Call = runtime_types::pallet_democracy::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Propose {
					pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::picasso_runtime::RuntimeCall,
					>,
					#[codec::codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Propose {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "propose";
				}
				pub struct Second {
					#[codec::codec(compact)]
					pub proposal: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Second {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "second";
				}
				pub struct Vote {
					#[codec::codec(compact)]
					pub ref_index: ::core::primitive::u32,
					pub vote:
						runtime_types::pallet_democracy::vote::AccountVote<::core::primitive::u128>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Vote {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "vote";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct EmergencyCancel {
					pub ref_index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for EmergencyCancel {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "emergency_cancel";
				}
				pub struct ExternalPropose {
					pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::picasso_runtime::RuntimeCall,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ExternalPropose {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "external_propose";
				}
				pub struct ExternalProposeMajority {
					pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::picasso_runtime::RuntimeCall,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ExternalProposeMajority {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "external_propose_majority";
				}
				pub struct ExternalProposeDefault {
					pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::picasso_runtime::RuntimeCall,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ExternalProposeDefault {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "external_propose_default";
				}
				pub struct FastTrack {
					pub proposal_hash: runtime_types::primitive_types::H256,
					pub voting_period: ::core::primitive::u32,
					pub delay: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for FastTrack {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "fast_track";
				}
				pub struct VetoExternal {
					pub proposal_hash: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for VetoExternal {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "veto_external";
				}
				pub struct CancelReferendum {
					#[codec::codec(compact)]
					pub ref_index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for CancelReferendum {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "cancel_referendum";
				}
				pub struct Delegate {
					pub to: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub conviction: runtime_types::pallet_democracy::conviction::Conviction,
					pub balance: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Delegate {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "delegate";
				}
				pub struct Undelegate;
				impl ::subxt::blocks::StaticExtrinsic for Undelegate {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "undelegate";
				}
				pub struct ClearPublicProposals;
				impl ::subxt::blocks::StaticExtrinsic for ClearPublicProposals {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "clear_public_proposals";
				}
				pub struct Unlock {
					pub target: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Unlock {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "unlock";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct RemoveVote {
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveVote {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "remove_vote";
				}
				pub struct RemoveOtherVote {
					pub target: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveOtherVote {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "remove_other_vote";
				}
				pub struct Blacklist {
					pub proposal_hash: runtime_types::primitive_types::H256,
					pub maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Blacklist {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "blacklist";
				}
				pub struct CancelProposal {
					#[codec::codec(compact)]
					pub prop_index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for CancelProposal {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "cancel_proposal";
				}
				pub struct SetMetadata {
					pub owner: runtime_types::pallet_democracy::types::MetadataOwner,
					pub maybe_hash: ::core::option::Option<runtime_types::primitive_types::H256>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMetadata {
					const PALLET: &'static str = "Democracy";
					const CALL: &'static str = "set_metadata";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn propose(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::picasso_runtime::RuntimeCall,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Propose> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"propose",
						types::Propose { proposal, value },
						[
							164u8, 45u8, 183u8, 137u8, 222u8, 27u8, 138u8, 45u8, 20u8, 18u8, 234u8,
							211u8, 52u8, 184u8, 234u8, 222u8, 193u8, 9u8, 160u8, 58u8, 198u8,
							106u8, 236u8, 210u8, 172u8, 34u8, 194u8, 107u8, 135u8, 83u8, 22u8,
							238u8,
						],
					)
				}
				pub fn second(
					&self,
					proposal: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Second> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"second",
						types::Second { proposal },
						[
							195u8, 55u8, 178u8, 55u8, 129u8, 64u8, 10u8, 131u8, 217u8, 79u8, 1u8,
							187u8, 73u8, 126u8, 191u8, 221u8, 110u8, 10u8, 13u8, 65u8, 190u8,
							107u8, 21u8, 236u8, 175u8, 130u8, 227u8, 179u8, 173u8, 39u8, 32u8,
							147u8,
						],
					)
				}
				pub fn vote(
					&self,
					ref_index: ::core::primitive::u32,
					vote: runtime_types::pallet_democracy::vote::AccountVote<
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::Payload<types::Vote> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"vote",
						types::Vote { ref_index, vote },
						[
							106u8, 195u8, 229u8, 44u8, 217u8, 214u8, 8u8, 234u8, 175u8, 62u8, 97u8,
							83u8, 193u8, 180u8, 103u8, 26u8, 174u8, 8u8, 2u8, 158u8, 25u8, 122u8,
							203u8, 122u8, 32u8, 14u8, 107u8, 169u8, 43u8, 240u8, 143u8, 103u8,
						],
					)
				}
				pub fn emergency_cancel(
					&self,
					ref_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::EmergencyCancel> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"emergency_cancel",
						types::EmergencyCancel { ref_index },
						[
							82u8, 232u8, 19u8, 158u8, 88u8, 69u8, 96u8, 225u8, 106u8, 253u8, 6u8,
							136u8, 87u8, 0u8, 68u8, 128u8, 122u8, 16u8, 107u8, 76u8, 209u8, 14u8,
							230u8, 49u8, 228u8, 100u8, 187u8, 10u8, 76u8, 71u8, 197u8, 72u8,
						],
					)
				}
				pub fn external_propose(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::picasso_runtime::RuntimeCall,
					>,
				) -> ::subxt::tx::Payload<types::ExternalPropose> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"external_propose",
						types::ExternalPropose { proposal },
						[
							99u8, 120u8, 61u8, 124u8, 244u8, 68u8, 12u8, 240u8, 11u8, 168u8, 4u8,
							50u8, 19u8, 152u8, 255u8, 97u8, 20u8, 195u8, 141u8, 199u8, 31u8, 250u8,
							222u8, 136u8, 47u8, 162u8, 0u8, 32u8, 215u8, 110u8, 94u8, 109u8,
						],
					)
				}
				pub fn external_propose_majority(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::picasso_runtime::RuntimeCall,
					>,
				) -> ::subxt::tx::Payload<types::ExternalProposeMajority> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"external_propose_majority",
						types::ExternalProposeMajority { proposal },
						[
							35u8, 61u8, 130u8, 81u8, 81u8, 180u8, 127u8, 202u8, 67u8, 84u8, 105u8,
							113u8, 112u8, 210u8, 1u8, 191u8, 10u8, 39u8, 157u8, 164u8, 9u8, 231u8,
							75u8, 25u8, 17u8, 175u8, 128u8, 180u8, 238u8, 58u8, 236u8, 214u8,
						],
					)
				}
				pub fn external_propose_default(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::picasso_runtime::RuntimeCall,
					>,
				) -> ::subxt::tx::Payload<types::ExternalProposeDefault> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"external_propose_default",
						types::ExternalProposeDefault { proposal },
						[
							136u8, 199u8, 244u8, 69u8, 5u8, 174u8, 166u8, 251u8, 102u8, 196u8,
							25u8, 6u8, 33u8, 216u8, 141u8, 78u8, 118u8, 125u8, 128u8, 218u8, 120u8,
							170u8, 166u8, 15u8, 124u8, 216u8, 128u8, 178u8, 5u8, 74u8, 170u8, 25u8,
						],
					)
				}
				pub fn fast_track(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
					voting_period: ::core::primitive::u32,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::FastTrack> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"fast_track",
						types::FastTrack { proposal_hash, voting_period, delay },
						[
							96u8, 201u8, 216u8, 109u8, 4u8, 244u8, 52u8, 237u8, 120u8, 234u8, 30u8,
							102u8, 186u8, 132u8, 214u8, 22u8, 40u8, 75u8, 118u8, 23u8, 56u8, 68u8,
							192u8, 129u8, 74u8, 61u8, 247u8, 98u8, 103u8, 127u8, 200u8, 171u8,
						],
					)
				}
				pub fn veto_external(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::VetoExternal> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"veto_external",
						types::VetoExternal { proposal_hash },
						[
							121u8, 217u8, 249u8, 134u8, 45u8, 19u8, 126u8, 166u8, 218u8, 223u8,
							165u8, 124u8, 162u8, 59u8, 56u8, 200u8, 227u8, 125u8, 23u8, 133u8,
							196u8, 93u8, 210u8, 15u8, 39u8, 26u8, 58u8, 236u8, 9u8, 101u8, 202u8,
							168u8,
						],
					)
				}
				pub fn cancel_referendum(
					&self,
					ref_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::CancelReferendum> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"cancel_referendum",
						types::CancelReferendum { ref_index },
						[
							149u8, 120u8, 70u8, 20u8, 126u8, 21u8, 30u8, 33u8, 82u8, 124u8, 229u8,
							179u8, 169u8, 243u8, 173u8, 146u8, 140u8, 22u8, 124u8, 154u8, 228u8,
							117u8, 109u8, 88u8, 11u8, 100u8, 235u8, 243u8, 118u8, 99u8, 250u8,
							140u8,
						],
					)
				}
				pub fn delegate(
					&self,
					to: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					conviction: runtime_types::pallet_democracy::conviction::Conviction,
					balance: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Delegate> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"delegate",
						types::Delegate { to, conviction, balance },
						[
							98u8, 204u8, 103u8, 220u8, 240u8, 72u8, 17u8, 89u8, 31u8, 234u8, 53u8,
							234u8, 85u8, 150u8, 42u8, 130u8, 14u8, 164u8, 148u8, 103u8, 199u8,
							230u8, 119u8, 192u8, 95u8, 200u8, 10u8, 214u8, 48u8, 252u8, 64u8, 45u8,
						],
					)
				}
				pub fn undelegate(&self) -> ::subxt::tx::Payload<types::Undelegate> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"undelegate",
						types::Undelegate {},
						[
							225u8, 156u8, 102u8, 1u8, 172u8, 145u8, 88u8, 12u8, 89u8, 32u8, 51u8,
							83u8, 25u8, 149u8, 132u8, 203u8, 246u8, 98u8, 155u8, 36u8, 165u8,
							206u8, 233u8, 169u8, 91u8, 85u8, 105u8, 67u8, 46u8, 134u8, 244u8,
							250u8,
						],
					)
				}
				pub fn clear_public_proposals(
					&self,
				) -> ::subxt::tx::Payload<types::ClearPublicProposals> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"clear_public_proposals",
						types::ClearPublicProposals {},
						[
							116u8, 160u8, 246u8, 216u8, 23u8, 188u8, 144u8, 63u8, 97u8, 198u8,
							11u8, 243u8, 165u8, 84u8, 159u8, 153u8, 235u8, 169u8, 166u8, 15u8,
							23u8, 116u8, 30u8, 56u8, 133u8, 31u8, 158u8, 114u8, 158u8, 86u8, 106u8,
							93u8,
						],
					)
				}
				pub fn unlock(
					&self,
					target: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::Unlock> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"unlock",
						types::Unlock { target },
						[
							116u8, 108u8, 113u8, 20u8, 39u8, 227u8, 153u8, 96u8, 178u8, 223u8,
							155u8, 95u8, 111u8, 168u8, 169u8, 32u8, 230u8, 125u8, 119u8, 162u8,
							8u8, 40u8, 57u8, 237u8, 22u8, 160u8, 100u8, 203u8, 247u8, 20u8, 251u8,
							99u8,
						],
					)
				}
				pub fn remove_vote(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::RemoveVote> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"remove_vote",
						types::RemoveVote { index },
						[
							98u8, 146u8, 215u8, 63u8, 222u8, 70u8, 61u8, 186u8, 90u8, 34u8, 63u8,
							25u8, 195u8, 119u8, 228u8, 189u8, 38u8, 163u8, 58u8, 210u8, 216u8,
							156u8, 20u8, 204u8, 136u8, 192u8, 33u8, 210u8, 124u8, 65u8, 153u8,
							105u8,
						],
					)
				}
				pub fn remove_other_vote(
					&self,
					target: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::RemoveOtherVote> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"remove_other_vote",
						types::RemoveOtherVote { target, index },
						[
							71u8, 148u8, 41u8, 68u8, 78u8, 40u8, 128u8, 217u8, 49u8, 83u8, 128u8,
							13u8, 225u8, 24u8, 41u8, 69u8, 119u8, 229u8, 241u8, 178u8, 20u8, 91u8,
							1u8, 180u8, 113u8, 127u8, 8u8, 2u8, 233u8, 174u8, 192u8, 140u8,
						],
					)
				}
				pub fn blacklist(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
					maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::Payload<types::Blacklist> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"blacklist",
						types::Blacklist { proposal_hash, maybe_ref_index },
						[
							227u8, 200u8, 88u8, 154u8, 134u8, 121u8, 131u8, 177u8, 94u8, 119u8,
							12u8, 129u8, 150u8, 59u8, 108u8, 103u8, 109u8, 55u8, 220u8, 211u8,
							250u8, 103u8, 160u8, 170u8, 63u8, 142u8, 112u8, 244u8, 29u8, 238u8,
							101u8, 24u8,
						],
					)
				}
				pub fn cancel_proposal(
					&self,
					prop_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::CancelProposal> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"cancel_proposal",
						types::CancelProposal { prop_index },
						[
							213u8, 5u8, 215u8, 209u8, 71u8, 229u8, 66u8, 38u8, 171u8, 38u8, 14u8,
							103u8, 248u8, 176u8, 217u8, 143u8, 234u8, 89u8, 110u8, 250u8, 3u8,
							190u8, 151u8, 74u8, 55u8, 58u8, 249u8, 138u8, 25u8, 191u8, 55u8, 142u8,
						],
					)
				}
				pub fn set_metadata(
					&self,
					owner: runtime_types::pallet_democracy::types::MetadataOwner,
					maybe_hash: ::core::option::Option<runtime_types::primitive_types::H256>,
				) -> ::subxt::tx::Payload<types::SetMetadata> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"set_metadata",
						types::SetMetadata { owner, maybe_hash },
						[
							191u8, 200u8, 139u8, 27u8, 167u8, 250u8, 72u8, 78u8, 18u8, 98u8, 108u8,
							1u8, 122u8, 120u8, 47u8, 77u8, 174u8, 60u8, 247u8, 69u8, 228u8, 196u8,
							149u8, 107u8, 239u8, 45u8, 47u8, 118u8, 87u8, 233u8, 79u8, 29u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_democracy::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Proposed {
				pub proposal_index: ::core::primitive::u32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Proposed";
			}
			pub struct Tabled {
				pub proposal_index: ::core::primitive::u32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Tabled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Tabled";
			}
			pub struct ExternalTabled;
			impl ::subxt::events::StaticEvent for ExternalTabled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "ExternalTabled";
			}
			pub struct Started {
				pub ref_index: ::core::primitive::u32,
				pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
			}
			impl ::subxt::events::StaticEvent for Started {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Started";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct Passed {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Passed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Passed";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct NotPassed {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NotPassed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "NotPassed";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct Cancelled {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Cancelled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Cancelled";
			}
			pub struct Delegated {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub target: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Delegated {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Delegated";
			}
			pub struct Undelegated {
				pub account: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Undelegated {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Undelegated";
			}
			pub struct Vetoed {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub until: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Vetoed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Vetoed";
			}
			pub struct Blacklisted {
				pub proposal_hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for Blacklisted {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Blacklisted";
			}
			pub struct Voted {
				pub voter: runtime_types::sp_core::crypto::AccountId32,
				pub ref_index: ::core::primitive::u32,
				pub vote:
					runtime_types::pallet_democracy::vote::AccountVote<::core::primitive::u128>,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Voted";
			}
			pub struct Seconded {
				pub seconder: runtime_types::sp_core::crypto::AccountId32,
				pub prop_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Seconded {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Seconded";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct ProposalCanceled {
				pub prop_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ProposalCanceled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "ProposalCanceled";
			}
			pub struct MetadataSet {
				pub owner: runtime_types::pallet_democracy::types::MetadataOwner,
				pub hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for MetadataSet {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "MetadataSet";
			}
			pub struct MetadataCleared {
				pub owner: runtime_types::pallet_democracy::types::MetadataOwner,
				pub hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for MetadataCleared {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "MetadataCleared";
			}
			pub struct MetadataTransferred {
				pub prev_owner: runtime_types::pallet_democracy::types::MetadataOwner,
				pub owner: runtime_types::pallet_democracy::types::MetadataOwner,
				pub hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for MetadataTransferred {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "MetadataTransferred";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn public_prop_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"PublicPropCount",
						vec![],
						[
							51u8, 175u8, 184u8, 94u8, 91u8, 212u8, 100u8, 108u8, 127u8, 162u8,
							233u8, 137u8, 12u8, 209u8, 29u8, 130u8, 125u8, 179u8, 208u8, 160u8,
							173u8, 149u8, 12u8, 111u8, 1u8, 82u8, 196u8, 137u8, 51u8, 204u8, 153u8,
							198u8,
						],
					)
				}
				pub fn public_props(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u32,
						runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::picasso_runtime::RuntimeCall,
						>,
						runtime_types::sp_core::crypto::AccountId32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"PublicProps",
						vec![],
						[
							174u8, 85u8, 209u8, 117u8, 29u8, 193u8, 230u8, 16u8, 94u8, 219u8, 69u8,
							29u8, 116u8, 35u8, 252u8, 43u8, 127u8, 0u8, 43u8, 218u8, 240u8, 176u8,
							73u8, 81u8, 207u8, 131u8, 227u8, 132u8, 242u8, 45u8, 172u8, 50u8,
						],
					)
				}
				pub fn deposit_of(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::sp_core::crypto::AccountId32,
						>,
						::core::primitive::u128,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"DepositOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							115u8, 12u8, 250u8, 191u8, 201u8, 165u8, 90u8, 140u8, 101u8, 47u8,
							46u8, 3u8, 78u8, 30u8, 180u8, 22u8, 28u8, 154u8, 36u8, 99u8, 255u8,
							84u8, 33u8, 21u8, 65u8, 110u8, 52u8, 245u8, 19u8, 6u8, 104u8, 167u8,
						],
					)
				}
				pub fn deposit_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::sp_core::crypto::AccountId32,
						>,
						::core::primitive::u128,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"DepositOf",
						Vec::new(),
						[
							115u8, 12u8, 250u8, 191u8, 201u8, 165u8, 90u8, 140u8, 101u8, 47u8,
							46u8, 3u8, 78u8, 30u8, 180u8, 22u8, 28u8, 154u8, 36u8, 99u8, 255u8,
							84u8, 33u8, 21u8, 65u8, 110u8, 52u8, 245u8, 19u8, 6u8, 104u8, 167u8,
						],
					)
				}
				pub fn referendum_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"ReferendumCount",
						vec![],
						[
							64u8, 145u8, 232u8, 153u8, 121u8, 87u8, 128u8, 253u8, 170u8, 192u8,
							139u8, 18u8, 0u8, 33u8, 243u8, 11u8, 238u8, 222u8, 244u8, 5u8, 247u8,
							198u8, 149u8, 31u8, 122u8, 208u8, 86u8, 179u8, 166u8, 167u8, 93u8,
							67u8,
						],
					)
				}
				pub fn lowest_unbaked(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"LowestUnbaked",
						vec![],
						[
							237u8, 222u8, 144u8, 214u8, 0u8, 186u8, 81u8, 176u8, 51u8, 14u8, 204u8,
							184u8, 147u8, 97u8, 187u8, 84u8, 40u8, 8u8, 86u8, 241u8, 16u8, 157u8,
							202u8, 44u8, 185u8, 111u8, 70u8, 114u8, 40u8, 135u8, 1u8, 155u8,
						],
					)
				}
				pub fn referendum_info_of(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_democracy::types::ReferendumInfo<
						::core::primitive::u32,
						runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::picasso_runtime::RuntimeCall,
						>,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"ReferendumInfoOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							245u8, 152u8, 149u8, 236u8, 59u8, 164u8, 120u8, 142u8, 130u8, 25u8,
							119u8, 158u8, 103u8, 140u8, 203u8, 213u8, 110u8, 151u8, 137u8, 226u8,
							186u8, 130u8, 233u8, 245u8, 145u8, 145u8, 140u8, 54u8, 222u8, 219u8,
							234u8, 206u8,
						],
					)
				}
				pub fn referendum_info_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_democracy::types::ReferendumInfo<
						::core::primitive::u32,
						runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::picasso_runtime::RuntimeCall,
						>,
						::core::primitive::u128,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"ReferendumInfoOf",
						Vec::new(),
						[
							245u8, 152u8, 149u8, 236u8, 59u8, 164u8, 120u8, 142u8, 130u8, 25u8,
							119u8, 158u8, 103u8, 140u8, 203u8, 213u8, 110u8, 151u8, 137u8, 226u8,
							186u8, 130u8, 233u8, 245u8, 145u8, 145u8, 140u8, 54u8, 222u8, 219u8,
							234u8, 206u8,
						],
					)
				}
				pub fn voting_of(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_democracy::vote::Voting<
						::core::primitive::u128,
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"VotingOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							234u8, 35u8, 206u8, 197u8, 17u8, 251u8, 1u8, 230u8, 80u8, 235u8, 108u8,
							126u8, 82u8, 145u8, 39u8, 104u8, 209u8, 16u8, 209u8, 52u8, 165u8,
							231u8, 110u8, 92u8, 113u8, 212u8, 72u8, 57u8, 60u8, 73u8, 107u8, 118u8,
						],
					)
				}
				pub fn voting_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_democracy::vote::Voting<
						::core::primitive::u128,
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"VotingOf",
						Vec::new(),
						[
							234u8, 35u8, 206u8, 197u8, 17u8, 251u8, 1u8, 230u8, 80u8, 235u8, 108u8,
							126u8, 82u8, 145u8, 39u8, 104u8, 209u8, 16u8, 209u8, 52u8, 165u8,
							231u8, 110u8, 92u8, 113u8, 212u8, 72u8, 57u8, 60u8, 73u8, 107u8, 118u8,
						],
					)
				}
				pub fn last_tabled_was_external(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"LastTabledWasExternal",
						vec![],
						[
							162u8, 201u8, 72u8, 9u8, 78u8, 49u8, 72u8, 62u8, 240u8, 69u8, 20u8,
							135u8, 26u8, 59u8, 71u8, 46u8, 19u8, 25u8, 195u8, 11u8, 99u8, 31u8,
							104u8, 4u8, 24u8, 129u8, 47u8, 69u8, 219u8, 178u8, 104u8, 190u8,
						],
					)
				}
				pub fn next_external(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::picasso_runtime::RuntimeCall,
						>,
						runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					),
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"NextExternal",
						vec![],
						[
							240u8, 58u8, 238u8, 86u8, 35u8, 48u8, 192u8, 51u8, 91u8, 4u8, 47u8,
							202u8, 21u8, 74u8, 158u8, 64u8, 107u8, 247u8, 248u8, 240u8, 122u8,
							109u8, 204u8, 180u8, 103u8, 239u8, 156u8, 68u8, 141u8, 253u8, 131u8,
							239u8,
						],
					)
				}
				pub fn blacklist(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u32,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::sp_core::crypto::AccountId32,
						>,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"Blacklist",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							12u8, 231u8, 204u8, 151u8, 57u8, 182u8, 5u8, 74u8, 231u8, 100u8, 165u8,
							28u8, 147u8, 109u8, 119u8, 37u8, 138u8, 159u8, 7u8, 175u8, 41u8, 110u8,
							205u8, 69u8, 17u8, 9u8, 39u8, 102u8, 90u8, 244u8, 165u8, 141u8,
						],
					)
				}
				pub fn blacklist_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u32,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::sp_core::crypto::AccountId32,
						>,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"Blacklist",
						Vec::new(),
						[
							12u8, 231u8, 204u8, 151u8, 57u8, 182u8, 5u8, 74u8, 231u8, 100u8, 165u8,
							28u8, 147u8, 109u8, 119u8, 37u8, 138u8, 159u8, 7u8, 175u8, 41u8, 110u8,
							205u8, 69u8, 17u8, 9u8, 39u8, 102u8, 90u8, 244u8, 165u8, 141u8,
						],
					)
				}
				pub fn cancellations(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"Cancellations",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							80u8, 190u8, 98u8, 105u8, 129u8, 25u8, 167u8, 180u8, 74u8, 128u8,
							232u8, 29u8, 193u8, 209u8, 185u8, 60u8, 18u8, 180u8, 59u8, 192u8,
							149u8, 13u8, 123u8, 232u8, 34u8, 208u8, 48u8, 104u8, 35u8, 181u8,
							186u8, 244u8,
						],
					)
				}
				pub fn cancellations_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"Cancellations",
						Vec::new(),
						[
							80u8, 190u8, 98u8, 105u8, 129u8, 25u8, 167u8, 180u8, 74u8, 128u8,
							232u8, 29u8, 193u8, 209u8, 185u8, 60u8, 18u8, 180u8, 59u8, 192u8,
							149u8, 13u8, 123u8, 232u8, 34u8, 208u8, 48u8, 104u8, 35u8, 181u8,
							186u8, 244u8,
						],
					)
				}
				pub fn metadata_of(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::pallet_democracy::types::MetadataOwner,
					>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitive_types::H256,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"MetadataOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							52u8, 151u8, 124u8, 110u8, 85u8, 173u8, 181u8, 86u8, 174u8, 183u8,
							102u8, 22u8, 8u8, 36u8, 224u8, 114u8, 98u8, 0u8, 220u8, 215u8, 19u8,
							147u8, 32u8, 238u8, 242u8, 187u8, 235u8, 163u8, 183u8, 235u8, 9u8,
							180u8,
						],
					)
				}
				pub fn metadata_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::primitive_types::H256,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"MetadataOf",
						Vec::new(),
						[
							52u8, 151u8, 124u8, 110u8, 85u8, 173u8, 181u8, 86u8, 174u8, 183u8,
							102u8, 22u8, 8u8, 36u8, 224u8, 114u8, 98u8, 0u8, 220u8, 215u8, 19u8,
							147u8, 32u8, 238u8, 242u8, 187u8, 235u8, 163u8, 183u8, 235u8, 9u8,
							180u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn enactment_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"EnactmentPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn launch_period(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"LaunchPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn voting_period(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"VotingPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn vote_locking_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"VoteLockingPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn minimum_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"MinimumDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn instant_allowed(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::bool> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"InstantAllowed",
						[
							165u8, 28u8, 112u8, 190u8, 18u8, 129u8, 182u8, 206u8, 237u8, 1u8, 68u8,
							252u8, 125u8, 234u8, 185u8, 50u8, 149u8, 164u8, 47u8, 126u8, 134u8,
							100u8, 14u8, 86u8, 209u8, 39u8, 20u8, 4u8, 233u8, 115u8, 102u8, 131u8,
						],
					)
				}
				pub fn fast_track_voting_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"FastTrackVotingPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn cooloff_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"CooloffPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_votes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"MaxVotes",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_proposals(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"MaxProposals",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_deposits(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"MaxDeposits",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_blacklisted(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Democracy",
						"MaxBlacklisted",
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
	pub mod technical_committee {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_collective::pallet::Error2;
		pub type Call = runtime_types::pallet_collective::pallet::Call2;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct SetMembers {
					pub new_members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					pub prime: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					pub old_count: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMembers {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "set_members";
				}
				pub struct Execute {
					pub proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Execute {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "execute";
				}
				pub struct Propose {
					#[codec::codec(compact)]
					pub threshold: ::core::primitive::u32,
					pub proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Propose {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "propose";
				}
				pub struct Vote {
					pub proposal: runtime_types::primitive_types::H256,
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					pub approve: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for Vote {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "vote";
				}
				pub struct CloseOldWeight {
					pub proposal_hash: runtime_types::primitive_types::H256,
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					#[codec::codec(compact)]
					pub proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for CloseOldWeight {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "close_old_weight";
				}
				pub struct DisapproveProposal {
					pub proposal_hash: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for DisapproveProposal {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "disapprove_proposal";
				}
				pub struct Close {
					pub proposal_hash: runtime_types::primitive_types::H256,
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Close {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "close";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					prime: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::SetMembers> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"set_members",
						types::SetMembers { new_members, prime, old_count },
						[
							66u8, 224u8, 186u8, 178u8, 41u8, 208u8, 67u8, 192u8, 57u8, 242u8,
							141u8, 31u8, 216u8, 118u8, 192u8, 43u8, 125u8, 213u8, 226u8, 85u8,
							142u8, 225u8, 131u8, 45u8, 172u8, 142u8, 12u8, 9u8, 73u8, 7u8, 218u8,
							61u8,
						],
					)
				}
				pub fn execute(
					&self,
					proposal: runtime_types::picasso_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Execute> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"execute",
						types::Execute { proposal: ::std::boxed::Box::new(proposal), length_bound },
						[
							8u8, 62u8, 67u8, 160u8, 212u8, 186u8, 251u8, 31u8, 81u8, 184u8, 27u8,
							240u8, 151u8, 199u8, 6u8, 120u8, 138u8, 68u8, 228u8, 161u8, 217u8,
							59u8, 237u8, 202u8, 39u8, 158u8, 245u8, 17u8, 66u8, 117u8, 68u8, 180u8,
						],
					)
				}
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::picasso_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Propose> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"propose",
						types::Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							44u8, 9u8, 29u8, 46u8, 210u8, 210u8, 159u8, 203u8, 193u8, 217u8, 106u8,
							125u8, 22u8, 220u8, 157u8, 88u8, 202u8, 66u8, 94u8, 194u8, 234u8, 44u8,
							1u8, 98u8, 160u8, 0u8, 140u8, 94u8, 223u8, 129u8, 30u8, 132u8,
						],
					)
				}
				pub fn vote(
					&self,
					proposal: runtime_types::primitive_types::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::Vote> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"vote",
						types::Vote { proposal, index, approve },
						[
							110u8, 141u8, 24u8, 33u8, 91u8, 7u8, 89u8, 198u8, 54u8, 10u8, 76u8,
							129u8, 45u8, 20u8, 216u8, 104u8, 231u8, 246u8, 174u8, 205u8, 190u8,
							176u8, 171u8, 113u8, 33u8, 37u8, 155u8, 203u8, 251u8, 34u8, 25u8,
							120u8,
						],
					)
				}
				pub fn close_old_weight(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::CloseOldWeight> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"close_old_weight",
						types::CloseOldWeight {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							121u8, 45u8, 63u8, 33u8, 230u8, 94u8, 125u8, 81u8, 89u8, 96u8, 247u8,
							228u8, 148u8, 166u8, 46u8, 135u8, 232u8, 107u8, 44u8, 77u8, 10u8,
							180u8, 121u8, 208u8, 127u8, 111u8, 249u8, 92u8, 108u8, 119u8, 156u8,
							220u8,
						],
					)
				}
				pub fn disapprove_proposal(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::DisapproveProposal> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"disapprove_proposal",
						types::DisapproveProposal { proposal_hash },
						[
							26u8, 140u8, 111u8, 193u8, 229u8, 59u8, 53u8, 196u8, 230u8, 60u8, 7u8,
							155u8, 168u8, 7u8, 201u8, 177u8, 70u8, 103u8, 190u8, 57u8, 244u8,
							156u8, 67u8, 101u8, 228u8, 6u8, 213u8, 83u8, 225u8, 95u8, 148u8, 96u8,
						],
					)
				}
				pub fn close(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Close> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"close",
						types::Close { proposal_hash, index, proposal_weight_bound, length_bound },
						[
							136u8, 48u8, 243u8, 34u8, 60u8, 109u8, 186u8, 158u8, 72u8, 48u8, 62u8,
							34u8, 167u8, 46u8, 33u8, 142u8, 239u8, 43u8, 238u8, 125u8, 94u8, 80u8,
							157u8, 245u8, 220u8, 126u8, 58u8, 244u8, 186u8, 195u8, 30u8, 127u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_collective::pallet::Event2;
		pub mod events {
			use super::runtime_types;
			pub struct Proposed {
				pub account: runtime_types::sp_core::crypto::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Proposed";
			}
			pub struct Voted {
				pub account: runtime_types::sp_core::crypto::AccountId32,
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Voted";
			}
			pub struct Approved {
				pub proposal_hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Approved";
			}
			pub struct Disapproved {
				pub proposal_hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Disapproved";
			}
			pub struct Executed {
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Executed";
			}
			pub struct MemberExecuted {
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "MemberExecuted";
			}
			pub struct Closed {
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Closed {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Closed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn proposals(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::primitive_types::H256,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Proposals",
						vec![],
						[
							210u8, 234u8, 7u8, 29u8, 231u8, 80u8, 17u8, 36u8, 189u8, 34u8, 175u8,
							147u8, 56u8, 92u8, 201u8, 104u8, 207u8, 150u8, 58u8, 110u8, 90u8, 28u8,
							198u8, 79u8, 236u8, 245u8, 19u8, 38u8, 68u8, 59u8, 215u8, 74u8,
						],
					)
				}
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::picasso_runtime::RuntimeCall,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"ProposalOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							109u8, 178u8, 223u8, 96u8, 191u8, 147u8, 141u8, 122u8, 119u8, 151u8,
							153u8, 178u8, 150u8, 69u8, 78u8, 72u8, 136u8, 244u8, 222u8, 64u8, 80u8,
							196u8, 76u8, 19u8, 2u8, 143u8, 158u8, 65u8, 59u8, 61u8, 110u8, 255u8,
						],
					)
				}
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::picasso_runtime::RuntimeCall,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"ProposalOf",
						Vec::new(),
						[
							109u8, 178u8, 223u8, 96u8, 191u8, 147u8, 141u8, 122u8, 119u8, 151u8,
							153u8, 178u8, 150u8, 69u8, 78u8, 72u8, 136u8, 244u8, 222u8, 64u8, 80u8,
							196u8, 76u8, 19u8, 2u8, 143u8, 158u8, 65u8, 59u8, 61u8, 110u8, 255u8,
						],
					)
				}
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Voting",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							109u8, 198u8, 2u8, 13u8, 29u8, 14u8, 241u8, 217u8, 55u8, 147u8, 147u8,
							4u8, 176u8, 69u8, 132u8, 228u8, 158u8, 203u8, 110u8, 239u8, 158u8,
							137u8, 97u8, 46u8, 228u8, 118u8, 251u8, 201u8, 88u8, 208u8, 94u8,
							132u8,
						],
					)
				}
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Voting",
						Vec::new(),
						[
							109u8, 198u8, 2u8, 13u8, 29u8, 14u8, 241u8, 217u8, 55u8, 147u8, 147u8,
							4u8, 176u8, 69u8, 132u8, 228u8, 158u8, 203u8, 110u8, 239u8, 158u8,
							137u8, 97u8, 46u8, 228u8, 118u8, 251u8, 201u8, 88u8, 208u8, 94u8,
							132u8,
						],
					)
				}
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"ProposalCount",
						vec![],
						[
							91u8, 238u8, 246u8, 106u8, 95u8, 66u8, 83u8, 134u8, 1u8, 225u8, 164u8,
							216u8, 113u8, 101u8, 203u8, 200u8, 113u8, 97u8, 246u8, 228u8, 140u8,
							29u8, 29u8, 48u8, 176u8, 137u8, 93u8, 230u8, 56u8, 75u8, 51u8, 149u8,
						],
					)
				}
				pub fn members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Members",
						vec![],
						[
							16u8, 29u8, 32u8, 222u8, 175u8, 136u8, 111u8, 101u8, 43u8, 74u8, 209u8,
							81u8, 47u8, 97u8, 129u8, 39u8, 225u8, 243u8, 110u8, 229u8, 237u8, 21u8,
							90u8, 127u8, 80u8, 239u8, 156u8, 32u8, 90u8, 109u8, 179u8, 0u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Prime",
						vec![],
						[
							72u8, 128u8, 214u8, 72u8, 78u8, 80u8, 100u8, 198u8, 114u8, 215u8, 59u8,
							3u8, 103u8, 14u8, 152u8, 202u8, 12u8, 165u8, 224u8, 10u8, 41u8, 154u8,
							77u8, 95u8, 116u8, 143u8, 250u8, 250u8, 176u8, 92u8, 238u8, 154u8,
						],
					)
				}
			}
		}
	}
	pub mod technical_committee_membership {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_membership::pallet::Error2;
		pub type Call = runtime_types::pallet_membership::pallet::Call2;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct AddMember {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddMember {
					const PALLET: &'static str = "TechnicalCommitteeMembership";
					const CALL: &'static str = "add_member";
				}
				pub struct RemoveMember {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveMember {
					const PALLET: &'static str = "TechnicalCommitteeMembership";
					const CALL: &'static str = "remove_member";
				}
				pub struct SwapMember {
					pub remove: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub add: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SwapMember {
					const PALLET: &'static str = "TechnicalCommitteeMembership";
					const CALL: &'static str = "swap_member";
				}
				pub struct ResetMembers {
					pub members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ResetMembers {
					const PALLET: &'static str = "TechnicalCommitteeMembership";
					const CALL: &'static str = "reset_members";
				}
				pub struct ChangeKey {
					pub new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ChangeKey {
					const PALLET: &'static str = "TechnicalCommitteeMembership";
					const CALL: &'static str = "change_key";
				}
				pub struct SetPrime {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetPrime {
					const PALLET: &'static str = "TechnicalCommitteeMembership";
					const CALL: &'static str = "set_prime";
				}
				pub struct ClearPrime;
				impl ::subxt::blocks::StaticExtrinsic for ClearPrime {
					const PALLET: &'static str = "TechnicalCommitteeMembership";
					const CALL: &'static str = "clear_prime";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_member(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::AddMember> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"add_member",
						types::AddMember { who },
						[
							237u8, 212u8, 114u8, 106u8, 42u8, 1u8, 28u8, 244u8, 161u8, 140u8, 80u8,
							251u8, 217u8, 116u8, 9u8, 155u8, 234u8, 94u8, 3u8, 156u8, 5u8, 68u8,
							197u8, 90u8, 47u8, 223u8, 235u8, 200u8, 194u8, 195u8, 90u8, 70u8,
						],
					)
				}
				pub fn remove_member(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::RemoveMember> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"remove_member",
						types::RemoveMember { who },
						[
							182u8, 80u8, 149u8, 27u8, 252u8, 160u8, 231u8, 23u8, 208u8, 93u8,
							116u8, 252u8, 5u8, 28u8, 162u8, 221u8, 14u8, 150u8, 62u8, 65u8, 169u8,
							164u8, 226u8, 53u8, 152u8, 237u8, 37u8, 171u8, 209u8, 52u8, 235u8,
							11u8,
						],
					)
				}
				pub fn swap_member(
					&self,
					remove: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					add: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SwapMember> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"swap_member",
						types::SwapMember { remove, add },
						[
							11u8, 68u8, 57u8, 232u8, 242u8, 45u8, 34u8, 178u8, 250u8, 195u8, 254u8,
							22u8, 13u8, 164u8, 86u8, 193u8, 47u8, 198u8, 164u8, 143u8, 66u8, 245u8,
							222u8, 24u8, 240u8, 97u8, 241u8, 166u8, 198u8, 236u8, 148u8, 195u8,
						],
					)
				}
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::Payload<types::ResetMembers> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"reset_members",
						types::ResetMembers { members },
						[
							212u8, 144u8, 99u8, 156u8, 70u8, 4u8, 219u8, 227u8, 150u8, 25u8, 86u8,
							8u8, 215u8, 128u8, 193u8, 206u8, 33u8, 193u8, 71u8, 15u8, 20u8, 92u8,
							99u8, 89u8, 174u8, 236u8, 102u8, 82u8, 164u8, 234u8, 12u8, 45u8,
						],
					)
				}
				pub fn change_key(
					&self,
					new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::ChangeKey> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"change_key",
						types::ChangeKey { new },
						[
							189u8, 13u8, 214u8, 17u8, 2u8, 171u8, 123u8, 104u8, 227u8, 136u8,
							198u8, 88u8, 216u8, 105u8, 246u8, 70u8, 25u8, 19u8, 244u8, 19u8, 112u8,
							105u8, 14u8, 16u8, 75u8, 183u8, 85u8, 183u8, 131u8, 186u8, 129u8,
							231u8,
						],
					)
				}
				pub fn set_prime(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SetPrime> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"set_prime",
						types::SetPrime { who },
						[
							76u8, 172u8, 20u8, 105u8, 143u8, 253u8, 234u8, 255u8, 138u8, 98u8,
							81u8, 183u8, 147u8, 232u8, 49u8, 234u8, 17u8, 129u8, 20u8, 119u8,
							116u8, 214u8, 131u8, 112u8, 38u8, 21u8, 93u8, 139u8, 161u8, 242u8,
							88u8, 152u8,
						],
					)
				}
				pub fn clear_prime(&self) -> ::subxt::tx::Payload<types::ClearPrime> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"clear_prime",
						types::ClearPrime {},
						[
							71u8, 213u8, 34u8, 23u8, 186u8, 63u8, 240u8, 216u8, 190u8, 251u8, 84u8,
							109u8, 140u8, 137u8, 210u8, 211u8, 242u8, 231u8, 212u8, 133u8, 151u8,
							125u8, 25u8, 46u8, 210u8, 53u8, 133u8, 222u8, 21u8, 107u8, 120u8, 52u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_membership::pallet::Event2;
		pub mod events {
			use super::runtime_types;
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MembersReset";
			}
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "KeyChanged";
			}
			pub struct Dummy;
			impl ::subxt::events::StaticEvent for Dummy {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "Dummy";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_core::crypto::AccountId32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommitteeMembership",
						"Members",
						vec![],
						[
							109u8, 100u8, 14u8, 195u8, 213u8, 67u8, 44u8, 218u8, 84u8, 254u8, 76u8,
							80u8, 210u8, 155u8, 155u8, 30u8, 18u8, 169u8, 195u8, 92u8, 208u8,
							223u8, 242u8, 97u8, 147u8, 20u8, 168u8, 145u8, 254u8, 115u8, 225u8,
							193u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommitteeMembership",
						"Prime",
						vec![],
						[
							72u8, 128u8, 214u8, 72u8, 78u8, 80u8, 100u8, 198u8, 114u8, 215u8, 59u8,
							3u8, 103u8, 14u8, 152u8, 202u8, 12u8, 165u8, 224u8, 10u8, 41u8, 154u8,
							77u8, 95u8, 116u8, 143u8, 250u8, 250u8, 176u8, 92u8, 238u8, 154u8,
						],
					)
				}
			}
		}
	}
	pub mod release_committee {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_collective::pallet::Error3;
		pub type Call = runtime_types::pallet_collective::pallet::Call3;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct SetMembers {
					pub new_members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					pub prime: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					pub old_count: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMembers {
					const PALLET: &'static str = "ReleaseCommittee";
					const CALL: &'static str = "set_members";
				}
				pub struct Execute {
					pub proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Execute {
					const PALLET: &'static str = "ReleaseCommittee";
					const CALL: &'static str = "execute";
				}
				pub struct Propose {
					#[codec::codec(compact)]
					pub threshold: ::core::primitive::u32,
					pub proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Propose {
					const PALLET: &'static str = "ReleaseCommittee";
					const CALL: &'static str = "propose";
				}
				pub struct Vote {
					pub proposal: runtime_types::primitive_types::H256,
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					pub approve: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for Vote {
					const PALLET: &'static str = "ReleaseCommittee";
					const CALL: &'static str = "vote";
				}
				pub struct CloseOldWeight {
					pub proposal_hash: runtime_types::primitive_types::H256,
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					#[codec::codec(compact)]
					pub proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for CloseOldWeight {
					const PALLET: &'static str = "ReleaseCommittee";
					const CALL: &'static str = "close_old_weight";
				}
				pub struct DisapproveProposal {
					pub proposal_hash: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for DisapproveProposal {
					const PALLET: &'static str = "ReleaseCommittee";
					const CALL: &'static str = "disapprove_proposal";
				}
				pub struct Close {
					pub proposal_hash: runtime_types::primitive_types::H256,
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					#[codec::codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Close {
					const PALLET: &'static str = "ReleaseCommittee";
					const CALL: &'static str = "close";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					prime: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::SetMembers> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"set_members",
						types::SetMembers { new_members, prime, old_count },
						[
							66u8, 224u8, 186u8, 178u8, 41u8, 208u8, 67u8, 192u8, 57u8, 242u8,
							141u8, 31u8, 216u8, 118u8, 192u8, 43u8, 125u8, 213u8, 226u8, 85u8,
							142u8, 225u8, 131u8, 45u8, 172u8, 142u8, 12u8, 9u8, 73u8, 7u8, 218u8,
							61u8,
						],
					)
				}
				pub fn execute(
					&self,
					proposal: runtime_types::picasso_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Execute> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"execute",
						types::Execute { proposal: ::std::boxed::Box::new(proposal), length_bound },
						[
							8u8, 62u8, 67u8, 160u8, 212u8, 186u8, 251u8, 31u8, 81u8, 184u8, 27u8,
							240u8, 151u8, 199u8, 6u8, 120u8, 138u8, 68u8, 228u8, 161u8, 217u8,
							59u8, 237u8, 202u8, 39u8, 158u8, 245u8, 17u8, 66u8, 117u8, 68u8, 180u8,
						],
					)
				}
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::picasso_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Propose> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"propose",
						types::Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							44u8, 9u8, 29u8, 46u8, 210u8, 210u8, 159u8, 203u8, 193u8, 217u8, 106u8,
							125u8, 22u8, 220u8, 157u8, 88u8, 202u8, 66u8, 94u8, 194u8, 234u8, 44u8,
							1u8, 98u8, 160u8, 0u8, 140u8, 94u8, 223u8, 129u8, 30u8, 132u8,
						],
					)
				}
				pub fn vote(
					&self,
					proposal: runtime_types::primitive_types::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::Vote> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"vote",
						types::Vote { proposal, index, approve },
						[
							110u8, 141u8, 24u8, 33u8, 91u8, 7u8, 89u8, 198u8, 54u8, 10u8, 76u8,
							129u8, 45u8, 20u8, 216u8, 104u8, 231u8, 246u8, 174u8, 205u8, 190u8,
							176u8, 171u8, 113u8, 33u8, 37u8, 155u8, 203u8, 251u8, 34u8, 25u8,
							120u8,
						],
					)
				}
				pub fn close_old_weight(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::CloseOldWeight> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"close_old_weight",
						types::CloseOldWeight {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							121u8, 45u8, 63u8, 33u8, 230u8, 94u8, 125u8, 81u8, 89u8, 96u8, 247u8,
							228u8, 148u8, 166u8, 46u8, 135u8, 232u8, 107u8, 44u8, 77u8, 10u8,
							180u8, 121u8, 208u8, 127u8, 111u8, 249u8, 92u8, 108u8, 119u8, 156u8,
							220u8,
						],
					)
				}
				pub fn disapprove_proposal(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::DisapproveProposal> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"disapprove_proposal",
						types::DisapproveProposal { proposal_hash },
						[
							26u8, 140u8, 111u8, 193u8, 229u8, 59u8, 53u8, 196u8, 230u8, 60u8, 7u8,
							155u8, 168u8, 7u8, 201u8, 177u8, 70u8, 103u8, 190u8, 57u8, 244u8,
							156u8, 67u8, 101u8, 228u8, 6u8, 213u8, 83u8, 225u8, 95u8, 148u8, 96u8,
						],
					)
				}
				pub fn close(
					&self,
					proposal_hash: runtime_types::primitive_types::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Close> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"close",
						types::Close { proposal_hash, index, proposal_weight_bound, length_bound },
						[
							136u8, 48u8, 243u8, 34u8, 60u8, 109u8, 186u8, 158u8, 72u8, 48u8, 62u8,
							34u8, 167u8, 46u8, 33u8, 142u8, 239u8, 43u8, 238u8, 125u8, 94u8, 80u8,
							157u8, 245u8, 220u8, 126u8, 58u8, 244u8, 186u8, 195u8, 30u8, 127u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_collective::pallet::Event3;
		pub mod events {
			use super::runtime_types;
			pub struct Proposed {
				pub account: runtime_types::sp_core::crypto::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Proposed";
			}
			pub struct Voted {
				pub account: runtime_types::sp_core::crypto::AccountId32,
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Voted";
			}
			pub struct Approved {
				pub proposal_hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Approved";
			}
			pub struct Disapproved {
				pub proposal_hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Disapproved";
			}
			pub struct Executed {
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Executed";
			}
			pub struct MemberExecuted {
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "MemberExecuted";
			}
			pub struct Closed {
				pub proposal_hash: runtime_types::primitive_types::H256,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Closed {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Closed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn proposals(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::primitive_types::H256,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"Proposals",
						vec![],
						[
							210u8, 234u8, 7u8, 29u8, 231u8, 80u8, 17u8, 36u8, 189u8, 34u8, 175u8,
							147u8, 56u8, 92u8, 201u8, 104u8, 207u8, 150u8, 58u8, 110u8, 90u8, 28u8,
							198u8, 79u8, 236u8, 245u8, 19u8, 38u8, 68u8, 59u8, 215u8, 74u8,
						],
					)
				}
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::picasso_runtime::RuntimeCall,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"ProposalOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							109u8, 178u8, 223u8, 96u8, 191u8, 147u8, 141u8, 122u8, 119u8, 151u8,
							153u8, 178u8, 150u8, 69u8, 78u8, 72u8, 136u8, 244u8, 222u8, 64u8, 80u8,
							196u8, 76u8, 19u8, 2u8, 143u8, 158u8, 65u8, 59u8, 61u8, 110u8, 255u8,
						],
					)
				}
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::picasso_runtime::RuntimeCall,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"ProposalOf",
						Vec::new(),
						[
							109u8, 178u8, 223u8, 96u8, 191u8, 147u8, 141u8, 122u8, 119u8, 151u8,
							153u8, 178u8, 150u8, 69u8, 78u8, 72u8, 136u8, 244u8, 222u8, 64u8, 80u8,
							196u8, 76u8, 19u8, 2u8, 143u8, 158u8, 65u8, 59u8, 61u8, 110u8, 255u8,
						],
					)
				}
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"Voting",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							109u8, 198u8, 2u8, 13u8, 29u8, 14u8, 241u8, 217u8, 55u8, 147u8, 147u8,
							4u8, 176u8, 69u8, 132u8, 228u8, 158u8, 203u8, 110u8, 239u8, 158u8,
							137u8, 97u8, 46u8, 228u8, 118u8, 251u8, 201u8, 88u8, 208u8, 94u8,
							132u8,
						],
					)
				}
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"Voting",
						Vec::new(),
						[
							109u8, 198u8, 2u8, 13u8, 29u8, 14u8, 241u8, 217u8, 55u8, 147u8, 147u8,
							4u8, 176u8, 69u8, 132u8, 228u8, 158u8, 203u8, 110u8, 239u8, 158u8,
							137u8, 97u8, 46u8, 228u8, 118u8, 251u8, 201u8, 88u8, 208u8, 94u8,
							132u8,
						],
					)
				}
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"ProposalCount",
						vec![],
						[
							91u8, 238u8, 246u8, 106u8, 95u8, 66u8, 83u8, 134u8, 1u8, 225u8, 164u8,
							216u8, 113u8, 101u8, 203u8, 200u8, 113u8, 97u8, 246u8, 228u8, 140u8,
							29u8, 29u8, 48u8, 176u8, 137u8, 93u8, 230u8, 56u8, 75u8, 51u8, 149u8,
						],
					)
				}
				pub fn members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"Members",
						vec![],
						[
							16u8, 29u8, 32u8, 222u8, 175u8, 136u8, 111u8, 101u8, 43u8, 74u8, 209u8,
							81u8, 47u8, 97u8, 129u8, 39u8, 225u8, 243u8, 110u8, 229u8, 237u8, 21u8,
							90u8, 127u8, 80u8, 239u8, 156u8, 32u8, 90u8, 109u8, 179u8, 0u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"Prime",
						vec![],
						[
							72u8, 128u8, 214u8, 72u8, 78u8, 80u8, 100u8, 198u8, 114u8, 215u8, 59u8,
							3u8, 103u8, 14u8, 152u8, 202u8, 12u8, 165u8, 224u8, 10u8, 41u8, 154u8,
							77u8, 95u8, 116u8, 143u8, 250u8, 250u8, 176u8, 92u8, 238u8, 154u8,
						],
					)
				}
			}
		}
	}
	pub mod release_membership {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_membership::pallet::Error3;
		pub type Call = runtime_types::pallet_membership::pallet::Call3;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct AddMember {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddMember {
					const PALLET: &'static str = "ReleaseMembership";
					const CALL: &'static str = "add_member";
				}
				pub struct RemoveMember {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveMember {
					const PALLET: &'static str = "ReleaseMembership";
					const CALL: &'static str = "remove_member";
				}
				pub struct SwapMember {
					pub remove: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub add: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SwapMember {
					const PALLET: &'static str = "ReleaseMembership";
					const CALL: &'static str = "swap_member";
				}
				pub struct ResetMembers {
					pub members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ResetMembers {
					const PALLET: &'static str = "ReleaseMembership";
					const CALL: &'static str = "reset_members";
				}
				pub struct ChangeKey {
					pub new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ChangeKey {
					const PALLET: &'static str = "ReleaseMembership";
					const CALL: &'static str = "change_key";
				}
				pub struct SetPrime {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetPrime {
					const PALLET: &'static str = "ReleaseMembership";
					const CALL: &'static str = "set_prime";
				}
				pub struct ClearPrime;
				impl ::subxt::blocks::StaticExtrinsic for ClearPrime {
					const PALLET: &'static str = "ReleaseMembership";
					const CALL: &'static str = "clear_prime";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_member(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::AddMember> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"add_member",
						types::AddMember { who },
						[
							237u8, 212u8, 114u8, 106u8, 42u8, 1u8, 28u8, 244u8, 161u8, 140u8, 80u8,
							251u8, 217u8, 116u8, 9u8, 155u8, 234u8, 94u8, 3u8, 156u8, 5u8, 68u8,
							197u8, 90u8, 47u8, 223u8, 235u8, 200u8, 194u8, 195u8, 90u8, 70u8,
						],
					)
				}
				pub fn remove_member(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::RemoveMember> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"remove_member",
						types::RemoveMember { who },
						[
							182u8, 80u8, 149u8, 27u8, 252u8, 160u8, 231u8, 23u8, 208u8, 93u8,
							116u8, 252u8, 5u8, 28u8, 162u8, 221u8, 14u8, 150u8, 62u8, 65u8, 169u8,
							164u8, 226u8, 53u8, 152u8, 237u8, 37u8, 171u8, 209u8, 52u8, 235u8,
							11u8,
						],
					)
				}
				pub fn swap_member(
					&self,
					remove: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					add: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SwapMember> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"swap_member",
						types::SwapMember { remove, add },
						[
							11u8, 68u8, 57u8, 232u8, 242u8, 45u8, 34u8, 178u8, 250u8, 195u8, 254u8,
							22u8, 13u8, 164u8, 86u8, 193u8, 47u8, 198u8, 164u8, 143u8, 66u8, 245u8,
							222u8, 24u8, 240u8, 97u8, 241u8, 166u8, 198u8, 236u8, 148u8, 195u8,
						],
					)
				}
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::Payload<types::ResetMembers> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"reset_members",
						types::ResetMembers { members },
						[
							212u8, 144u8, 99u8, 156u8, 70u8, 4u8, 219u8, 227u8, 150u8, 25u8, 86u8,
							8u8, 215u8, 128u8, 193u8, 206u8, 33u8, 193u8, 71u8, 15u8, 20u8, 92u8,
							99u8, 89u8, 174u8, 236u8, 102u8, 82u8, 164u8, 234u8, 12u8, 45u8,
						],
					)
				}
				pub fn change_key(
					&self,
					new: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::ChangeKey> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"change_key",
						types::ChangeKey { new },
						[
							189u8, 13u8, 214u8, 17u8, 2u8, 171u8, 123u8, 104u8, 227u8, 136u8,
							198u8, 88u8, 216u8, 105u8, 246u8, 70u8, 25u8, 19u8, 244u8, 19u8, 112u8,
							105u8, 14u8, 16u8, 75u8, 183u8, 85u8, 183u8, 131u8, 186u8, 129u8,
							231u8,
						],
					)
				}
				pub fn set_prime(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SetPrime> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"set_prime",
						types::SetPrime { who },
						[
							76u8, 172u8, 20u8, 105u8, 143u8, 253u8, 234u8, 255u8, 138u8, 98u8,
							81u8, 183u8, 147u8, 232u8, 49u8, 234u8, 17u8, 129u8, 20u8, 119u8,
							116u8, 214u8, 131u8, 112u8, 38u8, 21u8, 93u8, 139u8, 161u8, 242u8,
							88u8, 152u8,
						],
					)
				}
				pub fn clear_prime(&self) -> ::subxt::tx::Payload<types::ClearPrime> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"clear_prime",
						types::ClearPrime {},
						[
							71u8, 213u8, 34u8, 23u8, 186u8, 63u8, 240u8, 216u8, 190u8, 251u8, 84u8,
							109u8, 140u8, 137u8, 210u8, 211u8, 242u8, 231u8, 212u8, 133u8, 151u8,
							125u8, 25u8, 46u8, 210u8, 53u8, 133u8, 222u8, 21u8, 107u8, 120u8, 52u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_membership::pallet::Event3;
		pub mod events {
			use super::runtime_types;
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MembersReset";
			}
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "KeyChanged";
			}
			pub struct Dummy;
			impl ::subxt::events::StaticEvent for Dummy {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "Dummy";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_core::crypto::AccountId32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseMembership",
						"Members",
						vec![],
						[
							109u8, 100u8, 14u8, 195u8, 213u8, 67u8, 44u8, 218u8, 84u8, 254u8, 76u8,
							80u8, 210u8, 155u8, 155u8, 30u8, 18u8, 169u8, 195u8, 92u8, 208u8,
							223u8, 242u8, 97u8, 147u8, 20u8, 168u8, 145u8, 254u8, 115u8, 225u8,
							193u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseMembership",
						"Prime",
						vec![],
						[
							72u8, 128u8, 214u8, 72u8, 78u8, 80u8, 100u8, 198u8, 114u8, 215u8, 59u8,
							3u8, 103u8, 14u8, 152u8, 202u8, 12u8, 165u8, 224u8, 10u8, 41u8, 154u8,
							77u8, 95u8, 116u8, 143u8, 250u8, 250u8, 176u8, 92u8, 238u8, 154u8,
						],
					)
				}
			}
		}
	}
	pub mod scheduler {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_scheduler::pallet::Error;
		pub type Call = runtime_types::pallet_scheduler::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Schedule {
					pub when: ::core::primitive::u32,
					pub maybe_periodic:
						::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
					pub priority: ::core::primitive::u8,
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Schedule {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "schedule";
				}
				pub struct Cancel {
					pub when: ::core::primitive::u32,
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Cancel {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "cancel";
				}
				pub struct ScheduleNamed {
					pub id: [::core::primitive::u8; 32usize],
					pub when: ::core::primitive::u32,
					pub maybe_periodic:
						::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
					pub priority: ::core::primitive::u8,
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ScheduleNamed {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "schedule_named";
				}
				pub struct CancelNamed {
					pub id: [::core::primitive::u8; 32usize],
				}
				impl ::subxt::blocks::StaticExtrinsic for CancelNamed {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "cancel_named";
				}
				pub struct ScheduleAfter {
					pub after: ::core::primitive::u32,
					pub maybe_periodic:
						::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
					pub priority: ::core::primitive::u8,
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ScheduleAfter {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "schedule_after";
				}
				pub struct ScheduleNamedAfter {
					pub id: [::core::primitive::u8; 32usize],
					pub after: ::core::primitive::u32,
					pub maybe_periodic:
						::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
					pub priority: ::core::primitive::u8,
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ScheduleNamedAfter {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "schedule_named_after";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn schedule(
					&self,
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::picasso_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::Schedule> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule",
						types::Schedule {
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							63u8, 226u8, 235u8, 107u8, 233u8, 147u8, 234u8, 163u8, 191u8, 253u8,
							249u8, 238u8, 228u8, 87u8, 161u8, 85u8, 102u8, 138u8, 40u8, 32u8, 30u8,
							35u8, 122u8, 13u8, 87u8, 245u8, 32u8, 203u8, 129u8, 167u8, 143u8, 30u8,
						],
					)
				}
				pub fn cancel(
					&self,
					when: ::core::primitive::u32,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Cancel> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"cancel",
						types::Cancel { when, index },
						[
							183u8, 204u8, 143u8, 86u8, 17u8, 130u8, 132u8, 91u8, 133u8, 168u8,
							103u8, 129u8, 114u8, 56u8, 123u8, 42u8, 123u8, 120u8, 221u8, 211u8,
							26u8, 85u8, 82u8, 246u8, 192u8, 39u8, 254u8, 45u8, 147u8, 56u8, 178u8,
							133u8,
						],
					)
				}
				pub fn schedule_named(
					&self,
					id: [::core::primitive::u8; 32usize],
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::picasso_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::ScheduleNamed> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule_named",
						types::ScheduleNamed {
							id,
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							165u8, 198u8, 210u8, 68u8, 245u8, 58u8, 188u8, 66u8, 153u8, 137u8,
							38u8, 30u8, 159u8, 42u8, 21u8, 141u8, 126u8, 108u8, 180u8, 6u8, 15u8,
							224u8, 112u8, 132u8, 103u8, 5u8, 231u8, 162u8, 31u8, 230u8, 237u8,
							165u8,
						],
					)
				}
				pub fn cancel_named(
					&self,
					id: [::core::primitive::u8; 32usize],
				) -> ::subxt::tx::Payload<types::CancelNamed> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"cancel_named",
						types::CancelNamed { id },
						[
							205u8, 35u8, 28u8, 57u8, 224u8, 7u8, 49u8, 233u8, 236u8, 163u8, 93u8,
							236u8, 103u8, 69u8, 65u8, 51u8, 121u8, 84u8, 9u8, 196u8, 147u8, 122u8,
							227u8, 200u8, 181u8, 233u8, 62u8, 240u8, 174u8, 83u8, 129u8, 193u8,
						],
					)
				}
				pub fn schedule_after(
					&self,
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::picasso_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::ScheduleAfter> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule_after",
						types::ScheduleAfter {
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							41u8, 80u8, 20u8, 95u8, 222u8, 247u8, 86u8, 39u8, 154u8, 217u8, 112u8,
							223u8, 219u8, 219u8, 53u8, 246u8, 65u8, 209u8, 22u8, 102u8, 33u8, 52u8,
							165u8, 218u8, 206u8, 75u8, 223u8, 5u8, 238u8, 240u8, 12u8, 128u8,
						],
					)
				}
				pub fn schedule_named_after(
					&self,
					id: [::core::primitive::u8; 32usize],
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::picasso_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::ScheduleNamedAfter> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule_named_after",
						types::ScheduleNamedAfter {
							id,
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							71u8, 122u8, 120u8, 188u8, 220u8, 51u8, 137u8, 131u8, 229u8, 218u8,
							94u8, 84u8, 54u8, 96u8, 120u8, 185u8, 228u8, 161u8, 137u8, 4u8, 72u8,
							77u8, 249u8, 48u8, 122u8, 106u8, 187u8, 117u8, 246u8, 57u8, 92u8, 6u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_scheduler::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Scheduled {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Scheduled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Scheduled";
			}
			pub struct Canceled {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Canceled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Canceled";
			}
			pub struct Dispatched {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Dispatched {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Dispatched";
			}
			pub struct CallUnavailable {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for CallUnavailable {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "CallUnavailable";
			}
			pub struct PeriodicFailed {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for PeriodicFailed {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "PeriodicFailed";
			}
			pub struct PermanentlyOverweight {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for PermanentlyOverweight {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "PermanentlyOverweight";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn incomplete_since(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Scheduler",
						"IncompleteSince",
						vec![],
						[
							250u8, 83u8, 64u8, 167u8, 205u8, 59u8, 225u8, 97u8, 205u8, 12u8, 76u8,
							130u8, 197u8, 4u8, 111u8, 208u8, 92u8, 217u8, 145u8, 119u8, 38u8,
							135u8, 1u8, 242u8, 228u8, 143u8, 56u8, 25u8, 115u8, 233u8, 227u8, 66u8,
						],
					)
				}
				pub fn agenda(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::option::Option<
							runtime_types::pallet_scheduler::Scheduled<
								[::core::primitive::u8; 32usize],
								runtime_types::frame_support::traits::preimages::Bounded<
									runtime_types::picasso_runtime::RuntimeCall,
								>,
								::core::primitive::u32,
								runtime_types::picasso_runtime::OriginCaller,
								runtime_types::sp_core::crypto::AccountId32,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Scheduler",
						"Agenda",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							246u8, 40u8, 29u8, 125u8, 228u8, 123u8, 138u8, 101u8, 15u8, 204u8,
							242u8, 130u8, 186u8, 42u8, 13u8, 182u8, 75u8, 186u8, 169u8, 195u8,
							253u8, 238u8, 151u8, 107u8, 34u8, 181u8, 175u8, 87u8, 242u8, 7u8, 56u8,
							47u8,
						],
					)
				}
				pub fn agenda_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::option::Option<
							runtime_types::pallet_scheduler::Scheduled<
								[::core::primitive::u8; 32usize],
								runtime_types::frame_support::traits::preimages::Bounded<
									runtime_types::picasso_runtime::RuntimeCall,
								>,
								::core::primitive::u32,
								runtime_types::picasso_runtime::OriginCaller,
								runtime_types::sp_core::crypto::AccountId32,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Scheduler",
						"Agenda",
						Vec::new(),
						[
							246u8, 40u8, 29u8, 125u8, 228u8, 123u8, 138u8, 101u8, 15u8, 204u8,
							242u8, 130u8, 186u8, 42u8, 13u8, 182u8, 75u8, 186u8, 169u8, 195u8,
							253u8, 238u8, 151u8, 107u8, 34u8, 181u8, 175u8, 87u8, 242u8, 7u8, 56u8,
							47u8,
						],
					)
				}
				pub fn lookup(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::core::primitive::u32),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Scheduler",
						"Lookup",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							24u8, 87u8, 96u8, 127u8, 136u8, 205u8, 238u8, 174u8, 71u8, 110u8, 65u8,
							98u8, 228u8, 167u8, 99u8, 71u8, 171u8, 186u8, 12u8, 218u8, 137u8, 70u8,
							70u8, 228u8, 153u8, 111u8, 165u8, 114u8, 229u8, 136u8, 118u8, 131u8,
						],
					)
				}
				pub fn lookup_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::core::primitive::u32),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Scheduler",
						"Lookup",
						Vec::new(),
						[
							24u8, 87u8, 96u8, 127u8, 136u8, 205u8, 238u8, 174u8, 71u8, 110u8, 65u8,
							98u8, 228u8, 167u8, 99u8, 71u8, 171u8, 186u8, 12u8, 218u8, 137u8, 70u8,
							70u8, 228u8, 153u8, 111u8, 165u8, 114u8, 229u8, 136u8, 118u8, 131u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn maximum_weight(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_weights::weight_v2::Weight> {
					::subxt::constants::Address::new_static(
						"Scheduler",
						"MaximumWeight",
						[
							149u8, 252u8, 129u8, 80u8, 169u8, 36u8, 79u8, 127u8, 240u8, 156u8,
							56u8, 202u8, 219u8, 86u8, 5u8, 65u8, 245u8, 148u8, 138u8, 243u8, 210u8,
							128u8, 234u8, 216u8, 240u8, 219u8, 123u8, 235u8, 21u8, 158u8, 237u8,
							112u8,
						],
					)
				}
				pub fn max_scheduled_per_block(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Scheduler",
						"MaxScheduledPerBlock",
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
	pub mod utility {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_utility::pallet::Error;
		pub type Call = runtime_types::pallet_utility::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Batch {
					pub calls: ::std::vec::Vec<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Batch {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "batch";
				}
				pub struct AsDerivative {
					pub index: ::core::primitive::u16,
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AsDerivative {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "as_derivative";
				}
				pub struct BatchAll {
					pub calls: ::std::vec::Vec<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for BatchAll {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "batch_all";
				}
				pub struct DispatchAs {
					pub as_origin: ::std::boxed::Box<runtime_types::picasso_runtime::OriginCaller>,
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for DispatchAs {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "dispatch_as";
				}
				pub struct ForceBatch {
					pub calls: ::std::vec::Vec<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceBatch {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "force_batch";
				}
				pub struct WithWeight {
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for WithWeight {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "with_weight";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::picasso_runtime::RuntimeCall>,
				) -> ::subxt::tx::Payload<types::Batch> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"batch",
						types::Batch { calls },
						[
							32u8, 222u8, 210u8, 5u8, 102u8, 135u8, 143u8, 26u8, 173u8, 232u8, 71u8,
							152u8, 192u8, 56u8, 10u8, 230u8, 107u8, 145u8, 164u8, 162u8, 1u8, 69u8,
							91u8, 96u8, 155u8, 11u8, 33u8, 72u8, 239u8, 2u8, 234u8, 13u8,
						],
					)
				}
				pub fn as_derivative(
					&self,
					index: ::core::primitive::u16,
					call: runtime_types::picasso_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::AsDerivative> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"as_derivative",
						types::AsDerivative { index, call: ::std::boxed::Box::new(call) },
						[
							53u8, 148u8, 27u8, 172u8, 139u8, 227u8, 153u8, 54u8, 154u8, 226u8,
							104u8, 228u8, 69u8, 202u8, 64u8, 91u8, 77u8, 240u8, 98u8, 56u8, 108u8,
							187u8, 186u8, 252u8, 5u8, 95u8, 148u8, 199u8, 73u8, 162u8, 51u8, 59u8,
						],
					)
				}
				pub fn batch_all(
					&self,
					calls: ::std::vec::Vec<runtime_types::picasso_runtime::RuntimeCall>,
				) -> ::subxt::tx::Payload<types::BatchAll> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"batch_all",
						types::BatchAll { calls },
						[
							52u8, 6u8, 112u8, 6u8, 41u8, 124u8, 28u8, 116u8, 120u8, 14u8, 29u8,
							252u8, 221u8, 12u8, 135u8, 103u8, 120u8, 237u8, 201u8, 24u8, 76u8,
							80u8, 73u8, 245u8, 50u8, 25u8, 248u8, 246u8, 88u8, 223u8, 141u8, 112u8,
						],
					)
				}
				pub fn dispatch_as(
					&self,
					as_origin: runtime_types::picasso_runtime::OriginCaller,
					call: runtime_types::picasso_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::DispatchAs> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"dispatch_as",
						types::DispatchAs {
							as_origin: ::std::boxed::Box::new(as_origin),
							call: ::std::boxed::Box::new(call),
						},
						[
							47u8, 22u8, 162u8, 246u8, 82u8, 118u8, 92u8, 68u8, 65u8, 124u8, 24u8,
							59u8, 226u8, 247u8, 241u8, 95u8, 208u8, 224u8, 249u8, 221u8, 129u8,
							187u8, 158u8, 144u8, 0u8, 84u8, 118u8, 174u8, 97u8, 142u8, 28u8, 233u8,
						],
					)
				}
				pub fn force_batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::picasso_runtime::RuntimeCall>,
				) -> ::subxt::tx::Payload<types::ForceBatch> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"force_batch",
						types::ForceBatch { calls },
						[
							62u8, 196u8, 238u8, 249u8, 156u8, 93u8, 4u8, 122u8, 6u8, 244u8, 92u8,
							176u8, 7u8, 228u8, 60u8, 234u8, 51u8, 97u8, 89u8, 43u8, 83u8, 198u8,
							26u8, 228u8, 97u8, 255u8, 253u8, 142u8, 89u8, 145u8, 232u8, 57u8,
						],
					)
				}
				pub fn with_weight(
					&self,
					call: runtime_types::picasso_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::WithWeight> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"with_weight",
						types::WithWeight { call: ::std::boxed::Box::new(call), weight },
						[
							154u8, 134u8, 211u8, 222u8, 6u8, 227u8, 232u8, 110u8, 242u8, 204u8,
							190u8, 233u8, 33u8, 202u8, 137u8, 218u8, 80u8, 117u8, 63u8, 0u8, 147u8,
							85u8, 139u8, 227u8, 89u8, 42u8, 136u8, 160u8, 174u8, 122u8, 53u8, 78u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_utility::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct BatchInterrupted {
				pub index: ::core::primitive::u32,
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for BatchInterrupted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchInterrupted";
			}
			pub struct BatchCompleted;
			impl ::subxt::events::StaticEvent for BatchCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompleted";
			}
			pub struct BatchCompletedWithErrors;
			impl ::subxt::events::StaticEvent for BatchCompletedWithErrors {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompletedWithErrors";
			}
			pub struct ItemCompleted;
			impl ::subxt::events::StaticEvent for ItemCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemCompleted";
			}
			pub struct ItemFailed {
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for ItemFailed {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemFailed";
			}
			pub struct DispatchedAs {
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for DispatchedAs {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "DispatchedAs";
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn batched_calls_limit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Utility",
						"batched_calls_limit",
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
	pub mod preimage {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_preimage::pallet::Error;
		pub type Call = runtime_types::pallet_preimage::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct NotePreimage {
					pub bytes: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for NotePreimage {
					const PALLET: &'static str = "Preimage";
					const CALL: &'static str = "note_preimage";
				}
				pub struct UnnotePreimage {
					pub hash: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for UnnotePreimage {
					const PALLET: &'static str = "Preimage";
					const CALL: &'static str = "unnote_preimage";
				}
				pub struct RequestPreimage {
					pub hash: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for RequestPreimage {
					const PALLET: &'static str = "Preimage";
					const CALL: &'static str = "request_preimage";
				}
				pub struct UnrequestPreimage {
					pub hash: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for UnrequestPreimage {
					const PALLET: &'static str = "Preimage";
					const CALL: &'static str = "unrequest_preimage";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn note_preimage(
					&self,
					bytes: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::NotePreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"note_preimage",
						types::NotePreimage { bytes },
						[
							121u8, 88u8, 18u8, 92u8, 176u8, 15u8, 192u8, 198u8, 146u8, 198u8, 38u8,
							242u8, 213u8, 83u8, 7u8, 230u8, 14u8, 110u8, 235u8, 32u8, 215u8, 26u8,
							192u8, 217u8, 113u8, 224u8, 206u8, 96u8, 177u8, 198u8, 246u8, 33u8,
						],
					)
				}
				pub fn unnote_preimage(
					&self,
					hash: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::UnnotePreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"unnote_preimage",
						types::UnnotePreimage { hash },
						[
							188u8, 116u8, 222u8, 22u8, 127u8, 215u8, 2u8, 133u8, 96u8, 202u8,
							190u8, 123u8, 203u8, 43u8, 200u8, 161u8, 226u8, 24u8, 49u8, 36u8,
							221u8, 160u8, 130u8, 119u8, 30u8, 138u8, 144u8, 85u8, 5u8, 164u8,
							252u8, 222u8,
						],
					)
				}
				pub fn request_preimage(
					&self,
					hash: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::RequestPreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"request_preimage",
						types::RequestPreimage { hash },
						[
							87u8, 0u8, 204u8, 111u8, 43u8, 115u8, 64u8, 209u8, 133u8, 13u8, 83u8,
							45u8, 164u8, 166u8, 233u8, 105u8, 242u8, 238u8, 235u8, 208u8, 113u8,
							134u8, 93u8, 242u8, 86u8, 32u8, 7u8, 152u8, 107u8, 208u8, 79u8, 59u8,
						],
					)
				}
				pub fn unrequest_preimage(
					&self,
					hash: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::UnrequestPreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"unrequest_preimage",
						types::UnrequestPreimage { hash },
						[
							55u8, 37u8, 224u8, 149u8, 142u8, 120u8, 8u8, 68u8, 183u8, 225u8, 255u8,
							240u8, 254u8, 111u8, 58u8, 200u8, 113u8, 217u8, 177u8, 203u8, 107u8,
							104u8, 233u8, 87u8, 252u8, 53u8, 33u8, 112u8, 116u8, 254u8, 117u8,
							134u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_preimage::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Noted {
				pub hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for Noted {
				const PALLET: &'static str = "Preimage";
				const EVENT: &'static str = "Noted";
			}
			pub struct Requested {
				pub hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for Requested {
				const PALLET: &'static str = "Preimage";
				const EVENT: &'static str = "Requested";
			}
			pub struct Cleared {
				pub hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for Cleared {
				const PALLET: &'static str = "Preimage";
				const EVENT: &'static str = "Cleared";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn status_for(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_preimage::RequestStatus<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Preimage",
						"StatusFor",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							187u8, 100u8, 54u8, 112u8, 96u8, 129u8, 36u8, 149u8, 127u8, 226u8,
							126u8, 171u8, 72u8, 189u8, 59u8, 126u8, 204u8, 125u8, 67u8, 204u8,
							231u8, 6u8, 212u8, 135u8, 166u8, 252u8, 5u8, 46u8, 111u8, 120u8, 54u8,
							209u8,
						],
					)
				}
				pub fn status_for_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_preimage::RequestStatus<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Preimage",
						"StatusFor",
						Vec::new(),
						[
							187u8, 100u8, 54u8, 112u8, 96u8, 129u8, 36u8, 149u8, 127u8, 226u8,
							126u8, 171u8, 72u8, 189u8, 59u8, 126u8, 204u8, 125u8, 67u8, 204u8,
							231u8, 6u8, 212u8, 135u8, 166u8, 252u8, 5u8, 46u8, 111u8, 120u8, 54u8,
							209u8,
						],
					)
				}
				pub fn preimage_for(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Preimage",
						"PreimageFor",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							106u8, 5u8, 17u8, 46u8, 6u8, 184u8, 177u8, 113u8, 169u8, 34u8, 119u8,
							141u8, 117u8, 40u8, 30u8, 94u8, 187u8, 35u8, 206u8, 216u8, 143u8,
							208u8, 49u8, 156u8, 200u8, 255u8, 109u8, 200u8, 210u8, 134u8, 24u8,
							139u8,
						],
					)
				}
				pub fn preimage_for_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Preimage",
						"PreimageFor",
						Vec::new(),
						[
							106u8, 5u8, 17u8, 46u8, 6u8, 184u8, 177u8, 113u8, 169u8, 34u8, 119u8,
							141u8, 117u8, 40u8, 30u8, 94u8, 187u8, 35u8, 206u8, 216u8, 143u8,
							208u8, 49u8, 156u8, 200u8, 255u8, 109u8, 200u8, 210u8, 134u8, 24u8,
							139u8,
						],
					)
				}
			}
		}
	}
	pub mod proxy {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_proxy::pallet::Error;
		pub type Call = runtime_types::pallet_proxy::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Proxy {
					pub real: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub force_proxy_type: ::core::option::Option<
						runtime_types::composable_traits::account_proxy::ProxyType,
					>,
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Proxy {
					const PALLET: &'static str = "Proxy";
					const CALL: &'static str = "proxy";
				}
				pub struct AddProxy {
					pub delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					pub delay: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddProxy {
					const PALLET: &'static str = "Proxy";
					const CALL: &'static str = "add_proxy";
				}
				pub struct RemoveProxy {
					pub delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					pub delay: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveProxy {
					const PALLET: &'static str = "Proxy";
					const CALL: &'static str = "remove_proxy";
				}
				pub struct RemoveProxies;
				impl ::subxt::blocks::StaticExtrinsic for RemoveProxies {
					const PALLET: &'static str = "Proxy";
					const CALL: &'static str = "remove_proxies";
				}
				pub struct CreatePure {
					pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					pub delay: ::core::primitive::u32,
					pub index: ::core::primitive::u16,
				}
				impl ::subxt::blocks::StaticExtrinsic for CreatePure {
					const PALLET: &'static str = "Proxy";
					const CALL: &'static str = "create_pure";
				}
				pub struct KillPure {
					pub spawner: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					pub index: ::core::primitive::u16,
					#[codec::codec(compact)]
					pub height: ::core::primitive::u32,
					#[codec::codec(compact)]
					pub ext_index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for KillPure {
					const PALLET: &'static str = "Proxy";
					const CALL: &'static str = "kill_pure";
				}
				pub struct Announce {
					pub real: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub call_hash: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for Announce {
					const PALLET: &'static str = "Proxy";
					const CALL: &'static str = "announce";
				}
				pub struct RemoveAnnouncement {
					pub real: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub call_hash: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveAnnouncement {
					const PALLET: &'static str = "Proxy";
					const CALL: &'static str = "remove_announcement";
				}
				pub struct RejectAnnouncement {
					pub delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub call_hash: runtime_types::primitive_types::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for RejectAnnouncement {
					const PALLET: &'static str = "Proxy";
					const CALL: &'static str = "reject_announcement";
				}
				pub struct ProxyAnnounced {
					pub delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub real: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub force_proxy_type: ::core::option::Option<
						runtime_types::composable_traits::account_proxy::ProxyType,
					>,
					pub call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ProxyAnnounced {
					const PALLET: &'static str = "Proxy";
					const CALL: &'static str = "proxy_announced";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn proxy(
					&self,
					real: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					force_proxy_type: ::core::option::Option<
						runtime_types::composable_traits::account_proxy::ProxyType,
					>,
					call: runtime_types::picasso_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::Proxy> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"proxy",
						types::Proxy { real, force_proxy_type, call: ::std::boxed::Box::new(call) },
						[
							66u8, 98u8, 197u8, 251u8, 63u8, 245u8, 137u8, 15u8, 71u8, 118u8, 250u8,
							99u8, 191u8, 184u8, 219u8, 220u8, 5u8, 111u8, 55u8, 97u8, 129u8, 16u8,
							119u8, 65u8, 139u8, 27u8, 59u8, 7u8, 241u8, 76u8, 100u8, 67u8,
						],
					)
				}
				pub fn add_proxy(
					&self,
					delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::AddProxy> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"add_proxy",
						types::AddProxy { delegate, proxy_type, delay },
						[
							182u8, 165u8, 22u8, 28u8, 42u8, 174u8, 153u8, 169u8, 148u8, 191u8,
							95u8, 104u8, 230u8, 3u8, 150u8, 146u8, 46u8, 155u8, 241u8, 17u8, 28u8,
							103u8, 233u8, 235u8, 181u8, 206u8, 206u8, 110u8, 138u8, 76u8, 153u8,
							87u8,
						],
					)
				}
				pub fn remove_proxy(
					&self,
					delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::RemoveProxy> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"remove_proxy",
						types::RemoveProxy { delegate, proxy_type, delay },
						[
							99u8, 175u8, 2u8, 28u8, 173u8, 219u8, 40u8, 183u8, 102u8, 219u8, 148u8,
							107u8, 250u8, 114u8, 239u8, 57u8, 112u8, 212u8, 146u8, 25u8, 251u8,
							185u8, 3u8, 250u8, 223u8, 239u8, 40u8, 222u8, 18u8, 91u8, 53u8, 236u8,
						],
					)
				}
				pub fn remove_proxies(&self) -> ::subxt::tx::Payload<types::RemoveProxies> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"remove_proxies",
						types::RemoveProxies {},
						[
							1u8, 126u8, 36u8, 227u8, 185u8, 34u8, 218u8, 236u8, 125u8, 231u8, 68u8,
							185u8, 145u8, 63u8, 250u8, 225u8, 103u8, 3u8, 189u8, 37u8, 172u8,
							195u8, 197u8, 216u8, 99u8, 210u8, 240u8, 162u8, 158u8, 132u8, 24u8,
							6u8,
						],
					)
				}
				pub fn create_pure(
					&self,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					delay: ::core::primitive::u32,
					index: ::core::primitive::u16,
				) -> ::subxt::tx::Payload<types::CreatePure> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"create_pure",
						types::CreatePure { proxy_type, delay, index },
						[
							165u8, 248u8, 178u8, 202u8, 234u8, 208u8, 97u8, 52u8, 80u8, 66u8,
							120u8, 212u8, 62u8, 109u8, 196u8, 1u8, 136u8, 223u8, 129u8, 156u8,
							148u8, 5u8, 55u8, 164u8, 13u8, 162u8, 146u8, 59u8, 203u8, 5u8, 170u8,
							7u8,
						],
					)
				}
				pub fn kill_pure(
					&self,
					spawner: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					index: ::core::primitive::u16,
					height: ::core::primitive::u32,
					ext_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::KillPure> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"kill_pure",
						types::KillPure { spawner, proxy_type, index, height, ext_index },
						[
							249u8, 240u8, 223u8, 4u8, 122u8, 46u8, 42u8, 27u8, 103u8, 144u8, 223u8,
							103u8, 86u8, 43u8, 241u8, 28u8, 142u8, 128u8, 32u8, 232u8, 163u8,
							191u8, 92u8, 109u8, 153u8, 55u8, 171u8, 70u8, 77u8, 80u8, 100u8, 214u8,
						],
					)
				}
				pub fn announce(
					&self,
					real: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					call_hash: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::Announce> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"announce",
						types::Announce { real, call_hash },
						[
							32u8, 88u8, 145u8, 33u8, 55u8, 44u8, 136u8, 153u8, 26u8, 111u8, 73u8,
							15u8, 247u8, 188u8, 14u8, 236u8, 221u8, 222u8, 60u8, 97u8, 71u8, 229u8,
							18u8, 120u8, 182u8, 43u8, 67u8, 248u8, 169u8, 80u8, 170u8, 207u8,
						],
					)
				}
				pub fn remove_announcement(
					&self,
					real: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					call_hash: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::RemoveAnnouncement> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"remove_announcement",
						types::RemoveAnnouncement { real, call_hash },
						[
							195u8, 224u8, 61u8, 33u8, 27u8, 100u8, 168u8, 18u8, 105u8, 23u8, 220u8,
							168u8, 207u8, 231u8, 136u8, 46u8, 181u8, 85u8, 15u8, 151u8, 126u8,
							227u8, 97u8, 162u8, 232u8, 39u8, 45u8, 255u8, 44u8, 167u8, 237u8, 38u8,
						],
					)
				}
				pub fn reject_announcement(
					&self,
					delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					call_hash: runtime_types::primitive_types::H256,
				) -> ::subxt::tx::Payload<types::RejectAnnouncement> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"reject_announcement",
						types::RejectAnnouncement { delegate, call_hash },
						[
							29u8, 140u8, 243u8, 165u8, 143u8, 166u8, 205u8, 203u8, 111u8, 196u8,
							11u8, 2u8, 4u8, 230u8, 11u8, 136u8, 249u8, 139u8, 224u8, 242u8, 96u8,
							146u8, 118u8, 210u8, 104u8, 77u8, 168u8, 28u8, 67u8, 244u8, 91u8, 65u8,
						],
					)
				}
				pub fn proxy_announced(
					&self,
					delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					real: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					force_proxy_type: ::core::option::Option<
						runtime_types::composable_traits::account_proxy::ProxyType,
					>,
					call: runtime_types::picasso_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::ProxyAnnounced> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"proxy_announced",
						types::ProxyAnnounced {
							delegate,
							real,
							force_proxy_type,
							call: ::std::boxed::Box::new(call),
						},
						[
							180u8, 255u8, 5u8, 53u8, 67u8, 158u8, 33u8, 59u8, 35u8, 196u8, 156u8,
							19u8, 93u8, 33u8, 108u8, 245u8, 50u8, 1u8, 88u8, 120u8, 112u8, 159u8,
							75u8, 18u8, 107u8, 117u8, 251u8, 140u8, 159u8, 228u8, 236u8, 17u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_proxy::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct ProxyExecuted {
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for ProxyExecuted {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "ProxyExecuted";
			}
			pub struct PureCreated {
				pub pure: runtime_types::sp_core::crypto::AccountId32,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub disambiguation_index: ::core::primitive::u16,
			}
			impl ::subxt::events::StaticEvent for PureCreated {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "PureCreated";
			}
			pub struct Announced {
				pub real: runtime_types::sp_core::crypto::AccountId32,
				pub proxy: runtime_types::sp_core::crypto::AccountId32,
				pub call_hash: runtime_types::primitive_types::H256,
			}
			impl ::subxt::events::StaticEvent for Announced {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "Announced";
			}
			pub struct ProxyAdded {
				pub delegator: runtime_types::sp_core::crypto::AccountId32,
				pub delegatee: runtime_types::sp_core::crypto::AccountId32,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ProxyAdded {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "ProxyAdded";
			}
			pub struct ProxyRemoved {
				pub delegator: runtime_types::sp_core::crypto::AccountId32,
				pub delegatee: runtime_types::sp_core::crypto::AccountId32,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ProxyRemoved {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "ProxyRemoved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn proxies(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::ProxyDefinition<
								runtime_types::sp_core::crypto::AccountId32,
								runtime_types::composable_traits::account_proxy::ProxyType,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Proxy",
						"Proxies",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							68u8, 195u8, 37u8, 187u8, 13u8, 190u8, 195u8, 54u8, 26u8, 16u8, 132u8,
							26u8, 65u8, 172u8, 7u8, 238u8, 57u8, 162u8, 133u8, 84u8, 175u8, 113u8,
							227u8, 166u8, 232u8, 203u8, 12u8, 116u8, 24u8, 236u8, 64u8, 74u8,
						],
					)
				}
				pub fn proxies_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::ProxyDefinition<
								runtime_types::sp_core::crypto::AccountId32,
								runtime_types::composable_traits::account_proxy::ProxyType,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Proxy",
						"Proxies",
						Vec::new(),
						[
							68u8, 195u8, 37u8, 187u8, 13u8, 190u8, 195u8, 54u8, 26u8, 16u8, 132u8,
							26u8, 65u8, 172u8, 7u8, 238u8, 57u8, 162u8, 133u8, 84u8, 175u8, 113u8,
							227u8, 166u8, 232u8, 203u8, 12u8, 116u8, 24u8, 236u8, 64u8, 74u8,
						],
					)
				}
				pub fn announcements(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::Announcement<
								runtime_types::sp_core::crypto::AccountId32,
								runtime_types::primitive_types::H256,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Proxy",
						"Announcements",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							129u8, 228u8, 198u8, 210u8, 90u8, 69u8, 151u8, 198u8, 206u8, 174u8,
							148u8, 58u8, 134u8, 14u8, 53u8, 56u8, 234u8, 71u8, 84u8, 247u8, 246u8,
							207u8, 117u8, 221u8, 84u8, 72u8, 254u8, 215u8, 102u8, 49u8, 21u8,
							173u8,
						],
					)
				}
				pub fn announcements_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::Announcement<
								runtime_types::sp_core::crypto::AccountId32,
								runtime_types::primitive_types::H256,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Proxy",
						"Announcements",
						Vec::new(),
						[
							129u8, 228u8, 198u8, 210u8, 90u8, 69u8, 151u8, 198u8, 206u8, 174u8,
							148u8, 58u8, 134u8, 14u8, 53u8, 56u8, 234u8, 71u8, 84u8, 247u8, 246u8,
							207u8, 117u8, 221u8, 84u8, 72u8, 254u8, 215u8, 102u8, 49u8, 21u8,
							173u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn proxy_deposit_base(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Proxy",
						"ProxyDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn proxy_deposit_factor(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Proxy",
						"ProxyDepositFactor",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn max_proxies(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Proxy",
						"MaxProxies",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_pending(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Proxy",
						"MaxPending",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn announcement_deposit_base(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Proxy",
						"AnnouncementDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn announcement_deposit_factor(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Proxy",
						"AnnouncementDepositFactor",
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
	pub mod xcmp_queue {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::cumulus_pallet_xcmp_queue::pallet::Error;
		pub type Call = runtime_types::cumulus_pallet_xcmp_queue::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct ServiceOverweight {
					pub index: ::core::primitive::u64,
					pub weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for ServiceOverweight {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "service_overweight";
				}
				pub struct SuspendXcmExecution;
				impl ::subxt::blocks::StaticExtrinsic for SuspendXcmExecution {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "suspend_xcm_execution";
				}
				pub struct ResumeXcmExecution;
				impl ::subxt::blocks::StaticExtrinsic for ResumeXcmExecution {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "resume_xcm_execution";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct UpdateSuspendThreshold {
					pub new: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateSuspendThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_suspend_threshold";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct UpdateDropThreshold {
					pub new: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateDropThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_drop_threshold";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct UpdateResumeThreshold {
					pub new: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateResumeThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_resume_threshold";
				}
				pub struct UpdateThresholdWeight {
					pub new: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateThresholdWeight {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_threshold_weight";
				}
				pub struct UpdateWeightRestrictDecay {
					pub new: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateWeightRestrictDecay {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_weight_restrict_decay";
				}
				pub struct UpdateXcmpMaxIndividualWeight {
					pub new: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateXcmpMaxIndividualWeight {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_xcmp_max_individual_weight";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::ServiceOverweight> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"service_overweight",
						types::ServiceOverweight { index, weight_limit },
						[
							235u8, 203u8, 220u8, 162u8, 173u8, 117u8, 224u8, 194u8, 176u8, 125u8,
							50u8, 74u8, 180u8, 37u8, 126u8, 227u8, 138u8, 213u8, 227u8, 35u8,
							247u8, 18u8, 160u8, 231u8, 97u8, 149u8, 144u8, 49u8, 34u8, 146u8, 32u8,
							7u8,
						],
					)
				}
				pub fn suspend_xcm_execution(
					&self,
				) -> ::subxt::tx::Payload<types::SuspendXcmExecution> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"suspend_xcm_execution",
						types::SuspendXcmExecution {},
						[
							54u8, 120u8, 33u8, 251u8, 74u8, 56u8, 29u8, 76u8, 104u8, 218u8, 115u8,
							198u8, 148u8, 237u8, 9u8, 191u8, 241u8, 48u8, 33u8, 24u8, 60u8, 144u8,
							22u8, 78u8, 58u8, 50u8, 26u8, 188u8, 231u8, 42u8, 201u8, 76u8,
						],
					)
				}
				pub fn resume_xcm_execution(
					&self,
				) -> ::subxt::tx::Payload<types::ResumeXcmExecution> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"resume_xcm_execution",
						types::ResumeXcmExecution {},
						[
							173u8, 231u8, 78u8, 253u8, 108u8, 234u8, 199u8, 124u8, 184u8, 154u8,
							95u8, 194u8, 13u8, 77u8, 175u8, 7u8, 7u8, 112u8, 161u8, 72u8, 133u8,
							71u8, 63u8, 218u8, 97u8, 226u8, 133u8, 6u8, 93u8, 177u8, 247u8, 109u8,
						],
					)
				}
				pub fn update_suspend_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::UpdateSuspendThreshold> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_suspend_threshold",
						types::UpdateSuspendThreshold { new },
						[
							64u8, 91u8, 172u8, 51u8, 220u8, 174u8, 54u8, 47u8, 57u8, 89u8, 75u8,
							39u8, 126u8, 198u8, 143u8, 35u8, 70u8, 125u8, 167u8, 14u8, 17u8, 18u8,
							146u8, 222u8, 100u8, 92u8, 81u8, 239u8, 173u8, 43u8, 42u8, 174u8,
						],
					)
				}
				pub fn update_drop_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::UpdateDropThreshold> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_drop_threshold",
						types::UpdateDropThreshold { new },
						[
							123u8, 54u8, 12u8, 180u8, 165u8, 198u8, 141u8, 200u8, 149u8, 168u8,
							186u8, 237u8, 162u8, 91u8, 89u8, 242u8, 229u8, 16u8, 32u8, 254u8, 59u8,
							168u8, 31u8, 134u8, 217u8, 251u8, 0u8, 102u8, 113u8, 194u8, 175u8, 9u8,
						],
					)
				}
				pub fn update_resume_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::UpdateResumeThreshold> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_resume_threshold",
						types::UpdateResumeThreshold { new },
						[
							172u8, 136u8, 11u8, 106u8, 42u8, 157u8, 167u8, 183u8, 87u8, 62u8,
							182u8, 17u8, 184u8, 59u8, 215u8, 230u8, 18u8, 243u8, 212u8, 34u8, 54u8,
							188u8, 95u8, 119u8, 173u8, 20u8, 91u8, 206u8, 212u8, 57u8, 136u8, 77u8,
						],
					)
				}
				pub fn update_threshold_weight(
					&self,
					new: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::UpdateThresholdWeight> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_threshold_weight",
						types::UpdateThresholdWeight { new },
						[
							79u8, 1u8, 102u8, 119u8, 93u8, 104u8, 197u8, 189u8, 248u8, 215u8, 30u8,
							227u8, 83u8, 26u8, 149u8, 99u8, 174u8, 191u8, 97u8, 82u8, 168u8, 128u8,
							130u8, 136u8, 185u8, 54u8, 104u8, 186u8, 231u8, 11u8, 66u8, 184u8,
						],
					)
				}
				pub fn update_weight_restrict_decay(
					&self,
					new: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::UpdateWeightRestrictDecay> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_weight_restrict_decay",
						types::UpdateWeightRestrictDecay { new },
						[
							37u8, 210u8, 52u8, 253u8, 67u8, 66u8, 63u8, 238u8, 117u8, 80u8, 77u8,
							102u8, 166u8, 103u8, 173u8, 135u8, 54u8, 139u8, 100u8, 225u8, 115u8,
							214u8, 160u8, 228u8, 195u8, 221u8, 160u8, 62u8, 192u8, 105u8, 188u8,
							139u8,
						],
					)
				}
				pub fn update_xcmp_max_individual_weight(
					&self,
					new: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::UpdateXcmpMaxIndividualWeight> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_xcmp_max_individual_weight",
						types::UpdateXcmpMaxIndividualWeight { new },
						[
							185u8, 199u8, 32u8, 102u8, 179u8, 139u8, 101u8, 14u8, 48u8, 173u8,
							123u8, 158u8, 161u8, 153u8, 81u8, 109u8, 196u8, 217u8, 235u8, 150u8,
							176u8, 55u8, 168u8, 31u8, 34u8, 251u8, 128u8, 53u8, 160u8, 220u8,
							140u8, 174u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Success {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for Success {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Success";
			}
			pub struct Fail {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
				pub error: runtime_types::xcm::v3::traits::Error,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for Fail {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Fail";
			}
			pub struct BadVersion {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for BadVersion {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadVersion";
			}
			pub struct BadFormat {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for BadFormat {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadFormat";
			}
			pub struct XcmpMessageSent {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for XcmpMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "XcmpMessageSent";
			}
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
				pub fn inbound_xcmp_status(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::cumulus_pallet_xcmp_queue::InboundChannelDetails,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"InboundXcmpStatus",
						vec![],
						[
							216u8, 138u8, 138u8, 71u8, 210u8, 155u8, 255u8, 91u8, 44u8, 147u8,
							80u8, 187u8, 203u8, 88u8, 34u8, 54u8, 80u8, 232u8, 249u8, 20u8, 169u8,
							138u8, 123u8, 139u8, 182u8, 184u8, 0u8, 205u8, 101u8, 9u8, 194u8,
							122u8,
						],
					)
				}
				pub fn inbound_xcmp_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"InboundXcmpMessages",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							60u8, 227u8, 118u8, 144u8, 41u8, 170u8, 15u8, 80u8, 148u8, 229u8,
							213u8, 6u8, 213u8, 186u8, 20u8, 199u8, 229u8, 159u8, 17u8, 39u8, 116u8,
							85u8, 34u8, 82u8, 109u8, 100u8, 174u8, 85u8, 245u8, 247u8, 84u8, 116u8,
						],
					)
				}
				pub fn inbound_xcmp_messages_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"InboundXcmpMessages",
						Vec::new(),
						[
							60u8, 227u8, 118u8, 144u8, 41u8, 170u8, 15u8, 80u8, 148u8, 229u8,
							213u8, 6u8, 213u8, 186u8, 20u8, 199u8, 229u8, 159u8, 17u8, 39u8, 116u8,
							85u8, 34u8, 82u8, 109u8, 100u8, 174u8, 85u8, 245u8, 247u8, 84u8, 116u8,
						],
					)
				}
				pub fn outbound_xcmp_status(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::cumulus_pallet_xcmp_queue::OutboundChannelDetails,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"OutboundXcmpStatus",
						vec![],
						[
							181u8, 5u8, 216u8, 176u8, 154u8, 233u8, 116u8, 14u8, 151u8, 1u8, 114u8,
							16u8, 42u8, 20u8, 63u8, 233u8, 79u8, 122u8, 87u8, 255u8, 75u8, 149u8,
							176u8, 106u8, 23u8, 101u8, 228u8, 120u8, 217u8, 167u8, 127u8, 117u8,
						],
					)
				}
				pub fn outbound_xcmp_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u16>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"OutboundXcmpMessages",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							156u8, 3u8, 202u8, 175u8, 175u8, 129u8, 38u8, 144u8, 35u8, 59u8, 228u8,
							159u8, 142u8, 25u8, 19u8, 73u8, 73u8, 6u8, 115u8, 19u8, 236u8, 235u8,
							144u8, 172u8, 31u8, 168u8, 24u8, 65u8, 115u8, 95u8, 77u8, 63u8,
						],
					)
				}
				pub fn outbound_xcmp_messages_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"OutboundXcmpMessages",
						Vec::new(),
						[
							156u8, 3u8, 202u8, 175u8, 175u8, 129u8, 38u8, 144u8, 35u8, 59u8, 228u8,
							159u8, 142u8, 25u8, 19u8, 73u8, 73u8, 6u8, 115u8, 19u8, 236u8, 235u8,
							144u8, 172u8, 31u8, 168u8, 24u8, 65u8, 115u8, 95u8, 77u8, 63u8,
						],
					)
				}
				pub fn signal_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"SignalMessages",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							182u8, 143u8, 233u8, 233u8, 111u8, 137u8, 174u8, 165u8, 166u8, 7u8,
							229u8, 183u8, 99u8, 108u8, 30u8, 162u8, 71u8, 55u8, 122u8, 124u8,
							249u8, 203u8, 142u8, 124u8, 158u8, 213u8, 182u8, 159u8, 206u8, 249u8,
							180u8, 24u8,
						],
					)
				}
				pub fn signal_messages_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"SignalMessages",
						Vec::new(),
						[
							182u8, 143u8, 233u8, 233u8, 111u8, 137u8, 174u8, 165u8, 166u8, 7u8,
							229u8, 183u8, 99u8, 108u8, 30u8, 162u8, 71u8, 55u8, 122u8, 124u8,
							249u8, 203u8, 142u8, 124u8, 158u8, 213u8, 182u8, 159u8, 206u8, 249u8,
							180u8, 24u8,
						],
					)
				}
				pub fn queue_config(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"QueueConfig",
						vec![],
						[
							112u8, 136u8, 198u8, 133u8, 5u8, 66u8, 33u8, 29u8, 99u8, 72u8, 70u8,
							56u8, 182u8, 57u8, 48u8, 10u8, 135u8, 63u8, 103u8, 13u8, 143u8, 121u8,
							12u8, 126u8, 207u8, 56u8, 244u8, 63u8, 126u8, 51u8, 100u8, 69u8,
						],
					)
				}
				pub fn overweight(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"Overweight",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							4u8, 180u8, 123u8, 50u8, 174u8, 195u8, 68u8, 214u8, 187u8, 92u8, 131u8,
							234u8, 166u8, 124u8, 19u8, 202u8, 0u8, 249u8, 246u8, 239u8, 199u8,
							27u8, 129u8, 252u8, 22u8, 92u8, 206u8, 159u8, 136u8, 222u8, 238u8,
							81u8,
						],
					)
				}
				pub fn overweight_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"Overweight",
						Vec::new(),
						[
							4u8, 180u8, 123u8, 50u8, 174u8, 195u8, 68u8, 214u8, 187u8, 92u8, 131u8,
							234u8, 166u8, 124u8, 19u8, 202u8, 0u8, 249u8, 246u8, 239u8, 199u8,
							27u8, 129u8, 252u8, 22u8, 92u8, 206u8, 159u8, 136u8, 222u8, 238u8,
							81u8,
						],
					)
				}
				pub fn counter_for_overweight(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"CounterForOverweight",
						vec![],
						[
							44u8, 249u8, 133u8, 204u8, 169u8, 253u8, 23u8, 157u8, 132u8, 193u8,
							28u8, 178u8, 156u8, 176u8, 206u8, 46u8, 79u8, 254u8, 174u8, 236u8,
							143u8, 219u8, 59u8, 43u8, 36u8, 109u8, 244u8, 206u8, 48u8, 126u8,
							247u8, 0u8,
						],
					)
				}
				pub fn overweight_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"OverweightCount",
						vec![],
						[
							28u8, 72u8, 218u8, 167u8, 253u8, 30u8, 10u8, 51u8, 49u8, 101u8, 86u8,
							26u8, 146u8, 2u8, 153u8, 232u8, 129u8, 38u8, 111u8, 105u8, 246u8, 84u8,
							192u8, 157u8, 193u8, 57u8, 222u8, 122u8, 38u8, 160u8, 56u8, 39u8,
						],
					)
				}
				pub fn queue_suspended(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"QueueSuspended",
						vec![],
						[
							165u8, 66u8, 105u8, 244u8, 113u8, 43u8, 177u8, 252u8, 212u8, 243u8,
							143u8, 184u8, 87u8, 51u8, 163u8, 104u8, 29u8, 84u8, 119u8, 74u8, 233u8,
							129u8, 203u8, 105u8, 2u8, 101u8, 19u8, 170u8, 69u8, 253u8, 80u8, 132u8,
						],
					)
				}
			}
		}
	}
	pub mod polkadot_xcm {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_xcm::pallet::Error;
		pub type Call = runtime_types::pallet_xcm::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Send {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Send {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "send";
				}
				pub struct TeleportAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
					pub fee_asset_item: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for TeleportAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "teleport_assets";
				}
				pub struct ReserveTransferAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
					pub fee_asset_item: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ReserveTransferAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "reserve_transfer_assets";
				}
				pub struct Execute {
					pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm2>,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for Execute {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "execute";
				}
				pub struct ForceXcmVersion {
					pub location:
						::std::boxed::Box<runtime_types::xcm::v3::multilocation::MultiLocation>,
					pub xcm_version: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceXcmVersion {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_xcm_version";
				}
				pub struct ForceDefaultXcmVersion {
					pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceDefaultXcmVersion {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_default_xcm_version";
				}
				pub struct ForceSubscribeVersionNotify {
					pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceSubscribeVersionNotify {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_subscribe_version_notify";
				}
				pub struct ForceUnsubscribeVersionNotify {
					pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceUnsubscribeVersionNotify {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_unsubscribe_version_notify";
				}
				pub struct LimitedReserveTransferAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
					pub fee_asset_item: ::core::primitive::u32,
					pub weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for LimitedReserveTransferAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "limited_reserve_transfer_assets";
				}
				pub struct LimitedTeleportAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
					pub fee_asset_item: ::core::primitive::u32,
					pub weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for LimitedTeleportAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "limited_teleport_assets";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn send(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					message: runtime_types::xcm::VersionedXcm,
				) -> ::subxt::tx::Payload<types::Send> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"send",
						types::Send {
							dest: ::std::boxed::Box::new(dest),
							message: ::std::boxed::Box::new(message),
						},
						[
							147u8, 255u8, 86u8, 82u8, 17u8, 159u8, 225u8, 145u8, 220u8, 89u8, 71u8,
							23u8, 193u8, 249u8, 12u8, 70u8, 19u8, 140u8, 232u8, 97u8, 12u8, 220u8,
							113u8, 65u8, 4u8, 255u8, 138u8, 10u8, 231u8, 122u8, 67u8, 105u8,
						],
					)
				}
				pub fn teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::TeleportAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"teleport_assets",
						types::TeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							56u8, 144u8, 237u8, 60u8, 157u8, 5u8, 7u8, 129u8, 41u8, 149u8, 160u8,
							100u8, 233u8, 102u8, 181u8, 140u8, 115u8, 213u8, 29u8, 132u8, 16u8,
							30u8, 23u8, 82u8, 140u8, 134u8, 37u8, 87u8, 3u8, 99u8, 172u8, 42u8,
						],
					)
				}
				pub fn reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ReserveTransferAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"reserve_transfer_assets",
						types::ReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							21u8, 167u8, 44u8, 22u8, 210u8, 73u8, 148u8, 7u8, 91u8, 108u8, 148u8,
							205u8, 170u8, 243u8, 142u8, 224u8, 205u8, 119u8, 252u8, 22u8, 203u8,
							32u8, 73u8, 200u8, 178u8, 14u8, 167u8, 147u8, 166u8, 55u8, 14u8, 231u8,
						],
					)
				}
				pub fn execute(
					&self,
					message: runtime_types::xcm::VersionedXcm2,
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::Execute> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"execute",
						types::Execute { message: ::std::boxed::Box::new(message), max_weight },
						[
							15u8, 97u8, 86u8, 111u8, 105u8, 116u8, 109u8, 206u8, 70u8, 8u8, 57u8,
							232u8, 133u8, 132u8, 30u8, 219u8, 34u8, 69u8, 0u8, 213u8, 98u8, 241u8,
							186u8, 93u8, 216u8, 39u8, 73u8, 24u8, 193u8, 87u8, 92u8, 31u8,
						],
					)
				}
				pub fn force_xcm_version(
					&self,
					location: runtime_types::xcm::v3::multilocation::MultiLocation,
					xcm_version: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ForceXcmVersion> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_xcm_version",
						types::ForceXcmVersion {
							location: ::std::boxed::Box::new(location),
							xcm_version,
						},
						[
							84u8, 212u8, 64u8, 161u8, 17u8, 129u8, 213u8, 129u8, 79u8, 86u8, 117u8,
							246u8, 93u8, 1u8, 161u8, 23u8, 35u8, 171u8, 163u8, 200u8, 69u8, 157u8,
							71u8, 8u8, 225u8, 149u8, 254u8, 124u8, 38u8, 250u8, 164u8, 218u8,
						],
					)
				}
				pub fn force_default_xcm_version(
					&self,
					maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::Payload<types::ForceDefaultXcmVersion> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_default_xcm_version",
						types::ForceDefaultXcmVersion { maybe_xcm_version },
						[
							43u8, 114u8, 102u8, 104u8, 209u8, 234u8, 108u8, 173u8, 109u8, 188u8,
							94u8, 214u8, 136u8, 43u8, 153u8, 75u8, 161u8, 192u8, 76u8, 12u8, 221u8,
							237u8, 158u8, 247u8, 41u8, 193u8, 35u8, 174u8, 183u8, 207u8, 79u8,
							213u8,
						],
					)
				}
				pub fn force_subscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::tx::Payload<types::ForceSubscribeVersionNotify> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_subscribe_version_notify",
						types::ForceSubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						},
						[
							112u8, 254u8, 138u8, 12u8, 203u8, 176u8, 251u8, 167u8, 223u8, 0u8,
							71u8, 148u8, 19u8, 179u8, 47u8, 96u8, 188u8, 189u8, 14u8, 172u8, 1u8,
							1u8, 192u8, 107u8, 137u8, 158u8, 22u8, 9u8, 138u8, 241u8, 32u8, 47u8,
						],
					)
				}
				pub fn force_unsubscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::tx::Payload<types::ForceUnsubscribeVersionNotify> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_unsubscribe_version_notify",
						types::ForceUnsubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						},
						[
							205u8, 143u8, 230u8, 143u8, 166u8, 184u8, 53u8, 252u8, 118u8, 184u8,
							209u8, 227u8, 225u8, 184u8, 254u8, 244u8, 101u8, 56u8, 27u8, 128u8,
							40u8, 159u8, 178u8, 62u8, 63u8, 164u8, 59u8, 236u8, 1u8, 168u8, 202u8,
							42u8,
						],
					)
				}
				pub fn limited_reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::LimitedReserveTransferAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"limited_reserve_transfer_assets",
						types::LimitedReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							10u8, 139u8, 165u8, 239u8, 92u8, 178u8, 169u8, 62u8, 166u8, 236u8,
							50u8, 12u8, 196u8, 3u8, 233u8, 209u8, 3u8, 159u8, 184u8, 234u8, 171u8,
							46u8, 145u8, 134u8, 241u8, 155u8, 221u8, 173u8, 166u8, 94u8, 147u8,
							88u8,
						],
					)
				}
				pub fn limited_teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::LimitedTeleportAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"limited_teleport_assets",
						types::LimitedTeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							156u8, 205u8, 105u8, 18u8, 120u8, 130u8, 144u8, 67u8, 152u8, 188u8,
							109u8, 121u8, 4u8, 240u8, 123u8, 112u8, 72u8, 153u8, 2u8, 111u8, 183u8,
							170u8, 199u8, 82u8, 33u8, 117u8, 43u8, 133u8, 208u8, 44u8, 118u8,
							107u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Attempted(pub runtime_types::xcm::v3::traits::Outcome);
			impl ::subxt::events::StaticEvent for Attempted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Attempted";
			}
			pub struct Sent(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::Xcm,
			);
			impl ::subxt::events::StaticEvent for Sent {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Sent";
			}
			pub struct UnexpectedResponse(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for UnexpectedResponse {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "UnexpectedResponse";
			}
			pub struct ResponseReady(
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v3::Response,
			);
			impl ::subxt::events::StaticEvent for ResponseReady {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseReady";
			}
			pub struct Notified(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for Notified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Notified";
			}
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
			pub struct NotifyDispatchError(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for NotifyDispatchError {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDispatchError";
			}
			pub struct NotifyDecodeFailed(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for NotifyDecodeFailed {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDecodeFailed";
			}
			pub struct InvalidResponder(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub ::core::option::Option<runtime_types::xcm::v3::multilocation::MultiLocation>,
			);
			impl ::subxt::events::StaticEvent for InvalidResponder {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponder";
			}
			pub struct InvalidResponderVersion(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for InvalidResponderVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponderVersion";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct ResponseTaken(pub ::core::primitive::u64);
			impl ::subxt::events::StaticEvent for ResponseTaken {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseTaken";
			}
			pub struct AssetsTrapped(
				pub runtime_types::primitive_types::H256,
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::VersionedMultiAssets,
			);
			impl ::subxt::events::StaticEvent for AssetsTrapped {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AssetsTrapped";
			}
			pub struct VersionChangeNotified(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u32,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionChangeNotified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionChangeNotified";
			}
			pub struct SupportedVersionChanged(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for SupportedVersionChanged {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "SupportedVersionChanged";
			}
			pub struct NotifyTargetSendFail(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v3::traits::Error,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetSendFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetSendFail";
			}
			pub struct NotifyTargetMigrationFail(
				pub runtime_types::xcm::VersionedMultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetMigrationFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetMigrationFail";
			}
			pub struct InvalidQuerierVersion(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for InvalidQuerierVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidQuerierVersion";
			}
			pub struct InvalidQuerier(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::option::Option<runtime_types::xcm::v3::multilocation::MultiLocation>,
			);
			impl ::subxt::events::StaticEvent for InvalidQuerier {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidQuerier";
			}
			pub struct VersionNotifyStarted(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionNotifyStarted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyStarted";
			}
			pub struct VersionNotifyRequested(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionNotifyRequested {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyRequested";
			}
			pub struct VersionNotifyUnrequested(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionNotifyUnrequested {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyUnrequested";
			}
			pub struct FeesPaid(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for FeesPaid {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "FeesPaid";
			}
			pub struct AssetsClaimed(
				pub runtime_types::primitive_types::H256,
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
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
				pub fn query_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"QueryCounter",
						vec![],
						[
							216u8, 73u8, 160u8, 232u8, 60u8, 245u8, 218u8, 219u8, 152u8, 68u8,
							146u8, 219u8, 255u8, 7u8, 86u8, 112u8, 83u8, 49u8, 94u8, 173u8, 64u8,
							203u8, 147u8, 226u8, 236u8, 39u8, 129u8, 106u8, 209u8, 113u8, 150u8,
							50u8,
						],
					)
				}
				pub fn queries(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"Queries",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							119u8, 5u8, 12u8, 91u8, 117u8, 240u8, 52u8, 192u8, 135u8, 139u8, 220u8,
							78u8, 207u8, 199u8, 71u8, 163u8, 100u8, 17u8, 6u8, 65u8, 200u8, 245u8,
							191u8, 82u8, 232u8, 128u8, 126u8, 70u8, 39u8, 63u8, 148u8, 219u8,
						],
					)
				}
				pub fn queries_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"Queries",
						Vec::new(),
						[
							119u8, 5u8, 12u8, 91u8, 117u8, 240u8, 52u8, 192u8, 135u8, 139u8, 220u8,
							78u8, 207u8, 199u8, 71u8, 163u8, 100u8, 17u8, 6u8, 65u8, 200u8, 245u8,
							191u8, 82u8, 232u8, 128u8, 126u8, 70u8, 39u8, 63u8, 148u8, 219u8,
						],
					)
				}
				pub fn asset_traps(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitive_types::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"AssetTraps",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							148u8, 41u8, 254u8, 134u8, 61u8, 172u8, 126u8, 146u8, 78u8, 178u8,
							50u8, 77u8, 226u8, 8u8, 200u8, 78u8, 77u8, 91u8, 26u8, 133u8, 104u8,
							126u8, 28u8, 28u8, 202u8, 62u8, 87u8, 183u8, 231u8, 191u8, 5u8, 181u8,
						],
					)
				}
				pub fn asset_traps_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"AssetTraps",
						Vec::new(),
						[
							148u8, 41u8, 254u8, 134u8, 61u8, 172u8, 126u8, 146u8, 78u8, 178u8,
							50u8, 77u8, 226u8, 8u8, 200u8, 78u8, 77u8, 91u8, 26u8, 133u8, 104u8,
							126u8, 28u8, 28u8, 202u8, 62u8, 87u8, 183u8, 231u8, 191u8, 5u8, 181u8,
						],
					)
				}
				pub fn safe_xcm_version(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"SafeXcmVersion",
						vec![],
						[
							187u8, 8u8, 74u8, 126u8, 80u8, 215u8, 177u8, 60u8, 223u8, 123u8, 196u8,
							155u8, 166u8, 66u8, 25u8, 164u8, 191u8, 66u8, 116u8, 131u8, 116u8,
							188u8, 224u8, 122u8, 75u8, 195u8, 246u8, 188u8, 83u8, 134u8, 49u8,
							143u8,
						],
					)
				}
				pub fn supported_version(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"SupportedVersion",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							144u8, 22u8, 91u8, 30u8, 139u8, 164u8, 95u8, 149u8, 97u8, 247u8, 12u8,
							212u8, 96u8, 16u8, 134u8, 236u8, 74u8, 57u8, 244u8, 169u8, 68u8, 63u8,
							111u8, 86u8, 65u8, 229u8, 104u8, 51u8, 44u8, 100u8, 47u8, 191u8,
						],
					)
				}
				pub fn supported_version_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"SupportedVersion",
						Vec::new(),
						[
							144u8, 22u8, 91u8, 30u8, 139u8, 164u8, 95u8, 149u8, 97u8, 247u8, 12u8,
							212u8, 96u8, 16u8, 134u8, 236u8, 74u8, 57u8, 244u8, 169u8, 68u8, 63u8,
							111u8, 86u8, 65u8, 229u8, 104u8, 51u8, 44u8, 100u8, 47u8, 191u8,
						],
					)
				}
				pub fn version_notifiers(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifiers",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							49u8, 190u8, 73u8, 67u8, 91u8, 69u8, 121u8, 206u8, 25u8, 82u8, 29u8,
							170u8, 157u8, 201u8, 168u8, 93u8, 181u8, 55u8, 226u8, 142u8, 136u8,
							46u8, 117u8, 208u8, 130u8, 90u8, 129u8, 39u8, 151u8, 92u8, 118u8, 75u8,
						],
					)
				}
				pub fn version_notifiers_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifiers",
						Vec::new(),
						[
							49u8, 190u8, 73u8, 67u8, 91u8, 69u8, 121u8, 206u8, 25u8, 82u8, 29u8,
							170u8, 157u8, 201u8, 168u8, 93u8, 181u8, 55u8, 226u8, 142u8, 136u8,
							46u8, 117u8, 208u8, 130u8, 90u8, 129u8, 39u8, 151u8, 92u8, 118u8, 75u8,
						],
					)
				}
				pub fn version_notify_targets(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u64,
						runtime_types::sp_weights::weight_v2::Weight,
						::core::primitive::u32,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifyTargets",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							1u8, 195u8, 40u8, 83u8, 216u8, 175u8, 241u8, 95u8, 42u8, 7u8, 85u8,
							253u8, 223u8, 241u8, 195u8, 41u8, 41u8, 21u8, 17u8, 171u8, 216u8,
							150u8, 39u8, 165u8, 215u8, 194u8, 201u8, 225u8, 179u8, 12u8, 52u8,
							173u8,
						],
					)
				}
				pub fn version_notify_targets_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u64,
						runtime_types::sp_weights::weight_v2::Weight,
						::core::primitive::u32,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifyTargets",
						Vec::new(),
						[
							1u8, 195u8, 40u8, 83u8, 216u8, 175u8, 241u8, 95u8, 42u8, 7u8, 85u8,
							253u8, 223u8, 241u8, 195u8, 41u8, 41u8, 21u8, 17u8, 171u8, 216u8,
							150u8, 39u8, 165u8, 215u8, 194u8, 201u8, 225u8, 179u8, 12u8, 52u8,
							173u8,
						],
					)
				}
				pub fn version_discovery_queue(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::xcm::VersionedMultiLocation,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionDiscoveryQueue",
						vec![],
						[
							110u8, 87u8, 102u8, 193u8, 125u8, 129u8, 0u8, 221u8, 218u8, 229u8,
							101u8, 94u8, 74u8, 229u8, 246u8, 180u8, 113u8, 11u8, 15u8, 159u8, 98u8,
							90u8, 30u8, 112u8, 164u8, 236u8, 151u8, 220u8, 19u8, 83u8, 67u8, 248u8,
						],
					)
				}
				pub fn current_migration(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::VersionMigrationStage,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"CurrentMigration",
						vec![],
						[
							74u8, 138u8, 181u8, 162u8, 59u8, 251u8, 37u8, 28u8, 232u8, 51u8, 30u8,
							152u8, 252u8, 133u8, 95u8, 195u8, 47u8, 127u8, 21u8, 44u8, 62u8, 143u8,
							170u8, 234u8, 160u8, 37u8, 131u8, 179u8, 57u8, 241u8, 140u8, 124u8,
						],
					)
				}
				pub fn remote_locked_fungibles(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
					_2: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedAssetId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::RemoteLockedFungibleRecord,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"RemoteLockedFungibles",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_2.borrow()),
						],
						[
							236u8, 199u8, 155u8, 149u8, 81u8, 35u8, 254u8, 175u8, 66u8, 88u8,
							159u8, 237u8, 4u8, 35u8, 26u8, 203u8, 40u8, 84u8, 66u8, 195u8, 243u8,
							50u8, 165u8, 52u8, 100u8, 139u8, 124u8, 48u8, 248u8, 254u8, 162u8,
							96u8,
						],
					)
				}
				pub fn remote_locked_fungibles_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::RemoteLockedFungibleRecord,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"RemoteLockedFungibles",
						Vec::new(),
						[
							236u8, 199u8, 155u8, 149u8, 81u8, 35u8, 254u8, 175u8, 66u8, 88u8,
							159u8, 237u8, 4u8, 35u8, 26u8, 203u8, 40u8, 84u8, 66u8, 195u8, 243u8,
							50u8, 165u8, 52u8, 100u8, 139u8, 124u8, 48u8, 248u8, 254u8, 162u8,
							96u8,
						],
					)
				}
				pub fn locked_fungibles(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u128,
						runtime_types::xcm::VersionedMultiLocation,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"LockedFungibles",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							110u8, 220u8, 127u8, 176u8, 219u8, 23u8, 132u8, 36u8, 224u8, 187u8,
							25u8, 103u8, 126u8, 99u8, 34u8, 105u8, 57u8, 182u8, 162u8, 69u8, 24u8,
							67u8, 221u8, 103u8, 79u8, 139u8, 187u8, 162u8, 113u8, 109u8, 163u8,
							35u8,
						],
					)
				}
				pub fn locked_fungibles_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u128,
						runtime_types::xcm::VersionedMultiLocation,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"LockedFungibles",
						Vec::new(),
						[
							110u8, 220u8, 127u8, 176u8, 219u8, 23u8, 132u8, 36u8, 224u8, 187u8,
							25u8, 103u8, 126u8, 99u8, 34u8, 105u8, 57u8, 182u8, 162u8, 69u8, 24u8,
							67u8, 221u8, 103u8, 79u8, 139u8, 187u8, 162u8, 113u8, 109u8, 163u8,
							35u8,
						],
					)
				}
			}
		}
	}
	pub mod cumulus_xcm {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::cumulus_pallet_xcm::pallet::Error;
		pub type Call = runtime_types::cumulus_pallet_xcm::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "InvalidFormat";
			}
			pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			pub struct ExecutedDownward(
				pub [::core::primitive::u8; 32usize],
				pub runtime_types::xcm::v3::traits::Outcome,
			);
			impl ::subxt::events::StaticEvent for ExecutedDownward {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "ExecutedDownward";
			}
		}
	}
	pub mod dmp_queue {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::cumulus_pallet_dmp_queue::pallet::Error;
		pub type Call = runtime_types::cumulus_pallet_dmp_queue::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct ServiceOverweight {
					pub index: ::core::primitive::u64,
					pub weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for ServiceOverweight {
					const PALLET: &'static str = "DmpQueue";
					const CALL: &'static str = "service_overweight";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::ServiceOverweight> {
					::subxt::tx::Payload::new_static(
						"DmpQueue",
						"service_overweight",
						types::ServiceOverweight { index, weight_limit },
						[
							235u8, 203u8, 220u8, 162u8, 173u8, 117u8, 224u8, 194u8, 176u8, 125u8,
							50u8, 74u8, 180u8, 37u8, 126u8, 227u8, 138u8, 213u8, 227u8, 35u8,
							247u8, 18u8, 160u8, 231u8, 97u8, 149u8, 144u8, 49u8, 34u8, 146u8, 32u8,
							7u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_dmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct InvalidFormat {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "InvalidFormat";
			}
			pub struct UnsupportedVersion {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			pub struct ExecutedDownward {
				pub message_id: [::core::primitive::u8; 32usize],
				pub outcome: runtime_types::xcm::v3::traits::Outcome,
			}
			impl ::subxt::events::StaticEvent for ExecutedDownward {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "ExecutedDownward";
			}
			pub struct WeightExhausted {
				pub message_id: [::core::primitive::u8; 32usize],
				pub remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
				pub required_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for WeightExhausted {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "WeightExhausted";
			}
			pub struct OverweightEnqueued {
				pub message_id: [::core::primitive::u8; 32usize],
				pub overweight_index: ::core::primitive::u64,
				pub required_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightEnqueued {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			pub struct OverweightServiced {
				pub overweight_index: ::core::primitive::u64,
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightServiced {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightServiced";
			}
			pub struct MaxMessagesExhausted {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for MaxMessagesExhausted {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "MaxMessagesExhausted";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn configuration(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_pallet_dmp_queue::ConfigData,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"Configuration",
						vec![],
						[
							28u8, 58u8, 57u8, 84u8, 115u8, 69u8, 158u8, 234u8, 180u8, 37u8, 138u8,
							120u8, 182u8, 145u8, 109u8, 203u8, 62u8, 102u8, 168u8, 56u8, 236u8,
							10u8, 236u8, 104u8, 232u8, 245u8, 107u8, 143u8, 247u8, 232u8, 135u8,
							131u8,
						],
					)
				}
				pub fn page_index(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_pallet_dmp_queue::PageIndexData,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"PageIndex",
						vec![],
						[
							246u8, 129u8, 111u8, 255u8, 168u8, 54u8, 121u8, 21u8, 159u8, 142u8,
							252u8, 173u8, 3u8, 191u8, 202u8, 158u8, 86u8, 26u8, 76u8, 134u8, 201u8,
							138u8, 103u8, 75u8, 223u8, 57u8, 36u8, 45u8, 171u8, 190u8, 21u8, 60u8,
						],
					)
				}
				pub fn pages(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"Pages",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							24u8, 215u8, 210u8, 131u8, 23u8, 56u8, 71u8, 143u8, 35u8, 151u8, 223u8,
							133u8, 42u8, 32u8, 180u8, 85u8, 146u8, 166u8, 6u8, 168u8, 227u8, 128u8,
							30u8, 108u8, 103u8, 16u8, 169u8, 235u8, 238u8, 224u8, 247u8, 233u8,
						],
					)
				}
				pub fn pages_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"Pages",
						Vec::new(),
						[
							24u8, 215u8, 210u8, 131u8, 23u8, 56u8, 71u8, 143u8, 35u8, 151u8, 223u8,
							133u8, 42u8, 32u8, 180u8, 85u8, 146u8, 166u8, 6u8, 168u8, 227u8, 128u8,
							30u8, 108u8, 103u8, 16u8, 169u8, 235u8, 238u8, 224u8, 247u8, 233u8,
						],
					)
				}
				pub fn overweight(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::std::vec::Vec<::core::primitive::u8>),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"Overweight",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							86u8, 97u8, 243u8, 7u8, 134u8, 189u8, 7u8, 126u8, 8u8, 108u8, 152u8,
							48u8, 230u8, 8u8, 71u8, 83u8, 151u8, 125u8, 18u8, 168u8, 38u8, 38u8,
							117u8, 85u8, 143u8, 187u8, 122u8, 13u8, 104u8, 52u8, 198u8, 138u8,
						],
					)
				}
				pub fn overweight_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::std::vec::Vec<::core::primitive::u8>),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"Overweight",
						Vec::new(),
						[
							86u8, 97u8, 243u8, 7u8, 134u8, 189u8, 7u8, 126u8, 8u8, 108u8, 152u8,
							48u8, 230u8, 8u8, 71u8, 83u8, 151u8, 125u8, 18u8, 168u8, 38u8, 38u8,
							117u8, 85u8, 143u8, 187u8, 122u8, 13u8, 104u8, 52u8, 198u8, 138u8,
						],
					)
				}
				pub fn counter_for_overweight(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"DmpQueue",
						"CounterForOverweight",
						vec![],
						[
							44u8, 249u8, 133u8, 204u8, 169u8, 253u8, 23u8, 157u8, 132u8, 193u8,
							28u8, 178u8, 156u8, 176u8, 206u8, 46u8, 79u8, 254u8, 174u8, 236u8,
							143u8, 219u8, 59u8, 43u8, 36u8, 109u8, 244u8, 206u8, 48u8, 126u8,
							247u8, 0u8,
						],
					)
				}
			}
		}
	}
	pub mod x_tokens {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::orml_xtokens::module::Error;
		pub type Call = runtime_types::orml_xtokens::module::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Transfer {
					pub currency_id: runtime_types::primitives::currency::CurrencyId,
					pub amount: ::core::primitive::u128,
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for Transfer {
					const PALLET: &'static str = "XTokens";
					const CALL: &'static str = "transfer";
				}
				pub struct TransferMultiasset {
					pub asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferMultiasset {
					const PALLET: &'static str = "XTokens";
					const CALL: &'static str = "transfer_multiasset";
				}
				pub struct TransferWithFee {
					pub currency_id: runtime_types::primitives::currency::CurrencyId,
					pub amount: ::core::primitive::u128,
					pub fee: ::core::primitive::u128,
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferWithFee {
					const PALLET: &'static str = "XTokens";
					const CALL: &'static str = "transfer_with_fee";
				}
				pub struct TransferMultiassetWithFee {
					pub asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
					pub fee: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferMultiassetWithFee {
					const PALLET: &'static str = "XTokens";
					const CALL: &'static str = "transfer_multiasset_with_fee";
				}
				pub struct TransferMulticurrencies {
					pub currencies: ::std::vec::Vec<(
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					)>,
					pub fee_item: ::core::primitive::u32,
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferMulticurrencies {
					const PALLET: &'static str = "XTokens";
					const CALL: &'static str = "transfer_multicurrencies";
				}
				pub struct TransferMultiassets {
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
					pub fee_item: ::core::primitive::u32,
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferMultiassets {
					const PALLET: &'static str = "XTokens";
					const CALL: &'static str = "transfer_multiassets";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn transfer(
					&self,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::Transfer> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer",
						types::Transfer {
							currency_id,
							amount,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							248u8, 69u8, 26u8, 125u8, 60u8, 0u8, 39u8, 2u8, 226u8, 250u8, 36u8,
							101u8, 101u8, 44u8, 58u8, 31u8, 247u8, 190u8, 103u8, 99u8, 84u8, 246u8,
							147u8, 115u8, 22u8, 166u8, 137u8, 27u8, 97u8, 136u8, 15u8, 109u8,
						],
					)
				}
				pub fn transfer_multiasset(
					&self,
					asset: runtime_types::xcm::VersionedMultiAsset,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::TransferMultiasset> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer_multiasset",
						types::TransferMultiasset {
							asset: ::std::boxed::Box::new(asset),
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							41u8, 136u8, 12u8, 95u8, 87u8, 89u8, 79u8, 94u8, 22u8, 194u8, 186u8,
							131u8, 93u8, 130u8, 219u8, 51u8, 45u8, 231u8, 203u8, 58u8, 230u8, 43u8,
							165u8, 117u8, 141u8, 64u8, 99u8, 128u8, 62u8, 30u8, 122u8, 27u8,
						],
					)
				}
				pub fn transfer_with_fee(
					&self,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
					fee: ::core::primitive::u128,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::TransferWithFee> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer_with_fee",
						types::TransferWithFee {
							currency_id,
							amount,
							fee,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							151u8, 61u8, 25u8, 253u8, 31u8, 74u8, 113u8, 94u8, 126u8, 247u8, 127u8,
							7u8, 194u8, 180u8, 217u8, 1u8, 228u8, 116u8, 196u8, 22u8, 182u8, 84u8,
							157u8, 52u8, 156u8, 0u8, 240u8, 161u8, 93u8, 167u8, 88u8, 167u8,
						],
					)
				}
				pub fn transfer_multiasset_with_fee(
					&self,
					asset: runtime_types::xcm::VersionedMultiAsset,
					fee: runtime_types::xcm::VersionedMultiAsset,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::TransferMultiassetWithFee> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer_multiasset_with_fee",
						types::TransferMultiassetWithFee {
							asset: ::std::boxed::Box::new(asset),
							fee: ::std::boxed::Box::new(fee),
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							255u8, 67u8, 191u8, 126u8, 174u8, 56u8, 54u8, 112u8, 9u8, 247u8, 225u8,
							96u8, 118u8, 107u8, 27u8, 185u8, 42u8, 217u8, 21u8, 26u8, 139u8, 78u8,
							82u8, 179u8, 143u8, 209u8, 122u8, 145u8, 157u8, 209u8, 219u8, 211u8,
						],
					)
				}
				pub fn transfer_multicurrencies(
					&self,
					currencies: ::std::vec::Vec<(
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					)>,
					fee_item: ::core::primitive::u32,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::TransferMulticurrencies> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer_multicurrencies",
						types::TransferMulticurrencies {
							currencies,
							fee_item,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							224u8, 138u8, 30u8, 41u8, 165u8, 207u8, 86u8, 128u8, 72u8, 191u8,
							170u8, 81u8, 231u8, 225u8, 68u8, 111u8, 66u8, 21u8, 172u8, 210u8,
							251u8, 2u8, 98u8, 135u8, 108u8, 13u8, 226u8, 63u8, 188u8, 213u8, 146u8,
							92u8,
						],
					)
				}
				pub fn transfer_multiassets(
					&self,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_item: ::core::primitive::u32,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::TransferMultiassets> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer_multiassets",
						types::TransferMultiassets {
							assets: ::std::boxed::Box::new(assets),
							fee_item,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							62u8, 97u8, 148u8, 111u8, 216u8, 123u8, 216u8, 191u8, 196u8, 99u8,
							227u8, 31u8, 20u8, 178u8, 202u8, 25u8, 48u8, 132u8, 186u8, 196u8,
							150u8, 69u8, 51u8, 252u8, 250u8, 237u8, 177u8, 214u8, 246u8, 85u8,
							239u8, 137u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::orml_xtokens::module::Event;
		pub mod events {
			use super::runtime_types;
			pub struct TransferredMultiAssets {
				pub sender: runtime_types::sp_core::crypto::AccountId32,
				pub assets: runtime_types::xcm::v3::multiasset::MultiAssets,
				pub fee: runtime_types::xcm::v3::multiasset::MultiAsset,
				pub dest: runtime_types::xcm::v3::multilocation::MultiLocation,
			}
			impl ::subxt::events::StaticEvent for TransferredMultiAssets {
				const PALLET: &'static str = "XTokens";
				const EVENT: &'static str = "TransferredMultiAssets";
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn self_location(
					&self,
				) -> ::subxt::constants::Address<runtime_types::xcm::v3::multilocation::MultiLocation>
				{
					::subxt::constants::Address::new_static(
						"XTokens",
						"SelfLocation",
						[
							111u8, 216u8, 110u8, 49u8, 228u8, 60u8, 105u8, 227u8, 141u8, 11u8,
							125u8, 36u8, 252u8, 184u8, 156u8, 242u8, 146u8, 64u8, 244u8, 7u8, 70u8,
							186u8, 11u8, 235u8, 13u8, 254u8, 132u8, 2u8, 154u8, 117u8, 85u8, 154u8,
						],
					)
				}
				pub fn base_xcm_weight(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_weights::weight_v2::Weight> {
					::subxt::constants::Address::new_static(
						"XTokens",
						"BaseXcmWeight",
						[
							149u8, 252u8, 129u8, 80u8, 169u8, 36u8, 79u8, 127u8, 240u8, 156u8,
							56u8, 202u8, 219u8, 86u8, 5u8, 65u8, 245u8, 148u8, 138u8, 243u8, 210u8,
							128u8, 234u8, 216u8, 240u8, 219u8, 123u8, 235u8, 21u8, 158u8, 237u8,
							112u8,
						],
					)
				}
			}
		}
	}
	pub mod unknown_tokens {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::orml_unknown_tokens::module::Error;
		pub type Call = runtime_types::orml_unknown_tokens::module::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub type Event = runtime_types::orml_unknown_tokens::module::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Deposited {
				pub asset: runtime_types::xcm::v3::multiasset::MultiAsset,
				pub who: runtime_types::xcm::v3::multilocation::MultiLocation,
			}
			impl ::subxt::events::StaticEvent for Deposited {
				const PALLET: &'static str = "UnknownTokens";
				const EVENT: &'static str = "Deposited";
			}
			pub struct Withdrawn {
				pub asset: runtime_types::xcm::v3::multiasset::MultiAsset,
				pub who: runtime_types::xcm::v3::multilocation::MultiLocation,
			}
			impl ::subxt::events::StaticEvent for Withdrawn {
				const PALLET: &'static str = "UnknownTokens";
				const EVENT: &'static str = "Withdrawn";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn concrete_fungible_balances(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::xcm::v3::multilocation::MultiLocation>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::v3::multilocation::MultiLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"UnknownTokens",
						"ConcreteFungibleBalances",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							228u8, 155u8, 200u8, 118u8, 186u8, 86u8, 50u8, 35u8, 240u8, 17u8, 69u8,
							4u8, 146u8, 149u8, 215u8, 144u8, 23u8, 163u8, 5u8, 69u8, 229u8, 60u8,
							93u8, 236u8, 213u8, 78u8, 199u8, 4u8, 80u8, 57u8, 112u8, 170u8,
						],
					)
				}
				pub fn concrete_fungible_balances_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"UnknownTokens",
						"ConcreteFungibleBalances",
						Vec::new(),
						[
							228u8, 155u8, 200u8, 118u8, 186u8, 86u8, 50u8, 35u8, 240u8, 17u8, 69u8,
							4u8, 146u8, 149u8, 215u8, 144u8, 23u8, 163u8, 5u8, 69u8, 229u8, 60u8,
							93u8, 236u8, 213u8, 78u8, 199u8, 4u8, 80u8, 57u8, 112u8, 170u8,
						],
					)
				}
				pub fn abstract_fungible_balances(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::xcm::v3::multilocation::MultiLocation>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"UnknownTokens",
						"AbstractFungibleBalances",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							222u8, 175u8, 193u8, 209u8, 199u8, 214u8, 193u8, 232u8, 181u8, 205u8,
							127u8, 184u8, 127u8, 50u8, 238u8, 14u8, 93u8, 84u8, 28u8, 99u8, 255u8,
							186u8, 22u8, 105u8, 86u8, 42u8, 253u8, 42u8, 119u8, 91u8, 170u8, 89u8,
						],
					)
				}
				pub fn abstract_fungible_balances_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"UnknownTokens",
						"AbstractFungibleBalances",
						Vec::new(),
						[
							222u8, 175u8, 193u8, 209u8, 199u8, 214u8, 193u8, 232u8, 181u8, 205u8,
							127u8, 184u8, 127u8, 50u8, 238u8, 14u8, 93u8, 84u8, 28u8, 99u8, 255u8,
							186u8, 22u8, 105u8, 86u8, 42u8, 253u8, 42u8, 119u8, 91u8, 170u8, 89u8,
						],
					)
				}
			}
		}
	}
	pub mod tokens {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::orml_tokens::module::Error;
		pub type Call = runtime_types::orml_tokens::module::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Transfer {
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub currency_id: runtime_types::primitives::currency::CurrencyId,
					#[codec::codec(compact)]
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Transfer {
					const PALLET: &'static str = "Tokens";
					const CALL: &'static str = "transfer";
				}
				pub struct TransferAll {
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub currency_id: runtime_types::primitives::currency::CurrencyId,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAll {
					const PALLET: &'static str = "Tokens";
					const CALL: &'static str = "transfer_all";
				}
				pub struct TransferKeepAlive {
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub currency_id: runtime_types::primitives::currency::CurrencyId,
					#[codec::codec(compact)]
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
					const PALLET: &'static str = "Tokens";
					const CALL: &'static str = "transfer_keep_alive";
				}
				pub struct ForceTransfer {
					pub source: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub currency_id: runtime_types::primitives::currency::CurrencyId,
					#[codec::codec(compact)]
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "Tokens";
					const CALL: &'static str = "force_transfer";
				}
				pub struct SetBalance {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub currency_id: runtime_types::primitives::currency::CurrencyId,
					#[codec::codec(compact)]
					pub new_free: ::core::primitive::u128,
					#[codec::codec(compact)]
					pub new_reserved: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetBalance {
					const PALLET: &'static str = "Tokens";
					const CALL: &'static str = "set_balance";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn transfer(
					&self,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Transfer> {
					::subxt::tx::Payload::new_static(
						"Tokens",
						"transfer",
						types::Transfer { dest, currency_id, amount },
						[
							155u8, 164u8, 35u8, 37u8, 234u8, 147u8, 149u8, 51u8, 30u8, 5u8, 130u8,
							70u8, 182u8, 87u8, 84u8, 100u8, 170u8, 164u8, 145u8, 159u8, 100u8,
							157u8, 151u8, 233u8, 245u8, 68u8, 228u8, 160u8, 101u8, 206u8, 237u8,
							221u8,
						],
					)
				}
				pub fn transfer_all(
					&self,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::TransferAll> {
					::subxt::tx::Payload::new_static(
						"Tokens",
						"transfer_all",
						types::TransferAll { dest, currency_id, keep_alive },
						[
							150u8, 176u8, 20u8, 65u8, 222u8, 26u8, 159u8, 5u8, 137u8, 70u8, 2u8,
							35u8, 16u8, 20u8, 114u8, 87u8, 161u8, 116u8, 115u8, 42u8, 19u8, 86u8,
							2u8, 25u8, 162u8, 69u8, 154u8, 103u8, 181u8, 213u8, 231u8, 231u8,
						],
					)
				}
				pub fn transfer_keep_alive(
					&self,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
					::subxt::tx::Payload::new_static(
						"Tokens",
						"transfer_keep_alive",
						types::TransferKeepAlive { dest, currency_id, amount },
						[
							28u8, 165u8, 96u8, 101u8, 126u8, 53u8, 40u8, 115u8, 42u8, 190u8, 0u8,
							140u8, 249u8, 11u8, 156u8, 31u8, 124u8, 251u8, 210u8, 93u8, 201u8,
							30u8, 162u8, 253u8, 163u8, 244u8, 85u8, 229u8, 209u8, 110u8, 197u8,
							214u8,
						],
					)
				}
				pub fn force_transfer(
					&self,
					source: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Tokens",
						"force_transfer",
						types::ForceTransfer { source, dest, currency_id, amount },
						[
							238u8, 69u8, 119u8, 98u8, 183u8, 196u8, 3u8, 233u8, 171u8, 236u8,
							163u8, 252u8, 88u8, 184u8, 156u8, 32u8, 15u8, 247u8, 102u8, 156u8,
							24u8, 171u8, 244u8, 75u8, 198u8, 50u8, 180u8, 247u8, 4u8, 104u8, 94u8,
							198u8,
						],
					)
				}
				pub fn set_balance(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::SetBalance> {
					::subxt::tx::Payload::new_static(
						"Tokens",
						"set_balance",
						types::SetBalance { who, currency_id, new_free, new_reserved },
						[
							92u8, 154u8, 179u8, 188u8, 197u8, 93u8, 178u8, 196u8, 87u8, 215u8,
							40u8, 57u8, 102u8, 76u8, 140u8, 242u8, 11u8, 128u8, 32u8, 16u8, 153u8,
							205u8, 201u8, 130u8, 57u8, 218u8, 81u8, 118u8, 164u8, 78u8, 238u8,
							157u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::orml_tokens::module::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Endowed {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Endowed";
			}
			pub struct DustLost {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "DustLost";
			}
			pub struct Transfer {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub from: runtime_types::sp_core::crypto::AccountId32,
				pub to: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Transfer";
			}
			pub struct Reserved {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Reserved";
			}
			pub struct Unreserved {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Unreserved";
			}
			pub struct ReserveRepatriated {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub from: runtime_types::sp_core::crypto::AccountId32,
				pub to: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
				pub status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			pub struct BalanceSet {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub free: ::core::primitive::u128,
				pub reserved: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "BalanceSet";
			}
			pub struct TotalIssuanceSet {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TotalIssuanceSet {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "TotalIssuanceSet";
			}
			pub struct Withdrawn {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdrawn {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Withdrawn";
			}
			pub struct Slashed {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub free_amount: ::core::primitive::u128,
				pub reserved_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Slashed";
			}
			pub struct Deposited {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposited {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Deposited";
			}
			pub struct LockSet {
				pub lock_id: [::core::primitive::u8; 8usize],
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for LockSet {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "LockSet";
			}
			pub struct LockRemoved {
				pub lock_id: [::core::primitive::u8; 8usize],
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for LockRemoved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "LockRemoved";
			}
			pub struct Locked {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Locked {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Locked";
			}
			pub struct Unlocked {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unlocked {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Unlocked";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn total_issuance(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tokens",
						"TotalIssuance",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							188u8, 142u8, 153u8, 118u8, 124u8, 144u8, 106u8, 2u8, 123u8, 63u8,
							211u8, 111u8, 79u8, 238u8, 10u8, 162u8, 112u8, 19u8, 226u8, 251u8,
							152u8, 53u8, 220u8, 208u8, 9u8, 214u8, 50u8, 30u8, 226u8, 65u8, 142u8,
							173u8,
						],
					)
				}
				pub fn total_issuance_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tokens",
						"TotalIssuance",
						Vec::new(),
						[
							188u8, 142u8, 153u8, 118u8, 124u8, 144u8, 106u8, 2u8, 123u8, 63u8,
							211u8, 111u8, 79u8, 238u8, 10u8, 162u8, 112u8, 19u8, 226u8, 251u8,
							152u8, 53u8, 220u8, 208u8, 9u8, 214u8, 50u8, 30u8, 226u8, 65u8, 142u8,
							173u8,
						],
					)
				}
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
					_1: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::orml_tokens::BalanceLock<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tokens",
						"Locks",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							104u8, 10u8, 162u8, 168u8, 35u8, 44u8, 14u8, 140u8, 188u8, 93u8, 195u8,
							216u8, 65u8, 154u8, 153u8, 86u8, 140u8, 234u8, 235u8, 203u8, 112u8,
							236u8, 152u8, 5u8, 146u8, 233u8, 240u8, 177u8, 25u8, 161u8, 253u8,
							20u8,
						],
					)
				}
				pub fn locks_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::orml_tokens::BalanceLock<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tokens",
						"Locks",
						Vec::new(),
						[
							104u8, 10u8, 162u8, 168u8, 35u8, 44u8, 14u8, 140u8, 188u8, 93u8, 195u8,
							216u8, 65u8, 154u8, 153u8, 86u8, 140u8, 234u8, 235u8, 203u8, 112u8,
							236u8, 152u8, 5u8, 146u8, 233u8, 240u8, 177u8, 25u8, 161u8, 253u8,
							20u8,
						],
					)
				}
				pub fn accounts(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
					_1: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::orml_tokens::AccountData<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tokens",
						"Accounts",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							130u8, 6u8, 59u8, 156u8, 189u8, 242u8, 152u8, 83u8, 130u8, 192u8,
							134u8, 235u8, 119u8, 180u8, 227u8, 230u8, 121u8, 220u8, 196u8, 105u8,
							7u8, 153u8, 24u8, 199u8, 112u8, 239u8, 206u8, 29u8, 237u8, 255u8, 72u8,
							225u8,
						],
					)
				}
				pub fn accounts_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::orml_tokens::AccountData<::core::primitive::u128>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tokens",
						"Accounts",
						Vec::new(),
						[
							130u8, 6u8, 59u8, 156u8, 189u8, 242u8, 152u8, 83u8, 130u8, 192u8,
							134u8, 235u8, 119u8, 180u8, 227u8, 230u8, 121u8, 220u8, 196u8, 105u8,
							7u8, 153u8, 24u8, 199u8, 112u8, 239u8, 206u8, 29u8, 237u8, 255u8, 72u8,
							225u8,
						],
					)
				}
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
					_1: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::orml_tokens::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tokens",
						"Reserves",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							240u8, 151u8, 89u8, 215u8, 145u8, 52u8, 106u8, 217u8, 62u8, 12u8,
							253u8, 94u8, 13u8, 190u8, 62u8, 231u8, 218u8, 254u8, 55u8, 68u8, 197u8,
							112u8, 153u8, 113u8, 22u8, 130u8, 194u8, 207u8, 69u8, 242u8, 70u8,
							20u8,
						],
					)
				}
				pub fn reserves_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::orml_tokens::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tokens",
						"Reserves",
						Vec::new(),
						[
							240u8, 151u8, 89u8, 215u8, 145u8, 52u8, 106u8, 217u8, 62u8, 12u8,
							253u8, 94u8, 13u8, 190u8, 62u8, 231u8, 218u8, 254u8, 55u8, 68u8, 197u8,
							112u8, 153u8, 113u8, 22u8, 130u8, 194u8, 207u8, 69u8, 242u8, 70u8,
							20u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Tokens",
						"MaxLocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Tokens",
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
	pub mod currency_factory {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_currency_factory::pallet::Error;
		pub type Call = runtime_types::pallet_currency_factory::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct AddRange {
					pub length: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddRange {
					const PALLET: &'static str = "CurrencyFactory";
					const CALL: &'static str = "add_range";
				}
				pub struct SetMetadata {
					pub asset_id: runtime_types::primitives::currency::CurrencyId,
					pub metadata: runtime_types::composable_traits::assets::BasicAssetMetadata,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMetadata {
					const PALLET: &'static str = "CurrencyFactory";
					const CALL: &'static str = "set_metadata";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_range(
					&self,
					length: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::AddRange> {
					::subxt::tx::Payload::new_static(
						"CurrencyFactory",
						"add_range",
						types::AddRange { length },
						[
							190u8, 153u8, 246u8, 239u8, 248u8, 25u8, 37u8, 117u8, 87u8, 72u8,
							130u8, 101u8, 36u8, 132u8, 8u8, 125u8, 72u8, 139u8, 74u8, 3u8, 190u8,
							228u8, 53u8, 184u8, 120u8, 245u8, 92u8, 2u8, 184u8, 49u8, 23u8, 157u8,
						],
					)
				}
				pub fn set_metadata(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					metadata: runtime_types::composable_traits::assets::BasicAssetMetadata,
				) -> ::subxt::tx::Payload<types::SetMetadata> {
					::subxt::tx::Payload::new_static(
						"CurrencyFactory",
						"set_metadata",
						types::SetMetadata { asset_id, metadata },
						[
							155u8, 165u8, 132u8, 71u8, 156u8, 202u8, 139u8, 13u8, 70u8, 202u8,
							166u8, 225u8, 255u8, 85u8, 78u8, 90u8, 140u8, 1u8, 23u8, 203u8, 166u8,
							154u8, 40u8, 222u8, 143u8, 64u8, 39u8, 66u8, 84u8, 165u8, 253u8, 107u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_currency_factory::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct RangeCreated {
				pub range: runtime_types::pallet_currency_factory::ranges::Range<
					runtime_types::primitives::currency::CurrencyId,
				>,
			}
			impl ::subxt::events::StaticEvent for RangeCreated {
				const PALLET: &'static str = "CurrencyFactory";
				const EVENT: &'static str = "RangeCreated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn asset_id_ranges(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_currency_factory::ranges::Ranges<
						runtime_types::primitives::currency::CurrencyId,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CurrencyFactory",
						"AssetIdRanges",
						vec![],
						[
							87u8, 186u8, 203u8, 98u8, 241u8, 180u8, 153u8, 214u8, 81u8, 85u8,
							149u8, 55u8, 194u8, 16u8, 4u8, 222u8, 145u8, 73u8, 190u8, 240u8, 205u8,
							206u8, 204u8, 40u8, 4u8, 53u8, 161u8, 101u8, 86u8, 253u8, 78u8, 27u8,
						],
					)
				}
				pub fn asset_ed(
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
						"CurrencyFactory",
						"AssetEd",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							246u8, 33u8, 253u8, 183u8, 229u8, 24u8, 41u8, 99u8, 204u8, 227u8,
							118u8, 12u8, 109u8, 5u8, 129u8, 163u8, 60u8, 149u8, 183u8, 224u8,
							119u8, 36u8, 210u8, 181u8, 216u8, 119u8, 121u8, 136u8, 224u8, 98u8,
							187u8, 116u8,
						],
					)
				}
				pub fn asset_ed_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CurrencyFactory",
						"AssetEd",
						Vec::new(),
						[
							246u8, 33u8, 253u8, 183u8, 229u8, 24u8, 41u8, 99u8, 204u8, 227u8,
							118u8, 12u8, 109u8, 5u8, 129u8, 163u8, 60u8, 149u8, 183u8, 224u8,
							119u8, 36u8, 210u8, 181u8, 216u8, 119u8, 121u8, 136u8, 224u8, 98u8,
							187u8, 116u8,
						],
					)
				}
				pub fn asset_metadata(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_traits::assets::BasicAssetMetadata,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CurrencyFactory",
						"AssetMetadata",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							75u8, 211u8, 186u8, 191u8, 110u8, 251u8, 213u8, 238u8, 84u8, 112u8,
							15u8, 221u8, 105u8, 23u8, 35u8, 213u8, 4u8, 46u8, 39u8, 69u8, 185u8,
							88u8, 244u8, 135u8, 39u8, 229u8, 78u8, 123u8, 64u8, 142u8, 73u8, 251u8,
						],
					)
				}
				pub fn asset_metadata_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_traits::assets::BasicAssetMetadata,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CurrencyFactory",
						"AssetMetadata",
						Vec::new(),
						[
							75u8, 211u8, 186u8, 191u8, 110u8, 251u8, 213u8, 238u8, 84u8, 112u8,
							15u8, 221u8, 105u8, 23u8, 35u8, 213u8, 4u8, 46u8, 39u8, 69u8, 185u8,
							88u8, 244u8, 135u8, 39u8, 229u8, 78u8, 123u8, 64u8, 142u8, 73u8, 251u8,
						],
					)
				}
			}
		}
	}
	pub mod crowdloan_rewards {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_crowdloan_rewards::pallet::Error;
		pub type Call = runtime_types::pallet_crowdloan_rewards::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Initialize;
				impl ::subxt::blocks::StaticExtrinsic for Initialize {
					const PALLET: &'static str = "CrowdloanRewards";
					const CALL: &'static str = "initialize";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct InitializeAt {
					pub at: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for InitializeAt {
					const PALLET: &'static str = "CrowdloanRewards";
					const CALL: &'static str = "initialize_at";
				}
				pub struct Populate {
					pub rewards: ::std::vec::Vec<(
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							runtime_types::sp_core::crypto::AccountId32,
						>,
						::core::primitive::u128,
						::core::primitive::u64,
					)>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Populate {
					const PALLET: &'static str = "CrowdloanRewards";
					const CALL: &'static str = "populate";
				}
				pub struct Associate {
					pub reward_account: runtime_types::sp_core::crypto::AccountId32,
					pub proof: runtime_types::pallet_crowdloan_rewards::models::Proof<
						runtime_types::sp_core::crypto::AccountId32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Associate {
					const PALLET: &'static str = "CrowdloanRewards";
					const CALL: &'static str = "associate";
				}
				pub struct Claim;
				impl ::subxt::blocks::StaticExtrinsic for Claim {
					const PALLET: &'static str = "CrowdloanRewards";
					const CALL: &'static str = "claim";
				}
				pub struct UnlockRewardsFor {
					pub reward_accounts:
						::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for UnlockRewardsFor {
					const PALLET: &'static str = "CrowdloanRewards";
					const CALL: &'static str = "unlock_rewards_for";
				}
				pub struct Add {
					pub additions: ::std::vec::Vec<(
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							runtime_types::sp_core::crypto::AccountId32,
						>,
						::core::primitive::u128,
						::core::primitive::u64,
					)>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Add {
					const PALLET: &'static str = "CrowdloanRewards";
					const CALL: &'static str = "add";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn initialize(&self) -> ::subxt::tx::Payload<types::Initialize> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"initialize",
						types::Initialize {},
						[
							189u8, 34u8, 252u8, 48u8, 174u8, 108u8, 147u8, 165u8, 11u8, 115u8,
							31u8, 224u8, 186u8, 218u8, 34u8, 98u8, 5u8, 143u8, 91u8, 54u8, 142u8,
							160u8, 71u8, 28u8, 159u8, 241u8, 16u8, 134u8, 205u8, 98u8, 63u8, 131u8,
						],
					)
				}
				pub fn initialize_at(
					&self,
					at: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::InitializeAt> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"initialize_at",
						types::InitializeAt { at },
						[
							158u8, 95u8, 219u8, 224u8, 226u8, 63u8, 74u8, 161u8, 129u8, 81u8, 85u8,
							168u8, 117u8, 204u8, 137u8, 91u8, 117u8, 198u8, 81u8, 128u8, 98u8,
							161u8, 16u8, 181u8, 82u8, 230u8, 180u8, 101u8, 110u8, 78u8, 30u8, 65u8,
						],
					)
				}
				pub fn populate(
					&self,
					rewards: ::std::vec::Vec<(
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							runtime_types::sp_core::crypto::AccountId32,
						>,
						::core::primitive::u128,
						::core::primitive::u64,
					)>,
				) -> ::subxt::tx::Payload<types::Populate> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"populate",
						types::Populate { rewards },
						[
							8u8, 72u8, 162u8, 246u8, 145u8, 133u8, 238u8, 102u8, 73u8, 36u8, 156u8,
							20u8, 135u8, 25u8, 94u8, 90u8, 243u8, 19u8, 158u8, 103u8, 18u8, 146u8,
							182u8, 200u8, 76u8, 58u8, 92u8, 237u8, 222u8, 249u8, 189u8, 199u8,
						],
					)
				}
				pub fn associate(
					&self,
					reward_account: runtime_types::sp_core::crypto::AccountId32,
					proof: runtime_types::pallet_crowdloan_rewards::models::Proof<
						runtime_types::sp_core::crypto::AccountId32,
					>,
				) -> ::subxt::tx::Payload<types::Associate> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"associate",
						types::Associate { reward_account, proof },
						[
							76u8, 4u8, 128u8, 144u8, 78u8, 14u8, 31u8, 110u8, 165u8, 36u8, 140u8,
							81u8, 243u8, 15u8, 115u8, 107u8, 172u8, 8u8, 108u8, 21u8, 1u8, 85u8,
							36u8, 18u8, 106u8, 166u8, 85u8, 231u8, 178u8, 34u8, 181u8, 42u8,
						],
					)
				}
				pub fn claim(&self) -> ::subxt::tx::Payload<types::Claim> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"claim",
						types::Claim {},
						[
							190u8, 235u8, 250u8, 234u8, 67u8, 158u8, 3u8, 42u8, 214u8, 76u8, 33u8,
							240u8, 24u8, 129u8, 206u8, 143u8, 161u8, 126u8, 135u8, 66u8, 242u8,
							137u8, 95u8, 1u8, 242u8, 86u8, 156u8, 142u8, 121u8, 110u8, 178u8, 0u8,
						],
					)
				}
				pub fn unlock_rewards_for(
					&self,
					reward_accounts: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::Payload<types::UnlockRewardsFor> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"unlock_rewards_for",
						types::UnlockRewardsFor { reward_accounts },
						[
							97u8, 124u8, 229u8, 162u8, 82u8, 230u8, 144u8, 8u8, 83u8, 168u8, 219u8,
							151u8, 134u8, 43u8, 247u8, 244u8, 46u8, 129u8, 103u8, 51u8, 56u8,
							158u8, 181u8, 129u8, 29u8, 52u8, 16u8, 239u8, 48u8, 67u8, 75u8, 55u8,
						],
					)
				}
				pub fn add(
					&self,
					additions: ::std::vec::Vec<(
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							runtime_types::sp_core::crypto::AccountId32,
						>,
						::core::primitive::u128,
						::core::primitive::u64,
					)>,
				) -> ::subxt::tx::Payload<types::Add> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"add",
						types::Add { additions },
						[
							135u8, 199u8, 135u8, 198u8, 122u8, 179u8, 27u8, 154u8, 39u8, 196u8,
							23u8, 212u8, 165u8, 127u8, 88u8, 168u8, 26u8, 3u8, 248u8, 9u8, 26u8,
							190u8, 70u8, 145u8, 195u8, 240u8, 212u8, 212u8, 22u8, 25u8, 138u8,
							69u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_crowdloan_rewards::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct Initialized {
				pub at: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for Initialized {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "Initialized";
			}
			pub struct Claimed {
				pub remote_account: runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
					runtime_types::sp_core::crypto::AccountId32,
				>,
				pub reward_account: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Claimed {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "Claimed";
			}
			pub struct Associated {
				pub remote_account: runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
					runtime_types::sp_core::crypto::AccountId32,
				>,
				pub reward_account: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Associated {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "Associated";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct OverFunded {
				pub excess_funds: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for OverFunded {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "OverFunded";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct RewardsUnlocked {
				pub at: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for RewardsUnlocked {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "RewardsUnlocked";
			}
			pub struct RewardsAdded {
				pub additions: ::std::vec::Vec<(
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						runtime_types::sp_core::crypto::AccountId32,
					>,
					::core::primitive::u128,
					::core::primitive::u64,
				)>,
			}
			impl ::subxt::events::StaticEvent for RewardsAdded {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "RewardsAdded";
			}
			pub struct RewardsDeleted {
				pub deletions: ::std::vec::Vec<
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						runtime_types::sp_core::crypto::AccountId32,
					>,
				>,
			}
			impl ::subxt::events::StaticEvent for RewardsDeleted {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "RewardsDeleted";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn rewards(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							runtime_types::sp_core::crypto::AccountId32,
						>,
					>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_crowdloan_rewards::models::Reward<
						::core::primitive::u128,
						::core::primitive::u64,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CrowdloanRewards",
						"Rewards",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							150u8, 27u8, 5u8, 57u8, 168u8, 233u8, 144u8, 108u8, 181u8, 60u8, 73u8,
							48u8, 183u8, 64u8, 17u8, 243u8, 99u8, 31u8, 165u8, 104u8, 80u8, 100u8,
							21u8, 16u8, 7u8, 104u8, 162u8, 57u8, 180u8, 175u8, 92u8, 7u8,
						],
					)
				}
				pub fn rewards_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_crowdloan_rewards::models::Reward<
						::core::primitive::u128,
						::core::primitive::u64,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CrowdloanRewards",
						"Rewards",
						Vec::new(),
						[
							150u8, 27u8, 5u8, 57u8, 168u8, 233u8, 144u8, 108u8, 181u8, 60u8, 73u8,
							48u8, 183u8, 64u8, 17u8, 243u8, 99u8, 31u8, 165u8, 104u8, 80u8, 100u8,
							21u8, 16u8, 7u8, 104u8, 162u8, 57u8, 180u8, 175u8, 92u8, 7u8,
						],
					)
				}
				pub fn total_rewards(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CrowdloanRewards",
						"TotalRewards",
						vec![],
						[
							8u8, 98u8, 70u8, 113u8, 254u8, 107u8, 181u8, 27u8, 5u8, 142u8, 36u8,
							95u8, 3u8, 24u8, 58u8, 61u8, 45u8, 125u8, 69u8, 68u8, 230u8, 95u8,
							41u8, 131u8, 50u8, 242u8, 200u8, 58u8, 13u8, 189u8, 139u8, 137u8,
						],
					)
				}
				pub fn claimed_rewards(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CrowdloanRewards",
						"ClaimedRewards",
						vec![],
						[
							219u8, 227u8, 173u8, 110u8, 4u8, 133u8, 223u8, 85u8, 47u8, 199u8, 29u8,
							248u8, 226u8, 116u8, 82u8, 160u8, 22u8, 21u8, 137u8, 144u8, 255u8,
							131u8, 53u8, 84u8, 121u8, 115u8, 47u8, 91u8, 178u8, 195u8, 183u8, 6u8,
						],
					)
				}
				pub fn total_contributors(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CrowdloanRewards",
						"TotalContributors",
						vec![],
						[
							182u8, 128u8, 61u8, 169u8, 43u8, 194u8, 117u8, 112u8, 214u8, 202u8,
							73u8, 1u8, 2u8, 194u8, 212u8, 5u8, 70u8, 45u8, 50u8, 62u8, 97u8, 205u8,
							199u8, 200u8, 103u8, 181u8, 205u8, 6u8, 217u8, 109u8, 47u8, 227u8,
						],
					)
				}
				pub fn vesting_time_start(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CrowdloanRewards",
						"VestingTimeStart",
						vec![],
						[
							144u8, 64u8, 171u8, 124u8, 170u8, 28u8, 168u8, 148u8, 246u8, 71u8,
							79u8, 213u8, 61u8, 85u8, 137u8, 236u8, 87u8, 141u8, 127u8, 96u8, 54u8,
							117u8, 179u8, 103u8, 253u8, 242u8, 253u8, 47u8, 91u8, 149u8, 212u8,
							12u8,
						],
					)
				}
				pub fn associations(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						runtime_types::sp_core::crypto::AccountId32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CrowdloanRewards",
						"Associations",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							175u8, 140u8, 94u8, 121u8, 236u8, 217u8, 144u8, 206u8, 50u8, 25u8,
							41u8, 198u8, 226u8, 77u8, 67u8, 255u8, 10u8, 229u8, 190u8, 72u8, 216u8,
							28u8, 255u8, 24u8, 52u8, 174u8, 43u8, 146u8, 50u8, 110u8, 222u8, 131u8,
						],
					)
				}
				pub fn associations_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						runtime_types::sp_core::crypto::AccountId32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CrowdloanRewards",
						"Associations",
						Vec::new(),
						[
							175u8, 140u8, 94u8, 121u8, 236u8, 217u8, 144u8, 206u8, 50u8, 25u8,
							41u8, 198u8, 226u8, 77u8, 67u8, 255u8, 10u8, 229u8, 190u8, 72u8, 216u8,
							28u8, 255u8, 24u8, 52u8, 174u8, 43u8, 146u8, 50u8, 110u8, 222u8, 131u8,
						],
					)
				}
				pub fn remove_reward_locks(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(),
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CrowdloanRewards",
						"RemoveRewardLocks",
						vec![],
						[
							112u8, 97u8, 1u8, 2u8, 35u8, 35u8, 201u8, 82u8, 166u8, 23u8, 48u8,
							28u8, 204u8, 220u8, 47u8, 51u8, 128u8, 248u8, 50u8, 215u8, 46u8, 131u8,
							6u8, 132u8, 154u8, 174u8, 65u8, 84u8, 131u8, 55u8, 147u8, 189u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn initial_payment(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Perbill>
				{
					::subxt::constants::Address::new_static(
						"CrowdloanRewards",
						"InitialPayment",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
				pub fn over_funded_threshold(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Perbill>
				{
					::subxt::constants::Address::new_static(
						"CrowdloanRewards",
						"OverFundedThreshold",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
				pub fn vesting_step(&self) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"CrowdloanRewards",
						"VestingStep",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				pub fn prefix(
					&self,
				) -> ::subxt::constants::Address<::std::vec::Vec<::core::primitive::u8>> {
					::subxt::constants::Address::new_static(
						"CrowdloanRewards",
						"Prefix",
						[
							64u8, 190u8, 244u8, 122u8, 87u8, 182u8, 217u8, 16u8, 55u8, 223u8,
							128u8, 6u8, 112u8, 30u8, 236u8, 222u8, 153u8, 53u8, 247u8, 102u8,
							196u8, 31u8, 6u8, 186u8, 251u8, 209u8, 114u8, 125u8, 213u8, 222u8,
							240u8, 8u8,
						],
					)
				}
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"CrowdloanRewards",
						"PalletId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
				pub fn lock_id(
					&self,
				) -> ::subxt::constants::Address<[::core::primitive::u8; 8usize]> {
					::subxt::constants::Address::new_static(
						"CrowdloanRewards",
						"LockId",
						[
							157u8, 118u8, 79u8, 88u8, 241u8, 22u8, 185u8, 37u8, 42u8, 20u8, 133u8,
							240u8, 11u8, 25u8, 66u8, 154u8, 84u8, 163u8, 78u8, 92u8, 171u8, 82u8,
							248u8, 76u8, 189u8, 70u8, 142u8, 249u8, 153u8, 84u8, 180u8, 60u8,
						],
					)
				}
				pub fn lock_by_default(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::bool> {
					::subxt::constants::Address::new_static(
						"CrowdloanRewards",
						"LockByDefault",
						[
							165u8, 28u8, 112u8, 190u8, 18u8, 129u8, 182u8, 206u8, 237u8, 1u8, 68u8,
							252u8, 125u8, 234u8, 185u8, 50u8, 149u8, 164u8, 47u8, 126u8, 134u8,
							100u8, 14u8, 86u8, 209u8, 39u8, 20u8, 4u8, 233u8, 115u8, 102u8, 131u8,
						],
					)
				}
				pub fn account_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_core::crypto::AccountId32> {
					::subxt::constants::Address::new_static(
						"CrowdloanRewards",
						"account_id",
						[
							115u8, 233u8, 13u8, 223u8, 88u8, 20u8, 202u8, 139u8, 153u8, 28u8,
							155u8, 157u8, 224u8, 66u8, 3u8, 250u8, 23u8, 53u8, 88u8, 168u8, 211u8,
							204u8, 122u8, 166u8, 248u8, 23u8, 174u8, 225u8, 99u8, 108u8, 89u8,
							135u8,
						],
					)
				}
			}
		}
	}
	pub mod vesting {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_vesting::module::Error;
		pub type Call = runtime_types::pallet_vesting::module::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Claim {
					pub asset: runtime_types::primitives::currency::CurrencyId,
					pub vesting_schedule_ids:
						runtime_types::pallet_vesting::types::VestingScheduleIdSet<
							::core::primitive::u128,
						>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Claim {
					const PALLET: &'static str = "Vesting";
					const CALL: &'static str = "claim";
				}
				pub struct VestedTransfer {
					pub from: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub beneficiary: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub asset: runtime_types::primitives::currency::CurrencyId,
					pub schedule_info: runtime_types::pallet_vesting::types::VestingScheduleInfo<
						::core::primitive::u32,
						::core::primitive::u64,
						::core::primitive::u128,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for VestedTransfer {
					const PALLET: &'static str = "Vesting";
					const CALL: &'static str = "vested_transfer";
				}
				pub struct UpdateVestingSchedules {
					pub who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub asset: runtime_types::primitives::currency::CurrencyId,
					pub vesting_schedules: ::std::vec::Vec<
						runtime_types::pallet_vesting::types::VestingScheduleInfo<
							::core::primitive::u32,
							::core::primitive::u64,
							::core::primitive::u128,
						>,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateVestingSchedules {
					const PALLET: &'static str = "Vesting";
					const CALL: &'static str = "update_vesting_schedules";
				}
				pub struct ClaimFor {
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub asset: runtime_types::primitives::currency::CurrencyId,
					pub vesting_schedule_ids:
						runtime_types::pallet_vesting::types::VestingScheduleIdSet<
							::core::primitive::u128,
						>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ClaimFor {
					const PALLET: &'static str = "Vesting";
					const CALL: &'static str = "claim_for";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn claim(
					&self,
					asset: runtime_types::primitives::currency::CurrencyId,
					vesting_schedule_ids : runtime_types :: pallet_vesting :: types :: VestingScheduleIdSet < :: core :: primitive :: u128 >,
				) -> ::subxt::tx::Payload<types::Claim> {
					::subxt::tx::Payload::new_static(
						"Vesting",
						"claim",
						types::Claim { asset, vesting_schedule_ids },
						[
							131u8, 36u8, 111u8, 43u8, 127u8, 174u8, 179u8, 246u8, 98u8, 91u8,
							216u8, 232u8, 17u8, 183u8, 167u8, 235u8, 237u8, 116u8, 179u8, 103u8,
							61u8, 10u8, 22u8, 41u8, 66u8, 37u8, 166u8, 200u8, 16u8, 6u8, 202u8,
							54u8,
						],
					)
				}
				pub fn vested_transfer(
					&self,
					from: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					beneficiary: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					asset: runtime_types::primitives::currency::CurrencyId,
					schedule_info: runtime_types::pallet_vesting::types::VestingScheduleInfo<
						::core::primitive::u32,
						::core::primitive::u64,
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::Payload<types::VestedTransfer> {
					::subxt::tx::Payload::new_static(
						"Vesting",
						"vested_transfer",
						types::VestedTransfer { from, beneficiary, asset, schedule_info },
						[
							56u8, 38u8, 222u8, 90u8, 106u8, 138u8, 172u8, 78u8, 177u8, 128u8,
							202u8, 151u8, 31u8, 100u8, 14u8, 192u8, 157u8, 145u8, 151u8, 20u8,
							71u8, 107u8, 69u8, 64u8, 46u8, 191u8, 54u8, 245u8, 59u8, 175u8, 50u8,
							76u8,
						],
					)
				}
				pub fn update_vesting_schedules(
					&self,
					who: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					asset: runtime_types::primitives::currency::CurrencyId,
					vesting_schedules: ::std::vec::Vec<
						runtime_types::pallet_vesting::types::VestingScheduleInfo<
							::core::primitive::u32,
							::core::primitive::u64,
							::core::primitive::u128,
						>,
					>,
				) -> ::subxt::tx::Payload<types::UpdateVestingSchedules> {
					::subxt::tx::Payload::new_static(
						"Vesting",
						"update_vesting_schedules",
						types::UpdateVestingSchedules { who, asset, vesting_schedules },
						[
							159u8, 10u8, 126u8, 190u8, 205u8, 37u8, 145u8, 20u8, 97u8, 186u8,
							122u8, 130u8, 184u8, 186u8, 145u8, 27u8, 78u8, 91u8, 200u8, 205u8,
							10u8, 202u8, 86u8, 145u8, 236u8, 213u8, 209u8, 179u8, 6u8, 174u8,
							165u8, 28u8,
						],
					)
				}
				pub fn claim_for(
					&self,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					asset: runtime_types::primitives::currency::CurrencyId,
					vesting_schedule_ids : runtime_types :: pallet_vesting :: types :: VestingScheduleIdSet < :: core :: primitive :: u128 >,
				) -> ::subxt::tx::Payload<types::ClaimFor> {
					::subxt::tx::Payload::new_static(
						"Vesting",
						"claim_for",
						types::ClaimFor { dest, asset, vesting_schedule_ids },
						[
							58u8, 66u8, 85u8, 212u8, 51u8, 216u8, 21u8, 133u8, 95u8, 236u8, 128u8,
							121u8, 8u8, 101u8, 253u8, 249u8, 130u8, 105u8, 59u8, 90u8, 240u8,
							220u8, 134u8, 231u8, 182u8, 89u8, 75u8, 223u8, 240u8, 213u8, 38u8,
							231u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_vesting::module::Event;
		pub mod events {
			use super::runtime_types;
			pub struct VestingScheduleAdded {
				pub from: runtime_types::sp_core::crypto::AccountId32,
				pub to: runtime_types::sp_core::crypto::AccountId32,
				pub asset: runtime_types::primitives::currency::CurrencyId,
				pub vesting_schedule_id: ::core::primitive::u128,
				pub schedule: runtime_types::pallet_vesting::types::VestingSchedule<
					::core::primitive::u128,
					::core::primitive::u32,
					::core::primitive::u64,
					::core::primitive::u128,
				>,
				pub schedule_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for VestingScheduleAdded {
				const PALLET: &'static str = "Vesting";
				const EVENT: &'static str = "VestingScheduleAdded";
			}
			pub struct Claimed {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub asset: runtime_types::primitives::currency::CurrencyId,
				pub vesting_schedule_ids:
					runtime_types::pallet_vesting::types::VestingScheduleIdSet<
						::core::primitive::u128,
					>,
				pub locked_amount: ::core::primitive::u128,
				pub claimed_amount_per_schedule:
					runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
						::core::primitive::u128,
						::core::primitive::u128,
					>,
			}
			impl ::subxt::events::StaticEvent for Claimed {
				const PALLET: &'static str = "Vesting";
				const EVENT: &'static str = "Claimed";
			}
			pub struct VestingSchedulesUpdated {
				pub who: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for VestingSchedulesUpdated {
				const PALLET: &'static str = "Vesting";
				const EVENT: &'static str = "VestingSchedulesUpdated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn vesting_schedules(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
					_1: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
						::core::primitive::u128,
						runtime_types::pallet_vesting::types::VestingSchedule<
							::core::primitive::u128,
							::core::primitive::u32,
							::core::primitive::u64,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Vesting",
						"VestingSchedules",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							6u8, 188u8, 204u8, 251u8, 186u8, 136u8, 148u8, 86u8, 73u8, 227u8,
							209u8, 122u8, 37u8, 101u8, 133u8, 77u8, 131u8, 236u8, 119u8, 22u8,
							12u8, 203u8, 234u8, 63u8, 145u8, 110u8, 13u8, 65u8, 50u8, 50u8, 86u8,
							54u8,
						],
					)
				}
				pub fn vesting_schedules_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
						::core::primitive::u128,
						runtime_types::pallet_vesting::types::VestingSchedule<
							::core::primitive::u128,
							::core::primitive::u32,
							::core::primitive::u64,
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Vesting",
						"VestingSchedules",
						Vec::new(),
						[
							6u8, 188u8, 204u8, 251u8, 186u8, 136u8, 148u8, 86u8, 73u8, 227u8,
							209u8, 122u8, 37u8, 101u8, 133u8, 77u8, 131u8, 236u8, 119u8, 22u8,
							12u8, 203u8, 234u8, 63u8, 145u8, 110u8, 13u8, 65u8, 50u8, 50u8, 86u8,
							54u8,
						],
					)
				}
				pub fn vesting_schedule_nonce(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Vesting",
						"VestingScheduleNonce",
						vec![],
						[
							89u8, 249u8, 104u8, 67u8, 103u8, 176u8, 244u8, 101u8, 208u8, 10u8,
							111u8, 230u8, 192u8, 81u8, 30u8, 5u8, 136u8, 120u8, 216u8, 206u8,
							117u8, 216u8, 33u8, 30u8, 249u8, 77u8, 172u8, 27u8, 36u8, 1u8, 170u8,
							245u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn min_vested_transfer(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Vesting",
						"MinVestedTransfer",
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
	pub mod bonded_finance {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_bonded_finance::pallet::Error;
		pub type Call = runtime_types::pallet_bonded_finance::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Offer {
					pub offer: runtime_types::composable_traits::bonded_finance::BondOffer<
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
						::core::primitive::u32,
					>,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for Offer {
					const PALLET: &'static str = "BondedFinance";
					const CALL: &'static str = "offer";
				}
				pub struct Bond {
					pub offer_id: ::core::primitive::u128,
					pub nb_of_bonds: ::core::primitive::u128,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for Bond {
					const PALLET: &'static str = "BondedFinance";
					const CALL: &'static str = "bond";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct Cancel {
					pub offer_id: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Cancel {
					const PALLET: &'static str = "BondedFinance";
					const CALL: &'static str = "cancel";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn offer(
					&self,
					offer: runtime_types::composable_traits::bonded_finance::BondOffer<
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::Offer> {
					::subxt::tx::Payload::new_static(
						"BondedFinance",
						"offer",
						types::Offer { offer, keep_alive },
						[
							223u8, 156u8, 179u8, 146u8, 78u8, 60u8, 157u8, 133u8, 143u8, 47u8,
							53u8, 172u8, 54u8, 247u8, 124u8, 201u8, 173u8, 218u8, 173u8, 61u8,
							21u8, 195u8, 111u8, 212u8, 166u8, 215u8, 130u8, 117u8, 73u8, 214u8,
							99u8, 177u8,
						],
					)
				}
				pub fn bond(
					&self,
					offer_id: ::core::primitive::u128,
					nb_of_bonds: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::Bond> {
					::subxt::tx::Payload::new_static(
						"BondedFinance",
						"bond",
						types::Bond { offer_id, nb_of_bonds, keep_alive },
						[
							93u8, 8u8, 35u8, 102u8, 143u8, 7u8, 45u8, 137u8, 70u8, 240u8, 92u8,
							92u8, 42u8, 38u8, 7u8, 124u8, 58u8, 114u8, 51u8, 178u8, 242u8, 128u8,
							135u8, 25u8, 92u8, 176u8, 124u8, 234u8, 37u8, 83u8, 103u8, 243u8,
						],
					)
				}
				pub fn cancel(
					&self,
					offer_id: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Cancel> {
					::subxt::tx::Payload::new_static(
						"BondedFinance",
						"cancel",
						types::Cancel { offer_id },
						[
							160u8, 11u8, 20u8, 212u8, 150u8, 133u8, 116u8, 181u8, 114u8, 69u8,
							216u8, 72u8, 215u8, 172u8, 192u8, 203u8, 147u8, 17u8, 173u8, 179u8,
							43u8, 231u8, 36u8, 150u8, 66u8, 76u8, 167u8, 24u8, 207u8, 89u8, 27u8,
							214u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_bonded_finance::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct NewOffer {
				pub offer_id: ::core::primitive::u128,
				pub beneficiary: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for NewOffer {
				const PALLET: &'static str = "BondedFinance";
				const EVENT: &'static str = "NewOffer";
			}
			pub struct NewBond {
				pub offer_id: ::core::primitive::u128,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub nb_of_bonds: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for NewBond {
				const PALLET: &'static str = "BondedFinance";
				const EVENT: &'static str = "NewBond";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct OfferCancelled {
				pub offer_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for OfferCancelled {
				const PALLET: &'static str = "BondedFinance";
				const EVENT: &'static str = "OfferCancelled";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct OfferCompleted {
				pub offer_id: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for OfferCompleted {
				const PALLET: &'static str = "BondedFinance";
				const EVENT: &'static str = "OfferCompleted";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn bond_offer_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"BondedFinance",
						"BondOfferCount",
						vec![],
						[
							127u8, 98u8, 53u8, 139u8, 188u8, 57u8, 239u8, 108u8, 137u8, 58u8,
							208u8, 248u8, 103u8, 252u8, 217u8, 64u8, 165u8, 173u8, 230u8, 171u8,
							179u8, 128u8, 171u8, 244u8, 138u8, 252u8, 206u8, 94u8, 210u8, 1u8,
							83u8, 51u8,
						],
					)
				}
				pub fn bond_offers(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::composable_traits::bonded_finance::BondOffer<
							runtime_types::sp_core::crypto::AccountId32,
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
							::core::primitive::u32,
						>,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"BondedFinance",
						"BondOffers",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							231u8, 140u8, 118u8, 37u8, 4u8, 4u8, 97u8, 245u8, 137u8, 165u8, 83u8,
							233u8, 171u8, 235u8, 219u8, 185u8, 112u8, 52u8, 38u8, 19u8, 182u8,
							206u8, 69u8, 39u8, 53u8, 92u8, 233u8, 55u8, 30u8, 34u8, 23u8, 40u8,
						],
					)
				}
				pub fn bond_offers_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::composable_traits::bonded_finance::BondOffer<
							runtime_types::sp_core::crypto::AccountId32,
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
							::core::primitive::u32,
						>,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"BondedFinance",
						"BondOffers",
						Vec::new(),
						[
							231u8, 140u8, 118u8, 37u8, 4u8, 4u8, 97u8, 245u8, 137u8, 165u8, 83u8,
							233u8, 171u8, 235u8, 219u8, 185u8, 112u8, 52u8, 38u8, 19u8, 182u8,
							206u8, 69u8, 39u8, 53u8, 92u8, 233u8, 55u8, 30u8, 34u8, 23u8, 40u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"BondedFinance",
						"PalletId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
				pub fn stake(&self) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"BondedFinance",
						"Stake",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				pub fn min_reward(&self) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"BondedFinance",
						"MinReward",
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
	pub mod assets_registry {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_assets_registry::pallet::Error;
		pub type Call = runtime_types::pallet_assets_registry::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct RegisterAsset {
					pub protocol_id: [::core::primitive::u8; 4usize],
					pub nonce: ::core::primitive::u64,
					pub location:
						::core::option::Option<runtime_types::primitives::currency::ForeignAssetId>,
					pub asset_info: runtime_types::composable_traits::assets::AssetInfo<
						::core::primitive::u128,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RegisterAsset {
					const PALLET: &'static str = "AssetsRegistry";
					const CALL: &'static str = "register_asset";
				}
				pub struct UpdateAsset {
					pub asset_id: runtime_types::primitives::currency::CurrencyId,
					pub asset_info: runtime_types::composable_traits::assets::AssetInfoUpdate<
						::core::primitive::u128,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateAsset {
					const PALLET: &'static str = "AssetsRegistry";
					const CALL: &'static str = "update_asset";
				}
				pub struct SetMinFee {
					pub target_parachain_id: ::core::primitive::u32,
					pub foreign_asset_id: runtime_types::primitives::currency::ForeignAssetId,
					pub amount: ::core::option::Option<::core::primitive::u128>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMinFee {
					const PALLET: &'static str = "AssetsRegistry";
					const CALL: &'static str = "set_min_fee";
				}
				pub struct UpdateAssetLocation {
					pub asset_id: runtime_types::primitives::currency::CurrencyId,
					pub location:
						::core::option::Option<runtime_types::primitives::currency::ForeignAssetId>,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateAssetLocation {
					const PALLET: &'static str = "AssetsRegistry";
					const CALL: &'static str = "update_asset_location";
				}
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
				) -> ::subxt::tx::Payload<types::RegisterAsset> {
					::subxt::tx::Payload::new_static(
						"AssetsRegistry",
						"register_asset",
						types::RegisterAsset { protocol_id, nonce, location, asset_info },
						[
							223u8, 245u8, 195u8, 109u8, 59u8, 115u8, 171u8, 90u8, 106u8, 6u8,
							208u8, 226u8, 159u8, 216u8, 120u8, 140u8, 27u8, 225u8, 168u8, 127u8,
							73u8, 184u8, 87u8, 235u8, 109u8, 98u8, 61u8, 148u8, 107u8, 92u8, 60u8,
							52u8,
						],
					)
				}
				pub fn update_asset(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					asset_info: runtime_types::composable_traits::assets::AssetInfoUpdate<
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::Payload<types::UpdateAsset> {
					::subxt::tx::Payload::new_static(
						"AssetsRegistry",
						"update_asset",
						types::UpdateAsset { asset_id, asset_info },
						[
							119u8, 129u8, 188u8, 154u8, 49u8, 221u8, 121u8, 222u8, 227u8, 236u8,
							238u8, 114u8, 241u8, 16u8, 13u8, 116u8, 186u8, 9u8, 171u8, 8u8, 31u8,
							26u8, 136u8, 151u8, 166u8, 171u8, 13u8, 12u8, 7u8, 71u8, 167u8, 244u8,
						],
					)
				}
				pub fn set_min_fee(
					&self,
					target_parachain_id: ::core::primitive::u32,
					foreign_asset_id: runtime_types::primitives::currency::ForeignAssetId,
					amount: ::core::option::Option<::core::primitive::u128>,
				) -> ::subxt::tx::Payload<types::SetMinFee> {
					::subxt::tx::Payload::new_static(
						"AssetsRegistry",
						"set_min_fee",
						types::SetMinFee { target_parachain_id, foreign_asset_id, amount },
						[
							24u8, 248u8, 48u8, 136u8, 76u8, 75u8, 90u8, 99u8, 200u8, 124u8, 190u8,
							39u8, 214u8, 35u8, 72u8, 191u8, 153u8, 169u8, 49u8, 164u8, 3u8, 116u8,
							209u8, 243u8, 211u8, 217u8, 47u8, 110u8, 228u8, 253u8, 245u8, 194u8,
						],
					)
				}
				pub fn update_asset_location(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					location: ::core::option::Option<
						runtime_types::primitives::currency::ForeignAssetId,
					>,
				) -> ::subxt::tx::Payload<types::UpdateAssetLocation> {
					::subxt::tx::Payload::new_static(
						"AssetsRegistry",
						"update_asset_location",
						types::UpdateAssetLocation { asset_id, location },
						[
							43u8, 7u8, 23u8, 153u8, 40u8, 25u8, 238u8, 58u8, 87u8, 204u8, 191u8,
							86u8, 104u8, 227u8, 188u8, 127u8, 213u8, 163u8, 75u8, 218u8, 60u8,
							174u8, 51u8, 80u8, 38u8, 107u8, 90u8, 12u8, 193u8, 66u8, 113u8, 241u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_assets_registry::pallet::Event;
		pub mod events {
			use super::runtime_types;
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
			pub struct AssetLocationUpdated {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub location: runtime_types::primitives::currency::ForeignAssetId,
			}
			impl ::subxt::events::StaticEvent for AssetLocationUpdated {
				const PALLET: &'static str = "AssetsRegistry";
				const EVENT: &'static str = "AssetLocationUpdated";
			}
			pub struct AssetLocationRemoved {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			impl ::subxt::events::StaticEvent for AssetLocationRemoved {
				const PALLET: &'static str = "AssetsRegistry";
				const EVENT: &'static str = "AssetLocationRemoved";
			}
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
							255u8, 55u8, 181u8, 82u8, 206u8, 165u8, 209u8, 196u8, 157u8, 99u8,
							214u8, 194u8, 76u8, 62u8, 151u8, 44u8, 126u8, 197u8, 176u8, 59u8, 27u8,
							105u8, 40u8, 147u8, 185u8, 122u8, 35u8, 41u8, 40u8, 177u8, 5u8, 73u8,
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
							255u8, 55u8, 181u8, 82u8, 206u8, 165u8, 209u8, 196u8, 157u8, 99u8,
							214u8, 194u8, 76u8, 62u8, 151u8, 44u8, 126u8, 197u8, 176u8, 59u8, 27u8,
							105u8, 40u8, 147u8, 185u8, 122u8, 35u8, 41u8, 40u8, 177u8, 5u8, 73u8,
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
							13u8, 117u8, 167u8, 112u8, 37u8, 125u8, 245u8, 200u8, 50u8, 89u8,
							175u8, 0u8, 183u8, 223u8, 32u8, 109u8, 16u8, 174u8, 156u8, 210u8, 43u8,
							177u8, 79u8, 20u8, 10u8, 70u8, 148u8, 100u8, 127u8, 54u8, 134u8, 75u8,
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
							13u8, 117u8, 167u8, 112u8, 37u8, 125u8, 245u8, 200u8, 50u8, 89u8,
							175u8, 0u8, 183u8, 223u8, 32u8, 109u8, 16u8, 174u8, 156u8, 210u8, 43u8,
							177u8, 79u8, 20u8, 10u8, 70u8, 148u8, 100u8, 127u8, 54u8, 134u8, 75u8,
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
							54u8, 156u8, 61u8, 65u8, 197u8, 46u8, 185u8, 54u8, 52u8, 2u8, 4u8,
							94u8, 64u8, 114u8, 78u8, 189u8, 144u8, 236u8, 189u8, 149u8, 33u8,
							100u8, 129u8, 44u8, 120u8, 16u8, 241u8, 196u8, 59u8, 226u8, 37u8, 92u8,
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
							54u8, 156u8, 61u8, 65u8, 197u8, 46u8, 185u8, 54u8, 52u8, 2u8, 4u8,
							94u8, 64u8, 114u8, 78u8, 189u8, 144u8, 236u8, 189u8, 149u8, 33u8,
							100u8, 129u8, 44u8, 120u8, 16u8, 241u8, 196u8, 59u8, 226u8, 37u8, 92u8,
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
							189u8, 179u8, 117u8, 254u8, 184u8, 121u8, 42u8, 61u8, 182u8, 152u8,
							190u8, 110u8, 25u8, 192u8, 23u8, 200u8, 113u8, 132u8, 112u8, 227u8,
							100u8, 58u8, 162u8, 106u8, 18u8, 241u8, 34u8, 125u8, 67u8, 200u8, 5u8,
							12u8,
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
							189u8, 179u8, 117u8, 254u8, 184u8, 121u8, 42u8, 61u8, 182u8, 152u8,
							190u8, 110u8, 25u8, 192u8, 23u8, 200u8, 113u8, 132u8, 112u8, 227u8,
							100u8, 58u8, 162u8, 106u8, 18u8, 241u8, 34u8, 125u8, 67u8, 200u8, 5u8,
							12u8,
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
							94u8, 2u8, 113u8, 66u8, 13u8, 196u8, 23u8, 139u8, 225u8, 186u8, 126u8,
							171u8, 85u8, 131u8, 65u8, 222u8, 113u8, 168u8, 248u8, 38u8, 37u8,
							158u8, 191u8, 98u8, 71u8, 32u8, 109u8, 205u8, 198u8, 240u8, 118u8,
							224u8,
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
							94u8, 2u8, 113u8, 66u8, 13u8, 196u8, 23u8, 139u8, 225u8, 186u8, 126u8,
							171u8, 85u8, 131u8, 65u8, 222u8, 113u8, 168u8, 248u8, 38u8, 37u8,
							158u8, 191u8, 98u8, 71u8, 32u8, 109u8, 205u8, 198u8, 240u8, 118u8,
							224u8,
						],
					)
				}				pub fn asset_name (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: primitives :: currency :: CurrencyId > ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetName",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							95u8, 88u8, 136u8, 170u8, 255u8, 243u8, 237u8, 186u8, 161u8, 95u8,
							187u8, 138u8, 249u8, 255u8, 190u8, 106u8, 179u8, 97u8, 124u8, 46u8,
							150u8, 171u8, 218u8, 92u8, 156u8, 34u8, 224u8, 36u8, 38u8, 223u8, 87u8,
							131u8,
						],
					)
				}				pub fn asset_name_root (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetName",
						Vec::new(),
						[
							95u8, 88u8, 136u8, 170u8, 255u8, 243u8, 237u8, 186u8, 161u8, 95u8,
							187u8, 138u8, 249u8, 255u8, 190u8, 106u8, 179u8, 97u8, 124u8, 46u8,
							150u8, 171u8, 218u8, 92u8, 156u8, 34u8, 224u8, 36u8, 38u8, 223u8, 87u8,
							131u8,
						],
					)
				}				pub fn asset_symbol (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: primitives :: currency :: CurrencyId > ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetSymbol",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							165u8, 55u8, 83u8, 22u8, 52u8, 121u8, 209u8, 233u8, 41u8, 204u8, 195u8,
							110u8, 255u8, 236u8, 21u8, 247u8, 159u8, 239u8, 79u8, 240u8, 178u8,
							144u8, 175u8, 26u8, 109u8, 235u8, 239u8, 15u8, 44u8, 137u8, 35u8,
							121u8,
						],
					)
				}				pub fn asset_symbol_root (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::Address::new_static(
						"AssetsRegistry",
						"AssetSymbol",
						Vec::new(),
						[
							165u8, 55u8, 83u8, 22u8, 52u8, 121u8, 209u8, 233u8, 41u8, 204u8, 195u8,
							110u8, 255u8, 236u8, 21u8, 247u8, 159u8, 239u8, 79u8, 240u8, 178u8,
							144u8, 175u8, 26u8, 109u8, 235u8, 239u8, 15u8, 44u8, 137u8, 35u8,
							121u8,
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
							147u8, 209u8, 44u8, 78u8, 198u8, 145u8, 143u8, 65u8, 136u8, 127u8,
							54u8, 200u8, 84u8, 240u8, 179u8, 138u8, 86u8, 230u8, 218u8, 225u8,
							182u8, 100u8, 26u8, 226u8, 48u8, 65u8, 61u8, 102u8, 127u8, 68u8, 11u8,
							249u8,
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
							147u8, 209u8, 44u8, 78u8, 198u8, 145u8, 143u8, 65u8, 136u8, 127u8,
							54u8, 200u8, 84u8, 240u8, 179u8, 138u8, 86u8, 230u8, 218u8, 225u8,
							182u8, 100u8, 26u8, 226u8, 48u8, 65u8, 61u8, 102u8, 127u8, 68u8, 11u8,
							249u8,
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
	pub mod pablo {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_pablo::pallet::Error;
		pub type Call = runtime_types::pallet_pablo::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Create {
					pub pool: runtime_types::pallet_pablo::pallet::PoolInitConfiguration<
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Create {
					const PALLET: &'static str = "Pablo";
					const CALL: &'static str = "create";
				}
				pub struct Buy {
					pub pool_id: ::core::primitive::u128,
					pub in_asset_id: runtime_types::primitives::currency::CurrencyId,
					pub out_asset: runtime_types::composable_traits::dex::AssetAmount<
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					>,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for Buy {
					const PALLET: &'static str = "Pablo";
					const CALL: &'static str = "buy";
				}
				pub struct Swap {
					pub pool_id: ::core::primitive::u128,
					pub in_asset: runtime_types::composable_traits::dex::AssetAmount<
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					>,
					pub min_receive: runtime_types::composable_traits::dex::AssetAmount<
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					>,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for Swap {
					const PALLET: &'static str = "Pablo";
					const CALL: &'static str = "swap";
				}
				pub struct AddLiquidity {
					pub pool_id: ::core::primitive::u128,
					pub assets: ::std::collections::BTreeMap<
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					>,
					pub min_mint_amount: ::core::primitive::u128,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddLiquidity {
					const PALLET: &'static str = "Pablo";
					const CALL: &'static str = "add_liquidity";
				}
				pub struct RemoveLiquidity {
					pub pool_id: ::core::primitive::u128,
					pub lp_amount: ::core::primitive::u128,
					pub min_receive: ::std::collections::BTreeMap<
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveLiquidity {
					const PALLET: &'static str = "Pablo";
					const CALL: &'static str = "remove_liquidity";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct EnableTwap {
					pub pool_id: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for EnableTwap {
					const PALLET: &'static str = "Pablo";
					const CALL: &'static str = "enable_twap";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn create(
					&self,
					pool: runtime_types::pallet_pablo::pallet::PoolInitConfiguration<
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
					>,
				) -> ::subxt::tx::Payload<types::Create> {
					::subxt::tx::Payload::new_static(
						"Pablo",
						"create",
						types::Create { pool },
						[
							45u8, 103u8, 128u8, 208u8, 34u8, 193u8, 52u8, 170u8, 34u8, 151u8,
							114u8, 87u8, 13u8, 223u8, 197u8, 17u8, 176u8, 93u8, 51u8, 16u8, 219u8,
							124u8, 18u8, 195u8, 186u8, 84u8, 148u8, 18u8, 84u8, 157u8, 203u8, 57u8,
						],
					)
				}
				pub fn buy(
					&self,
					pool_id: ::core::primitive::u128,
					in_asset_id: runtime_types::primitives::currency::CurrencyId,
					out_asset: runtime_types::composable_traits::dex::AssetAmount<
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::Buy> {
					::subxt::tx::Payload::new_static(
						"Pablo",
						"buy",
						types::Buy { pool_id, in_asset_id, out_asset, keep_alive },
						[
							75u8, 64u8, 232u8, 144u8, 243u8, 105u8, 163u8, 88u8, 255u8, 0u8, 13u8,
							86u8, 243u8, 177u8, 162u8, 161u8, 238u8, 91u8, 67u8, 185u8, 138u8,
							190u8, 172u8, 87u8, 162u8, 191u8, 20u8, 238u8, 144u8, 158u8, 248u8,
							5u8,
						],
					)
				}
				pub fn swap(
					&self,
					pool_id: ::core::primitive::u128,
					in_asset: runtime_types::composable_traits::dex::AssetAmount<
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					>,
					min_receive: runtime_types::composable_traits::dex::AssetAmount<
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::Swap> {
					::subxt::tx::Payload::new_static(
						"Pablo",
						"swap",
						types::Swap { pool_id, in_asset, min_receive, keep_alive },
						[
							104u8, 144u8, 213u8, 29u8, 12u8, 38u8, 255u8, 99u8, 239u8, 155u8, 4u8,
							131u8, 60u8, 169u8, 58u8, 245u8, 38u8, 60u8, 236u8, 164u8, 69u8, 176u8,
							49u8, 79u8, 24u8, 208u8, 93u8, 154u8, 40u8, 78u8, 113u8, 62u8,
						],
					)
				}
				pub fn add_liquidity(
					&self,
					pool_id: ::core::primitive::u128,
					assets: ::std::collections::BTreeMap<
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					>,
					min_mint_amount: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::AddLiquidity> {
					::subxt::tx::Payload::new_static(
						"Pablo",
						"add_liquidity",
						types::AddLiquidity { pool_id, assets, min_mint_amount, keep_alive },
						[
							63u8, 255u8, 206u8, 144u8, 244u8, 220u8, 16u8, 121u8, 51u8, 219u8,
							252u8, 92u8, 209u8, 7u8, 139u8, 148u8, 245u8, 191u8, 130u8, 102u8,
							237u8, 195u8, 101u8, 50u8, 22u8, 6u8, 40u8, 220u8, 43u8, 210u8, 224u8,
							31u8,
						],
					)
				}
				pub fn remove_liquidity(
					&self,
					pool_id: ::core::primitive::u128,
					lp_amount: ::core::primitive::u128,
					min_receive: ::std::collections::BTreeMap<
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::Payload<types::RemoveLiquidity> {
					::subxt::tx::Payload::new_static(
						"Pablo",
						"remove_liquidity",
						types::RemoveLiquidity { pool_id, lp_amount, min_receive },
						[
							242u8, 132u8, 161u8, 151u8, 187u8, 182u8, 125u8, 243u8, 250u8, 0u8,
							56u8, 220u8, 42u8, 178u8, 170u8, 209u8, 209u8, 27u8, 72u8, 210u8, 41u8,
							182u8, 42u8, 96u8, 226u8, 199u8, 109u8, 22u8, 13u8, 31u8, 249u8, 11u8,
						],
					)
				}
				pub fn enable_twap(
					&self,
					pool_id: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::EnableTwap> {
					::subxt::tx::Payload::new_static(
						"Pablo",
						"enable_twap",
						types::EnableTwap { pool_id },
						[
							189u8, 45u8, 59u8, 4u8, 246u8, 154u8, 29u8, 101u8, 49u8, 140u8, 89u8,
							166u8, 122u8, 155u8, 76u8, 77u8, 216u8, 104u8, 167u8, 2u8, 149u8,
							146u8, 13u8, 208u8, 220u8, 243u8, 153u8, 37u8, 44u8, 43u8, 228u8, 17u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_pablo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct PoolCreated {
				pub pool_id: ::core::primitive::u128,
				pub owner: runtime_types::sp_core::crypto::AccountId32,
				pub asset_weights: ::std::collections::BTreeMap<
					runtime_types::primitives::currency::CurrencyId,
					runtime_types::sp_arithmetic::per_things::Permill,
				>,
				pub lp_token_id: runtime_types::primitives::currency::CurrencyId,
			}
			impl ::subxt::events::StaticEvent for PoolCreated {
				const PALLET: &'static str = "Pablo";
				const EVENT: &'static str = "PoolCreated";
			}
			pub struct LiquidityAdded {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub pool_id: ::core::primitive::u128,
				pub asset_amounts: ::std::collections::BTreeMap<
					runtime_types::primitives::currency::CurrencyId,
					::core::primitive::u128,
				>,
				pub minted_lp: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for LiquidityAdded {
				const PALLET: &'static str = "Pablo";
				const EVENT: &'static str = "LiquidityAdded";
			}
			pub struct LiquidityRemoved {
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub pool_id: ::core::primitive::u128,
				pub asset_amounts: ::std::collections::BTreeMap<
					runtime_types::primitives::currency::CurrencyId,
					::core::primitive::u128,
				>,
			}
			impl ::subxt::events::StaticEvent for LiquidityRemoved {
				const PALLET: &'static str = "Pablo";
				const EVENT: &'static str = "LiquidityRemoved";
			}
			pub struct Swapped {
				pub pool_id: ::core::primitive::u128,
				pub who: runtime_types::sp_core::crypto::AccountId32,
				pub base_asset: runtime_types::primitives::currency::CurrencyId,
				pub quote_asset: runtime_types::primitives::currency::CurrencyId,
				pub base_amount: ::core::primitive::u128,
				pub quote_amount: ::core::primitive::u128,
				pub fee: runtime_types::composable_traits::dex::Fee<
					runtime_types::primitives::currency::CurrencyId,
					::core::primitive::u128,
				>,
			}
			impl ::subxt::events::StaticEvent for Swapped {
				const PALLET: &'static str = "Pablo";
				const EVENT: &'static str = "Swapped";
			}
			pub struct TwapUpdated {
				pub pool_id: ::core::primitive::u128,
				pub timestamp: ::core::primitive::u64,
				pub twaps: ::std::collections::BTreeMap<
					runtime_types::primitives::currency::CurrencyId,
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
				>,
			}
			impl ::subxt::events::StaticEvent for TwapUpdated {
				const PALLET: &'static str = "Pablo";
				const EVENT: &'static str = "TwapUpdated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn pool_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Pablo",
						"PoolCount",
						vec![],
						[
							183u8, 255u8, 60u8, 24u8, 240u8, 29u8, 31u8, 174u8, 60u8, 71u8, 252u8,
							76u8, 217u8, 193u8, 116u8, 93u8, 55u8, 97u8, 33u8, 229u8, 234u8, 153u8,
							2u8, 174u8, 152u8, 245u8, 122u8, 92u8, 123u8, 163u8, 242u8, 157u8,
						],
					)
				}
				pub fn pools(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_pablo::pallet::PoolConfiguration<
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Pablo",
						"Pools",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							223u8, 229u8, 232u8, 203u8, 241u8, 39u8, 138u8, 158u8, 188u8, 189u8,
							10u8, 134u8, 61u8, 212u8, 83u8, 76u8, 40u8, 78u8, 249u8, 114u8, 174u8,
							204u8, 144u8, 6u8, 70u8, 48u8, 215u8, 116u8, 17u8, 63u8, 20u8, 39u8,
						],
					)
				}
				pub fn pools_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_pablo::pallet::PoolConfiguration<
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Pablo",
						"Pools",
						Vec::new(),
						[
							223u8, 229u8, 232u8, 203u8, 241u8, 39u8, 138u8, 158u8, 188u8, 189u8,
							10u8, 134u8, 61u8, 212u8, 83u8, 76u8, 40u8, 78u8, 249u8, 114u8, 174u8,
							204u8, 144u8, 6u8, 70u8, 48u8, 215u8, 116u8, 17u8, 63u8, 20u8, 39u8,
						],
					)
				}
				pub fn twap_state(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_pablo::types::TimeWeightedAveragePrice<
						::core::primitive::u64,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Pablo",
						"TWAPState",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							54u8, 76u8, 169u8, 149u8, 75u8, 97u8, 21u8, 149u8, 143u8, 34u8, 221u8,
							80u8, 104u8, 166u8, 70u8, 4u8, 82u8, 6u8, 182u8, 229u8, 156u8, 176u8,
							166u8, 46u8, 27u8, 141u8, 121u8, 69u8, 229u8, 133u8, 181u8, 223u8,
						],
					)
				}
				pub fn twap_state_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_pablo::types::TimeWeightedAveragePrice<
						::core::primitive::u64,
						::core::primitive::u128,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Pablo",
						"TWAPState",
						Vec::new(),
						[
							54u8, 76u8, 169u8, 149u8, 75u8, 97u8, 21u8, 149u8, 143u8, 34u8, 221u8,
							80u8, 104u8, 166u8, 70u8, 4u8, 82u8, 6u8, 182u8, 229u8, 156u8, 176u8,
							166u8, 46u8, 27u8, 141u8, 121u8, 69u8, 229u8, 133u8, 181u8, 223u8,
						],
					)
				}
				pub fn price_cumulative_state(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_pablo::types::PriceCumulative<
						::core::primitive::u64,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Pablo",
						"PriceCumulativeState",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							190u8, 254u8, 1u8, 177u8, 88u8, 106u8, 218u8, 237u8, 159u8, 178u8,
							225u8, 59u8, 66u8, 107u8, 17u8, 238u8, 14u8, 162u8, 224u8, 139u8, 36u8,
							255u8, 194u8, 161u8, 135u8, 147u8, 83u8, 200u8, 235u8, 28u8, 154u8,
							180u8,
						],
					)
				}
				pub fn price_cumulative_state_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_pablo::types::PriceCumulative<
						::core::primitive::u64,
						::core::primitive::u128,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Pablo",
						"PriceCumulativeState",
						Vec::new(),
						[
							190u8, 254u8, 1u8, 177u8, 88u8, 106u8, 218u8, 237u8, 159u8, 178u8,
							225u8, 59u8, 66u8, 107u8, 17u8, 238u8, 14u8, 162u8, 224u8, 139u8, 36u8,
							255u8, 194u8, 161u8, 135u8, 147u8, 83u8, 200u8, 235u8, 28u8, 154u8,
							180u8,
						],
					)
				}
				pub fn lpt_nonce(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Pablo",
						"LPTNonce",
						vec![],
						[
							222u8, 15u8, 214u8, 107u8, 74u8, 235u8, 107u8, 190u8, 255u8, 72u8,
							41u8, 84u8, 241u8, 202u8, 218u8, 20u8, 82u8, 220u8, 114u8, 120u8, 8u8,
							3u8, 127u8, 118u8, 12u8, 112u8, 105u8, 56u8, 21u8, 175u8, 164u8, 43u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"Pablo",
						"PalletId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
				pub fn twap_interval(&self) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Pablo",
						"TWAPInterval",
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
	pub mod oracle {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_oracle::pallet::Error;
		pub type Call = runtime_types::pallet_oracle::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct AddAssetAndInfo {
					pub asset_id: runtime_types::primitives::currency::CurrencyId,
					pub threshold: runtime_types::sp_arithmetic::per_things::Percent,
					pub min_answers: ::core::primitive::u32,
					pub max_answers: ::core::primitive::u32,
					pub block_interval: ::core::primitive::u32,
					pub reward_weight: ::core::primitive::u128,
					pub slash: ::core::primitive::u128,
					pub emit_price_changes: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddAssetAndInfo {
					const PALLET: &'static str = "Oracle";
					const CALL: &'static str = "add_asset_and_info";
				}
				pub struct SetSigner {
					pub who: runtime_types::sp_core::crypto::AccountId32,
					pub signer: runtime_types::sp_core::crypto::AccountId32,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetSigner {
					const PALLET: &'static str = "Oracle";
					const CALL: &'static str = "set_signer";
				}
				pub struct AdjustRewards {
					pub annual_cost_per_oracle: ::core::primitive::u128,
					pub num_ideal_oracles: ::core::primitive::u8,
				}
				impl ::subxt::blocks::StaticExtrinsic for AdjustRewards {
					const PALLET: &'static str = "Oracle";
					const CALL: &'static str = "adjust_rewards";
				}
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct AddStake {
					pub stake: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddStake {
					const PALLET: &'static str = "Oracle";
					const CALL: &'static str = "add_stake";
				}
				pub struct RemoveStake;
				impl ::subxt::blocks::StaticExtrinsic for RemoveStake {
					const PALLET: &'static str = "Oracle";
					const CALL: &'static str = "remove_stake";
				}
				pub struct ReclaimStake;
				impl ::subxt::blocks::StaticExtrinsic for ReclaimStake {
					const PALLET: &'static str = "Oracle";
					const CALL: &'static str = "reclaim_stake";
				}
				pub struct SubmitPrice {
					pub price: ::core::primitive::u128,
					pub asset_id: runtime_types::primitives::currency::CurrencyId,
				}
				impl ::subxt::blocks::StaticExtrinsic for SubmitPrice {
					const PALLET: &'static str = "Oracle";
					const CALL: &'static str = "submit_price";
				}
				pub struct RemoveSigner {
					pub who: runtime_types::sp_core::crypto::AccountId32,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveSigner {
					const PALLET: &'static str = "Oracle";
					const CALL: &'static str = "remove_signer";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_asset_and_info(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					threshold: runtime_types::sp_arithmetic::per_things::Percent,
					min_answers: ::core::primitive::u32,
					max_answers: ::core::primitive::u32,
					block_interval: ::core::primitive::u32,
					reward_weight: ::core::primitive::u128,
					slash: ::core::primitive::u128,
					emit_price_changes: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::AddAssetAndInfo> {
					::subxt::tx::Payload::new_static(
						"Oracle",
						"add_asset_and_info",
						types::AddAssetAndInfo {
							asset_id,
							threshold,
							min_answers,
							max_answers,
							block_interval,
							reward_weight,
							slash,
							emit_price_changes,
						},
						[
							99u8, 5u8, 212u8, 116u8, 127u8, 74u8, 1u8, 57u8, 220u8, 13u8, 231u8,
							155u8, 118u8, 80u8, 197u8, 108u8, 41u8, 5u8, 236u8, 96u8, 140u8, 172u8,
							78u8, 233u8, 216u8, 67u8, 43u8, 6u8, 121u8, 206u8, 69u8, 107u8,
						],
					)
				}
				pub fn set_signer(
					&self,
					who: runtime_types::sp_core::crypto::AccountId32,
					signer: runtime_types::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::Payload<types::SetSigner> {
					::subxt::tx::Payload::new_static(
						"Oracle",
						"set_signer",
						types::SetSigner { who, signer },
						[
							222u8, 186u8, 33u8, 126u8, 40u8, 237u8, 163u8, 155u8, 147u8, 26u8,
							122u8, 26u8, 181u8, 31u8, 171u8, 150u8, 185u8, 6u8, 29u8, 128u8, 243u8,
							82u8, 39u8, 195u8, 254u8, 25u8, 147u8, 78u8, 25u8, 154u8, 122u8, 76u8,
						],
					)
				}
				pub fn adjust_rewards(
					&self,
					annual_cost_per_oracle: ::core::primitive::u128,
					num_ideal_oracles: ::core::primitive::u8,
				) -> ::subxt::tx::Payload<types::AdjustRewards> {
					::subxt::tx::Payload::new_static(
						"Oracle",
						"adjust_rewards",
						types::AdjustRewards { annual_cost_per_oracle, num_ideal_oracles },
						[
							246u8, 137u8, 196u8, 148u8, 26u8, 60u8, 154u8, 149u8, 204u8, 162u8,
							85u8, 113u8, 22u8, 252u8, 238u8, 138u8, 227u8, 12u8, 48u8, 99u8, 28u8,
							241u8, 106u8, 17u8, 137u8, 176u8, 255u8, 243u8, 170u8, 197u8, 154u8,
							87u8,
						],
					)
				}
				pub fn add_stake(
					&self,
					stake: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::AddStake> {
					::subxt::tx::Payload::new_static(
						"Oracle",
						"add_stake",
						types::AddStake { stake },
						[
							112u8, 71u8, 42u8, 65u8, 55u8, 22u8, 190u8, 33u8, 99u8, 200u8, 194u8,
							40u8, 241u8, 196u8, 118u8, 229u8, 154u8, 49u8, 31u8, 169u8, 245u8,
							212u8, 183u8, 117u8, 250u8, 191u8, 147u8, 237u8, 61u8, 195u8, 163u8,
							36u8,
						],
					)
				}
				pub fn remove_stake(&self) -> ::subxt::tx::Payload<types::RemoveStake> {
					::subxt::tx::Payload::new_static(
						"Oracle",
						"remove_stake",
						types::RemoveStake {},
						[
							229u8, 100u8, 125u8, 215u8, 116u8, 137u8, 46u8, 40u8, 112u8, 148u8,
							29u8, 124u8, 5u8, 122u8, 161u8, 157u8, 196u8, 158u8, 84u8, 24u8, 194u8,
							23u8, 37u8, 45u8, 189u8, 61u8, 161u8, 32u8, 255u8, 241u8, 217u8, 150u8,
						],
					)
				}
				pub fn reclaim_stake(&self) -> ::subxt::tx::Payload<types::ReclaimStake> {
					::subxt::tx::Payload::new_static(
						"Oracle",
						"reclaim_stake",
						types::ReclaimStake {},
						[
							53u8, 237u8, 19u8, 122u8, 122u8, 101u8, 126u8, 141u8, 121u8, 181u8,
							150u8, 254u8, 187u8, 29u8, 43u8, 38u8, 86u8, 41u8, 150u8, 107u8, 80u8,
							179u8, 145u8, 178u8, 165u8, 121u8, 13u8, 68u8, 169u8, 190u8, 216u8,
							83u8,
						],
					)
				}
				pub fn submit_price(
					&self,
					price: ::core::primitive::u128,
					asset_id: runtime_types::primitives::currency::CurrencyId,
				) -> ::subxt::tx::Payload<types::SubmitPrice> {
					::subxt::tx::Payload::new_static(
						"Oracle",
						"submit_price",
						types::SubmitPrice { price, asset_id },
						[
							127u8, 221u8, 44u8, 195u8, 195u8, 131u8, 172u8, 211u8, 116u8, 149u8,
							239u8, 81u8, 136u8, 27u8, 146u8, 175u8, 192u8, 255u8, 28u8, 250u8,
							227u8, 92u8, 214u8, 29u8, 247u8, 57u8, 141u8, 214u8, 41u8, 161u8, 94u8,
							107u8,
						],
					)
				}
				pub fn remove_signer(
					&self,
					who: runtime_types::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::Payload<types::RemoveSigner> {
					::subxt::tx::Payload::new_static(
						"Oracle",
						"remove_signer",
						types::RemoveSigner { who },
						[
							191u8, 158u8, 168u8, 199u8, 224u8, 207u8, 0u8, 37u8, 156u8, 19u8, 57u8,
							51u8, 77u8, 23u8, 185u8, 193u8, 32u8, 120u8, 223u8, 241u8, 234u8, 67u8,
							174u8, 8u8, 203u8, 250u8, 246u8, 73u8, 173u8, 5u8, 0u8, 149u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_oracle::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct AssetInfoChange(
				pub runtime_types::primitives::currency::CurrencyId,
				pub runtime_types::sp_arithmetic::per_things::Percent,
				pub ::core::primitive::u32,
				pub ::core::primitive::u32,
				pub ::core::primitive::u32,
				pub ::core::primitive::u128,
				pub ::core::primitive::u128,
			);
			impl ::subxt::events::StaticEvent for AssetInfoChange {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "AssetInfoChange";
			}
			pub struct SignerSet(
				pub runtime_types::sp_core::crypto::AccountId32,
				pub runtime_types::sp_core::crypto::AccountId32,
			);
			impl ::subxt::events::StaticEvent for SignerSet {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "SignerSet";
			}
			pub struct StakeAdded(
				pub runtime_types::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
				pub ::core::primitive::u128,
			);
			impl ::subxt::events::StaticEvent for StakeAdded {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "StakeAdded";
			}
			pub struct StakeRemoved(
				pub runtime_types::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for StakeRemoved {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "StakeRemoved";
			}
			pub struct StakeReclaimed(
				pub runtime_types::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::events::StaticEvent for StakeReclaimed {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "StakeReclaimed";
			}
			pub struct PriceSubmitted(
				pub runtime_types::sp_core::crypto::AccountId32,
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::core::primitive::u128,
			);
			impl ::subxt::events::StaticEvent for PriceSubmitted {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "PriceSubmitted";
			}
			pub struct UserSlashed(
				pub runtime_types::sp_core::crypto::AccountId32,
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::core::primitive::u128,
			);
			impl ::subxt::events::StaticEvent for UserSlashed {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "UserSlashed";
			}
			pub struct OracleRewarded(
				pub runtime_types::sp_core::crypto::AccountId32,
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::core::primitive::u128,
			);
			impl ::subxt::events::StaticEvent for OracleRewarded {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "OracleRewarded";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct RewardingAdjustment(pub ::core::primitive::u64);
			impl ::subxt::events::StaticEvent for RewardingAdjustment {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "RewardingAdjustment";
			}
			pub struct AnswerPruned(
				pub runtime_types::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::events::StaticEvent for AnswerPruned {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "AnswerPruned";
			}
			pub struct PriceChanged(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::core::primitive::u128,
			);
			impl ::subxt::events::StaticEvent for PriceChanged {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "PriceChanged";
			}
			pub struct SignerRemoved(
				pub runtime_types::sp_core::crypto::AccountId32,
				pub runtime_types::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::events::StaticEvent for SignerRemoved {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "SignerRemoved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn assets_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"AssetsCount",
						vec![],
						[
							26u8, 170u8, 19u8, 33u8, 67u8, 150u8, 43u8, 77u8, 254u8, 45u8, 7u8,
							201u8, 194u8, 33u8, 155u8, 231u8, 182u8, 210u8, 44u8, 144u8, 97u8,
							12u8, 116u8, 26u8, 81u8, 146u8, 206u8, 234u8, 172u8, 50u8, 123u8,
							184u8,
						],
					)
				}
				pub fn reward_tracker_store(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_traits::oracle::RewardTracker<
						::core::primitive::u128,
						::core::primitive::u64,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"RewardTrackerStore",
						vec![],
						[
							34u8, 101u8, 61u8, 238u8, 92u8, 67u8, 109u8, 50u8, 85u8, 9u8, 97u8,
							212u8, 83u8, 28u8, 93u8, 204u8, 21u8, 246u8, 52u8, 239u8, 170u8, 80u8,
							13u8, 172u8, 120u8, 86u8, 92u8, 71u8, 196u8, 251u8, 152u8, 84u8,
						],
					)
				}
				pub fn signer_to_controller(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"SignerToController",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							23u8, 131u8, 208u8, 65u8, 254u8, 0u8, 144u8, 142u8, 2u8, 26u8, 112u8,
							75u8, 50u8, 103u8, 173u8, 253u8, 4u8, 27u8, 179u8, 235u8, 72u8, 237u8,
							60u8, 54u8, 227u8, 96u8, 35u8, 9u8, 30u8, 162u8, 19u8, 88u8,
						],
					)
				}
				pub fn signer_to_controller_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"SignerToController",
						Vec::new(),
						[
							23u8, 131u8, 208u8, 65u8, 254u8, 0u8, 144u8, 142u8, 2u8, 26u8, 112u8,
							75u8, 50u8, 103u8, 173u8, 253u8, 4u8, 27u8, 179u8, 235u8, 72u8, 237u8,
							60u8, 54u8, 227u8, 96u8, 35u8, 9u8, 30u8, 162u8, 19u8, 88u8,
						],
					)
				}
				pub fn controller_to_signer(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"ControllerToSigner",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							44u8, 71u8, 221u8, 56u8, 163u8, 156u8, 200u8, 100u8, 98u8, 153u8,
							254u8, 196u8, 239u8, 142u8, 167u8, 106u8, 148u8, 138u8, 56u8, 201u8,
							185u8, 59u8, 219u8, 229u8, 107u8, 134u8, 95u8, 85u8, 29u8, 244u8, 28u8,
							162u8,
						],
					)
				}
				pub fn controller_to_signer_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_core::crypto::AccountId32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"ControllerToSigner",
						Vec::new(),
						[
							44u8, 71u8, 221u8, 56u8, 163u8, 156u8, 200u8, 100u8, 98u8, 153u8,
							254u8, 196u8, 239u8, 142u8, 167u8, 106u8, 148u8, 138u8, 56u8, 201u8,
							185u8, 59u8, 219u8, 229u8, 107u8, 134u8, 95u8, 85u8, 29u8, 244u8, 28u8,
							162u8,
						],
					)
				}
				pub fn declared_withdraws(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_oracle::pallet::Withdraw<
						::core::primitive::u128,
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"DeclaredWithdraws",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							221u8, 89u8, 71u8, 73u8, 86u8, 137u8, 131u8, 76u8, 78u8, 251u8, 57u8,
							155u8, 112u8, 177u8, 16u8, 37u8, 250u8, 91u8, 109u8, 5u8, 205u8, 113u8,
							74u8, 73u8, 4u8, 53u8, 199u8, 248u8, 226u8, 95u8, 79u8, 201u8,
						],
					)
				}
				pub fn declared_withdraws_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_oracle::pallet::Withdraw<
						::core::primitive::u128,
						::core::primitive::u32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"DeclaredWithdraws",
						Vec::new(),
						[
							221u8, 89u8, 71u8, 73u8, 86u8, 137u8, 131u8, 76u8, 78u8, 251u8, 57u8,
							155u8, 112u8, 177u8, 16u8, 37u8, 250u8, 91u8, 109u8, 5u8, 205u8, 113u8,
							74u8, 73u8, 4u8, 53u8, 199u8, 248u8, 226u8, 95u8, 79u8, 201u8,
						],
					)
				}
				pub fn oracle_stake(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"OracleStake",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							47u8, 225u8, 166u8, 255u8, 251u8, 171u8, 132u8, 47u8, 250u8, 61u8,
							134u8, 93u8, 233u8, 34u8, 143u8, 105u8, 249u8, 0u8, 73u8, 33u8, 80u8,
							129u8, 108u8, 107u8, 44u8, 14u8, 62u8, 234u8, 253u8, 188u8, 214u8,
							114u8,
						],
					)
				}
				pub fn oracle_stake_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"OracleStake",
						Vec::new(),
						[
							47u8, 225u8, 166u8, 255u8, 251u8, 171u8, 132u8, 47u8, 250u8, 61u8,
							134u8, 93u8, 233u8, 34u8, 143u8, 105u8, 249u8, 0u8, 73u8, 33u8, 80u8,
							129u8, 108u8, 107u8, 44u8, 14u8, 62u8, 234u8, 253u8, 188u8, 214u8,
							114u8,
						],
					)
				}
				pub fn accumulated_rewards_per_asset(
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
						"Oracle",
						"AccumulatedRewardsPerAsset",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							224u8, 106u8, 164u8, 222u8, 249u8, 128u8, 124u8, 197u8, 228u8, 219u8,
							195u8, 145u8, 44u8, 225u8, 191u8, 65u8, 178u8, 100u8, 60u8, 61u8,
							129u8, 88u8, 20u8, 132u8, 247u8, 172u8, 193u8, 212u8, 161u8, 17u8, 6u8,
							143u8,
						],
					)
				}
				pub fn accumulated_rewards_per_asset_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"AccumulatedRewardsPerAsset",
						Vec::new(),
						[
							224u8, 106u8, 164u8, 222u8, 249u8, 128u8, 124u8, 197u8, 228u8, 219u8,
							195u8, 145u8, 44u8, 225u8, 191u8, 65u8, 178u8, 100u8, 60u8, 61u8,
							129u8, 88u8, 20u8, 132u8, 247u8, 172u8, 193u8, 212u8, 161u8, 17u8, 6u8,
							143u8,
						],
					)
				}
				pub fn answer_in_transit(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"AnswerInTransit",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							64u8, 220u8, 69u8, 28u8, 107u8, 150u8, 97u8, 232u8, 45u8, 41u8, 110u8,
							191u8, 105u8, 53u8, 132u8, 59u8, 47u8, 53u8, 172u8, 255u8, 84u8, 54u8,
							209u8, 49u8, 215u8, 192u8, 51u8, 247u8, 97u8, 59u8, 34u8, 119u8,
						],
					)
				}
				pub fn answer_in_transit_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"AnswerInTransit",
						Vec::new(),
						[
							64u8, 220u8, 69u8, 28u8, 107u8, 150u8, 97u8, 232u8, 45u8, 41u8, 110u8,
							191u8, 105u8, 53u8, 132u8, 59u8, 47u8, 53u8, 172u8, 255u8, 84u8, 54u8,
							209u8, 49u8, 215u8, 192u8, 51u8, 247u8, 97u8, 59u8, 34u8, 119u8,
						],
					)
				}
				pub fn prices(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_traits::oracle::Price<
						::core::primitive::u128,
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"Prices",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							35u8, 164u8, 171u8, 6u8, 176u8, 91u8, 7u8, 106u8, 34u8, 163u8, 216u8,
							217u8, 145u8, 22u8, 244u8, 186u8, 113u8, 242u8, 36u8, 155u8, 151u8,
							174u8, 10u8, 113u8, 115u8, 130u8, 24u8, 237u8, 219u8, 181u8, 67u8,
							30u8,
						],
					)
				}
				pub fn prices_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_traits::oracle::Price<
						::core::primitive::u128,
						::core::primitive::u32,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"Prices",
						Vec::new(),
						[
							35u8, 164u8, 171u8, 6u8, 176u8, 91u8, 7u8, 106u8, 34u8, 163u8, 216u8,
							217u8, 145u8, 22u8, 244u8, 186u8, 113u8, 242u8, 36u8, 155u8, 151u8,
							174u8, 10u8, 113u8, 115u8, 130u8, 24u8, 237u8, 219u8, 181u8, 67u8,
							30u8,
						],
					)
				}
				pub fn price_history(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::composable_traits::oracle::Price<
							::core::primitive::u128,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"PriceHistory",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							59u8, 190u8, 248u8, 173u8, 3u8, 171u8, 194u8, 1u8, 227u8, 100u8, 88u8,
							161u8, 250u8, 138u8, 235u8, 248u8, 133u8, 233u8, 84u8, 59u8, 207u8,
							204u8, 135u8, 20u8, 56u8, 123u8, 17u8, 65u8, 154u8, 20u8, 10u8, 56u8,
						],
					)
				}
				pub fn price_history_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::composable_traits::oracle::Price<
							::core::primitive::u128,
							::core::primitive::u32,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"PriceHistory",
						Vec::new(),
						[
							59u8, 190u8, 248u8, 173u8, 3u8, 171u8, 194u8, 1u8, 227u8, 100u8, 88u8,
							161u8, 250u8, 138u8, 235u8, 248u8, 133u8, 233u8, 84u8, 59u8, 207u8,
							204u8, 135u8, 20u8, 56u8, 123u8, 17u8, 65u8, 154u8, 20u8, 10u8, 56u8,
						],
					)
				}
				pub fn pre_prices(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_oracle::pallet::PrePrice<
							::core::primitive::u128,
							::core::primitive::u32,
							runtime_types::sp_core::crypto::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"PrePrices",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							24u8, 184u8, 217u8, 225u8, 59u8, 60u8, 114u8, 143u8, 46u8, 75u8, 60u8,
							174u8, 75u8, 201u8, 0u8, 135u8, 187u8, 0u8, 18u8, 120u8, 107u8, 163u8,
							223u8, 21u8, 28u8, 65u8, 200u8, 141u8, 9u8, 57u8, 120u8, 85u8,
						],
					)
				}
				pub fn pre_prices_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_oracle::pallet::PrePrice<
							::core::primitive::u128,
							::core::primitive::u32,
							runtime_types::sp_core::crypto::AccountId32,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"PrePrices",
						Vec::new(),
						[
							24u8, 184u8, 217u8, 225u8, 59u8, 60u8, 114u8, 143u8, 46u8, 75u8, 60u8,
							174u8, 75u8, 201u8, 0u8, 135u8, 187u8, 0u8, 18u8, 120u8, 107u8, 163u8,
							223u8, 21u8, 28u8, 65u8, 200u8, 141u8, 9u8, 57u8, 120u8, 85u8,
						],
					)
				}
				pub fn assets_info(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_oracle::pallet::AssetInfo<
						runtime_types::sp_arithmetic::per_things::Percent,
						::core::primitive::u32,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"AssetsInfo",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							184u8, 86u8, 174u8, 146u8, 2u8, 132u8, 154u8, 152u8, 50u8, 54u8, 162u8,
							43u8, 148u8, 249u8, 178u8, 64u8, 125u8, 167u8, 148u8, 179u8, 105u8,
							155u8, 20u8, 4u8, 63u8, 163u8, 92u8, 164u8, 52u8, 250u8, 8u8, 147u8,
						],
					)
				}
				pub fn assets_info_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_oracle::pallet::AssetInfo<
						runtime_types::sp_arithmetic::per_things::Percent,
						::core::primitive::u32,
						::core::primitive::u128,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Oracle",
						"AssetsInfo",
						Vec::new(),
						[
							184u8, 86u8, 174u8, 146u8, 2u8, 132u8, 154u8, 152u8, 50u8, 54u8, 162u8,
							43u8, 148u8, 249u8, 178u8, 64u8, 125u8, 167u8, 148u8, 179u8, 105u8,
							155u8, 20u8, 4u8, 63u8, 163u8, 92u8, 164u8, 52u8, 250u8, 8u8, 147u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn max_history(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Oracle",
						"MaxHistory",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn twap_window(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
					::subxt::constants::Address::new_static(
						"Oracle",
						"TwapWindow",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
							41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
							90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}
				pub fn max_pre_prices(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Oracle",
						"MaxPrePrices",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn ms_per_block(&self) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Oracle",
						"MsPerBlock",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"Oracle",
						"PalletId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
			}
		}
	}
	pub mod assets_transactor_router {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_assets_transactor_router::pallet::Error;
		pub type Call = runtime_types::pallet_assets_transactor_router::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Transfer {
					pub asset: runtime_types::primitives::currency::CurrencyId,
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub amount: ::core::primitive::u128,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for Transfer {
					const PALLET: &'static str = "AssetsTransactorRouter";
					const CALL: &'static str = "transfer";
				}
				pub struct TransferNative {
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub value: ::core::primitive::u128,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferNative {
					const PALLET: &'static str = "AssetsTransactorRouter";
					const CALL: &'static str = "transfer_native";
				}
				pub struct ForceTransfer {
					pub asset: runtime_types::primitives::currency::CurrencyId,
					pub source: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub value: ::core::primitive::u128,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "AssetsTransactorRouter";
					const CALL: &'static str = "force_transfer";
				}
				pub struct ForceTransferNative {
					pub source: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub value: ::core::primitive::u128,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransferNative {
					const PALLET: &'static str = "AssetsTransactorRouter";
					const CALL: &'static str = "force_transfer_native";
				}
				pub struct TransferAll {
					pub asset: runtime_types::primitives::currency::CurrencyId,
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAll {
					const PALLET: &'static str = "AssetsTransactorRouter";
					const CALL: &'static str = "transfer_all";
				}
				pub struct TransferAllNative {
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAllNative {
					const PALLET: &'static str = "AssetsTransactorRouter";
					const CALL: &'static str = "transfer_all_native";
				}
				pub struct MintInto {
					pub asset_id: runtime_types::primitives::currency::CurrencyId,
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for MintInto {
					const PALLET: &'static str = "AssetsTransactorRouter";
					const CALL: &'static str = "mint_into";
				}
				pub struct BurnFrom {
					pub asset_id: runtime_types::primitives::currency::CurrencyId,
					pub dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for BurnFrom {
					const PALLET: &'static str = "AssetsTransactorRouter";
					const CALL: &'static str = "burn_from";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn transfer(
					&self,
					asset: runtime_types::primitives::currency::CurrencyId,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::Transfer> {
					::subxt::tx::Payload::new_static(
						"AssetsTransactorRouter",
						"transfer",
						types::Transfer { asset, dest, amount, keep_alive },
						[
							184u8, 246u8, 251u8, 64u8, 75u8, 174u8, 137u8, 3u8, 133u8, 156u8,
							215u8, 168u8, 121u8, 36u8, 127u8, 104u8, 226u8, 81u8, 165u8, 146u8,
							130u8, 96u8, 240u8, 111u8, 17u8, 82u8, 58u8, 181u8, 238u8, 163u8,
							132u8, 180u8,
						],
					)
				}
				pub fn transfer_native(
					&self,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::TransferNative> {
					::subxt::tx::Payload::new_static(
						"AssetsTransactorRouter",
						"transfer_native",
						types::TransferNative { dest, value, keep_alive },
						[
							3u8, 196u8, 93u8, 10u8, 182u8, 116u8, 7u8, 138u8, 132u8, 34u8, 248u8,
							246u8, 236u8, 126u8, 219u8, 105u8, 115u8, 6u8, 150u8, 101u8, 225u8,
							87u8, 79u8, 44u8, 83u8, 74u8, 115u8, 191u8, 135u8, 142u8, 84u8, 37u8,
						],
					)
				}
				pub fn force_transfer(
					&self,
					asset: runtime_types::primitives::currency::CurrencyId,
					source: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"AssetsTransactorRouter",
						"force_transfer",
						types::ForceTransfer { asset, source, dest, value, keep_alive },
						[
							6u8, 101u8, 80u8, 210u8, 115u8, 214u8, 17u8, 57u8, 82u8, 60u8, 112u8,
							216u8, 28u8, 71u8, 107u8, 74u8, 102u8, 250u8, 65u8, 220u8, 24u8, 216u8,
							183u8, 137u8, 136u8, 200u8, 126u8, 156u8, 209u8, 114u8, 98u8, 239u8,
						],
					)
				}
				pub fn force_transfer_native(
					&self,
					source: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::ForceTransferNative> {
					::subxt::tx::Payload::new_static(
						"AssetsTransactorRouter",
						"force_transfer_native",
						types::ForceTransferNative { source, dest, value, keep_alive },
						[
							157u8, 74u8, 150u8, 83u8, 182u8, 65u8, 66u8, 81u8, 244u8, 27u8, 249u8,
							27u8, 57u8, 140u8, 183u8, 217u8, 226u8, 67u8, 188u8, 115u8, 136u8,
							89u8, 179u8, 48u8, 179u8, 228u8, 220u8, 211u8, 80u8, 127u8, 43u8,
							146u8,
						],
					)
				}
				pub fn transfer_all(
					&self,
					asset: runtime_types::primitives::currency::CurrencyId,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::TransferAll> {
					::subxt::tx::Payload::new_static(
						"AssetsTransactorRouter",
						"transfer_all",
						types::TransferAll { asset, dest, keep_alive },
						[
							138u8, 73u8, 135u8, 11u8, 146u8, 69u8, 73u8, 148u8, 153u8, 218u8,
							184u8, 228u8, 138u8, 195u8, 154u8, 16u8, 181u8, 80u8, 133u8, 38u8,
							27u8, 47u8, 35u8, 40u8, 49u8, 89u8, 239u8, 179u8, 244u8, 7u8, 169u8,
							7u8,
						],
					)
				}
				pub fn transfer_all_native(
					&self,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::TransferAllNative> {
					::subxt::tx::Payload::new_static(
						"AssetsTransactorRouter",
						"transfer_all_native",
						types::TransferAllNative { dest, keep_alive },
						[
							41u8, 133u8, 254u8, 200u8, 107u8, 184u8, 50u8, 154u8, 214u8, 251u8,
							82u8, 153u8, 123u8, 67u8, 197u8, 87u8, 95u8, 221u8, 245u8, 54u8, 255u8,
							143u8, 63u8, 6u8, 219u8, 246u8, 181u8, 238u8, 54u8, 74u8, 245u8, 225u8,
						],
					)
				}
				pub fn mint_into(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::MintInto> {
					::subxt::tx::Payload::new_static(
						"AssetsTransactorRouter",
						"mint_into",
						types::MintInto { asset_id, dest, amount },
						[
							105u8, 102u8, 159u8, 203u8, 163u8, 82u8, 147u8, 243u8, 51u8, 199u8,
							15u8, 218u8, 166u8, 2u8, 171u8, 225u8, 18u8, 47u8, 77u8, 97u8, 170u8,
							134u8, 60u8, 109u8, 199u8, 248u8, 79u8, 52u8, 92u8, 240u8, 0u8, 158u8,
						],
					)
				}
				pub fn burn_from(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::BurnFrom> {
					::subxt::tx::Payload::new_static(
						"AssetsTransactorRouter",
						"burn_from",
						types::BurnFrom { asset_id, dest, amount },
						[
							49u8, 24u8, 10u8, 194u8, 121u8, 88u8, 89u8, 163u8, 11u8, 49u8, 229u8,
							186u8, 121u8, 176u8, 153u8, 252u8, 139u8, 235u8, 38u8, 130u8, 100u8,
							131u8, 71u8, 157u8, 58u8, 149u8, 213u8, 225u8, 133u8, 20u8, 130u8,
							132u8,
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
						"AssetsTransactorRouter",
						"NativeAssetId",
						[
							62u8, 145u8, 102u8, 227u8, 159u8, 92u8, 27u8, 54u8, 159u8, 228u8,
							193u8, 99u8, 75u8, 196u8, 26u8, 250u8, 229u8, 230u8, 88u8, 109u8,
							246u8, 100u8, 152u8, 158u8, 14u8, 25u8, 224u8, 173u8, 224u8, 41u8,
							105u8, 231u8,
						],
					)
				}
			}
		}
	}
	pub mod farming_rewards {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::reward::pallet::Error;
		pub type Call = runtime_types::reward::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub type Event = runtime_types::reward::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct DepositStake {
				pub pool_id: runtime_types::primitives::currency::CurrencyId,
				pub stake_id: runtime_types::sp_core::crypto::AccountId32,
				pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
			}
			impl ::subxt::events::StaticEvent for DepositStake {
				const PALLET: &'static str = "FarmingRewards";
				const EVENT: &'static str = "DepositStake";
			}
			pub struct DistributeReward {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
			}
			impl ::subxt::events::StaticEvent for DistributeReward {
				const PALLET: &'static str = "FarmingRewards";
				const EVENT: &'static str = "DistributeReward";
			}
			pub struct WithdrawStake {
				pub pool_id: runtime_types::primitives::currency::CurrencyId,
				pub stake_id: runtime_types::sp_core::crypto::AccountId32,
				pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
			}
			impl ::subxt::events::StaticEvent for WithdrawStake {
				const PALLET: &'static str = "FarmingRewards";
				const EVENT: &'static str = "WithdrawStake";
			}
			pub struct WithdrawReward {
				pub pool_id: runtime_types::primitives::currency::CurrencyId,
				pub stake_id: runtime_types::sp_core::crypto::AccountId32,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
			}
			impl ::subxt::events::StaticEvent for WithdrawReward {
				const PALLET: &'static str = "FarmingRewards";
				const EVENT: &'static str = "WithdrawReward";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn total_stake(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedI128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"TotalStake",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							53u8, 223u8, 87u8, 64u8, 70u8, 235u8, 147u8, 41u8, 166u8, 92u8, 172u8,
							7u8, 110u8, 130u8, 188u8, 229u8, 10u8, 84u8, 207u8, 131u8, 219u8, 60u8,
							208u8, 23u8, 70u8, 126u8, 17u8, 67u8, 242u8, 99u8, 177u8, 248u8,
						],
					)
				}
				pub fn total_stake_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedI128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"TotalStake",
						Vec::new(),
						[
							53u8, 223u8, 87u8, 64u8, 70u8, 235u8, 147u8, 41u8, 166u8, 92u8, 172u8,
							7u8, 110u8, 130u8, 188u8, 229u8, 10u8, 84u8, 207u8, 131u8, 219u8, 60u8,
							208u8, 23u8, 70u8, 126u8, 17u8, 67u8, 242u8, 99u8, 177u8, 248u8,
						],
					)
				}
				pub fn total_rewards(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedI128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"TotalRewards",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							139u8, 195u8, 51u8, 38u8, 129u8, 121u8, 135u8, 117u8, 59u8, 156u8,
							219u8, 100u8, 189u8, 154u8, 58u8, 255u8, 255u8, 138u8, 154u8, 107u8,
							138u8, 17u8, 219u8, 218u8, 42u8, 1u8, 250u8, 181u8, 124u8, 20u8, 171u8,
							100u8,
						],
					)
				}
				pub fn total_rewards_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedI128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"TotalRewards",
						Vec::new(),
						[
							139u8, 195u8, 51u8, 38u8, 129u8, 121u8, 135u8, 117u8, 59u8, 156u8,
							219u8, 100u8, 189u8, 154u8, 58u8, 255u8, 255u8, 138u8, 154u8, 107u8,
							138u8, 17u8, 219u8, 218u8, 42u8, 1u8, 250u8, 181u8, 124u8, 20u8, 171u8,
							100u8,
						],
					)
				}
				pub fn reward_per_token(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
					_1: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedI128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"RewardPerToken",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							192u8, 229u8, 84u8, 183u8, 117u8, 203u8, 231u8, 26u8, 193u8, 115u8,
							186u8, 229u8, 151u8, 30u8, 55u8, 73u8, 11u8, 99u8, 56u8, 115u8, 48u8,
							251u8, 16u8, 108u8, 101u8, 190u8, 191u8, 219u8, 52u8, 100u8, 10u8,
							107u8,
						],
					)
				}
				pub fn reward_per_token_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedI128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"RewardPerToken",
						Vec::new(),
						[
							192u8, 229u8, 84u8, 183u8, 117u8, 203u8, 231u8, 26u8, 193u8, 115u8,
							186u8, 229u8, 151u8, 30u8, 55u8, 73u8, 11u8, 99u8, 56u8, 115u8, 48u8,
							251u8, 16u8, 108u8, 101u8, 190u8, 191u8, 219u8, 52u8, 100u8, 10u8,
							107u8,
						],
					)
				}
				pub fn stake(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
					_1: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedI128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"Stake",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							24u8, 247u8, 175u8, 157u8, 168u8, 237u8, 253u8, 76u8, 7u8, 43u8, 187u8,
							18u8, 7u8, 67u8, 200u8, 34u8, 113u8, 12u8, 237u8, 200u8, 235u8, 156u8,
							161u8, 75u8, 38u8, 69u8, 111u8, 133u8, 57u8, 241u8, 177u8, 141u8,
						],
					)
				}
				pub fn stake_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedI128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"Stake",
						Vec::new(),
						[
							24u8, 247u8, 175u8, 157u8, 168u8, 237u8, 253u8, 76u8, 7u8, 43u8, 187u8,
							18u8, 7u8, 67u8, 200u8, 34u8, 113u8, 12u8, 237u8, 200u8, 235u8, 156u8,
							161u8, 75u8, 38u8, 69u8, 111u8, 133u8, 57u8, 241u8, 177u8, 141u8,
						],
					)
				}
				pub fn reward_tally(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
					_1: impl ::std::borrow::Borrow<(
						runtime_types::primitives::currency::CurrencyId,
						runtime_types::sp_core::crypto::AccountId32,
					)>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedI128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"RewardTally",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							74u8, 221u8, 254u8, 237u8, 249u8, 243u8, 40u8, 128u8, 215u8, 123u8,
							38u8, 229u8, 141u8, 13u8, 33u8, 131u8, 77u8, 127u8, 134u8, 235u8,
							233u8, 44u8, 186u8, 249u8, 55u8, 194u8, 82u8, 177u8, 241u8, 197u8,
							225u8, 242u8,
						],
					)
				}
				pub fn reward_tally_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedI128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"RewardTally",
						Vec::new(),
						[
							74u8, 221u8, 254u8, 237u8, 249u8, 243u8, 40u8, 128u8, 215u8, 123u8,
							38u8, 229u8, 141u8, 13u8, 33u8, 131u8, 77u8, 127u8, 134u8, 235u8,
							233u8, 44u8, 186u8, 249u8, 55u8, 194u8, 82u8, 177u8, 241u8, 197u8,
							225u8, 242u8,
						],
					)
				}
				pub fn reward_currencies(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
						runtime_types::primitives::currency::CurrencyId,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"RewardCurrencies",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							49u8, 45u8, 84u8, 123u8, 144u8, 102u8, 219u8, 48u8, 231u8, 162u8, 41u8,
							185u8, 44u8, 148u8, 216u8, 59u8, 30u8, 173u8, 88u8, 32u8, 57u8, 216u8,
							146u8, 38u8, 88u8, 53u8, 70u8, 146u8, 124u8, 112u8, 201u8, 154u8,
						],
					)
				}
				pub fn reward_currencies_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
						runtime_types::primitives::currency::CurrencyId,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"FarmingRewards",
						"RewardCurrencies",
						Vec::new(),
						[
							49u8, 45u8, 84u8, 123u8, 144u8, 102u8, 219u8, 48u8, 231u8, 162u8, 41u8,
							185u8, 44u8, 148u8, 216u8, 59u8, 30u8, 173u8, 88u8, 32u8, 57u8, 216u8,
							146u8, 38u8, 88u8, 53u8, 70u8, 146u8, 124u8, 112u8, 201u8, 154u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn max_reward_currencies(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"FarmingRewards",
						"MaxRewardCurrencies",
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
	pub mod farming {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::farming::pallet::Error;
		pub type Call = runtime_types::farming::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct UpdateRewardSchedule {
					pub pool_currency_id: runtime_types::primitives::currency::CurrencyId,
					pub reward_currency_id: runtime_types::primitives::currency::CurrencyId,
					pub period_count: ::core::primitive::u32,
					#[codec::codec(compact)]
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateRewardSchedule {
					const PALLET: &'static str = "Farming";
					const CALL: &'static str = "update_reward_schedule";
				}
				pub struct RemoveRewardSchedule {
					pub pool_currency_id: runtime_types::primitives::currency::CurrencyId,
					pub reward_currency_id: runtime_types::primitives::currency::CurrencyId,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveRewardSchedule {
					const PALLET: &'static str = "Farming";
					const CALL: &'static str = "remove_reward_schedule";
				}
				pub struct Deposit {
					pub pool_currency_id: runtime_types::primitives::currency::CurrencyId,
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Deposit {
					const PALLET: &'static str = "Farming";
					const CALL: &'static str = "deposit";
				}
				pub struct Withdraw {
					pub pool_currency_id: runtime_types::primitives::currency::CurrencyId,
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Withdraw {
					const PALLET: &'static str = "Farming";
					const CALL: &'static str = "withdraw";
				}
				pub struct Claim {
					pub pool_currency_id: runtime_types::primitives::currency::CurrencyId,
					pub reward_currency_id: runtime_types::primitives::currency::CurrencyId,
				}
				impl ::subxt::blocks::StaticExtrinsic for Claim {
					const PALLET: &'static str = "Farming";
					const CALL: &'static str = "claim";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn update_reward_schedule(
					&self,
					pool_currency_id: runtime_types::primitives::currency::CurrencyId,
					reward_currency_id: runtime_types::primitives::currency::CurrencyId,
					period_count: ::core::primitive::u32,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::UpdateRewardSchedule> {
					::subxt::tx::Payload::new_static(
						"Farming",
						"update_reward_schedule",
						types::UpdateRewardSchedule {
							pool_currency_id,
							reward_currency_id,
							period_count,
							amount,
						},
						[
							154u8, 251u8, 7u8, 217u8, 194u8, 48u8, 90u8, 115u8, 25u8, 253u8, 5u8,
							140u8, 189u8, 118u8, 69u8, 234u8, 78u8, 58u8, 23u8, 6u8, 145u8, 68u8,
							65u8, 180u8, 62u8, 26u8, 223u8, 73u8, 122u8, 2u8, 199u8, 164u8,
						],
					)
				}
				pub fn remove_reward_schedule(
					&self,
					pool_currency_id: runtime_types::primitives::currency::CurrencyId,
					reward_currency_id: runtime_types::primitives::currency::CurrencyId,
				) -> ::subxt::tx::Payload<types::RemoveRewardSchedule> {
					::subxt::tx::Payload::new_static(
						"Farming",
						"remove_reward_schedule",
						types::RemoveRewardSchedule { pool_currency_id, reward_currency_id },
						[
							23u8, 91u8, 170u8, 80u8, 137u8, 128u8, 78u8, 236u8, 248u8, 205u8, 35u8,
							118u8, 156u8, 16u8, 246u8, 21u8, 159u8, 220u8, 83u8, 41u8, 30u8, 231u8,
							183u8, 187u8, 223u8, 23u8, 193u8, 100u8, 153u8, 184u8, 157u8, 147u8,
						],
					)
				}
				pub fn deposit(
					&self,
					pool_currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Deposit> {
					::subxt::tx::Payload::new_static(
						"Farming",
						"deposit",
						types::Deposit { pool_currency_id, amount },
						[
							232u8, 33u8, 196u8, 202u8, 43u8, 33u8, 19u8, 26u8, 173u8, 110u8, 30u8,
							199u8, 219u8, 137u8, 53u8, 37u8, 186u8, 176u8, 236u8, 0u8, 27u8, 132u8,
							20u8, 54u8, 204u8, 40u8, 17u8, 187u8, 2u8, 145u8, 93u8, 138u8,
						],
					)
				}
				pub fn withdraw(
					&self,
					pool_currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Withdraw> {
					::subxt::tx::Payload::new_static(
						"Farming",
						"withdraw",
						types::Withdraw { pool_currency_id, amount },
						[
							241u8, 236u8, 91u8, 60u8, 68u8, 43u8, 175u8, 224u8, 219u8, 247u8,
							230u8, 46u8, 54u8, 174u8, 45u8, 113u8, 13u8, 195u8, 226u8, 187u8,
							255u8, 8u8, 50u8, 204u8, 198u8, 152u8, 36u8, 36u8, 225u8, 71u8, 36u8,
							139u8,
						],
					)
				}
				pub fn claim(
					&self,
					pool_currency_id: runtime_types::primitives::currency::CurrencyId,
					reward_currency_id: runtime_types::primitives::currency::CurrencyId,
				) -> ::subxt::tx::Payload<types::Claim> {
					::subxt::tx::Payload::new_static(
						"Farming",
						"claim",
						types::Claim { pool_currency_id, reward_currency_id },
						[
							190u8, 93u8, 122u8, 238u8, 3u8, 214u8, 107u8, 249u8, 233u8, 61u8,
							127u8, 110u8, 193u8, 232u8, 13u8, 208u8, 47u8, 165u8, 109u8, 219u8,
							142u8, 230u8, 188u8, 20u8, 45u8, 253u8, 59u8, 178u8, 36u8, 123u8, 36u8,
							140u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::farming::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct RewardScheduleUpdated {
				pub pool_currency_id: runtime_types::primitives::currency::CurrencyId,
				pub reward_currency_id: runtime_types::primitives::currency::CurrencyId,
				pub period_count: ::core::primitive::u32,
				pub per_period: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for RewardScheduleUpdated {
				const PALLET: &'static str = "Farming";
				const EVENT: &'static str = "RewardScheduleUpdated";
			}
			pub struct RewardDistributed {
				pub pool_currency_id: runtime_types::primitives::currency::CurrencyId,
				pub reward_currency_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for RewardDistributed {
				const PALLET: &'static str = "Farming";
				const EVENT: &'static str = "RewardDistributed";
			}
			pub struct RewardClaimed {
				pub account_id: runtime_types::sp_core::crypto::AccountId32,
				pub pool_currency_id: runtime_types::primitives::currency::CurrencyId,
				pub reward_currency_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for RewardClaimed {
				const PALLET: &'static str = "Farming";
				const EVENT: &'static str = "RewardClaimed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn reward_schedules(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
					_1: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::farming::RewardSchedule<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Farming",
						"RewardSchedules",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							61u8, 77u8, 11u8, 198u8, 114u8, 181u8, 213u8, 10u8, 86u8, 218u8, 77u8,
							75u8, 251u8, 84u8, 255u8, 160u8, 199u8, 18u8, 43u8, 147u8, 140u8, 26u8,
							189u8, 50u8, 129u8, 252u8, 159u8, 151u8, 57u8, 156u8, 174u8, 152u8,
						],
					)
				}
				pub fn reward_schedules_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::farming::RewardSchedule<::core::primitive::u128>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Farming",
						"RewardSchedules",
						Vec::new(),
						[
							61u8, 77u8, 11u8, 198u8, 114u8, 181u8, 213u8, 10u8, 86u8, 218u8, 77u8,
							75u8, 251u8, 84u8, 255u8, 160u8, 199u8, 18u8, 43u8, 147u8, 140u8, 26u8,
							189u8, 50u8, 129u8, 252u8, 159u8, 151u8, 57u8, 156u8, 174u8, 152u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn farming_pallet_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"Farming",
						"FarmingPalletId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
				pub fn treasury_account_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_core::crypto::AccountId32> {
					::subxt::constants::Address::new_static(
						"Farming",
						"TreasuryAccountId",
						[
							115u8, 233u8, 13u8, 223u8, 88u8, 20u8, 202u8, 139u8, 153u8, 28u8,
							155u8, 157u8, 224u8, 66u8, 3u8, 250u8, 23u8, 53u8, 88u8, 168u8, 211u8,
							204u8, 122u8, 166u8, 248u8, 23u8, 174u8, 225u8, 99u8, 108u8, 89u8,
							135u8,
						],
					)
				}
				pub fn reward_period(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Farming",
						"RewardPeriod",
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
	pub mod call_filter {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_call_filter::pallet::Error;
		pub type Call = runtime_types::pallet_call_filter::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Disable {
					pub entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
						runtime_types::common::MaxStringSize,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Disable {
					const PALLET: &'static str = "CallFilter";
					const CALL: &'static str = "disable";
				}
				pub struct Enable {
					pub entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
						runtime_types::common::MaxStringSize,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Enable {
					const PALLET: &'static str = "CallFilter";
					const CALL: &'static str = "enable";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn disable(
					&self,
					entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
						runtime_types::common::MaxStringSize,
					>,
				) -> ::subxt::tx::Payload<types::Disable> {
					::subxt::tx::Payload::new_static(
						"CallFilter",
						"disable",
						types::Disable { entry },
						[
							188u8, 68u8, 40u8, 232u8, 136u8, 101u8, 41u8, 224u8, 80u8, 155u8,
							209u8, 49u8, 182u8, 253u8, 103u8, 16u8, 48u8, 243u8, 212u8, 228u8,
							152u8, 177u8, 169u8, 133u8, 132u8, 186u8, 91u8, 176u8, 160u8, 200u8,
							134u8, 7u8,
						],
					)
				}
				pub fn enable(
					&self,
					entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
						runtime_types::common::MaxStringSize,
					>,
				) -> ::subxt::tx::Payload<types::Enable> {
					::subxt::tx::Payload::new_static(
						"CallFilter",
						"enable",
						types::Enable { entry },
						[
							99u8, 190u8, 136u8, 50u8, 251u8, 149u8, 41u8, 228u8, 9u8, 250u8, 119u8,
							98u8, 236u8, 4u8, 42u8, 160u8, 0u8, 121u8, 34u8, 17u8, 57u8, 255u8,
							88u8, 143u8, 197u8, 79u8, 120u8, 180u8, 187u8, 28u8, 212u8, 249u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_call_filter::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Disabled {
				pub entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
					runtime_types::common::MaxStringSize,
				>,
			}
			impl ::subxt::events::StaticEvent for Disabled {
				const PALLET: &'static str = "CallFilter";
				const EVENT: &'static str = "Disabled";
			}
			pub struct Enabled {
				pub entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
					runtime_types::common::MaxStringSize,
				>,
			}
			impl ::subxt::events::StaticEvent for Enabled {
				const PALLET: &'static str = "CallFilter";
				const EVENT: &'static str = "Enabled";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn disabled_calls(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CallFilter",
						"DisabledCalls",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							168u8, 206u8, 116u8, 222u8, 205u8, 104u8, 83u8, 69u8, 166u8, 17u8,
							81u8, 183u8, 74u8, 115u8, 43u8, 188u8, 219u8, 121u8, 178u8, 70u8, 98u8,
							141u8, 10u8, 81u8, 104u8, 242u8, 213u8, 114u8, 137u8, 167u8, 26u8,
							137u8,
						],
					)
				}
				pub fn disabled_calls_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CallFilter",
						"DisabledCalls",
						Vec::new(),
						[
							168u8, 206u8, 116u8, 222u8, 205u8, 104u8, 83u8, 69u8, 166u8, 17u8,
							81u8, 183u8, 74u8, 115u8, 43u8, 188u8, 219u8, 121u8, 178u8, 70u8, 98u8,
							141u8, 10u8, 81u8, 104u8, 242u8, 213u8, 114u8, 137u8, 167u8, 26u8,
							137u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn max_string_size(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"CallFilter",
						"MaxStringSize",
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
	pub mod cosmwasm {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_cosmwasm::pallet::Error;
		pub type Call = runtime_types::pallet_cosmwasm::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Upload {
					pub code: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Upload {
					const PALLET: &'static str = "Cosmwasm";
					const CALL: &'static str = "upload";
				}
				pub struct Instantiate {
					pub code_identifier: runtime_types::pallet_cosmwasm::types::CodeIdentifier,
					pub salt: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub admin: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					pub label: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub funds:
						runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
							runtime_types::primitives::currency::CurrencyId,
							(::core::primitive::u128, ::core::primitive::bool),
						>,
					pub gas: ::core::primitive::u64,
					pub message: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Instantiate {
					const PALLET: &'static str = "Cosmwasm";
					const CALL: &'static str = "instantiate";
				}
				pub struct Execute {
					pub contract: runtime_types::sp_core::crypto::AccountId32,
					pub funds:
						runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
							runtime_types::primitives::currency::CurrencyId,
							(::core::primitive::u128, ::core::primitive::bool),
						>,
					pub gas: ::core::primitive::u64,
					pub message: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Execute {
					const PALLET: &'static str = "Cosmwasm";
					const CALL: &'static str = "execute";
				}
				pub struct Migrate {
					pub contract: runtime_types::sp_core::crypto::AccountId32,
					pub new_code_identifier: runtime_types::pallet_cosmwasm::types::CodeIdentifier,
					pub gas: ::core::primitive::u64,
					pub message: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Migrate {
					const PALLET: &'static str = "Cosmwasm";
					const CALL: &'static str = "migrate";
				}
				pub struct UpdateAdmin {
					pub contract: runtime_types::sp_core::crypto::AccountId32,
					pub new_admin:
						::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					pub gas: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateAdmin {
					const PALLET: &'static str = "Cosmwasm";
					const CALL: &'static str = "update_admin";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn upload(
					&self,
					code: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				) -> ::subxt::tx::Payload<types::Upload> {
					::subxt::tx::Payload::new_static(
						"Cosmwasm",
						"upload",
						types::Upload { code },
						[
							43u8, 113u8, 164u8, 68u8, 6u8, 139u8, 4u8, 132u8, 58u8, 177u8, 109u8,
							135u8, 151u8, 30u8, 123u8, 227u8, 200u8, 16u8, 220u8, 244u8, 60u8,
							191u8, 101u8, 245u8, 137u8, 207u8, 65u8, 51u8, 168u8, 64u8, 56u8,
							211u8,
						],
					)
				}
				pub fn instantiate(
					&self,
					code_identifier: runtime_types::pallet_cosmwasm::types::CodeIdentifier,
					salt: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					admin: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					label: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					funds: runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
						runtime_types::primitives::currency::CurrencyId,
						(::core::primitive::u128, ::core::primitive::bool),
					>,
					gas: ::core::primitive::u64,
					message: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				) -> ::subxt::tx::Payload<types::Instantiate> {
					::subxt::tx::Payload::new_static(
						"Cosmwasm",
						"instantiate",
						types::Instantiate {
							code_identifier,
							salt,
							admin,
							label,
							funds,
							gas,
							message,
						},
						[
							211u8, 172u8, 125u8, 248u8, 26u8, 251u8, 68u8, 141u8, 63u8, 195u8,
							74u8, 139u8, 236u8, 203u8, 78u8, 247u8, 77u8, 229u8, 72u8, 46u8, 89u8,
							211u8, 188u8, 99u8, 9u8, 8u8, 69u8, 222u8, 151u8, 78u8, 1u8, 3u8,
						],
					)
				}
				pub fn execute(
					&self,
					contract: runtime_types::sp_core::crypto::AccountId32,
					funds: runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
						runtime_types::primitives::currency::CurrencyId,
						(::core::primitive::u128, ::core::primitive::bool),
					>,
					gas: ::core::primitive::u64,
					message: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				) -> ::subxt::tx::Payload<types::Execute> {
					::subxt::tx::Payload::new_static(
						"Cosmwasm",
						"execute",
						types::Execute { contract, funds, gas, message },
						[
							2u8, 110u8, 187u8, 138u8, 217u8, 55u8, 15u8, 79u8, 38u8, 74u8, 121u8,
							125u8, 147u8, 26u8, 60u8, 249u8, 179u8, 225u8, 20u8, 6u8, 99u8, 221u8,
							209u8, 173u8, 245u8, 114u8, 129u8, 13u8, 249u8, 202u8, 214u8, 230u8,
						],
					)
				}
				pub fn migrate(
					&self,
					contract: runtime_types::sp_core::crypto::AccountId32,
					new_code_identifier: runtime_types::pallet_cosmwasm::types::CodeIdentifier,
					gas: ::core::primitive::u64,
					message: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				) -> ::subxt::tx::Payload<types::Migrate> {
					::subxt::tx::Payload::new_static(
						"Cosmwasm",
						"migrate",
						types::Migrate { contract, new_code_identifier, gas, message },
						[
							50u8, 211u8, 106u8, 160u8, 113u8, 105u8, 175u8, 8u8, 36u8, 178u8,
							190u8, 227u8, 192u8, 234u8, 80u8, 34u8, 119u8, 226u8, 238u8, 122u8,
							60u8, 183u8, 116u8, 44u8, 61u8, 166u8, 177u8, 3u8, 38u8, 14u8, 118u8,
							32u8,
						],
					)
				}
				pub fn update_admin(
					&self,
					contract: runtime_types::sp_core::crypto::AccountId32,
					new_admin: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					gas: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::UpdateAdmin> {
					::subxt::tx::Payload::new_static(
						"Cosmwasm",
						"update_admin",
						types::UpdateAdmin { contract, new_admin, gas },
						[
							122u8, 212u8, 225u8, 184u8, 12u8, 193u8, 159u8, 64u8, 4u8, 58u8, 198u8,
							245u8, 178u8, 129u8, 183u8, 104u8, 21u8, 204u8, 6u8, 94u8, 184u8, 97u8,
							175u8, 113u8, 24u8, 19u8, 33u8, 185u8, 140u8, 119u8, 232u8, 18u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_cosmwasm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct Uploaded {
				pub code_hash: [::core::primitive::u8; 32usize],
				pub code_id: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for Uploaded {
				const PALLET: &'static str = "Cosmwasm";
				const EVENT: &'static str = "Uploaded";
			}
			pub struct Instantiated {
				pub contract: runtime_types::sp_core::crypto::AccountId32,
				pub info: runtime_types::pallet_cosmwasm::types::ContractInfo<
					runtime_types::sp_core::crypto::AccountId32,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				>,
			}
			impl ::subxt::events::StaticEvent for Instantiated {
				const PALLET: &'static str = "Cosmwasm";
				const EVENT: &'static str = "Instantiated";
			}
			pub struct Executed {
				pub contract: runtime_types::sp_core::crypto::AccountId32,
				pub entrypoint: runtime_types::pallet_cosmwasm::types::EntryPoint,
				pub data: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "Cosmwasm";
				const EVENT: &'static str = "Executed";
			}
			pub struct ExecutionFailed {
				pub contract: runtime_types::sp_core::crypto::AccountId32,
				pub entrypoint: runtime_types::pallet_cosmwasm::types::EntryPoint,
				pub error: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ExecutionFailed {
				const PALLET: &'static str = "Cosmwasm";
				const EVENT: &'static str = "ExecutionFailed";
			}
			pub struct Emitted {
				pub contract: runtime_types::sp_core::crypto::AccountId32,
				pub ty: ::std::vec::Vec<::core::primitive::u8>,
				pub attributes: ::std::vec::Vec<(
					::std::vec::Vec<::core::primitive::u8>,
					::std::vec::Vec<::core::primitive::u8>,
				)>,
			}
			impl ::subxt::events::StaticEvent for Emitted {
				const PALLET: &'static str = "Cosmwasm";
				const EVENT: &'static str = "Emitted";
			}
			pub struct Migrated {
				pub contract: runtime_types::sp_core::crypto::AccountId32,
				pub to: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for Migrated {
				const PALLET: &'static str = "Cosmwasm";
				const EVENT: &'static str = "Migrated";
			}
			pub struct AdminUpdated {
				pub contract: runtime_types::sp_core::crypto::AccountId32,
				pub new_admin: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for AdminUpdated {
				const PALLET: &'static str = "Cosmwasm";
				const EVENT: &'static str = "AdminUpdated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn pristine_code(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"PristineCode",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							48u8, 203u8, 209u8, 177u8, 81u8, 120u8, 211u8, 53u8, 89u8, 15u8, 95u8,
							192u8, 19u8, 40u8, 247u8, 32u8, 176u8, 150u8, 176u8, 39u8, 163u8,
							249u8, 244u8, 227u8, 77u8, 202u8, 233u8, 6u8, 110u8, 92u8, 197u8,
							121u8,
						],
					)
				}
				pub fn pristine_code_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"PristineCode",
						Vec::new(),
						[
							48u8, 203u8, 209u8, 177u8, 81u8, 120u8, 211u8, 53u8, 89u8, 15u8, 95u8,
							192u8, 19u8, 40u8, 247u8, 32u8, 176u8, 150u8, 176u8, 39u8, 163u8,
							249u8, 244u8, 227u8, 77u8, 202u8, 233u8, 6u8, 110u8, 92u8, 197u8,
							121u8,
						],
					)
				}
				pub fn instrumented_code(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"InstrumentedCode",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							172u8, 78u8, 189u8, 23u8, 102u8, 30u8, 126u8, 53u8, 253u8, 49u8, 188u8,
							230u8, 144u8, 126u8, 70u8, 8u8, 126u8, 14u8, 107u8, 148u8, 120u8,
							220u8, 165u8, 188u8, 72u8, 108u8, 166u8, 55u8, 199u8, 249u8, 12u8,
							193u8,
						],
					)
				}
				pub fn instrumented_code_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"InstrumentedCode",
						Vec::new(),
						[
							172u8, 78u8, 189u8, 23u8, 102u8, 30u8, 126u8, 53u8, 253u8, 49u8, 188u8,
							230u8, 144u8, 126u8, 70u8, 8u8, 126u8, 14u8, 107u8, 148u8, 120u8,
							220u8, 165u8, 188u8, 72u8, 108u8, 166u8, 55u8, 199u8, 249u8, 12u8,
							193u8,
						],
					)
				}
				pub fn current_code_id(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"CurrentCodeId",
						vec![],
						[
							33u8, 61u8, 19u8, 82u8, 234u8, 123u8, 105u8, 200u8, 106u8, 178u8,
							178u8, 191u8, 130u8, 114u8, 67u8, 188u8, 222u8, 102u8, 29u8, 212u8,
							113u8, 200u8, 1u8, 100u8, 222u8, 142u8, 155u8, 111u8, 232u8, 185u8,
							49u8, 185u8,
						],
					)
				}
				pub fn code_id_to_info(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_cosmwasm::types::CodeInfo<
						runtime_types::sp_core::crypto::AccountId32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"CodeIdToInfo",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							116u8, 164u8, 16u8, 138u8, 229u8, 50u8, 235u8, 244u8, 122u8, 162u8,
							222u8, 202u8, 246u8, 222u8, 29u8, 53u8, 48u8, 255u8, 168u8, 120u8, 2u8,
							69u8, 112u8, 25u8, 164u8, 200u8, 0u8, 137u8, 93u8, 155u8, 86u8, 145u8,
						],
					)
				}
				pub fn code_id_to_info_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_cosmwasm::types::CodeInfo<
						runtime_types::sp_core::crypto::AccountId32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"CodeIdToInfo",
						Vec::new(),
						[
							116u8, 164u8, 16u8, 138u8, 229u8, 50u8, 235u8, 244u8, 122u8, 162u8,
							222u8, 202u8, 246u8, 222u8, 29u8, 53u8, 48u8, 255u8, 168u8, 120u8, 2u8,
							69u8, 112u8, 25u8, 164u8, 200u8, 0u8, 137u8, 93u8, 155u8, 86u8, 145u8,
						],
					)
				}
				pub fn code_hash_to_id(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"CodeHashToId",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							3u8, 166u8, 71u8, 157u8, 121u8, 173u8, 151u8, 24u8, 223u8, 18u8, 4u8,
							156u8, 175u8, 112u8, 235u8, 144u8, 173u8, 115u8, 221u8, 37u8, 221u8,
							126u8, 20u8, 68u8, 154u8, 43u8, 253u8, 246u8, 209u8, 33u8, 46u8, 32u8,
						],
					)
				}
				pub fn code_hash_to_id_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"CodeHashToId",
						Vec::new(),
						[
							3u8, 166u8, 71u8, 157u8, 121u8, 173u8, 151u8, 24u8, 223u8, 18u8, 4u8,
							156u8, 175u8, 112u8, 235u8, 144u8, 173u8, 115u8, 221u8, 37u8, 221u8,
							126u8, 20u8, 68u8, 154u8, 43u8, 253u8, 246u8, 209u8, 33u8, 46u8, 32u8,
						],
					)
				}
				pub fn current_nonce(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"CurrentNonce",
						vec![],
						[
							30u8, 65u8, 24u8, 145u8, 252u8, 147u8, 254u8, 86u8, 231u8, 139u8, 53u8,
							119u8, 201u8, 144u8, 60u8, 157u8, 121u8, 137u8, 193u8, 125u8, 155u8,
							37u8, 233u8, 243u8, 6u8, 176u8, 140u8, 109u8, 86u8, 114u8, 144u8, 92u8,
						],
					)
				}
				pub fn contract_to_info(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_cosmwasm::types::ContractInfo<
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"ContractToInfo",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							251u8, 58u8, 219u8, 92u8, 64u8, 234u8, 244u8, 114u8, 103u8, 55u8,
							100u8, 90u8, 111u8, 234u8, 56u8, 85u8, 95u8, 47u8, 105u8, 79u8, 82u8,
							158u8, 160u8, 238u8, 135u8, 86u8, 228u8, 105u8, 127u8, 75u8, 198u8,
							133u8,
						],
					)
				}
				pub fn contract_to_info_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_cosmwasm::types::ContractInfo<
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Cosmwasm",
						"ContractToInfo",
						Vec::new(),
						[
							251u8, 58u8, 219u8, 92u8, 64u8, 234u8, 244u8, 114u8, 103u8, 55u8,
							100u8, 90u8, 111u8, 234u8, 56u8, 85u8, 95u8, 47u8, 105u8, 79u8, 82u8,
							158u8, 160u8, 238u8, 135u8, 86u8, 228u8, 105u8, 127u8, 75u8, 198u8,
							133u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"PalletId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
				pub fn chain_id(&self) -> ::subxt::constants::Address<::std::string::String> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"ChainId",
						[
							251u8, 233u8, 211u8, 209u8, 5u8, 66u8, 94u8, 200u8, 148u8, 166u8,
							119u8, 200u8, 59u8, 180u8, 70u8, 77u8, 182u8, 127u8, 45u8, 65u8, 28u8,
							104u8, 253u8, 149u8, 167u8, 216u8, 2u8, 94u8, 39u8, 173u8, 198u8,
							219u8,
						],
					)
				}
				pub fn max_code_size(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"MaxCodeSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_instrumented_code_size(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"MaxInstrumentedCodeSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_message_size(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"MaxMessageSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_contract_label_size(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"MaxContractLabelSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_contract_trie_id_size(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"MaxContractTrieIdSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_instantiate_salt_size(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"MaxInstantiateSaltSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn max_funds_assets(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"MaxFundsAssets",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn code_table_size_limit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"CodeTableSizeLimit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn code_global_variable_limit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"CodeGlobalVariableLimit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn code_parameter_limit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"CodeParameterLimit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn code_branch_table_size_limit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"CodeBranchTableSizeLimit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn code_stack_limit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"CodeStackLimit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn code_storage_byte_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"CodeStorageByteDeposit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn contract_storage_byte_write_price(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"ContractStorageByteWritePrice",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn contract_storage_byte_read_price(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"ContractStorageByteReadPrice",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn wasm_cost_rules(
					&self,
				) -> ::subxt::constants::Address<
					runtime_types::pallet_cosmwasm::instrument::CostRules,
				> {
					::subxt::constants::Address::new_static(
						"Cosmwasm",
						"WasmCostRules",
						[
							114u8, 17u8, 73u8, 160u8, 170u8, 148u8, 78u8, 222u8, 1u8, 250u8, 229u8,
							154u8, 16u8, 171u8, 98u8, 121u8, 243u8, 164u8, 245u8, 38u8, 193u8,
							212u8, 208u8, 59u8, 64u8, 224u8, 211u8, 13u8, 40u8, 63u8, 29u8, 203u8,
						],
					)
				}
			}
		}
	}
	pub mod ibc {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_ibc::pallet::Error;
		pub type Call = runtime_types::pallet_ibc::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct Deliver {
					pub messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Deliver {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "deliver";
				}
				pub struct Transfer {
					pub params: runtime_types::pallet_ibc::TransferParams<
						runtime_types::sp_core::crypto::AccountId32,
					>,
					pub asset_id: runtime_types::primitives::currency::CurrencyId,
					pub amount: ::core::primitive::u128,
					pub memo: ::core::option::Option<::std::string::String>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Transfer {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "transfer";
				}
				pub struct UpgradeClient {
					pub params: runtime_types::pallet_ibc::UpgradeParams,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpgradeClient {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "upgrade_client";
				}
				pub struct FreezeClient {
					pub client_id: ::std::vec::Vec<::core::primitive::u8>,
					pub height: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for FreezeClient {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "freeze_client";
				}
				pub struct IncreaseCounters;
				impl ::subxt::blocks::StaticExtrinsic for IncreaseCounters {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "increase_counters";
				}
				pub struct AddChannelsToFeelessChannelList {
					pub source_channel: ::core::primitive::u64,
					pub destination_channel: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddChannelsToFeelessChannelList {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "add_channels_to_feeless_channel_list";
				}
				pub struct RemoveChannelsFromFeelessChannelList {
					pub source_channel: ::core::primitive::u64,
					pub destination_channel: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveChannelsFromFeelessChannelList {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "remove_channels_from_feeless_channel_list";
				}
				pub struct SetChildStorage {
					pub key: ::std::vec::Vec<::core::primitive::u8>,
					pub value: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetChildStorage {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "set_child_storage";
				}
				pub struct SubstituteClientState {
					pub client_id: ::std::string::String,
					pub height: runtime_types::ibc::core::ics02_client::height::Height,
					pub client_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
					pub consensus_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SubstituteClientState {
					const PALLET: &'static str = "Ibc";
					const CALL: &'static str = "substitute_client_state";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn deliver(
					&self,
					messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
				) -> ::subxt::tx::Payload<types::Deliver> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"deliver",
						types::Deliver { messages },
						[
							172u8, 108u8, 202u8, 218u8, 161u8, 9u8, 26u8, 231u8, 26u8, 10u8, 248u8,
							118u8, 238u8, 72u8, 137u8, 49u8, 125u8, 168u8, 175u8, 195u8, 152u8,
							13u8, 15u8, 109u8, 162u8, 50u8, 39u8, 76u8, 12u8, 121u8, 4u8, 88u8,
						],
					)
				}
				pub fn transfer(
					&self,
					params: runtime_types::pallet_ibc::TransferParams<
						runtime_types::sp_core::crypto::AccountId32,
					>,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
					memo: ::core::option::Option<::std::string::String>,
				) -> ::subxt::tx::Payload<types::Transfer> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"transfer",
						types::Transfer { params, asset_id, amount, memo },
						[
							109u8, 109u8, 251u8, 125u8, 36u8, 150u8, 155u8, 164u8, 195u8, 4u8,
							97u8, 109u8, 48u8, 204u8, 95u8, 26u8, 144u8, 207u8, 24u8, 164u8, 138u8,
							95u8, 109u8, 186u8, 139u8, 6u8, 174u8, 247u8, 51u8, 23u8, 145u8, 111u8,
						],
					)
				}
				pub fn upgrade_client(
					&self,
					params: runtime_types::pallet_ibc::UpgradeParams,
				) -> ::subxt::tx::Payload<types::UpgradeClient> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"upgrade_client",
						types::UpgradeClient { params },
						[
							253u8, 58u8, 242u8, 123u8, 79u8, 161u8, 224u8, 36u8, 91u8, 61u8, 168u8,
							14u8, 234u8, 105u8, 116u8, 173u8, 53u8, 152u8, 91u8, 94u8, 95u8, 210u8,
							86u8, 22u8, 164u8, 159u8, 10u8, 192u8, 174u8, 195u8, 207u8, 172u8,
						],
					)
				}
				pub fn freeze_client(
					&self,
					client_id: ::std::vec::Vec<::core::primitive::u8>,
					height: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::FreezeClient> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"freeze_client",
						types::FreezeClient { client_id, height },
						[
							113u8, 2u8, 129u8, 175u8, 26u8, 200u8, 192u8, 198u8, 33u8, 79u8, 157u8,
							230u8, 222u8, 140u8, 139u8, 83u8, 46u8, 104u8, 155u8, 252u8, 241u8,
							115u8, 83u8, 85u8, 255u8, 160u8, 27u8, 110u8, 216u8, 204u8, 36u8, 76u8,
						],
					)
				}
				pub fn increase_counters(&self) -> ::subxt::tx::Payload<types::IncreaseCounters> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"increase_counters",
						types::IncreaseCounters {},
						[
							23u8, 152u8, 106u8, 41u8, 249u8, 118u8, 33u8, 254u8, 129u8, 72u8, 33u8,
							238u8, 90u8, 101u8, 181u8, 145u8, 237u8, 244u8, 146u8, 226u8, 229u8,
							32u8, 140u8, 133u8, 218u8, 21u8, 207u8, 4u8, 76u8, 251u8, 237u8, 93u8,
						],
					)
				}
				pub fn add_channels_to_feeless_channel_list(
					&self,
					source_channel: ::core::primitive::u64,
					destination_channel: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::AddChannelsToFeelessChannelList> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"add_channels_to_feeless_channel_list",
						types::AddChannelsToFeelessChannelList {
							source_channel,
							destination_channel,
						},
						[
							240u8, 91u8, 98u8, 19u8, 68u8, 233u8, 245u8, 63u8, 201u8, 66u8, 237u8,
							30u8, 138u8, 115u8, 67u8, 200u8, 200u8, 120u8, 196u8, 39u8, 39u8, 48u8,
							235u8, 179u8, 16u8, 54u8, 207u8, 80u8, 174u8, 246u8, 0u8, 48u8,
						],
					)
				}
				pub fn remove_channels_from_feeless_channel_list(
					&self,
					source_channel: ::core::primitive::u64,
					destination_channel: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::RemoveChannelsFromFeelessChannelList> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"remove_channels_from_feeless_channel_list",
						types::RemoveChannelsFromFeelessChannelList {
							source_channel,
							destination_channel,
						},
						[
							187u8, 25u8, 10u8, 235u8, 7u8, 252u8, 97u8, 26u8, 161u8, 75u8, 217u8,
							214u8, 131u8, 112u8, 222u8, 79u8, 22u8, 255u8, 166u8, 176u8, 42u8,
							84u8, 23u8, 229u8, 154u8, 64u8, 87u8, 189u8, 131u8, 253u8, 26u8, 224u8,
						],
					)
				}
				pub fn set_child_storage(
					&self,
					key: ::std::vec::Vec<::core::primitive::u8>,
					value: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetChildStorage> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"set_child_storage",
						types::SetChildStorage { key, value },
						[
							87u8, 243u8, 160u8, 58u8, 82u8, 211u8, 1u8, 4u8, 253u8, 141u8, 120u8,
							60u8, 165u8, 48u8, 68u8, 239u8, 209u8, 68u8, 161u8, 133u8, 55u8, 198u8,
							24u8, 255u8, 102u8, 182u8, 203u8, 99u8, 103u8, 138u8, 226u8, 65u8,
						],
					)
				}
				pub fn substitute_client_state(
					&self,
					client_id: ::std::string::String,
					height: runtime_types::ibc::core::ics02_client::height::Height,
					client_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
					consensus_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SubstituteClientState> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"substitute_client_state",
						types::SubstituteClientState {
							client_id,
							height,
							client_state_bytes,
							consensus_state_bytes,
						},
						[
							12u8, 193u8, 205u8, 192u8, 147u8, 106u8, 20u8, 83u8, 109u8, 63u8,
							199u8, 252u8, 234u8, 179u8, 33u8, 59u8, 80u8, 152u8, 139u8, 33u8, 20u8,
							117u8, 67u8, 57u8, 39u8, 147u8, 224u8, 3u8, 192u8, 137u8, 101u8, 202u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_ibc::pallet::Event;
		pub mod events {
			use super::runtime_types;
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
			pub struct ChannelOpened {
				pub channel_id: ::std::vec::Vec<::core::primitive::u8>,
				pub port_id: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ChannelOpened {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChannelOpened";
			}
			pub struct ParamsUpdated {
				pub send_enabled: ::core::primitive::bool,
				pub receive_enabled: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for ParamsUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ParamsUpdated";
			}
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
			pub struct OnRecvPacketError {
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for OnRecvPacketError {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "OnRecvPacketError";
			}
			pub struct ClientUpgradeSet;
			impl ::subxt::events::StaticEvent for ClientUpgradeSet {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientUpgradeSet";
			}
			pub struct ClientFrozen {
				pub client_id: ::std::vec::Vec<::core::primitive::u8>,
				pub height: ::core::primitive::u64,
				pub revision_number: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ClientFrozen {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientFrozen";
			}
			pub struct AssetAdminUpdated {
				pub admin_account: runtime_types::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for AssetAdminUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "AssetAdminUpdated";
			}
			pub struct FeeLessChannelIdsAdded {
				pub source_channel: ::core::primitive::u64,
				pub destination_channel: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for FeeLessChannelIdsAdded {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "FeeLessChannelIdsAdded";
			}
			pub struct FeeLessChannelIdsRemoved {
				pub source_channel: ::core::primitive::u64,
				pub destination_channel: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for FeeLessChannelIdsRemoved {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "FeeLessChannelIdsRemoved";
			}
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
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct ChargingFeeConfirmed {
				pub sequence: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ChargingFeeConfirmed {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChargingFeeConfirmed";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct ChargingFeeTimeout {
				pub sequence: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ChargingFeeTimeout {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChargingFeeTimeout";
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct ChargingFeeFailedAcknowledgement {
				pub sequence: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ChargingFeeFailedAcknowledgement {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChargingFeeFailedAcknowledgement";
			}
			pub struct ChildStateUpdated;
			impl ::subxt::events::StaticEvent for ChildStateUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChildStateUpdated";
			}
			pub struct ClientStateSubstituted {
				pub client_id: ::std::string::String,
				pub height: runtime_types::ibc::core::ics02_client::height::Height,
			}
			impl ::subxt::events::StaticEvent for ClientStateSubstituted {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientStateSubstituted";
			}
			pub struct ExecuteMemoStarted {
				pub account_id: runtime_types::sp_core::crypto::AccountId32,
				pub memo: ::core::option::Option<::std::string::String>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoStarted {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoStarted";
			}
			pub struct ExecuteMemoIbcTokenTransferSuccess {
				pub from: runtime_types::sp_core::crypto::AccountId32,
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
			pub struct ExecuteMemoIbcTokenTransferFailedWithReason {
				pub from: runtime_types::sp_core::crypto::AccountId32,
				pub memo: ::std::string::String,
				pub reason: ::core::primitive::u8,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoIbcTokenTransferFailedWithReason {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoIbcTokenTransferFailedWithReason";
			}
			pub struct ExecuteMemoIbcTokenTransferFailed {
				pub from: runtime_types::sp_core::crypto::AccountId32,
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
			pub struct ExecuteMemoXcmSuccess {
				pub from: runtime_types::sp_core::crypto::AccountId32,
				pub to: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub para_id: ::core::option::Option<::core::primitive::u32>,
			}
			impl ::subxt::events::StaticEvent for ExecuteMemoXcmSuccess {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ExecuteMemoXcmSuccess";
			}
			pub struct ExecuteMemoXcmFailed {
				pub from: runtime_types::sp_core::crypto::AccountId32,
				pub to: runtime_types::sp_core::crypto::AccountId32,
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
							75u8, 243u8, 33u8, 182u8, 39u8, 208u8, 238u8, 74u8, 241u8, 105u8,
							206u8, 74u8, 15u8, 70u8, 35u8, 249u8, 115u8, 224u8, 16u8, 208u8, 95u8,
							92u8, 66u8, 193u8, 225u8, 33u8, 211u8, 162u8, 241u8, 233u8, 183u8,
							228u8,
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
							75u8, 243u8, 33u8, 182u8, 39u8, 208u8, 238u8, 74u8, 241u8, 105u8,
							206u8, 74u8, 15u8, 70u8, 35u8, 249u8, 115u8, 224u8, 16u8, 208u8, 95u8,
							92u8, 66u8, 193u8, 225u8, 33u8, 211u8, 162u8, 241u8, 233u8, 183u8,
							228u8,
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
							131u8, 102u8, 75u8, 64u8, 161u8, 69u8, 217u8, 111u8, 153u8, 9u8, 32u8,
							20u8, 79u8, 224u8, 110u8, 11u8, 215u8, 10u8, 235u8, 83u8, 154u8, 125u8,
							69u8, 205u8, 62u8, 167u8, 110u8, 18u8, 36u8, 95u8, 143u8, 58u8,
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
							92u8, 178u8, 20u8, 229u8, 210u8, 238u8, 5u8, 76u8, 17u8, 219u8, 134u8,
							109u8, 11u8, 221u8, 108u8, 177u8, 192u8, 25u8, 225u8, 197u8, 63u8,
							50u8, 204u8, 38u8, 191u8, 15u8, 98u8, 1u8, 78u8, 87u8, 42u8, 62u8,
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
							92u8, 178u8, 20u8, 229u8, 210u8, 238u8, 5u8, 76u8, 17u8, 219u8, 134u8,
							109u8, 11u8, 221u8, 108u8, 177u8, 192u8, 25u8, 225u8, 197u8, 63u8,
							50u8, 204u8, 38u8, 191u8, 15u8, 98u8, 1u8, 78u8, 87u8, 42u8, 62u8,
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
							61u8, 228u8, 58u8, 117u8, 65u8, 31u8, 204u8, 92u8, 12u8, 247u8, 4u8,
							176u8, 121u8, 146u8, 112u8, 118u8, 201u8, 138u8, 154u8, 6u8, 4u8,
							201u8, 218u8, 64u8, 250u8, 201u8, 50u8, 148u8, 87u8, 172u8, 215u8,
							152u8,
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
							6u8, 61u8, 72u8, 154u8, 126u8, 33u8, 9u8, 44u8, 64u8, 112u8, 73u8,
							229u8, 28u8, 106u8, 109u8, 206u8, 11u8, 152u8, 124u8, 157u8, 252u8,
							28u8, 153u8, 248u8, 16u8, 60u8, 33u8, 198u8, 78u8, 254u8, 10u8, 244u8,
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
							240u8, 108u8, 181u8, 37u8, 88u8, 248u8, 73u8, 155u8, 48u8, 181u8,
							152u8, 243u8, 109u8, 79u8, 129u8, 65u8, 243u8, 50u8, 10u8, 130u8,
							171u8, 201u8, 35u8, 141u8, 208u8, 69u8, 234u8, 105u8, 202u8, 15u8,
							217u8, 148u8,
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
							240u8, 108u8, 181u8, 37u8, 88u8, 248u8, 73u8, 155u8, 48u8, 181u8,
							152u8, 243u8, 109u8, 79u8, 129u8, 65u8, 243u8, 50u8, 10u8, 130u8,
							171u8, 201u8, 35u8, 141u8, 208u8, 69u8, 234u8, 105u8, 202u8, 15u8,
							217u8, 148u8,
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
							122u8, 208u8, 54u8, 30u8, 164u8, 139u8, 128u8, 106u8, 214u8, 8u8,
							106u8, 105u8, 95u8, 140u8, 17u8, 72u8, 88u8, 252u8, 41u8, 69u8, 101u8,
							65u8, 5u8, 94u8, 247u8, 244u8, 19u8, 208u8, 160u8, 159u8, 209u8, 15u8,
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
							122u8, 208u8, 54u8, 30u8, 164u8, 139u8, 128u8, 106u8, 214u8, 8u8,
							106u8, 105u8, 95u8, 140u8, 17u8, 72u8, 88u8, 252u8, 41u8, 69u8, 101u8,
							65u8, 5u8, 94u8, 247u8, 244u8, 19u8, 208u8, 160u8, 159u8, 209u8, 15u8,
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
							66u8, 41u8, 5u8, 19u8, 126u8, 113u8, 10u8, 96u8, 77u8, 206u8, 105u8,
							119u8, 8u8, 92u8, 211u8, 69u8, 216u8, 169u8, 91u8, 108u8, 227u8, 237u8,
							196u8, 25u8, 18u8, 68u8, 253u8, 253u8, 21u8, 205u8, 3u8, 184u8,
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
							66u8, 41u8, 5u8, 19u8, 126u8, 113u8, 10u8, 96u8, 77u8, 206u8, 105u8,
							119u8, 8u8, 92u8, 211u8, 69u8, 216u8, 169u8, 91u8, 108u8, 227u8, 237u8,
							196u8, 25u8, 18u8, 68u8, 253u8, 253u8, 21u8, 205u8, 3u8, 184u8,
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
							25u8, 194u8, 154u8, 231u8, 229u8, 240u8, 25u8, 94u8, 46u8, 3u8, 137u8,
							130u8, 211u8, 146u8, 87u8, 105u8, 17u8, 95u8, 56u8, 55u8, 184u8, 245u8,
							170u8, 189u8, 108u8, 17u8, 167u8, 134u8, 161u8, 89u8, 168u8, 243u8,
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
							244u8, 243u8, 169u8, 141u8, 145u8, 248u8, 15u8, 50u8, 150u8, 185u8,
							137u8, 105u8, 14u8, 154u8, 199u8, 20u8, 160u8, 78u8, 64u8, 36u8, 14u8,
							36u8, 46u8, 219u8, 114u8, 164u8, 64u8, 178u8, 1u8, 152u8, 80u8, 33u8,
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
							202u8, 105u8, 177u8, 209u8, 89u8, 60u8, 15u8, 21u8, 193u8, 73u8, 105u8,
							98u8, 195u8, 143u8, 40u8, 194u8, 50u8, 117u8, 157u8, 7u8, 45u8, 115u8,
							179u8, 143u8, 180u8, 121u8, 208u8, 171u8, 88u8, 133u8, 232u8, 84u8,
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
							67u8, 12u8, 175u8, 101u8, 87u8, 155u8, 170u8, 123u8, 89u8, 82u8, 227u8,
							122u8, 221u8, 201u8, 185u8, 186u8, 236u8, 193u8, 153u8, 59u8, 195u8,
							145u8, 19u8, 56u8, 254u8, 51u8, 130u8, 110u8, 167u8, 148u8, 210u8, 1u8,
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
							59u8, 1u8, 63u8, 9u8, 154u8, 217u8, 113u8, 42u8, 171u8, 155u8, 101u8,
							69u8, 209u8, 57u8, 6u8, 4u8, 8u8, 36u8, 221u8, 81u8, 101u8, 125u8,
							235u8, 250u8, 193u8, 43u8, 121u8, 151u8, 132u8, 127u8, 211u8, 226u8,
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
							59u8, 1u8, 63u8, 9u8, 154u8, 217u8, 113u8, 42u8, 171u8, 155u8, 101u8,
							69u8, 209u8, 57u8, 6u8, 4u8, 8u8, 36u8, 221u8, 81u8, 101u8, 125u8,
							235u8, 250u8, 193u8, 43u8, 121u8, 151u8, 132u8, 127u8, 211u8, 226u8,
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
							199u8, 124u8, 153u8, 65u8, 98u8, 208u8, 199u8, 109u8, 197u8, 21u8,
							186u8, 105u8, 203u8, 60u8, 251u8, 207u8, 121u8, 143u8, 119u8, 222u8,
							185u8, 203u8, 65u8, 253u8, 91u8, 145u8, 161u8, 228u8, 176u8, 116u8,
							173u8, 59u8,
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
							199u8, 124u8, 153u8, 65u8, 98u8, 208u8, 199u8, 109u8, 197u8, 21u8,
							186u8, 105u8, 203u8, 60u8, 251u8, 207u8, 121u8, 143u8, 119u8, 222u8,
							185u8, 203u8, 65u8, 253u8, 91u8, 145u8, 161u8, 228u8, 176u8, 116u8,
							173u8, 59u8,
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
							90u8, 132u8, 19u8, 129u8, 205u8, 73u8, 172u8, 84u8, 103u8, 136u8,
							108u8, 4u8, 28u8, 154u8, 26u8, 233u8, 237u8, 49u8, 179u8, 163u8, 27u8,
							207u8, 33u8, 151u8, 43u8, 149u8, 169u8, 224u8, 169u8, 185u8, 174u8,
							163u8,
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
							222u8, 110u8, 15u8, 247u8, 212u8, 125u8, 229u8, 50u8, 206u8, 243u8,
							23u8, 57u8, 212u8, 143u8, 131u8, 210u8, 241u8, 186u8, 239u8, 156u8,
							33u8, 176u8, 12u8, 149u8, 136u8, 178u8, 54u8, 34u8, 172u8, 20u8, 32u8,
							252u8,
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
							222u8, 110u8, 15u8, 247u8, 212u8, 125u8, 229u8, 50u8, 206u8, 243u8,
							23u8, 57u8, 212u8, 143u8, 131u8, 210u8, 241u8, 186u8, 239u8, 156u8,
							33u8, 176u8, 12u8, 149u8, 136u8, 178u8, 54u8, 34u8, 172u8, 20u8, 32u8,
							252u8,
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
							173u8, 80u8, 18u8, 126u8, 90u8, 140u8, 11u8, 142u8, 44u8, 11u8, 237u8,
							176u8, 99u8, 234u8, 76u8, 153u8, 72u8, 128u8, 135u8, 74u8, 200u8,
							125u8, 235u8, 251u8, 82u8, 40u8, 108u8, 251u8, 19u8, 100u8, 190u8,
							129u8,
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
							38u8, 162u8, 117u8, 100u8, 229u8, 142u8, 242u8, 166u8, 44u8, 113u8,
							65u8, 252u8, 7u8, 122u8, 116u8, 201u8, 12u8, 46u8, 231u8, 186u8, 254u8,
							172u8, 232u8, 237u8, 81u8, 13u8, 20u8, 156u8, 229u8, 38u8, 221u8, 89u8,
						],
					)
				}
				pub fn escrow_addresses(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::collections::BTreeSet<runtime_types::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"EscrowAddresses",
						vec![],
						[
							67u8, 10u8, 57u8, 221u8, 25u8, 4u8, 126u8, 69u8, 202u8, 103u8, 147u8,
							128u8, 228u8, 191u8, 222u8, 129u8, 252u8, 40u8, 103u8, 233u8, 145u8,
							178u8, 123u8, 64u8, 220u8, 165u8, 16u8, 239u8, 241u8, 109u8, 42u8,
							52u8,
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
							78u8, 76u8, 14u8, 119u8, 81u8, 198u8, 83u8, 244u8, 198u8, 220u8, 27u8,
							154u8, 45u8, 218u8, 225u8, 169u8, 178u8, 199u8, 62u8, 159u8, 162u8,
							108u8, 209u8, 40u8, 50u8, 94u8, 149u8, 20u8, 91u8, 30u8, 57u8, 86u8,
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
							78u8, 76u8, 14u8, 119u8, 81u8, 198u8, 83u8, 244u8, 198u8, 220u8, 27u8,
							154u8, 45u8, 218u8, 225u8, 169u8, 178u8, 199u8, 62u8, 159u8, 162u8,
							108u8, 209u8, 40u8, 50u8, 94u8, 149u8, 20u8, 91u8, 30u8, 57u8, 86u8,
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
							229u8, 165u8, 52u8, 91u8, 187u8, 130u8, 135u8, 118u8, 16u8, 183u8,
							26u8, 87u8, 232u8, 246u8, 206u8, 235u8, 110u8, 49u8, 147u8, 98u8,
							243u8, 196u8, 58u8, 18u8, 119u8, 188u8, 107u8, 100u8, 229u8, 77u8,
							34u8, 63u8,
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
							229u8, 165u8, 52u8, 91u8, 187u8, 130u8, 135u8, 118u8, 16u8, 183u8,
							26u8, 87u8, 232u8, 246u8, 206u8, 235u8, 110u8, 49u8, 147u8, 98u8,
							243u8, 196u8, 58u8, 18u8, 119u8, 188u8, 107u8, 100u8, 229u8, 77u8,
							34u8, 63u8,
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
							146u8, 31u8, 122u8, 162u8, 18u8, 81u8, 182u8, 243u8, 179u8, 89u8,
							226u8, 193u8, 25u8, 119u8, 67u8, 53u8, 249u8, 152u8, 160u8, 37u8,
							124u8, 150u8, 69u8, 148u8, 139u8, 201u8, 124u8, 183u8, 208u8, 128u8,
							222u8, 170u8,
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
							146u8, 31u8, 122u8, 162u8, 18u8, 81u8, 182u8, 243u8, 179u8, 89u8,
							226u8, 193u8, 25u8, 119u8, 67u8, 53u8, 249u8, 152u8, 160u8, 37u8,
							124u8, 150u8, 69u8, 148u8, 139u8, 201u8, 124u8, 183u8, 208u8, 128u8,
							222u8, 170u8,
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
							232u8, 191u8, 27u8, 246u8, 214u8, 78u8, 192u8, 81u8, 212u8, 124u8,
							251u8, 5u8, 25u8, 211u8, 80u8, 180u8, 218u8, 247u8, 243u8, 105u8,
							198u8, 97u8, 226u8, 120u8, 90u8, 41u8, 79u8, 63u8, 84u8, 217u8, 18u8,
							91u8,
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
							232u8, 191u8, 27u8, 246u8, 214u8, 78u8, 192u8, 81u8, 212u8, 124u8,
							251u8, 5u8, 25u8, 211u8, 80u8, 180u8, 218u8, 247u8, 243u8, 105u8,
							198u8, 97u8, 226u8, 120u8, 90u8, 41u8, 79u8, 63u8, 84u8, 217u8, 18u8,
							91u8,
						],
					)
				}
				pub fn pending_send_packet_seqs(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::collections::BTreeSet<::core::primitive::u64>, ::core::primitive::u64),
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
							63u8, 133u8, 125u8, 186u8, 127u8, 8u8, 57u8, 4u8, 103u8, 189u8, 27u8,
							160u8, 188u8, 175u8, 146u8, 0u8, 231u8, 211u8, 75u8, 73u8, 218u8,
							159u8, 94u8, 87u8, 77u8, 33u8, 222u8, 119u8, 79u8, 91u8, 54u8, 35u8,
						],
					)
				}
				pub fn pending_send_packet_seqs_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::collections::BTreeSet<::core::primitive::u64>, ::core::primitive::u64),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PendingSendPacketSeqs",
						Vec::new(),
						[
							63u8, 133u8, 125u8, 186u8, 127u8, 8u8, 57u8, 4u8, 103u8, 189u8, 27u8,
							160u8, 188u8, 175u8, 146u8, 0u8, 231u8, 211u8, 75u8, 73u8, 218u8,
							159u8, 94u8, 87u8, 77u8, 33u8, 222u8, 119u8, 79u8, 91u8, 54u8, 35u8,
						],
					)
				}
				pub fn pending_recv_packet_seqs(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::collections::BTreeSet<::core::primitive::u64>, ::core::primitive::u64),
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
							139u8, 134u8, 44u8, 28u8, 186u8, 252u8, 54u8, 60u8, 193u8, 148u8,
							153u8, 185u8, 85u8, 124u8, 145u8, 57u8, 63u8, 46u8, 233u8, 39u8, 177u8,
							164u8, 13u8, 227u8, 113u8, 54u8, 102u8, 36u8, 152u8, 186u8, 53u8,
							102u8,
						],
					)
				}
				pub fn pending_recv_packet_seqs_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::std::collections::BTreeSet<::core::primitive::u64>, ::core::primitive::u64),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Ibc",
						"PendingRecvPacketSeqs",
						Vec::new(),
						[
							139u8, 134u8, 44u8, 28u8, 186u8, 252u8, 54u8, 60u8, 193u8, 148u8,
							153u8, 185u8, 85u8, 124u8, 145u8, 57u8, 63u8, 46u8, 233u8, 39u8, 177u8,
							164u8, 13u8, 227u8, 113u8, 54u8, 102u8, 36u8, 152u8, 186u8, 53u8,
							102u8,
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
							62u8, 145u8, 102u8, 227u8, 159u8, 92u8, 27u8, 54u8, 159u8, 228u8,
							193u8, 99u8, 75u8, 196u8, 26u8, 250u8, 229u8, 230u8, 88u8, 109u8,
							246u8, 100u8, 152u8, 158u8, 14u8, 25u8, 224u8, 173u8, 224u8, 41u8,
							105u8, 231u8,
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
							64u8, 190u8, 244u8, 122u8, 87u8, 182u8, 217u8, 16u8, 55u8, 223u8,
							128u8, 6u8, 112u8, 30u8, 236u8, 222u8, 153u8, 53u8, 247u8, 102u8,
							196u8, 31u8, 6u8, 186u8, 251u8, 209u8, 114u8, 125u8, 213u8, 222u8,
							240u8, 8u8,
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
							240u8, 114u8, 208u8, 235u8, 143u8, 121u8, 26u8, 220u8, 3u8, 235u8,
							204u8, 181u8, 246u8, 38u8, 3u8, 168u8, 135u8, 78u8, 63u8, 128u8, 219u8,
							159u8, 37u8, 244u8, 202u8, 191u8, 202u8, 74u8, 203u8, 26u8, 44u8,
							132u8,
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
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
			}
		}
	}
	pub mod ics20_fee {
		use super::{root_mod, runtime_types};
		pub type Call = runtime_types::pallet_ibc::ics20_fee::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct SetCharge {
					pub charge: runtime_types::sp_arithmetic::per_things::Perbill,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCharge {
					const PALLET: &'static str = "Ics20Fee";
					const CALL: &'static str = "set_charge";
				}
				pub struct AddChannelsToFeelessChannelList {
					pub source_channel: ::core::primitive::u64,
					pub destination_channel: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddChannelsToFeelessChannelList {
					const PALLET: &'static str = "Ics20Fee";
					const CALL: &'static str = "add_channels_to_feeless_channel_list";
				}
				pub struct RemoveChannelsFromFeelessChannelList {
					pub source_channel: ::core::primitive::u64,
					pub destination_channel: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveChannelsFromFeelessChannelList {
					const PALLET: &'static str = "Ics20Fee";
					const CALL: &'static str = "remove_channels_from_feeless_channel_list";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_charge(
					&self,
					charge: runtime_types::sp_arithmetic::per_things::Perbill,
				) -> ::subxt::tx::Payload<types::SetCharge> {
					::subxt::tx::Payload::new_static(
						"Ics20Fee",
						"set_charge",
						types::SetCharge { charge },
						[
							61u8, 218u8, 232u8, 138u8, 52u8, 53u8, 69u8, 37u8, 54u8, 235u8, 127u8,
							163u8, 51u8, 167u8, 165u8, 183u8, 137u8, 10u8, 15u8, 238u8, 207u8,
							226u8, 173u8, 177u8, 178u8, 161u8, 97u8, 130u8, 17u8, 30u8, 1u8, 229u8,
						],
					)
				}
				pub fn add_channels_to_feeless_channel_list(
					&self,
					source_channel: ::core::primitive::u64,
					destination_channel: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::AddChannelsToFeelessChannelList> {
					::subxt::tx::Payload::new_static(
						"Ics20Fee",
						"add_channels_to_feeless_channel_list",
						types::AddChannelsToFeelessChannelList {
							source_channel,
							destination_channel,
						},
						[
							240u8, 91u8, 98u8, 19u8, 68u8, 233u8, 245u8, 63u8, 201u8, 66u8, 237u8,
							30u8, 138u8, 115u8, 67u8, 200u8, 200u8, 120u8, 196u8, 39u8, 39u8, 48u8,
							235u8, 179u8, 16u8, 54u8, 207u8, 80u8, 174u8, 246u8, 0u8, 48u8,
						],
					)
				}
				pub fn remove_channels_from_feeless_channel_list(
					&self,
					source_channel: ::core::primitive::u64,
					destination_channel: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::RemoveChannelsFromFeelessChannelList> {
					::subxt::tx::Payload::new_static(
						"Ics20Fee",
						"remove_channels_from_feeless_channel_list",
						types::RemoveChannelsFromFeelessChannelList {
							source_channel,
							destination_channel,
						},
						[
							187u8, 25u8, 10u8, 235u8, 7u8, 252u8, 97u8, 26u8, 161u8, 75u8, 217u8,
							214u8, 131u8, 112u8, 222u8, 79u8, 22u8, 255u8, 166u8, 176u8, 42u8,
							84u8, 23u8, 229u8, 154u8, 64u8, 87u8, 189u8, 131u8, 253u8, 26u8, 224u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_ibc::ics20_fee::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct IbcTransferFeeCollected {
				pub amount: ::core::primitive::u128,
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			impl ::subxt::events::StaticEvent for IbcTransferFeeCollected {
				const PALLET: &'static str = "Ics20Fee";
				const EVENT: &'static str = "IbcTransferFeeCollected";
			}
			pub struct FeeLessChannelIdsAdded {
				pub source_channel: ::core::primitive::u64,
				pub destination_channel: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for FeeLessChannelIdsAdded {
				const PALLET: &'static str = "Ics20Fee";
				const EVENT: &'static str = "FeeLessChannelIdsAdded";
			}
			pub struct FeeLessChannelIdsRemoved {
				pub source_channel: ::core::primitive::u64,
				pub destination_channel: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for FeeLessChannelIdsRemoved {
				const PALLET: &'static str = "Ics20Fee";
				const EVENT: &'static str = "FeeLessChannelIdsRemoved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn service_charge_in(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Ics20Fee",
						"ServiceChargeIn",
						vec![],
						[
							129u8, 39u8, 68u8, 5u8, 178u8, 99u8, 132u8, 149u8, 147u8, 107u8, 68u8,
							29u8, 176u8, 135u8, 1u8, 159u8, 190u8, 237u8, 73u8, 213u8, 66u8, 173u8,
							48u8, 55u8, 181u8, 238u8, 14u8, 30u8, 48u8, 229u8, 247u8, 96u8,
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
						"Ics20Fee",
						"FeeLessChannelIds",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							122u8, 208u8, 54u8, 30u8, 164u8, 139u8, 128u8, 106u8, 214u8, 8u8,
							106u8, 105u8, 95u8, 140u8, 17u8, 72u8, 88u8, 252u8, 41u8, 69u8, 101u8,
							65u8, 5u8, 94u8, 247u8, 244u8, 19u8, 208u8, 160u8, 159u8, 209u8, 15u8,
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
						"Ics20Fee",
						"FeeLessChannelIds",
						Vec::new(),
						[
							122u8, 208u8, 54u8, 30u8, 164u8, 139u8, 128u8, 106u8, 214u8, 8u8,
							106u8, 105u8, 95u8, 140u8, 17u8, 72u8, 88u8, 252u8, 41u8, 69u8, 101u8,
							65u8, 5u8, 94u8, 247u8, 244u8, 19u8, 208u8, 160u8, 159u8, 209u8, 15u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn service_charge_in(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Perbill>
				{
					::subxt::constants::Address::new_static(
						"Ics20Fee",
						"ServiceChargeIn",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"Ics20Fee",
						"PalletId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
			}
		}
	}
	pub mod pallet_multihop_xcm_ibc {
		use super::{root_mod, runtime_types};
		pub type Error = runtime_types::pallet_multihop_xcm_ibc::pallet::Error;
		pub type Call = runtime_types::pallet_multihop_xcm_ibc::pallet::Call;
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				pub struct AddRoute {
					pub route_id: ::core::primitive::u128,
					pub route: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::composable_traits::xcm::memo::ChainInfo,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					)>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddRoute {
					const PALLET: &'static str = "PalletMultihopXcmIbc";
					const CALL: &'static str = "add_route";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_route(
					&self,
					route_id: ::core::primitive::u128,
					route: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::composable_traits::xcm::memo::ChainInfo,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					)>,
				) -> ::subxt::tx::Payload<types::AddRoute> {
					::subxt::tx::Payload::new_static(
						"PalletMultihopXcmIbc",
						"add_route",
						types::AddRoute { route_id, route },
						[
							176u8, 90u8, 147u8, 13u8, 137u8, 218u8, 188u8, 154u8, 227u8, 186u8,
							4u8, 104u8, 241u8, 140u8, 221u8, 14u8, 60u8, 239u8, 186u8, 117u8, 65u8,
							142u8, 181u8, 224u8, 77u8, 127u8, 169u8, 76u8, 42u8, 227u8, 13u8, 22u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_multihop_xcm_ibc::pallet::Event;
		pub mod events {
			use super::runtime_types;
			pub struct SuccessXcmToIbc {
				pub origin_address: runtime_types::sp_core::crypto::AccountId32,
				pub to: [::core::primitive::u8; 32usize],
				pub amount: ::core::primitive::u128,
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub memo: ::core::option::Option<::std::string::String>,
			}
			impl ::subxt::events::StaticEvent for SuccessXcmToIbc {
				const PALLET: &'static str = "PalletMultihopXcmIbc";
				const EVENT: &'static str = "SuccessXcmToIbc";
			}
			pub struct FailedXcmToIbc {
				pub origin_address: runtime_types::sp_core::crypto::AccountId32,
				pub to: [::core::primitive::u8; 32usize],
				pub amount: ::core::primitive::u128,
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub memo: ::core::option::Option<::std::string::String>,
			}
			impl ::subxt::events::StaticEvent for FailedXcmToIbc {
				const PALLET: &'static str = "PalletMultihopXcmIbc";
				const EVENT: &'static str = "FailedXcmToIbc";
			}
			pub struct FailedCallback {
				pub origin_address: [::core::primitive::u8; 32usize],
				pub route_id: ::core::primitive::u128,
				pub reason: runtime_types::pallet_multihop_xcm_ibc::pallet::MultihopEventReason,
			}
			impl ::subxt::events::StaticEvent for FailedCallback {
				const PALLET: &'static str = "PalletMultihopXcmIbc";
				const EVENT: &'static str = "FailedCallback";
			}
			pub struct MultihopXcmMemo {
				pub reason: runtime_types::pallet_multihop_xcm_ibc::pallet::MultihopEventReason,
				pub from: runtime_types::sp_core::crypto::AccountId32,
				pub to: runtime_types::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
				pub asset_id: ::core::primitive::u128,
				pub is_error: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for MultihopXcmMemo {
				const PALLET: &'static str = "PalletMultihopXcmIbc";
				const EVENT: &'static str = "MultihopXcmMemo";
			}
			pub struct FailedMatchLocation;
			impl ::subxt::events::StaticEvent for FailedMatchLocation {
				const PALLET: &'static str = "PalletMultihopXcmIbc";
				const EVENT: &'static str = "FailedMatchLocation";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn route_id_to_route_path(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u128>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::composable_traits::xcm::memo::ChainInfo,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PalletMultihopXcmIbc",
						"RouteIdToRoutePath",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							187u8, 109u8, 156u8, 105u8, 159u8, 81u8, 244u8, 227u8, 179u8, 18u8,
							67u8, 254u8, 10u8, 221u8, 192u8, 185u8, 56u8, 4u8, 52u8, 190u8, 137u8,
							115u8, 23u8, 198u8, 84u8, 79u8, 103u8, 179u8, 7u8, 218u8, 179u8, 55u8,
						],
					)
				}
				pub fn route_id_to_route_path_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::composable_traits::xcm::memo::ChainInfo,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PalletMultihopXcmIbc",
						"RouteIdToRoutePath",
						Vec::new(),
						[
							187u8, 109u8, 156u8, 105u8, 159u8, 81u8, 244u8, 227u8, 179u8, 18u8,
							67u8, 254u8, 10u8, 221u8, 192u8, 185u8, 56u8, 4u8, 52u8, 190u8, 137u8,
							115u8, 23u8, 198u8, 84u8, 79u8, 103u8, 179u8, 7u8, 218u8, 179u8, 55u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn pallet_instance_id(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u8> {
					::subxt::constants::Address::new_static(
						"PalletMultihopXcmIbc",
						"PalletInstanceId",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
							28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
							114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
							165u8,
						],
					)
				}
				pub fn max_multihop_count(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"PalletMultihopXcmIbc",
						"MaxMultihopCount",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn chain_name_vec_limit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"PalletMultihopXcmIbc",
						"ChainNameVecLimit",
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
	pub mod runtime_types {
		use super::runtime_types;
		pub mod bounded_collections {
			use super::runtime_types;
			pub mod bounded_btree_map {
				use super::runtime_types;
				pub struct BoundedBTreeMap<_0, _1>(pub ::std::collections::BTreeMap<_0, _1>);
			}
			pub mod bounded_btree_set {
				use super::runtime_types;
				pub struct BoundedBTreeSet<_0>(pub ::std::collections::BTreeSet<_0>);
			}
			pub mod bounded_vec {
				use super::runtime_types;
				pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
			pub mod weak_bounded_vec {
				use super::runtime_types;
				pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
		}
		pub mod common {
			use super::runtime_types;
			pub struct MaxStringSize;
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
							pub struct BiBoundedVec<_0> {
								pub inner: ::std::vec::Vec<_0>,
							}
						}
					}
				}
			}
			pub mod types {
				use super::runtime_types;
				pub struct EcdsaSignature(pub [::core::primitive::u8; 65usize]);
				pub struct EthereumAddress(pub [::core::primitive::u8; 20usize]);
			}
		}
		pub mod composable_traits {
			use super::runtime_types;
			pub mod account_proxy {
				use super::runtime_types;
				pub enum ProxyType {
					#[codec::codec(index = 0)]
					Any,
					#[codec::codec(index = 1)]
					Governance,
					#[codec::codec(index = 2)]
					CancelProxy,
					#[codec::codec(index = 3)]
					Bridge,
					#[codec::codec(index = 4)]
					Assets,
					#[codec::codec(index = 5)]
					Defi,
					#[codec::codec(index = 6)]
					Oracle,
					#[codec::codec(index = 7)]
					Contracts,
				}
			}
			pub mod assets {
				use super::runtime_types;
				pub struct AssetInfo < _0 > { pub name : :: core :: option :: Option < runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > > , pub symbol : :: core :: option :: Option < runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > > , pub decimals : :: core :: option :: Option < :: core :: primitive :: u8 > , pub existential_deposit : _0 , pub ratio : :: core :: option :: Option < runtime_types :: composable_traits :: currency :: Rational64 > , }
				pub struct AssetInfoUpdate < _0 > { pub name : runtime_types :: composable_traits :: storage :: UpdateValue < :: core :: option :: Option < runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > > > , pub symbol : runtime_types :: composable_traits :: storage :: UpdateValue < :: core :: option :: Option < runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > > > , pub decimals : runtime_types :: composable_traits :: storage :: UpdateValue < :: core :: option :: Option < :: core :: primitive :: u8 > > , pub existential_deposit : runtime_types :: composable_traits :: storage :: UpdateValue < _0 > , pub ratio : runtime_types :: composable_traits :: storage :: UpdateValue < :: core :: option :: Option < runtime_types :: composable_traits :: currency :: Rational64 > > , }
				pub struct BasicAssetMetadata { pub symbol : runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , pub name : runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , }
			}
			pub mod bonded_finance {
				use super::runtime_types;
				pub enum BondDuration<_0> {
					#[codec::codec(index = 0)]
					Finite { return_in: _0 },
					#[codec::codec(index = 1)]
					Infinite,
				}
				pub struct BondOffer<_0, _1, _2, _3> {
					pub beneficiary: _0,
					pub asset: _1,
					pub bond_price: _2,
					pub nb_of_bonds: _2,
					pub maturity:
						runtime_types::composable_traits::bonded_finance::BondDuration<_3>,
					pub reward: runtime_types::composable_traits::bonded_finance::BondOfferReward<
						_1,
						_2,
						_3,
					>,
				}
				pub struct BondOfferReward<_0, _1, _2> {
					pub asset: _0,
					pub amount: _1,
					pub maturity: _2,
				}
			}
			pub mod currency {
				use super::runtime_types;
				pub struct Rational64 {
					pub n: ::core::primitive::u64,
					pub d: ::core::primitive::u64,
				}
			}
			pub mod dex {
				use super::runtime_types;
				pub struct AssetAmount<_0, _1> {
					pub asset_id: _0,
					pub amount: _1,
				}
				pub struct BasicPoolInfo<_0, _1> {
					pub owner: _0,
					pub assets_weights:
						runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
							_1,
							runtime_types::sp_arithmetic::per_things::Permill,
						>,
					pub lp_token: _1,
					pub fee_config: runtime_types::composable_traits::dex::FeeConfig,
				}
				pub struct Fee<_0, _1> {
					pub fee: _1,
					pub lp_fee: _1,
					pub owner_fee: _1,
					pub protocol_fee: _1,
					pub asset_id: _0,
				}
				pub struct FeeConfig {
					pub fee_rate: runtime_types::sp_arithmetic::per_things::Permill,
					pub owner_fee_rate: runtime_types::sp_arithmetic::per_things::Permill,
					pub protocol_fee_rate: runtime_types::sp_arithmetic::per_things::Permill,
				}
			}
			pub mod oracle {
				use super::runtime_types;
				pub struct Price<_0, _1> {
					pub price: _0,
					pub block: _1,
				}
				pub struct RewardTracker<_0, _1> {
					pub period: _1,
					pub start: _1,
					pub total_already_rewarded: _0,
					pub current_block_reward: _0,
					pub total_reward_weight: _0,
				}
			}
			pub mod storage {
				use super::runtime_types;
				pub enum UpdateValue<_0> {
					#[codec::codec(index = 0)]
					Ignore,
					#[codec::codec(index = 1)]
					Set(_0),
				}
			}
			pub mod xcm {
				use super::runtime_types;
				pub mod memo {
					use super::runtime_types;
					pub enum ChainHop {
						#[codec::codec(index = 0)]
						SubstrateIbc,
						#[codec::codec(index = 1)]
						CosmosIbc,
						#[codec::codec(index = 2)]
						Xcm,
					}
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
				pub enum Call {
					#[codec::codec(index = 0)]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					Unknown,
					#[codec::codec(index = 1)]
					OverLimit,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					InvalidFormat { message_id: [::core::primitive::u8; 32usize] },
					#[codec::codec(index = 1)]
					UnsupportedVersion { message_id: [::core::primitive::u8; 32usize] },
					#[codec::codec(index = 2)]
					ExecutedDownward {
						message_id: [::core::primitive::u8; 32usize],
						outcome: runtime_types::xcm::v3::traits::Outcome,
					},
					#[codec::codec(index = 3)]
					WeightExhausted {
						message_id: [::core::primitive::u8; 32usize],
						remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 4)]
					OverweightEnqueued {
						message_id: [::core::primitive::u8; 32usize],
						overweight_index: ::core::primitive::u64,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 5)]
					OverweightServiced {
						overweight_index: ::core::primitive::u64,
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 6)]
					MaxMessagesExhausted { message_id: [::core::primitive::u8; 32usize] },
				}
			}
			pub struct ConfigData {
				pub max_individual: runtime_types::sp_weights::weight_v2::Weight,
			}
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
				pub enum Call {
					# [codec::codec (index = 0)] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec::codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec::codec (index = 2)] authorize_upgrade { code_hash : runtime_types :: primitive_types :: H256 , } , # [codec::codec (index = 3)] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
				pub enum Error {
					#[codec::codec(index = 0)]
					OverlappingUpgrades,
					#[codec::codec(index = 1)]
					ProhibitedByPolkadot,
					#[codec::codec(index = 2)]
					TooBig,
					#[codec::codec(index = 3)]
					ValidationDataNotAvailable,
					#[codec::codec(index = 4)]
					HostConfigurationNotAvailable,
					#[codec::codec(index = 5)]
					NotScheduled,
					#[codec::codec(index = 6)]
					NothingAuthorized,
					#[codec::codec(index = 7)]
					Unauthorized,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					ValidationFunctionStored,
					#[codec::codec(index = 1)]
					ValidationFunctionApplied { relay_chain_block_num: ::core::primitive::u32 },
					#[codec::codec(index = 2)]
					ValidationFunctionDiscarded,
					#[codec::codec(index = 3)]
					UpgradeAuthorized { code_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 4)]
					DownwardMessagesReceived { count: ::core::primitive::u32 },
					#[codec::codec(index = 5)]
					DownwardMessagesProcessed {
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
						dmq_head: runtime_types::primitive_types::H256,
					},
					#[codec::codec(index = 6)]
					UpwardMessageSent {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			pub mod relay_state_snapshot {
				use super::runtime_types;
				pub struct MessagingStateSnapshot {
					pub dmq_mqc_head: runtime_types::primitive_types::H256,
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
				pub enum Call {}
				pub enum Error {}
				pub enum Event {
					#[codec::codec(index = 0)]
					InvalidFormat([::core::primitive::u8; 32usize]),
					#[codec::codec(index = 1)]
					UnsupportedVersion([::core::primitive::u8; 32usize]),
					#[codec::codec(index = 2)]
					ExecutedDownward(
						[::core::primitive::u8; 32usize],
						runtime_types::xcm::v3::traits::Outcome,
					),
				}
				pub enum Origin {
					#[codec::codec(index = 0)]
					Relay,
					#[codec::codec(index = 1)]
					SiblingParachain(runtime_types::polkadot_parachain::primitives::Id),
				}
			}
		}
		pub mod cumulus_pallet_xcmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 1)]
					suspend_xcm_execution,
					#[codec::codec(index = 2)]
					resume_xcm_execution,
					#[codec::codec(index = 3)]
					update_suspend_threshold { new: ::core::primitive::u32 },
					#[codec::codec(index = 4)]
					update_drop_threshold { new: ::core::primitive::u32 },
					#[codec::codec(index = 5)]
					update_resume_threshold { new: ::core::primitive::u32 },
					#[codec::codec(index = 6)]
					update_threshold_weight { new: runtime_types::sp_weights::weight_v2::Weight },
					#[codec::codec(index = 7)]
					update_weight_restrict_decay {
						new: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 8)]
					update_xcmp_max_individual_weight {
						new: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					FailedToSend,
					#[codec::codec(index = 1)]
					BadXcmOrigin,
					#[codec::codec(index = 2)]
					BadXcm,
					#[codec::codec(index = 3)]
					BadOverweightIndex,
					#[codec::codec(index = 4)]
					WeightOverLimit,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Success {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 1)]
					Fail {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						error: runtime_types::xcm::v3::traits::Error,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 2)]
					BadVersion {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec::codec(index = 3)]
					BadFormat {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec::codec(index = 4)]
					XcmpMessageSent {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec::codec(index = 5)]
					OverweightEnqueued {
						sender: runtime_types::polkadot_parachain::primitives::Id,
						sent_at: ::core::primitive::u32,
						index: ::core::primitive::u64,
						required: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 6)]
					OverweightServiced {
						index: ::core::primitive::u64,
						used: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
			}
			pub struct InboundChannelDetails {
				pub sender: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::InboundState,
				pub message_metadata: ::std::vec::Vec<(
					::core::primitive::u32,
					runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
				)>,
			}
			pub enum InboundState {
				#[codec::codec(index = 0)]
				Ok,
				#[codec::codec(index = 1)]
				Suspended,
			}
			pub struct OutboundChannelDetails {
				pub recipient: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::OutboundState,
				pub signals_exist: ::core::primitive::bool,
				pub first_index: ::core::primitive::u16,
				pub last_index: ::core::primitive::u16,
			}
			pub enum OutboundState {
				#[codec::codec(index = 0)]
				Ok,
				#[codec::codec(index = 1)]
				Suspended,
			}
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
			pub struct MessageQueueChain(pub runtime_types::primitive_types::H256);
			pub struct ParachainInherentData {
				pub validation_data:
					runtime_types::polkadot_primitives::v2::PersistedValidationData<
						runtime_types::primitive_types::H256,
						::core::primitive::u32,
					>,
				pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
				pub downward_messages: ::std::vec::Vec<
					runtime_types::polkadot_core_primitives::InboundDownwardMessage<
						::core::primitive::u32,
					>,
				>,
				pub horizontal_messages: ::std::collections::BTreeMap<
					runtime_types::polkadot_parachain::primitives::Id,
					::std::vec::Vec<
						runtime_types::polkadot_core_primitives::InboundHrmpMessage<
							::core::primitive::u32,
						>,
					>,
				>,
			}
		}
		pub mod farming {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					update_reward_schedule {
						pool_currency_id: runtime_types::primitives::currency::CurrencyId,
						reward_currency_id: runtime_types::primitives::currency::CurrencyId,
						period_count: ::core::primitive::u32,
						#[codec::codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 1)]
					remove_reward_schedule {
						pool_currency_id: runtime_types::primitives::currency::CurrencyId,
						reward_currency_id: runtime_types::primitives::currency::CurrencyId,
					},
					#[codec::codec(index = 2)]
					deposit {
						pool_currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 3)]
					withdraw {
						pool_currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 4)]
					claim {
						pool_currency_id: runtime_types::primitives::currency::CurrencyId,
						reward_currency_id: runtime_types::primitives::currency::CurrencyId,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					InsufficientStake,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					RewardScheduleUpdated {
						pool_currency_id: runtime_types::primitives::currency::CurrencyId,
						reward_currency_id: runtime_types::primitives::currency::CurrencyId,
						period_count: ::core::primitive::u32,
						per_period: ::core::primitive::u128,
					},
					#[codec::codec(index = 1)]
					RewardDistributed {
						pool_currency_id: runtime_types::primitives::currency::CurrencyId,
						reward_currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 2)]
					RewardClaimed {
						account_id: runtime_types::sp_core::crypto::AccountId32,
						pool_currency_id: runtime_types::primitives::currency::CurrencyId,
						reward_currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
					},
				}
			}
			pub struct RewardSchedule<_0> {
				pub period_count: ::core::primitive::u32,
				#[codec::codec(compact)]
				pub per_period: _0,
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				pub enum DispatchClass {
					#[codec::codec(index = 0)]
					Normal,
					#[codec::codec(index = 1)]
					Operational,
					#[codec::codec(index = 2)]
					Mandatory,
				}
				pub struct DispatchInfo {
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				pub enum Pays {
					#[codec::codec(index = 0)]
					Yes,
					#[codec::codec(index = 1)]
					No,
				}
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
				pub enum RawOrigin<_0> {
					#[codec::codec(index = 0)]
					Root,
					#[codec::codec(index = 1)]
					Signed(_0),
					#[codec::codec(index = 2)]
					None,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod preimages {
					use super::runtime_types;
					pub enum Bounded<_0> {
						#[codec::codec(index = 0)]
						Legacy {
							hash: runtime_types::primitive_types::H256,
						},
						#[codec::codec(index = 1)]
						Inline(
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec::codec(index = 2)]
						Lookup {
							hash: runtime_types::primitive_types::H256,
							len: ::core::primitive::u32,
						},
						__Ignore(::core::marker::PhantomData<_0>),
					}
				}
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						pub enum BalanceStatus {
							#[codec::codec(index = 0)]
							Free,
							#[codec::codec(index = 1)]
							Reserved,
						}
					}
				}
			}
			pub struct PalletId(pub [::core::primitive::u8; 8usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					pub struct CheckNonce(#[codec::codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				pub struct BlockLength {
					pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
						::core::primitive::u32,
					>,
				}
				pub struct BlockWeights {
					pub base_block: runtime_types::sp_weights::weight_v2::Weight,
					pub max_block: runtime_types::sp_weights::weight_v2::Weight,
					pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
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
				pub enum Call {
					#[codec::codec(index = 0)]
					remark { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec::codec(index = 1)]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec::codec(index = 2)]
					set_code { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec::codec(index = 3)]
					set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec::codec(index = 4)]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec::codec(index = 5)]
					kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
					#[codec::codec(index = 6)]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec::codec(index = 7)]
					remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					InvalidSpecName,
					#[codec::codec(index = 1)]
					SpecVersionNeedsToIncrease,
					#[codec::codec(index = 2)]
					FailedToExtractRuntimeVersion,
					#[codec::codec(index = 3)]
					NonDefaultComposite,
					#[codec::codec(index = 4)]
					NonZeroRefCount,
					#[codec::codec(index = 5)]
					CallFiltered,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec::codec(index = 1)]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec::codec(index = 2)]
					CodeUpdated,
					#[codec::codec(index = 3)]
					NewAccount { account: runtime_types::sp_core::crypto::AccountId32 },
					#[codec::codec(index = 4)]
					KilledAccount { account: runtime_types::sp_core::crypto::AccountId32 },
					#[codec::codec(index = 5)]
					Remarked {
						sender: runtime_types::sp_core::crypto::AccountId32,
						hash: runtime_types::primitive_types::H256,
					},
				}
			}
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: ::core::primitive::u32,
				pub providers: ::core::primitive::u32,
				pub sufficients: ::core::primitive::u32,
				pub data: _1,
			}
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			pub struct LastRuntimeUpgradeInfo {
				#[codec::codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			pub enum Phase {
				#[codec::codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec::codec(index = 1)]
				Finalization,
				#[codec::codec(index = 2)]
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
						pub struct BaseDenom(pub ::std::string::String);
						pub struct PrefixedDenom {
							pub trace_path:
								runtime_types::ibc::applications::transfer::denom::TracePath,
							pub base_denom:
								runtime_types::ibc::applications::transfer::denom::BaseDenom,
						}
						pub struct TracePath(
							pub  ::std::vec::Vec<
								runtime_types::ibc::applications::transfer::denom::TracePrefix,
							>,
						);
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
						pub struct ChannelId(pub ::std::string::String);
						pub struct PortId(pub ::std::string::String);
					}
				}
			}
			pub mod signer {
				use super::runtime_types;
				pub struct Signer(pub ::std::string::String);
			}
		}
		pub mod ibc_primitives {
			use super::runtime_types;
			pub enum Timeout {
				#[codec::codec(index = 0)]
				Offset {
					timestamp: ::core::option::Option<::core::primitive::u64>,
					height: ::core::option::Option<::core::primitive::u64>,
				},
				#[codec::codec(index = 1)]
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
				pub enum Call {
					#[codec::codec(index = 0)]
					transfer {
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec::codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 1)]
					transfer_all {
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 2)]
					transfer_keep_alive {
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec::codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 3)]
					force_transfer {
						source: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec::codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 4)]
					set_balance {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec::codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec::codec(compact)]
						new_reserved: ::core::primitive::u128,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					BalanceTooLow,
					#[codec::codec(index = 1)]
					AmountIntoBalanceFailed,
					#[codec::codec(index = 2)]
					LiquidityRestrictions,
					#[codec::codec(index = 3)]
					MaxLocksExceeded,
					#[codec::codec(index = 4)]
					KeepAlive,
					#[codec::codec(index = 5)]
					ExistentialDeposit,
					#[codec::codec(index = 6)]
					DeadAccount,
					#[codec::codec(index = 7)]
					TooManyReserves,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Endowed {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 1)]
					DustLost {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 2)]
					Transfer {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						from: runtime_types::sp_core::crypto::AccountId32,
						to: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 3)]
					Reserved {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 4)]
					Unreserved {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 5)]
					ReserveRepatriated {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						from: runtime_types::sp_core::crypto::AccountId32,
						to: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec::codec(index = 6)]
					BalanceSet {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
						free: ::core::primitive::u128,
						reserved: ::core::primitive::u128,
					},
					#[codec::codec(index = 7)]
					TotalIssuanceSet {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 8)]
					Withdrawn {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 9)]
					Slashed {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
						free_amount: ::core::primitive::u128,
						reserved_amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 10)]
					Deposited {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 11)]
					LockSet {
						lock_id: [::core::primitive::u8; 8usize],
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 12)]
					LockRemoved {
						lock_id: [::core::primitive::u8; 8usize],
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
					},
					#[codec::codec(index = 13)]
					Locked {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 14)]
					Unlocked {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
				}
			}
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub frozen: _0,
			}
			pub struct BalanceLock<_0> {
				pub id: [::core::primitive::u8; 8usize],
				pub amount: _0,
			}
			pub struct ReserveData<_0, _1> {
				pub id: _0,
				pub amount: _1,
			}
		}
		pub mod orml_unknown_tokens {
			use super::runtime_types;
			pub mod module {
				use super::runtime_types;
				pub enum Call {}
				pub enum Error {
					#[codec::codec(index = 0)]
					BalanceTooLow,
					#[codec::codec(index = 1)]
					BalanceOverflow,
					#[codec::codec(index = 2)]
					UnhandledAsset,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Deposited {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						who: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 1)]
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
				pub enum Call {
					#[codec::codec(index = 0)]
					transfer {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec::codec(index = 1)]
					transfer_multiasset {
						asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec::codec(index = 2)]
					transfer_with_fee {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						fee: ::core::primitive::u128,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec::codec(index = 3)]
					transfer_multiasset_with_fee {
						asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						fee: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec::codec(index = 4)]
					transfer_multicurrencies {
						currencies: ::std::vec::Vec<(
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
						)>,
						fee_item: ::core::primitive::u32,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec::codec(index = 5)]
					transfer_multiassets {
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_item: ::core::primitive::u32,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					AssetHasNoReserve,
					#[codec::codec(index = 1)]
					NotCrossChainTransfer,
					#[codec::codec(index = 2)]
					InvalidDest,
					#[codec::codec(index = 3)]
					NotCrossChainTransferableCurrency,
					#[codec::codec(index = 4)]
					UnweighableMessage,
					#[codec::codec(index = 5)]
					XcmExecutionFailed,
					#[codec::codec(index = 6)]
					CannotReanchor,
					#[codec::codec(index = 7)]
					InvalidAncestry,
					#[codec::codec(index = 8)]
					InvalidAsset,
					#[codec::codec(index = 9)]
					DestinationNotInvertible,
					#[codec::codec(index = 10)]
					BadVersion,
					#[codec::codec(index = 11)]
					DistinctReserveForAssetAndFee,
					#[codec::codec(index = 12)]
					ZeroFee,
					#[codec::codec(index = 13)]
					ZeroAmount,
					#[codec::codec(index = 14)]
					TooManyAssetsBeingSent,
					#[codec::codec(index = 15)]
					AssetIndexNonExistent,
					#[codec::codec(index = 16)]
					FeeNotEnough,
					#[codec::codec(index = 17)]
					NotSupportedMultiLocation,
					#[codec::codec(index = 18)]
					MinXcmFeeNotDefined,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					TransferredMultiAssets {
						sender: runtime_types::sp_core::crypto::AccountId32,
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
				pub enum Call {
					#[codec::codec(index = 0)]
					set_payment_asset {
						payer: runtime_types::sp_core::crypto::AccountId32,
						asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
					},
				}
			}
			pub struct ChargeAssetTxPayment {
				#[codec::codec(compact)]
				pub tip: ::core::primitive::u128,
				pub asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
			}
		}
		pub mod pallet_assets_registry {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
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
					#[codec::codec(index = 1)]
					update_asset {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						asset_info: runtime_types::composable_traits::assets::AssetInfoUpdate<
							::core::primitive::u128,
						>,
					},
					#[codec::codec(index = 2)]
					set_min_fee {
						target_parachain_id: ::core::primitive::u32,
						foreign_asset_id: runtime_types::primitives::currency::ForeignAssetId,
						amount: ::core::option::Option<::core::primitive::u128>,
					},
					#[codec::codec(index = 3)]
					update_asset_location {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						location: ::core::option::Option<
							runtime_types::primitives::currency::ForeignAssetId,
						>,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					AssetNotFound,
					#[codec::codec(index = 1)]
					AssetAlreadyRegistered,
					#[codec::codec(index = 2)]
					AssetLocationIsNone,
					#[codec::codec(index = 3)]
					StringExceedsMaxLength,
					#[codec::codec(index = 4)]
					LocationIsUsed,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					AssetRegistered {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						location: ::core::option::Option<
							runtime_types::primitives::currency::ForeignAssetId,
						>,
						asset_info: runtime_types::composable_traits::assets::AssetInfo<
							::core::primitive::u128,
						>,
					},
					#[codec::codec(index = 1)]
					AssetUpdated {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						asset_info: runtime_types::composable_traits::assets::AssetInfoUpdate<
							::core::primitive::u128,
						>,
					},
					#[codec::codec(index = 2)]
					AssetLocationUpdated {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						location: runtime_types::primitives::currency::ForeignAssetId,
					},
					#[codec::codec(index = 3)]
					AssetLocationRemoved {
						asset_id: runtime_types::primitives::currency::CurrencyId,
					},
					#[codec::codec(index = 4)]
					MinFeeUpdated {
						target_parachain_id: ::core::primitive::u32,
						foreign_asset_id: runtime_types::primitives::currency::ForeignAssetId,
						amount: ::core::option::Option<::core::primitive::u128>,
					},
				}
			}
		}
		pub mod pallet_assets_transactor_router {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					transfer {
						asset: runtime_types::primitives::currency::CurrencyId,
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						amount: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 1)]
					transfer_native {
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						value: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 2)]
					force_transfer {
						asset: runtime_types::primitives::currency::CurrencyId,
						source: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						value: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 3)]
					force_transfer_native {
						source: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						value: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 4)]
					transfer_all {
						asset: runtime_types::primitives::currency::CurrencyId,
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 5)]
					transfer_all_native {
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 6)]
					mint_into {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 7)]
					burn_from {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						amount: ::core::primitive::u128,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					CannotSetNewCurrencyToRegistry,
					#[codec::codec(index = 1)]
					InvalidCurrency,
				}
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					transfer {
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec::codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec::codec(index = 1)]
					set_balance {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec::codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec::codec(compact)]
						new_reserved: ::core::primitive::u128,
					},
					#[codec::codec(index = 2)]
					force_transfer {
						source: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec::codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec::codec(index = 3)]
					transfer_keep_alive {
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec::codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec::codec(index = 4)]
					transfer_all {
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 5)]
					force_unreserve {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						amount: ::core::primitive::u128,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					VestingBalance,
					#[codec::codec(index = 1)]
					LiquidityRestrictions,
					#[codec::codec(index = 2)]
					InsufficientBalance,
					#[codec::codec(index = 3)]
					ExistentialDeposit,
					#[codec::codec(index = 4)]
					KeepAlive,
					#[codec::codec(index = 5)]
					ExistingVestingSchedule,
					#[codec::codec(index = 6)]
					DeadAccount,
					#[codec::codec(index = 7)]
					TooManyReserves,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Endowed {
						account: runtime_types::sp_core::crypto::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec::codec(index = 1)]
					DustLost {
						account: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 2)]
					Transfer {
						from: runtime_types::sp_core::crypto::AccountId32,
						to: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 3)]
					BalanceSet {
						who: runtime_types::sp_core::crypto::AccountId32,
						free: ::core::primitive::u128,
						reserved: ::core::primitive::u128,
					},
					#[codec::codec(index = 4)]
					Reserved {
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 5)]
					Unreserved {
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 6)]
					ReserveRepatriated {
						from: runtime_types::sp_core::crypto::AccountId32,
						to: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec::codec(index = 7)]
					Deposit {
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 8)]
					Withdraw {
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 9)]
					Slashed {
						who: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
				}
			}
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub misc_frozen: _0,
				pub fee_frozen: _0,
			}
			pub struct BalanceLock<_0> {
				pub id: [::core::primitive::u8; 8usize],
				pub amount: _0,
				pub reasons: runtime_types::pallet_balances::Reasons,
			}
			pub enum Reasons {
				#[codec::codec(index = 0)]
				Fee,
				#[codec::codec(index = 1)]
				Misc,
				#[codec::codec(index = 2)]
				All,
			}
			pub struct ReserveData<_0, _1> {
				pub id: _0,
				pub amount: _1,
			}
		}
		pub mod pallet_bonded_finance {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					offer {
						offer: runtime_types::composable_traits::bonded_finance::BondOffer<
							runtime_types::sp_core::crypto::AccountId32,
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 1)]
					bond {
						offer_id: ::core::primitive::u128,
						nb_of_bonds: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 2)]
					cancel { offer_id: ::core::primitive::u128 },
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					BondOfferNotFound,
					#[codec::codec(index = 1)]
					InvalidBondOffer,
					#[codec::codec(index = 2)]
					OfferCompleted,
					#[codec::codec(index = 3)]
					InvalidNumberOfBonds,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					NewOffer {
						offer_id: ::core::primitive::u128,
						beneficiary: runtime_types::sp_core::crypto::AccountId32,
					},
					#[codec::codec(index = 1)]
					NewBond {
						offer_id: ::core::primitive::u128,
						who: runtime_types::sp_core::crypto::AccountId32,
						nb_of_bonds: ::core::primitive::u128,
					},
					#[codec::codec(index = 2)]
					OfferCancelled { offer_id: ::core::primitive::u128 },
					#[codec::codec(index = 3)]
					OfferCompleted { offer_id: ::core::primitive::u128 },
				}
			}
		}
		pub mod pallet_call_filter {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					disable {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					},
					#[codec::codec(index = 1)]
					enable {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					CannotDisable,
					#[codec::codec(index = 1)]
					InvalidString,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Disabled {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					},
					#[codec::codec(index = 1)]
					Enabled {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				pub struct CallFilterEntry<_0> {
					pub pallet_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub function_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					#[codec::codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_0>,
				}
			}
		}
		pub mod pallet_collator_selection {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					set_invulnerables {
						new: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					},
					#[codec::codec(index = 1)]
					set_desired_candidates { max: ::core::primitive::u32 },
					#[codec::codec(index = 2)]
					set_candidacy_bond { bond: ::core::primitive::u128 },
					#[codec::codec(index = 3)]
					register_as_candidate,
					#[codec::codec(index = 4)]
					leave_intent,
				}
				pub struct CandidateInfo<_0, _1> {
					pub who: _0,
					pub deposit: _1,
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					TooManyCandidates,
					#[codec::codec(index = 1)]
					TooFewCandidates,
					#[codec::codec(index = 2)]
					Unknown,
					#[codec::codec(index = 3)]
					Permission,
					#[codec::codec(index = 4)]
					AlreadyCandidate,
					#[codec::codec(index = 5)]
					NotCandidate,
					#[codec::codec(index = 6)]
					TooManyInvulnerables,
					#[codec::codec(index = 7)]
					AlreadyInvulnerable,
					#[codec::codec(index = 8)]
					NoAssociatedValidatorId,
					#[codec::codec(index = 9)]
					ValidatorNotRegistered,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					NewInvulnerables {
						invulnerables: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					},
					#[codec::codec(index = 1)]
					NewDesiredCandidates { desired_candidates: ::core::primitive::u32 },
					#[codec::codec(index = 2)]
					NewCandidacyBond { bond_amount: ::core::primitive::u128 },
					#[codec::codec(index = 3)]
					CandidateAdded {
						account_id: runtime_types::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec::codec(index = 4)]
					CandidateRemoved { account_id: runtime_types::sp_core::crypto::AccountId32 },
				}
			}
		}
		pub mod pallet_collective {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					set_members {
						new_members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
						prime: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
						old_count: ::core::primitive::u32,
					},
					#[codec::codec(index = 1)]
					execute {
						proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					propose {
						#[codec::codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec::codec(index = 3)]
					vote {
						proposal: runtime_types::primitive_types::H256,
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec::codec(index = 4)]
					close_old_weight {
						proposal_hash: runtime_types::primitive_types::H256,
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						#[codec::codec(compact)]
						proposal_weight_bound: runtime_types::sp_weights::OldWeight,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec::codec(index = 5)]
					disapprove_proposal { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 6)]
					close {
						proposal_hash: runtime_types::primitive_types::H256,
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
				}
				pub enum Call2 {
					#[codec::codec(index = 0)]
					set_members {
						new_members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
						prime: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
						old_count: ::core::primitive::u32,
					},
					#[codec::codec(index = 1)]
					execute {
						proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					propose {
						#[codec::codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec::codec(index = 3)]
					vote {
						proposal: runtime_types::primitive_types::H256,
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec::codec(index = 4)]
					close_old_weight {
						proposal_hash: runtime_types::primitive_types::H256,
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						#[codec::codec(compact)]
						proposal_weight_bound: runtime_types::sp_weights::OldWeight,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec::codec(index = 5)]
					disapprove_proposal { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 6)]
					close {
						proposal_hash: runtime_types::primitive_types::H256,
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
				}
				pub enum Call3 {
					#[codec::codec(index = 0)]
					set_members {
						new_members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
						prime: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
						old_count: ::core::primitive::u32,
					},
					#[codec::codec(index = 1)]
					execute {
						proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					propose {
						#[codec::codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec::codec(index = 3)]
					vote {
						proposal: runtime_types::primitive_types::H256,
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec::codec(index = 4)]
					close_old_weight {
						proposal_hash: runtime_types::primitive_types::H256,
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						#[codec::codec(compact)]
						proposal_weight_bound: runtime_types::sp_weights::OldWeight,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec::codec(index = 5)]
					disapprove_proposal { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 6)]
					close {
						proposal_hash: runtime_types::primitive_types::H256,
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
						#[codec::codec(compact)]
						length_bound: ::core::primitive::u32,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					NotMember,
					#[codec::codec(index = 1)]
					DuplicateProposal,
					#[codec::codec(index = 2)]
					ProposalMissing,
					#[codec::codec(index = 3)]
					WrongIndex,
					#[codec::codec(index = 4)]
					DuplicateVote,
					#[codec::codec(index = 5)]
					AlreadyInitialized,
					#[codec::codec(index = 6)]
					TooEarly,
					#[codec::codec(index = 7)]
					TooManyProposals,
					#[codec::codec(index = 8)]
					WrongProposalWeight,
					#[codec::codec(index = 9)]
					WrongProposalLength,
				}
				pub enum Error2 {
					#[codec::codec(index = 0)]
					NotMember,
					#[codec::codec(index = 1)]
					DuplicateProposal,
					#[codec::codec(index = 2)]
					ProposalMissing,
					#[codec::codec(index = 3)]
					WrongIndex,
					#[codec::codec(index = 4)]
					DuplicateVote,
					#[codec::codec(index = 5)]
					AlreadyInitialized,
					#[codec::codec(index = 6)]
					TooEarly,
					#[codec::codec(index = 7)]
					TooManyProposals,
					#[codec::codec(index = 8)]
					WrongProposalWeight,
					#[codec::codec(index = 9)]
					WrongProposalLength,
				}
				pub enum Error3 {
					#[codec::codec(index = 0)]
					NotMember,
					#[codec::codec(index = 1)]
					DuplicateProposal,
					#[codec::codec(index = 2)]
					ProposalMissing,
					#[codec::codec(index = 3)]
					WrongIndex,
					#[codec::codec(index = 4)]
					DuplicateVote,
					#[codec::codec(index = 5)]
					AlreadyInitialized,
					#[codec::codec(index = 6)]
					TooEarly,
					#[codec::codec(index = 7)]
					TooManyProposals,
					#[codec::codec(index = 8)]
					WrongProposalWeight,
					#[codec::codec(index = 9)]
					WrongProposalLength,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Proposed {
						account: runtime_types::sp_core::crypto::AccountId32,
						proposal_index: ::core::primitive::u32,
						proposal_hash: runtime_types::primitive_types::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec::codec(index = 1)]
					Voted {
						account: runtime_types::sp_core::crypto::AccountId32,
						proposal_hash: runtime_types::primitive_types::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					Approved { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 3)]
					Disapproved { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 4)]
					Executed {
						proposal_hash: runtime_types::primitive_types::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec::codec(index = 5)]
					MemberExecuted {
						proposal_hash: runtime_types::primitive_types::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec::codec(index = 6)]
					Closed {
						proposal_hash: runtime_types::primitive_types::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
				pub enum Event2 {
					#[codec::codec(index = 0)]
					Proposed {
						account: runtime_types::sp_core::crypto::AccountId32,
						proposal_index: ::core::primitive::u32,
						proposal_hash: runtime_types::primitive_types::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec::codec(index = 1)]
					Voted {
						account: runtime_types::sp_core::crypto::AccountId32,
						proposal_hash: runtime_types::primitive_types::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					Approved { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 3)]
					Disapproved { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 4)]
					Executed {
						proposal_hash: runtime_types::primitive_types::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec::codec(index = 5)]
					MemberExecuted {
						proposal_hash: runtime_types::primitive_types::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec::codec(index = 6)]
					Closed {
						proposal_hash: runtime_types::primitive_types::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
				pub enum Event3 {
					#[codec::codec(index = 0)]
					Proposed {
						account: runtime_types::sp_core::crypto::AccountId32,
						proposal_index: ::core::primitive::u32,
						proposal_hash: runtime_types::primitive_types::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec::codec(index = 1)]
					Voted {
						account: runtime_types::sp_core::crypto::AccountId32,
						proposal_hash: runtime_types::primitive_types::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					Approved { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 3)]
					Disapproved { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 4)]
					Executed {
						proposal_hash: runtime_types::primitive_types::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec::codec(index = 5)]
					MemberExecuted {
						proposal_hash: runtime_types::primitive_types::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec::codec(index = 6)]
					Closed {
						proposal_hash: runtime_types::primitive_types::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
			}
			pub enum RawOrigin<_0> {
				#[codec::codec(index = 0)]
				Members(::core::primitive::u32, ::core::primitive::u32),
				#[codec::codec(index = 1)]
				Member(_0),
				#[codec::codec(index = 2)]
				_Phantom,
			}
			pub struct Votes<_0, _1> {
				pub index: ::core::primitive::u32,
				pub threshold: ::core::primitive::u32,
				pub ayes: ::std::vec::Vec<_0>,
				pub nays: ::std::vec::Vec<_0>,
				pub end: _1,
			}
		}
		pub mod pallet_cosmwasm {
			use super::runtime_types;
			pub mod instrument {
				use super::runtime_types;
				pub struct CostRules {
					pub i64const: ::core::primitive::u32,
					pub f64const: ::core::primitive::u32,
					pub i64load: ::core::primitive::u32,
					pub f64load: ::core::primitive::u32,
					pub i64store: ::core::primitive::u32,
					pub f64store: ::core::primitive::u32,
					pub i64eq: ::core::primitive::u32,
					pub i64eqz: ::core::primitive::u32,
					pub i64ne: ::core::primitive::u32,
					pub i64lts: ::core::primitive::u32,
					pub i64gts: ::core::primitive::u32,
					pub i64les: ::core::primitive::u32,
					pub i64ges: ::core::primitive::u32,
					pub i64clz: ::core::primitive::u32,
					pub i64ctz: ::core::primitive::u32,
					pub i64popcnt: ::core::primitive::u32,
					pub i64add: ::core::primitive::u32,
					pub i64sub: ::core::primitive::u32,
					pub i64mul: ::core::primitive::u32,
					pub i64divs: ::core::primitive::u32,
					pub i64divu: ::core::primitive::u32,
					pub i64rems: ::core::primitive::u32,
					pub i64and: ::core::primitive::u32,
					pub i64or: ::core::primitive::u32,
					pub i64xor: ::core::primitive::u32,
					pub i64shl: ::core::primitive::u32,
					pub i64shrs: ::core::primitive::u32,
					pub i64rotl: ::core::primitive::u32,
					pub i64rotr: ::core::primitive::u32,
					pub i32wrapi64: ::core::primitive::u32,
					pub i64extendsi32: ::core::primitive::u32,
					pub f64eq: ::core::primitive::u32,
					pub f64ne: ::core::primitive::u32,
					pub f64lt: ::core::primitive::u32,
					pub f64gt: ::core::primitive::u32,
					pub f64le: ::core::primitive::u32,
					pub f64ge: ::core::primitive::u32,
					pub f64abs: ::core::primitive::u32,
					pub f64neg: ::core::primitive::u32,
					pub f64ceil: ::core::primitive::u32,
					pub f64floor: ::core::primitive::u32,
					pub f64trunc: ::core::primitive::u32,
					pub f64nearest: ::core::primitive::u32,
					pub f64sqrt: ::core::primitive::u32,
					pub f64add: ::core::primitive::u32,
					pub f64sub: ::core::primitive::u32,
					pub f64mul: ::core::primitive::u32,
					pub f64div: ::core::primitive::u32,
					pub f64min: ::core::primitive::u32,
					pub f64max: ::core::primitive::u32,
					pub f64copysign: ::core::primitive::u32,
					pub select: ::core::primitive::u32,
					pub if_: ::core::primitive::u32,
					pub else_: ::core::primitive::u32,
					pub getlocal: ::core::primitive::u32,
					pub setlocal: ::core::primitive::u32,
					pub teelocal: ::core::primitive::u32,
					pub setglobal: ::core::primitive::u32,
					pub getglobal: ::core::primitive::u32,
					pub currentmemory: ::core::primitive::u32,
					pub growmemory: ::core::primitive::u32,
					pub br: ::core::primitive::u32,
					pub brif: ::core::primitive::u32,
					pub brtable: ::core::primitive::u32,
					pub brtable_per_elem: ::core::primitive::u32,
					pub call: ::core::primitive::u32,
					pub call_indirect: ::core::primitive::u32,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					upload {
						code: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec::codec(index = 1)]
					instantiate {
						code_identifier: runtime_types::pallet_cosmwasm::types::CodeIdentifier,
						salt: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
						admin: ::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
						label: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
						funds:
							runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
								runtime_types::primitives::currency::CurrencyId,
								(::core::primitive::u128, ::core::primitive::bool),
							>,
						gas: ::core::primitive::u64,
						message: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec::codec(index = 2)]
					execute {
						contract: runtime_types::sp_core::crypto::AccountId32,
						funds:
							runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
								runtime_types::primitives::currency::CurrencyId,
								(::core::primitive::u128, ::core::primitive::bool),
							>,
						gas: ::core::primitive::u64,
						message: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec::codec(index = 3)]
					migrate {
						contract: runtime_types::sp_core::crypto::AccountId32,
						new_code_identifier: runtime_types::pallet_cosmwasm::types::CodeIdentifier,
						gas: ::core::primitive::u64,
						message: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec::codec(index = 4)]
					update_admin {
						contract: runtime_types::sp_core::crypto::AccountId32,
						new_admin:
							::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
						gas: ::core::primitive::u64,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					Instrumentation,
					#[codec::codec(index = 1)]
					VmCreation,
					#[codec::codec(index = 2)]
					ContractHasNoInfo,
					#[codec::codec(index = 3)]
					CodeDecoding,
					#[codec::codec(index = 4)]
					CodeValidation,
					#[codec::codec(index = 5)]
					CodeEncoding,
					#[codec::codec(index = 6)]
					CodeInstrumentation,
					#[codec::codec(index = 7)]
					InstrumentedCodeIsTooBig,
					#[codec::codec(index = 8)]
					CodeAlreadyExists,
					#[codec::codec(index = 9)]
					CodeNotFound,
					#[codec::codec(index = 10)]
					ContractAlreadyExists,
					#[codec::codec(index = 11)]
					ContractNotFound,
					#[codec::codec(index = 12)]
					SubstrateDispatch,
					#[codec::codec(index = 13)]
					AssetConversion,
					#[codec::codec(index = 14)]
					TransferFailed,
					#[codec::codec(index = 15)]
					LabelTooBig,
					#[codec::codec(index = 16)]
					UnknownDenom,
					#[codec::codec(index = 17)]
					StackOverflow,
					#[codec::codec(index = 18)]
					NotEnoughFundsForUpload,
					#[codec::codec(index = 19)]
					NonceOverflow,
					#[codec::codec(index = 20)]
					RefcountOverflow,
					#[codec::codec(index = 21)]
					VMDepthOverflow,
					#[codec::codec(index = 22)]
					SignatureVerificationError,
					#[codec::codec(index = 23)]
					IteratorIdOverflow,
					#[codec::codec(index = 24)]
					IteratorNotFound,
					#[codec::codec(index = 25)]
					IteratorValueNotFound,
					#[codec::codec(index = 26)]
					NotAuthorized,
					#[codec::codec(index = 27)]
					NotImplemented,
					#[codec::codec(index = 28)]
					Unsupported,
					#[codec::codec(index = 29)]
					ExecuteDeserialize,
					#[codec::codec(index = 30)]
					Ibc,
					#[codec::codec(index = 31)]
					FailedToSerialize,
					#[codec::codec(index = 32)]
					OutOfGas,
					#[codec::codec(index = 33)]
					InvalidGasCheckpoint,
					#[codec::codec(index = 34)]
					InvalidSalt,
					#[codec::codec(index = 35)]
					InvalidAccount,
					#[codec::codec(index = 36)]
					Interpreter,
					#[codec::codec(index = 37)]
					VirtualMachine,
					#[codec::codec(index = 38)]
					AccountConversionFailure,
					#[codec::codec(index = 39)]
					Aborted,
					#[codec::codec(index = 40)]
					ReadOnlyViolation,
					#[codec::codec(index = 41)]
					Rpc,
					#[codec::codec(index = 42)]
					Precompile,
					#[codec::codec(index = 43)]
					QueryDeserialize,
					#[codec::codec(index = 44)]
					ExecuteSerialize,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Uploaded {
						code_hash: [::core::primitive::u8; 32usize],
						code_id: ::core::primitive::u64,
					},
					#[codec::codec(index = 1)]
					Instantiated {
						contract: runtime_types::sp_core::crypto::AccountId32,
						info: runtime_types::pallet_cosmwasm::types::ContractInfo<
							runtime_types::sp_core::crypto::AccountId32,
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						>,
					},
					#[codec::codec(index = 2)]
					Executed {
						contract: runtime_types::sp_core::crypto::AccountId32,
						entrypoint: runtime_types::pallet_cosmwasm::types::EntryPoint,
						data: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec::codec(index = 3)]
					ExecutionFailed {
						contract: runtime_types::sp_core::crypto::AccountId32,
						entrypoint: runtime_types::pallet_cosmwasm::types::EntryPoint,
						error: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec::codec(index = 4)]
					Emitted {
						contract: runtime_types::sp_core::crypto::AccountId32,
						ty: ::std::vec::Vec<::core::primitive::u8>,
						attributes: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec::codec(index = 5)]
					Migrated {
						contract: runtime_types::sp_core::crypto::AccountId32,
						to: ::core::primitive::u64,
					},
					#[codec::codec(index = 6)]
					AdminUpdated {
						contract: runtime_types::sp_core::crypto::AccountId32,
						new_admin:
							::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				pub enum CodeIdentifier {
					#[codec::codec(index = 0)]
					CodeId(::core::primitive::u64),
					#[codec::codec(index = 1)]
					CodeHash([::core::primitive::u8; 32usize]),
				}
				pub struct CodeInfo<_0> {
					pub creator: _0,
					pub pristine_code_hash: [::core::primitive::u8; 32usize],
					pub instrumentation_version: ::core::primitive::u16,
					pub refcount: ::core::primitive::u32,
					pub ibc_capable: ::core::primitive::bool,
				}
				pub struct ContractInfo<_0, _1, _2> {
					pub code_id: ::core::primitive::u64,
					pub trie_id: _2,
					pub instantiator: _0,
					pub admin: ::core::option::Option<_0>,
					pub label: _1,
				}
				pub enum EntryPoint {
					#[codec::codec(index = 0)]
					Instantiate,
					#[codec::codec(index = 1)]
					Execute,
					#[codec::codec(index = 2)]
					Migrate,
					#[codec::codec(index = 3)]
					Reply,
					#[codec::codec(index = 4)]
					IbcChannelOpen,
					#[codec::codec(index = 5)]
					IbcChannelConnect,
					#[codec::codec(index = 6)]
					IbcChannelClose,
					#[codec::codec(index = 7)]
					IbcPacketTimeout,
					#[codec::codec(index = 8)]
					IbcPacketReceive,
					#[codec::codec(index = 9)]
					IbcPacketAck,
				}
			}
		}
		pub mod pallet_crowdloan_rewards {
			use super::runtime_types;
			pub mod models {
				use super::runtime_types;
				pub enum Proof<_0> {
					#[codec::codec(index = 0)]
					RelayChain(_0, runtime_types::sp_runtime::MultiSignature),
					#[codec::codec(index = 1)]
					Ethereum(runtime_types::composable_support::types::EcdsaSignature),
				}
				pub enum RemoteAccount<_0> {
					#[codec::codec(index = 0)]
					RelayChain(_0),
					#[codec::codec(index = 1)]
					Ethereum(runtime_types::composable_support::types::EthereumAddress),
				}
				pub struct Reward<_0, _1> {
					pub total: _0,
					pub claimed: _0,
					pub vesting_period: _1,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					initialize,
					#[codec::codec(index = 1)]
					initialize_at { at: ::core::primitive::u64 },
					#[codec::codec(index = 2)]
					populate {
						rewards: ::std::vec::Vec<(
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								runtime_types::sp_core::crypto::AccountId32,
							>,
							::core::primitive::u128,
							::core::primitive::u64,
						)>,
					},
					#[codec::codec(index = 3)]
					associate {
						reward_account: runtime_types::sp_core::crypto::AccountId32,
						proof: runtime_types::pallet_crowdloan_rewards::models::Proof<
							runtime_types::sp_core::crypto::AccountId32,
						>,
					},
					#[codec::codec(index = 4)]
					claim,
					#[codec::codec(index = 5)]
					unlock_rewards_for {
						reward_accounts:
							::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					},
					#[codec::codec(index = 6)]
					add {
						additions: ::std::vec::Vec<(
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								runtime_types::sp_core::crypto::AccountId32,
							>,
							::core::primitive::u128,
							::core::primitive::u64,
						)>,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					NotInitialized,
					#[codec::codec(index = 1)]
					AlreadyInitialized,
					#[codec::codec(index = 2)]
					BackToTheFuture,
					#[codec::codec(index = 3)]
					RewardsNotFunded,
					#[codec::codec(index = 4)]
					InvalidProof,
					#[codec::codec(index = 5)]
					InvalidClaim,
					#[codec::codec(index = 6)]
					NothingToClaim,
					#[codec::codec(index = 7)]
					NotAssociated,
					#[codec::codec(index = 8)]
					AlreadyAssociated,
					#[codec::codec(index = 9)]
					NotClaimableYet,
					#[codec::codec(index = 10)]
					UnexpectedRewardAmount,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Initialized { at: ::core::primitive::u64 },
					#[codec::codec(index = 1)]
					Claimed {
						remote_account:
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								runtime_types::sp_core::crypto::AccountId32,
							>,
						reward_account: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 2)]
					Associated {
						remote_account:
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								runtime_types::sp_core::crypto::AccountId32,
							>,
						reward_account: runtime_types::sp_core::crypto::AccountId32,
					},
					#[codec::codec(index = 3)]
					OverFunded { excess_funds: ::core::primitive::u128 },
					#[codec::codec(index = 4)]
					RewardsUnlocked { at: ::core::primitive::u64 },
					#[codec::codec(index = 5)]
					RewardsAdded {
						additions: ::std::vec::Vec<(
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								runtime_types::sp_core::crypto::AccountId32,
							>,
							::core::primitive::u128,
							::core::primitive::u64,
						)>,
					},
					#[codec::codec(index = 6)]
					RewardsDeleted {
						deletions: ::std::vec::Vec<
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								runtime_types::sp_core::crypto::AccountId32,
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
				pub enum Call {
					#[codec::codec(index = 0)]
					add_range { length: ::core::primitive::u64 },
					#[codec::codec(index = 1)]
					set_metadata {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						metadata: runtime_types::composable_traits::assets::BasicAssetMetadata,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					AssetNotFound,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					RangeCreated {
						range: runtime_types::pallet_currency_factory::ranges::Range<
							runtime_types::primitives::currency::CurrencyId,
						>,
					},
				}
			}
			pub mod ranges {
				use super::runtime_types;
				pub struct Range<_0> {
					pub current: _0,
					pub end: _0,
				}
				pub struct Ranges<_0> {
					pub ranges: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_currency_factory::ranges::Range<_0>,
					>,
				}
			}
		}
		pub mod pallet_democracy {
			use super::runtime_types;
			pub mod conviction {
				use super::runtime_types;
				pub enum Conviction {
					#[codec::codec(index = 0)]
					None,
					#[codec::codec(index = 1)]
					Locked1x,
					#[codec::codec(index = 2)]
					Locked2x,
					#[codec::codec(index = 3)]
					Locked3x,
					#[codec::codec(index = 4)]
					Locked4x,
					#[codec::codec(index = 5)]
					Locked5x,
					#[codec::codec(index = 6)]
					Locked6x,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					propose {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::picasso_runtime::RuntimeCall,
						>,
						#[codec::codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec::codec(index = 1)]
					second {
						#[codec::codec(compact)]
						proposal: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					vote {
						#[codec::codec(compact)]
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec::codec(index = 3)]
					emergency_cancel { ref_index: ::core::primitive::u32 },
					#[codec::codec(index = 4)]
					external_propose {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::picasso_runtime::RuntimeCall,
						>,
					},
					#[codec::codec(index = 5)]
					external_propose_majority {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::picasso_runtime::RuntimeCall,
						>,
					},
					#[codec::codec(index = 6)]
					external_propose_default {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::picasso_runtime::RuntimeCall,
						>,
					},
					#[codec::codec(index = 7)]
					fast_track {
						proposal_hash: runtime_types::primitive_types::H256,
						voting_period: ::core::primitive::u32,
						delay: ::core::primitive::u32,
					},
					#[codec::codec(index = 8)]
					veto_external { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 9)]
					cancel_referendum {
						#[codec::codec(compact)]
						ref_index: ::core::primitive::u32,
					},
					#[codec::codec(index = 10)]
					delegate {
						to: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						balance: ::core::primitive::u128,
					},
					#[codec::codec(index = 11)]
					undelegate,
					#[codec::codec(index = 12)]
					clear_public_proposals,
					#[codec::codec(index = 13)]
					unlock {
						target: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 14)]
					remove_vote { index: ::core::primitive::u32 },
					#[codec::codec(index = 15)]
					remove_other_vote {
						target: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						index: ::core::primitive::u32,
					},
					#[codec::codec(index = 16)]
					blacklist {
						proposal_hash: runtime_types::primitive_types::H256,
						maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec::codec(index = 17)]
					cancel_proposal {
						#[codec::codec(compact)]
						prop_index: ::core::primitive::u32,
					},
					#[codec::codec(index = 18)]
					set_metadata {
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						maybe_hash: ::core::option::Option<runtime_types::primitive_types::H256>,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					ValueLow,
					#[codec::codec(index = 1)]
					ProposalMissing,
					#[codec::codec(index = 2)]
					AlreadyCanceled,
					#[codec::codec(index = 3)]
					DuplicateProposal,
					#[codec::codec(index = 4)]
					ProposalBlacklisted,
					#[codec::codec(index = 5)]
					NotSimpleMajority,
					#[codec::codec(index = 6)]
					InvalidHash,
					#[codec::codec(index = 7)]
					NoProposal,
					#[codec::codec(index = 8)]
					AlreadyVetoed,
					#[codec::codec(index = 9)]
					ReferendumInvalid,
					#[codec::codec(index = 10)]
					NoneWaiting,
					#[codec::codec(index = 11)]
					NotVoter,
					#[codec::codec(index = 12)]
					NoPermission,
					#[codec::codec(index = 13)]
					AlreadyDelegating,
					#[codec::codec(index = 14)]
					InsufficientFunds,
					#[codec::codec(index = 15)]
					NotDelegating,
					#[codec::codec(index = 16)]
					VotesExist,
					#[codec::codec(index = 17)]
					InstantNotAllowed,
					#[codec::codec(index = 18)]
					Nonsense,
					#[codec::codec(index = 19)]
					WrongUpperBound,
					#[codec::codec(index = 20)]
					MaxVotesReached,
					#[codec::codec(index = 21)]
					TooMany,
					#[codec::codec(index = 22)]
					VotingPeriodLow,
					#[codec::codec(index = 23)]
					PreimageNotExist,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Proposed {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec::codec(index = 1)]
					Tabled {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec::codec(index = 2)]
					ExternalTabled,
					#[codec::codec(index = 3)]
					Started {
						ref_index: ::core::primitive::u32,
						threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					},
					#[codec::codec(index = 4)]
					Passed { ref_index: ::core::primitive::u32 },
					#[codec::codec(index = 5)]
					NotPassed { ref_index: ::core::primitive::u32 },
					#[codec::codec(index = 6)]
					Cancelled { ref_index: ::core::primitive::u32 },
					#[codec::codec(index = 7)]
					Delegated {
						who: runtime_types::sp_core::crypto::AccountId32,
						target: runtime_types::sp_core::crypto::AccountId32,
					},
					#[codec::codec(index = 8)]
					Undelegated { account: runtime_types::sp_core::crypto::AccountId32 },
					#[codec::codec(index = 9)]
					Vetoed {
						who: runtime_types::sp_core::crypto::AccountId32,
						proposal_hash: runtime_types::primitive_types::H256,
						until: ::core::primitive::u32,
					},
					#[codec::codec(index = 10)]
					Blacklisted { proposal_hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 11)]
					Voted {
						voter: runtime_types::sp_core::crypto::AccountId32,
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec::codec(index = 12)]
					Seconded {
						seconder: runtime_types::sp_core::crypto::AccountId32,
						prop_index: ::core::primitive::u32,
					},
					#[codec::codec(index = 13)]
					ProposalCanceled { prop_index: ::core::primitive::u32 },
					#[codec::codec(index = 14)]
					MetadataSet {
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: runtime_types::primitive_types::H256,
					},
					#[codec::codec(index = 15)]
					MetadataCleared {
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: runtime_types::primitive_types::H256,
					},
					#[codec::codec(index = 16)]
					MetadataTransferred {
						prev_owner: runtime_types::pallet_democracy::types::MetadataOwner,
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: runtime_types::primitive_types::H256,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				pub struct Delegations<_0> {
					pub votes: _0,
					pub capital: _0,
				}
				pub enum MetadataOwner {
					#[codec::codec(index = 0)]
					External,
					#[codec::codec(index = 1)]
					Proposal(::core::primitive::u32),
					#[codec::codec(index = 2)]
					Referendum(::core::primitive::u32),
				}
				pub enum ReferendumInfo<_0, _1, _2> {
					#[codec::codec(index = 0)]
					Ongoing(runtime_types::pallet_democracy::types::ReferendumStatus<_0, _1, _2>),
					#[codec::codec(index = 1)]
					Finished { approved: ::core::primitive::bool, end: _0 },
				}
				pub struct ReferendumStatus<_0, _1, _2> {
					pub end: _0,
					pub proposal: _1,
					pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					pub delay: _0,
					pub tally: runtime_types::pallet_democracy::types::Tally<_2>,
				}
				pub struct Tally<_0> {
					pub ayes: _0,
					pub nays: _0,
					pub turnout: _0,
				}
			}
			pub mod vote {
				use super::runtime_types;
				pub enum AccountVote<_0> {
					#[codec::codec(index = 0)]
					Standard { vote: runtime_types::pallet_democracy::vote::Vote, balance: _0 },
					#[codec::codec(index = 1)]
					Split { aye: _0, nay: _0 },
				}
				pub struct PriorLock<_0, _1>(pub _0, pub _1);
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct Vote(pub ::core::primitive::u8);
				pub enum Voting<_0, _1, _2> {
					#[codec::codec(index = 0)]
					Direct {
						votes: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							_2,
							runtime_types::pallet_democracy::vote::AccountVote<_0>,
						)>,
						delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
						prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
					},
					#[codec::codec(index = 1)]
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
				pub enum VoteThreshold {
					#[codec::codec(index = 0)]
					SuperMajorityApprove,
					#[codec::codec(index = 1)]
					SuperMajorityAgainst,
					#[codec::codec(index = 2)]
					SimpleMajority,
				}
			}
		}
		pub mod pallet_ibc {
			use super::runtime_types;
			pub mod errors {
				use super::runtime_types;
				pub enum IbcError {
					#[codec::codec(index = 0)]
					Ics02Client { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec::codec(index = 1)]
					Ics03Connection { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec::codec(index = 2)]
					Ics04Channel { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec::codec(index = 3)]
					Ics20FungibleTokenTransfer { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec::codec(index = 4)]
					UnknownMessageTypeUrl { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec::codec(index = 5)]
					MalformedMessageBytes { message: ::std::vec::Vec<::core::primitive::u8> },
				}
			}
			pub mod events {
				use super::runtime_types;
				pub enum IbcEvent {
					#[codec::codec(index = 0)]
					NewBlock {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
					},
					#[codec::codec(index = 1)]
					CreateClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec::codec(index = 2)]
					UpdateClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec::codec(index = 3)]
					UpgradeClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec::codec(index = 4)]
					ClientMisbehaviour {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec::codec(index = 5)]
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
					#[codec::codec(index = 6)]
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
					#[codec::codec(index = 7)]
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
					#[codec::codec(index = 8)]
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
					#[codec::codec(index = 9)]
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
					#[codec::codec(index = 10)]
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
					#[codec::codec(index = 11)]
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
					#[codec::codec(index = 12)]
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
					#[codec::codec(index = 13)]
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
					#[codec::codec(index = 14)]
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
					#[codec::codec(index = 15)]
					ReceivePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec::codec(index = 16)]
					SendPacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec::codec(index = 17)]
					AcknowledgePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec::codec(index = 18)]
					WriteAcknowledgement {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec::codec(index = 19)]
					TimeoutPacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec::codec(index = 20)]
					TimeoutOnClosePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec::codec(index = 21)]
					Empty,
					#[codec::codec(index = 22)]
					ChainError,
					#[codec::codec(index = 23)]
					AppModule {
						kind: ::std::vec::Vec<::core::primitive::u8>,
						module_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec::codec(index = 24)]
					PushWasmCode { wasm_code_id: ::std::vec::Vec<::core::primitive::u8> },
				}
			}
			pub mod ics20_fee {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					pub enum Call {
						#[codec::codec(index = 0)]
						set_charge { charge: runtime_types::sp_arithmetic::per_things::Perbill },
						#[codec::codec(index = 1)]
						add_channels_to_feeless_channel_list {
							source_channel: ::core::primitive::u64,
							destination_channel: ::core::primitive::u64,
						},
						#[codec::codec(index = 2)]
						remove_channels_from_feeless_channel_list {
							source_channel: ::core::primitive::u64,
							destination_channel: ::core::primitive::u64,
						},
					}
					pub enum Event {
						#[codec::codec(index = 0)]
						IbcTransferFeeCollected {
							amount: ::core::primitive::u128,
							asset_id: runtime_types::primitives::currency::CurrencyId,
						},
						#[codec::codec(index = 1)]
						FeeLessChannelIdsAdded {
							source_channel: ::core::primitive::u64,
							destination_channel: ::core::primitive::u64,
						},
						#[codec::codec(index = 2)]
						FeeLessChannelIdsRemoved {
							source_channel: ::core::primitive::u64,
							destination_channel: ::core::primitive::u64,
						},
					}
				}
			}
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					deliver { messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any> },
					#[codec::codec(index = 1)]
					transfer {
						params: runtime_types::pallet_ibc::TransferParams<
							runtime_types::sp_core::crypto::AccountId32,
						>,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						memo: ::core::option::Option<::std::string::String>,
					},
					#[codec::codec(index = 3)]
					upgrade_client { params: runtime_types::pallet_ibc::UpgradeParams },
					#[codec::codec(index = 4)]
					freeze_client {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						height: ::core::primitive::u64,
					},
					#[codec::codec(index = 5)]
					increase_counters,
					#[codec::codec(index = 6)]
					add_channels_to_feeless_channel_list {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec::codec(index = 7)]
					remove_channels_from_feeless_channel_list {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec::codec(index = 8)]
					set_child_storage {
						key: ::std::vec::Vec<::core::primitive::u8>,
						value: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec::codec(index = 9)]
					substitute_client_state {
						client_id: ::std::string::String,
						height: runtime_types::ibc::core::ics02_client::height::Height,
						client_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
						consensus_state_bytes: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					ProcessingError,
					#[codec::codec(index = 1)]
					DecodingError,
					#[codec::codec(index = 2)]
					EncodingError,
					#[codec::codec(index = 3)]
					ProofGenerationError,
					#[codec::codec(index = 4)]
					ConsensusStateNotFound,
					#[codec::codec(index = 5)]
					ChannelNotFound,
					#[codec::codec(index = 6)]
					ClientStateNotFound,
					#[codec::codec(index = 7)]
					ConnectionNotFound,
					#[codec::codec(index = 8)]
					PacketCommitmentNotFound,
					#[codec::codec(index = 9)]
					PacketReceiptNotFound,
					#[codec::codec(index = 10)]
					PacketAcknowledgmentNotFound,
					#[codec::codec(index = 11)]
					SendPacketError,
					#[codec::codec(index = 12)]
					InvalidChannelId,
					#[codec::codec(index = 13)]
					InvalidPortId,
					#[codec::codec(index = 14)]
					Other,
					#[codec::codec(index = 15)]
					InvalidRoute,
					#[codec::codec(index = 16)]
					InvalidMessageType,
					#[codec::codec(index = 17)]
					TransferInternals,
					#[codec::codec(index = 18)]
					TransferSerde,
					#[codec::codec(index = 19)]
					TransferOther,
					#[codec::codec(index = 20)]
					TransferProtocol,
					#[codec::codec(index = 21)]
					TransferSend,
					#[codec::codec(index = 22)]
					Utf8Error,
					#[codec::codec(index = 23)]
					InvalidAssetId,
					#[codec::codec(index = 24)]
					PrefixedDenomParse,
					#[codec::codec(index = 25)]
					InvalidAmount,
					#[codec::codec(index = 26)]
					InvalidTimestamp,
					#[codec::codec(index = 27)]
					FailedToGetRevisionNumber,
					#[codec::codec(index = 28)]
					InvalidParams,
					#[codec::codec(index = 29)]
					ChannelInitError,
					#[codec::codec(index = 30)]
					TimestampAndHeightNotFound,
					#[codec::codec(index = 31)]
					ChannelEscrowAddress,
					#[codec::codec(index = 32)]
					WriteAckError,
					#[codec::codec(index = 33)]
					ClientUpdateNotFound,
					#[codec::codec(index = 34)]
					ClientFreezeFailed,
					#[codec::codec(index = 35)]
					AccessDenied,
					#[codec::codec(index = 36)]
					RateLimiter,
					#[codec::codec(index = 37)]
					FailedSendFeeToAccount,
					#[codec::codec(index = 38)]
					OriginAddress,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Events {
						events: ::std::vec::Vec<
							::core::result::Result<
								runtime_types::pallet_ibc::events::IbcEvent,
								runtime_types::pallet_ibc::errors::IbcError,
							>,
						>,
					},
					#[codec::codec(index = 1)]
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
					#[codec::codec(index = 2)]
					ChannelOpened {
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec::codec(index = 3)]
					ParamsUpdated {
						send_enabled: ::core::primitive::bool,
						receive_enabled: ::core::primitive::bool,
					},
					#[codec::codec(index = 4)]
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
					#[codec::codec(index = 5)]
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
					#[codec::codec(index = 6)]
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
					#[codec::codec(index = 7)]
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
					#[codec::codec(index = 8)]
					OnRecvPacketError { msg: ::std::vec::Vec<::core::primitive::u8> },
					#[codec::codec(index = 9)]
					ClientUpgradeSet,
					#[codec::codec(index = 10)]
					ClientFrozen {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
					},
					#[codec::codec(index = 11)]
					AssetAdminUpdated { admin_account: runtime_types::sp_core::crypto::AccountId32 },
					#[codec::codec(index = 12)]
					FeeLessChannelIdsAdded {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec::codec(index = 13)]
					FeeLessChannelIdsRemoved {
						source_channel: ::core::primitive::u64,
						destination_channel: ::core::primitive::u64,
					},
					#[codec::codec(index = 14)]
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
					#[codec::codec(index = 15)]
					ChargingFeeConfirmed { sequence: ::core::primitive::u64 },
					#[codec::codec(index = 16)]
					ChargingFeeTimeout { sequence: ::core::primitive::u64 },
					#[codec::codec(index = 17)]
					ChargingFeeFailedAcknowledgement { sequence: ::core::primitive::u64 },
					#[codec::codec(index = 18)]
					ChildStateUpdated,
					#[codec::codec(index = 19)]
					ClientStateSubstituted {
						client_id: ::std::string::String,
						height: runtime_types::ibc::core::ics02_client::height::Height,
					},
					#[codec::codec(index = 20)]
					ExecuteMemoStarted {
						account_id: runtime_types::sp_core::crypto::AccountId32,
						memo: ::core::option::Option<::std::string::String>,
					},
					#[codec::codec(index = 21)]
					ExecuteMemoIbcTokenTransferSuccess {
						from: runtime_types::sp_core::crypto::AccountId32,
						to: ::std::vec::Vec<::core::primitive::u8>,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						channel: ::core::primitive::u64,
						next_memo: ::core::option::Option<::std::string::String>,
					},
					#[codec::codec(index = 22)]
					ExecuteMemoIbcTokenTransferFailedWithReason {
						from: runtime_types::sp_core::crypto::AccountId32,
						memo: ::std::string::String,
						reason: ::core::primitive::u8,
					},
					#[codec::codec(index = 23)]
					ExecuteMemoIbcTokenTransferFailed {
						from: runtime_types::sp_core::crypto::AccountId32,
						to: ::std::vec::Vec<::core::primitive::u8>,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						channel: ::core::primitive::u64,
						next_memo: ::core::option::Option<::std::string::String>,
					},
					#[codec::codec(index = 24)]
					ExecuteMemoXcmSuccess {
						from: runtime_types::sp_core::crypto::AccountId32,
						to: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						para_id: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec::codec(index = 25)]
					ExecuteMemoXcmFailed {
						from: runtime_types::sp_core::crypto::AccountId32,
						to: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						para_id: ::core::option::Option<::core::primitive::u32>,
					},
				}
			}
			pub struct Any {
				pub type_url: ::std::string::String,
				pub value: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub enum LightClientProtocol {
				#[codec::codec(index = 0)]
				Beefy,
				#[codec::codec(index = 1)]
				Grandpa,
			}
			pub enum MultiAddress<_0> {
				#[codec::codec(index = 0)]
				Id(_0),
				#[codec::codec(index = 1)]
				Raw(::std::vec::Vec<::core::primitive::u8>),
			}
			pub struct TransferParams<_0> {
				pub to: runtime_types::pallet_ibc::MultiAddress<_0>,
				pub source_channel: ::core::primitive::u64,
				pub timeout: runtime_types::ibc_primitives::Timeout,
			}
			pub struct UpgradeParams {
				pub client_state: ::std::vec::Vec<::core::primitive::u8>,
				pub consensus_state: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod pallet_identity {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					add_registrar {
						account: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 1)]
					set_identity {
						info:
							::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
					},
					#[codec::codec(index = 2)]
					set_subs {
						subs: ::std::vec::Vec<(
							runtime_types::sp_core::crypto::AccountId32,
							runtime_types::pallet_identity::types::Data,
						)>,
					},
					#[codec::codec(index = 3)]
					clear_identity,
					#[codec::codec(index = 4)]
					request_judgement {
						#[codec::codec(compact)]
						reg_index: ::core::primitive::u32,
						#[codec::codec(compact)]
						max_fee: ::core::primitive::u128,
					},
					#[codec::codec(index = 5)]
					cancel_request { reg_index: ::core::primitive::u32 },
					#[codec::codec(index = 6)]
					set_fee {
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						#[codec::codec(compact)]
						fee: ::core::primitive::u128,
					},
					#[codec::codec(index = 7)]
					set_account_id {
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						new: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 8)]
					set_fields {
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						fields: runtime_types::pallet_identity::types::BitFlags<
							runtime_types::pallet_identity::types::IdentityField,
						>,
					},
					#[codec::codec(index = 9)]
					provide_judgement {
						#[codec::codec(compact)]
						reg_index: ::core::primitive::u32,
						target: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						judgement: runtime_types::pallet_identity::types::Judgement<
							::core::primitive::u128,
						>,
						identity: runtime_types::primitive_types::H256,
					},
					#[codec::codec(index = 10)]
					kill_identity {
						target: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 11)]
					add_sub {
						sub: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						data: runtime_types::pallet_identity::types::Data,
					},
					#[codec::codec(index = 12)]
					rename_sub {
						sub: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						data: runtime_types::pallet_identity::types::Data,
					},
					#[codec::codec(index = 13)]
					remove_sub {
						sub: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 14)]
					quit_sub,
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					TooManySubAccounts,
					#[codec::codec(index = 1)]
					NotFound,
					#[codec::codec(index = 2)]
					NotNamed,
					#[codec::codec(index = 3)]
					EmptyIndex,
					#[codec::codec(index = 4)]
					FeeChanged,
					#[codec::codec(index = 5)]
					NoIdentity,
					#[codec::codec(index = 6)]
					StickyJudgement,
					#[codec::codec(index = 7)]
					JudgementGiven,
					#[codec::codec(index = 8)]
					InvalidJudgement,
					#[codec::codec(index = 9)]
					InvalidIndex,
					#[codec::codec(index = 10)]
					InvalidTarget,
					#[codec::codec(index = 11)]
					TooManyFields,
					#[codec::codec(index = 12)]
					TooManyRegistrars,
					#[codec::codec(index = 13)]
					AlreadyClaimed,
					#[codec::codec(index = 14)]
					NotSub,
					#[codec::codec(index = 15)]
					NotOwned,
					#[codec::codec(index = 16)]
					JudgementForDifferentIdentity,
					#[codec::codec(index = 17)]
					JudgementPaymentFailed,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					IdentitySet { who: runtime_types::sp_core::crypto::AccountId32 },
					#[codec::codec(index = 1)]
					IdentityCleared {
						who: runtime_types::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec::codec(index = 2)]
					IdentityKilled {
						who: runtime_types::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec::codec(index = 3)]
					JudgementRequested {
						who: runtime_types::sp_core::crypto::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec::codec(index = 4)]
					JudgementUnrequested {
						who: runtime_types::sp_core::crypto::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec::codec(index = 5)]
					JudgementGiven {
						target: runtime_types::sp_core::crypto::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec::codec(index = 6)]
					RegistrarAdded { registrar_index: ::core::primitive::u32 },
					#[codec::codec(index = 7)]
					SubIdentityAdded {
						sub: runtime_types::sp_core::crypto::AccountId32,
						main: runtime_types::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec::codec(index = 8)]
					SubIdentityRemoved {
						sub: runtime_types::sp_core::crypto::AccountId32,
						main: runtime_types::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec::codec(index = 9)]
					SubIdentityRevoked {
						sub: runtime_types::sp_core::crypto::AccountId32,
						main: runtime_types::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				// #[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct BitFlags<_0>(
					pub ::core::primitive::u64,
					#[codec::codec(skip)] pub ::core::marker::PhantomData<_0>,
				);
				pub enum Data {
					#[codec::codec(index = 0)]
					None,
					#[codec::codec(index = 1)]
					Raw0([::core::primitive::u8; 0usize]),
					#[codec::codec(index = 2)]
					Raw1([::core::primitive::u8; 1usize]),
					#[codec::codec(index = 3)]
					Raw2([::core::primitive::u8; 2usize]),
					#[codec::codec(index = 4)]
					Raw3([::core::primitive::u8; 3usize]),
					#[codec::codec(index = 5)]
					Raw4([::core::primitive::u8; 4usize]),
					#[codec::codec(index = 6)]
					Raw5([::core::primitive::u8; 5usize]),
					#[codec::codec(index = 7)]
					Raw6([::core::primitive::u8; 6usize]),
					#[codec::codec(index = 8)]
					Raw7([::core::primitive::u8; 7usize]),
					#[codec::codec(index = 9)]
					Raw8([::core::primitive::u8; 8usize]),
					#[codec::codec(index = 10)]
					Raw9([::core::primitive::u8; 9usize]),
					#[codec::codec(index = 11)]
					Raw10([::core::primitive::u8; 10usize]),
					#[codec::codec(index = 12)]
					Raw11([::core::primitive::u8; 11usize]),
					#[codec::codec(index = 13)]
					Raw12([::core::primitive::u8; 12usize]),
					#[codec::codec(index = 14)]
					Raw13([::core::primitive::u8; 13usize]),
					#[codec::codec(index = 15)]
					Raw14([::core::primitive::u8; 14usize]),
					#[codec::codec(index = 16)]
					Raw15([::core::primitive::u8; 15usize]),
					#[codec::codec(index = 17)]
					Raw16([::core::primitive::u8; 16usize]),
					#[codec::codec(index = 18)]
					Raw17([::core::primitive::u8; 17usize]),
					#[codec::codec(index = 19)]
					Raw18([::core::primitive::u8; 18usize]),
					#[codec::codec(index = 20)]
					Raw19([::core::primitive::u8; 19usize]),
					#[codec::codec(index = 21)]
					Raw20([::core::primitive::u8; 20usize]),
					#[codec::codec(index = 22)]
					Raw21([::core::primitive::u8; 21usize]),
					#[codec::codec(index = 23)]
					Raw22([::core::primitive::u8; 22usize]),
					#[codec::codec(index = 24)]
					Raw23([::core::primitive::u8; 23usize]),
					#[codec::codec(index = 25)]
					Raw24([::core::primitive::u8; 24usize]),
					#[codec::codec(index = 26)]
					Raw25([::core::primitive::u8; 25usize]),
					#[codec::codec(index = 27)]
					Raw26([::core::primitive::u8; 26usize]),
					#[codec::codec(index = 28)]
					Raw27([::core::primitive::u8; 27usize]),
					#[codec::codec(index = 29)]
					Raw28([::core::primitive::u8; 28usize]),
					#[codec::codec(index = 30)]
					Raw29([::core::primitive::u8; 29usize]),
					#[codec::codec(index = 31)]
					Raw30([::core::primitive::u8; 30usize]),
					#[codec::codec(index = 32)]
					Raw31([::core::primitive::u8; 31usize]),
					#[codec::codec(index = 33)]
					Raw32([::core::primitive::u8; 32usize]),
					#[codec::codec(index = 34)]
					BlakeTwo256([::core::primitive::u8; 32usize]),
					#[codec::codec(index = 35)]
					Sha256([::core::primitive::u8; 32usize]),
					#[codec::codec(index = 36)]
					Keccak256([::core::primitive::u8; 32usize]),
					#[codec::codec(index = 37)]
					ShaThree256([::core::primitive::u8; 32usize]),
				}
				pub enum IdentityField {
					#[codec::codec(index = 1)]
					Display,
					#[codec::codec(index = 2)]
					Legal,
					#[codec::codec(index = 4)]
					Web,
					#[codec::codec(index = 8)]
					Riot,
					#[codec::codec(index = 16)]
					Email,
					#[codec::codec(index = 32)]
					PgpFingerprint,
					#[codec::codec(index = 64)]
					Image,
					#[codec::codec(index = 128)]
					Twitter,
				}
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
				pub enum Judgement<_0> {
					#[codec::codec(index = 0)]
					Unknown,
					#[codec::codec(index = 1)]
					FeePaid(_0),
					#[codec::codec(index = 2)]
					Reasonable,
					#[codec::codec(index = 3)]
					KnownGood,
					#[codec::codec(index = 4)]
					OutOfDate,
					#[codec::codec(index = 5)]
					LowQuality,
					#[codec::codec(index = 6)]
					Erroneous,
				}
				pub struct RegistrarInfo<_0, _1> {
					pub account: _1,
					pub fee: _0,
					pub fields: runtime_types::pallet_identity::types::BitFlags<
						runtime_types::pallet_identity::types::IdentityField,
					>,
				}
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
		pub mod pallet_indices {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					claim { index: ::core::primitive::u32 },
					#[codec::codec(index = 1)]
					transfer {
						new: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						index: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					free { index: ::core::primitive::u32 },
					#[codec::codec(index = 3)]
					force_transfer {
						new: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						index: ::core::primitive::u32,
						freeze: ::core::primitive::bool,
					},
					#[codec::codec(index = 4)]
					freeze { index: ::core::primitive::u32 },
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					NotAssigned,
					#[codec::codec(index = 1)]
					NotOwner,
					#[codec::codec(index = 2)]
					InUse,
					#[codec::codec(index = 3)]
					NotTransfer,
					#[codec::codec(index = 4)]
					Permanent,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					IndexAssigned {
						who: runtime_types::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec::codec(index = 1)]
					IndexFreed { index: ::core::primitive::u32 },
					#[codec::codec(index = 2)]
					IndexFrozen {
						index: ::core::primitive::u32,
						who: runtime_types::sp_core::crypto::AccountId32,
					},
				}
			}
		}
		pub mod pallet_membership {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					add_member {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 1)]
					remove_member {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 2)]
					swap_member {
						remove: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						add: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 3)]
					reset_members {
						members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					},
					#[codec::codec(index = 4)]
					change_key {
						new: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 5)]
					set_prime {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 6)]
					clear_prime,
				}
				pub enum Call2 {
					#[codec::codec(index = 0)]
					add_member {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 1)]
					remove_member {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 2)]
					swap_member {
						remove: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						add: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 3)]
					reset_members {
						members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					},
					#[codec::codec(index = 4)]
					change_key {
						new: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 5)]
					set_prime {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 6)]
					clear_prime,
				}
				pub enum Call3 {
					#[codec::codec(index = 0)]
					add_member {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 1)]
					remove_member {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 2)]
					swap_member {
						remove: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						add: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 3)]
					reset_members {
						members: ::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
					},
					#[codec::codec(index = 4)]
					change_key {
						new: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 5)]
					set_prime {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 6)]
					clear_prime,
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					AlreadyMember,
					#[codec::codec(index = 1)]
					NotMember,
					#[codec::codec(index = 2)]
					TooManyMembers,
				}
				pub enum Error2 {
					#[codec::codec(index = 0)]
					AlreadyMember,
					#[codec::codec(index = 1)]
					NotMember,
					#[codec::codec(index = 2)]
					TooManyMembers,
				}
				pub enum Error3 {
					#[codec::codec(index = 0)]
					AlreadyMember,
					#[codec::codec(index = 1)]
					NotMember,
					#[codec::codec(index = 2)]
					TooManyMembers,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					MemberAdded,
					#[codec::codec(index = 1)]
					MemberRemoved,
					#[codec::codec(index = 2)]
					MembersSwapped,
					#[codec::codec(index = 3)]
					MembersReset,
					#[codec::codec(index = 4)]
					KeyChanged,
					#[codec::codec(index = 5)]
					Dummy,
				}
				pub enum Event2 {
					#[codec::codec(index = 0)]
					MemberAdded,
					#[codec::codec(index = 1)]
					MemberRemoved,
					#[codec::codec(index = 2)]
					MembersSwapped,
					#[codec::codec(index = 3)]
					MembersReset,
					#[codec::codec(index = 4)]
					KeyChanged,
					#[codec::codec(index = 5)]
					Dummy,
				}
				pub enum Event3 {
					#[codec::codec(index = 0)]
					MemberAdded,
					#[codec::codec(index = 1)]
					MemberRemoved,
					#[codec::codec(index = 2)]
					MembersSwapped,
					#[codec::codec(index = 3)]
					MembersReset,
					#[codec::codec(index = 4)]
					KeyChanged,
					#[codec::codec(index = 5)]
					Dummy,
				}
			}
		}
		pub mod pallet_multihop_xcm_ibc {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
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
				pub enum Error {
					#[codec::codec(index = 0)]
					IncorrectAddress { chain_id: ::core::primitive::u8 },
					#[codec::codec(index = 1)]
					IncorrectChainName { chain_id: ::core::primitive::u8 },
					#[codec::codec(index = 2)]
					FailedToEncodeBech32Address { chain_id: ::core::primitive::u8 },
					#[codec::codec(index = 3)]
					IncorrectMultiLocation,
					#[codec::codec(index = 4)]
					XcmDepositFailed,
					#[codec::codec(index = 5)]
					MultiHopRouteDoesNotExist,
					#[codec::codec(index = 6)]
					DoesNotSupportNonFungible,
					#[codec::codec(index = 7)]
					IncorrectCountOfAddresses,
					#[codec::codec(index = 8)]
					FailedToConstructMemo,
					#[codec::codec(index = 9)]
					FailedToDecodeAccountId,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					SuccessXcmToIbc {
						origin_address: runtime_types::sp_core::crypto::AccountId32,
						to: [::core::primitive::u8; 32usize],
						amount: ::core::primitive::u128,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						memo: ::core::option::Option<::std::string::String>,
					},
					#[codec::codec(index = 1)]
					FailedXcmToIbc {
						origin_address: runtime_types::sp_core::crypto::AccountId32,
						to: [::core::primitive::u8; 32usize],
						amount: ::core::primitive::u128,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						memo: ::core::option::Option<::std::string::String>,
					},
					#[codec::codec(index = 2)]
					FailedCallback {
						origin_address: [::core::primitive::u8; 32usize],
						route_id: ::core::primitive::u128,
						reason: runtime_types::pallet_multihop_xcm_ibc::pallet::MultihopEventReason,
					},
					#[codec::codec(index = 3)]
					MultihopXcmMemo {
						reason: runtime_types::pallet_multihop_xcm_ibc::pallet::MultihopEventReason,
						from: runtime_types::sp_core::crypto::AccountId32,
						to: runtime_types::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						asset_id: ::core::primitive::u128,
						is_error: ::core::primitive::bool,
					},
					#[codec::codec(index = 4)]
					FailedMatchLocation,
				}
				pub enum MultihopEventReason {
					#[codec::codec(index = 0)]
					FailedToConvertAddressToBytes,
					#[codec::codec(index = 1)]
					XcmTransferInitiated,
					#[codec::codec(index = 2)]
					IncorrectPalletId,
					#[codec::codec(index = 3)]
					MultiHopRouteDoesNotExist,
					#[codec::codec(index = 4)]
					MultiHopRouteExistButNotConfigured,
					#[codec::codec(index = 5)]
					IncorrectCountOfAddresses,
					#[codec::codec(index = 6)]
					FailedToDeriveCosmosAddressFromBytes,
					#[codec::codec(index = 7)]
					FailedToDeriveChainNameFromUtf8,
					#[codec::codec(index = 8)]
					FailedToEncodeBech32Address,
					#[codec::codec(index = 9)]
					FailedToDecodeDestAccountId,
					#[codec::codec(index = 10)]
					FailedToDecodeSenderAccountId,
					#[codec::codec(index = 11)]
					DoesNotSupportNonFungible,
					#[codec::codec(index = 12)]
					FailedCreateMemo,
					#[codec::codec(index = 13)]
					FailedToConvertMemoIntoPalletIbcMemoMessageType,
				}
			}
		}
		pub mod pallet_multisig {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					as_multi_threshold_1 {
						other_signatories:
							::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					},
					#[codec::codec(index = 1)]
					as_multi {
						threshold: ::core::primitive::u16,
						other_signatories:
							::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
						maybe_timepoint: ::core::option::Option<
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						>,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 2)]
					approve_as_multi {
						threshold: ::core::primitive::u16,
						other_signatories:
							::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
						maybe_timepoint: ::core::option::Option<
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						>,
						call_hash: [::core::primitive::u8; 32usize],
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 3)]
					cancel_as_multi {
						threshold: ::core::primitive::u16,
						other_signatories:
							::std::vec::Vec<runtime_types::sp_core::crypto::AccountId32>,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						call_hash: [::core::primitive::u8; 32usize],
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					MinimumThreshold,
					#[codec::codec(index = 1)]
					AlreadyApproved,
					#[codec::codec(index = 2)]
					NoApprovalsNeeded,
					#[codec::codec(index = 3)]
					TooFewSignatories,
					#[codec::codec(index = 4)]
					TooManySignatories,
					#[codec::codec(index = 5)]
					SignatoriesOutOfOrder,
					#[codec::codec(index = 6)]
					SenderInSignatories,
					#[codec::codec(index = 7)]
					NotFound,
					#[codec::codec(index = 8)]
					NotOwner,
					#[codec::codec(index = 9)]
					NoTimepoint,
					#[codec::codec(index = 10)]
					WrongTimepoint,
					#[codec::codec(index = 11)]
					UnexpectedTimepoint,
					#[codec::codec(index = 12)]
					MaxWeightTooLow,
					#[codec::codec(index = 13)]
					AlreadyStored,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					NewMultisig {
						approving: runtime_types::sp_core::crypto::AccountId32,
						multisig: runtime_types::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec::codec(index = 1)]
					MultisigApproval {
						approving: runtime_types::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: runtime_types::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec::codec(index = 2)]
					MultisigExecuted {
						approving: runtime_types::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: runtime_types::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec::codec(index = 3)]
					MultisigCancelled {
						cancelling: runtime_types::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: runtime_types::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
				}
			}
			pub struct Multisig<_0, _1, _2> {
				pub when: runtime_types::pallet_multisig::Timepoint<_0>,
				pub deposit: _1,
				pub depositor: _2,
				pub approvals: runtime_types::bounded_collections::bounded_vec::BoundedVec<_2>,
			}
			pub struct Timepoint<_0> {
				pub height: _0,
				pub index: ::core::primitive::u32,
			}
		}
		pub mod pallet_oracle {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub struct AssetInfo<_0, _1, _2> {
					pub threshold: _0,
					pub min_answers: ::core::primitive::u32,
					pub max_answers: ::core::primitive::u32,
					pub block_interval: _1,
					pub reward_weight: _2,
					pub slash: _2,
					pub emit_price_changes: ::core::primitive::bool,
				}
				pub enum Call {
					#[codec::codec(index = 0)]
					add_asset_and_info {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						threshold: runtime_types::sp_arithmetic::per_things::Percent,
						min_answers: ::core::primitive::u32,
						max_answers: ::core::primitive::u32,
						block_interval: ::core::primitive::u32,
						reward_weight: ::core::primitive::u128,
						slash: ::core::primitive::u128,
						emit_price_changes: ::core::primitive::bool,
					},
					#[codec::codec(index = 1)]
					set_signer {
						who: runtime_types::sp_core::crypto::AccountId32,
						signer: runtime_types::sp_core::crypto::AccountId32,
					},
					#[codec::codec(index = 2)]
					adjust_rewards {
						annual_cost_per_oracle: ::core::primitive::u128,
						num_ideal_oracles: ::core::primitive::u8,
					},
					#[codec::codec(index = 3)]
					add_stake { stake: ::core::primitive::u128 },
					#[codec::codec(index = 4)]
					remove_stake,
					#[codec::codec(index = 5)]
					reclaim_stake,
					#[codec::codec(index = 6)]
					submit_price {
						price: ::core::primitive::u128,
						asset_id: runtime_types::primitives::currency::CurrencyId,
					},
					#[codec::codec(index = 7)]
					remove_signer { who: runtime_types::sp_core::crypto::AccountId32 },
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					Unknown,
					#[codec::codec(index = 1)]
					NoPermission,
					#[codec::codec(index = 2)]
					NoStake,
					#[codec::codec(index = 3)]
					StakeLocked,
					#[codec::codec(index = 4)]
					NotEnoughStake,
					#[codec::codec(index = 5)]
					NotEnoughFunds,
					#[codec::codec(index = 6)]
					InvalidAssetId,
					#[codec::codec(index = 7)]
					AlreadySubmitted,
					#[codec::codec(index = 8)]
					MaxPrices,
					#[codec::codec(index = 9)]
					PriceNotRequested,
					#[codec::codec(index = 10)]
					UnsetSigner,
					#[codec::codec(index = 11)]
					AlreadySet,
					#[codec::codec(index = 12)]
					UnsetController,
					#[codec::codec(index = 13)]
					ControllerUsed,
					#[codec::codec(index = 14)]
					SignerUsed,
					#[codec::codec(index = 15)]
					AvoidPanic,
					#[codec::codec(index = 16)]
					ExceedMaxAnswers,
					#[codec::codec(index = 17)]
					InvalidMinAnswers,
					#[codec::codec(index = 18)]
					MaxAnswersLessThanMinAnswers,
					#[codec::codec(index = 19)]
					ExceedThreshold,
					#[codec::codec(index = 20)]
					ExceedAssetsCount,
					#[codec::codec(index = 21)]
					PriceNotFound,
					#[codec::codec(index = 22)]
					ExceedStake,
					#[codec::codec(index = 23)]
					MustSumTo100,
					#[codec::codec(index = 24)]
					DepthTooLarge,
					#[codec::codec(index = 25)]
					ArithmeticError,
					#[codec::codec(index = 26)]
					BlockIntervalLength,
					#[codec::codec(index = 27)]
					TransferError,
					#[codec::codec(index = 28)]
					MaxHistory,
					#[codec::codec(index = 29)]
					MaxPrePrices,
					#[codec::codec(index = 30)]
					NoRewardTrackerSet,
					#[codec::codec(index = 31)]
					AnnualRewardLessThanAlreadyRewarded,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					AssetInfoChange(
						runtime_types::primitives::currency::CurrencyId,
						runtime_types::sp_arithmetic::per_things::Percent,
						::core::primitive::u32,
						::core::primitive::u32,
						::core::primitive::u32,
						::core::primitive::u128,
						::core::primitive::u128,
					),
					#[codec::codec(index = 1)]
					SignerSet(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::sp_core::crypto::AccountId32,
					),
					#[codec::codec(index = 2)]
					StakeAdded(
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::u128,
					),
					#[codec::codec(index = 3)]
					StakeRemoved(
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::u32,
					),
					#[codec::codec(index = 4)]
					StakeReclaimed(
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec::codec(index = 5)]
					PriceSubmitted(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					),
					#[codec::codec(index = 6)]
					UserSlashed(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					),
					#[codec::codec(index = 7)]
					OracleRewarded(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					),
					#[codec::codec(index = 8)]
					RewardingAdjustment(::core::primitive::u64),
					#[codec::codec(index = 9)]
					AnswerPruned(
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec::codec(index = 10)]
					PriceChanged(
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					),
					#[codec::codec(index = 11)]
					SignerRemoved(
						runtime_types::sp_core::crypto::AccountId32,
						runtime_types::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
				}
				pub struct PrePrice<_0, _1, _2> {
					pub price: _0,
					pub block: _1,
					pub who: _2,
				}
				pub struct Withdraw<_0, _1> {
					pub stake: _0,
					pub unlock_block: _1,
				}
			}
		}
		pub mod pallet_pablo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					create {
						pool: runtime_types::pallet_pablo::pallet::PoolInitConfiguration<
							runtime_types::sp_core::crypto::AccountId32,
							runtime_types::primitives::currency::CurrencyId,
						>,
					},
					#[codec::codec(index = 1)]
					buy {
						pool_id: ::core::primitive::u128,
						in_asset_id: runtime_types::primitives::currency::CurrencyId,
						out_asset: runtime_types::composable_traits::dex::AssetAmount<
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 2)]
					swap {
						pool_id: ::core::primitive::u128,
						in_asset: runtime_types::composable_traits::dex::AssetAmount<
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
						>,
						min_receive: runtime_types::composable_traits::dex::AssetAmount<
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 3)]
					add_liquidity {
						pool_id: ::core::primitive::u128,
						assets: ::std::collections::BTreeMap<
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
						>,
						min_mint_amount: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec::codec(index = 4)]
					remove_liquidity {
						pool_id: ::core::primitive::u128,
						lp_amount: ::core::primitive::u128,
						min_receive: ::std::collections::BTreeMap<
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
						>,
					},
					#[codec::codec(index = 5)]
					enable_twap { pool_id: ::core::primitive::u128 },
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					PoolNotFound,
					#[codec::codec(index = 1)]
					NotEnoughLiquidity,
					#[codec::codec(index = 2)]
					NotEnoughLpToken,
					#[codec::codec(index = 3)]
					PairMismatch,
					#[codec::codec(index = 4)]
					AssetNotFound,
					#[codec::codec(index = 5)]
					MustBeOwner,
					#[codec::codec(index = 6)]
					InvalidSaleState,
					#[codec::codec(index = 7)]
					InvalidAmount,
					#[codec::codec(index = 8)]
					InvalidAsset,
					#[codec::codec(index = 9)]
					CannotRespectMinimumRequested,
					#[codec::codec(index = 10)]
					AssetAmountMustBePositiveNumber,
					#[codec::codec(index = 11)]
					InvalidPair,
					#[codec::codec(index = 12)]
					InvalidFees,
					#[codec::codec(index = 13)]
					AmpFactorMustBeGreaterThanZero,
					#[codec::codec(index = 14)]
					MissingAmount,
					#[codec::codec(index = 15)]
					MissingMinExpectedAmount,
					#[codec::codec(index = 16)]
					MoreThanTwoAssetsNotYetSupported,
					#[codec::codec(index = 17)]
					NoLpTokenForLbp,
					#[codec::codec(index = 18)]
					NoXTokenForLbp,
					#[codec::codec(index = 19)]
					WeightsMustBeNonZero,
					#[codec::codec(index = 20)]
					WeightsMustSumToOne,
					#[codec::codec(index = 21)]
					StakingPoolConfigError,
					#[codec::codec(index = 22)]
					IncorrectAssetAmounts,
					#[codec::codec(index = 23)]
					UnsupportedOperation,
					#[codec::codec(index = 24)]
					InitialDepositCannotBeZero,
					#[codec::codec(index = 25)]
					InitialDepositMustContainAllAssets,
					#[codec::codec(index = 26)]
					MinAmountsMustContainAtLeastOneAsset,
					#[codec::codec(index = 27)]
					MustDepositMinimumOneAsset,
					#[codec::codec(index = 28)]
					CannotSwapSameAsset,
					#[codec::codec(index = 29)]
					CannotBuyAssetWithItself,
					#[codec::codec(index = 30)]
					IncorrectPoolConfig,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					PoolCreated {
						pool_id: ::core::primitive::u128,
						owner: runtime_types::sp_core::crypto::AccountId32,
						asset_weights: ::std::collections::BTreeMap<
							runtime_types::primitives::currency::CurrencyId,
							runtime_types::sp_arithmetic::per_things::Permill,
						>,
						lp_token_id: runtime_types::primitives::currency::CurrencyId,
					},
					#[codec::codec(index = 1)]
					LiquidityAdded {
						who: runtime_types::sp_core::crypto::AccountId32,
						pool_id: ::core::primitive::u128,
						asset_amounts: ::std::collections::BTreeMap<
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
						>,
						minted_lp: ::core::primitive::u128,
					},
					#[codec::codec(index = 2)]
					LiquidityRemoved {
						who: runtime_types::sp_core::crypto::AccountId32,
						pool_id: ::core::primitive::u128,
						asset_amounts: ::std::collections::BTreeMap<
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
						>,
					},
					#[codec::codec(index = 3)]
					Swapped {
						pool_id: ::core::primitive::u128,
						who: runtime_types::sp_core::crypto::AccountId32,
						base_asset: runtime_types::primitives::currency::CurrencyId,
						quote_asset: runtime_types::primitives::currency::CurrencyId,
						base_amount: ::core::primitive::u128,
						quote_amount: ::core::primitive::u128,
						fee: runtime_types::composable_traits::dex::Fee<
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
						>,
					},
					#[codec::codec(index = 4)]
					TwapUpdated {
						pool_id: ::core::primitive::u128,
						timestamp: ::core::primitive::u64,
						twaps: ::std::collections::BTreeMap<
							runtime_types::primitives::currency::CurrencyId,
							runtime_types::sp_arithmetic::fixed_point::FixedU128,
						>,
					},
				}
				pub enum PoolConfiguration<_0, _1> {
					#[codec::codec(index = 0)]
					DualAssetConstantProduct(
						runtime_types::composable_traits::dex::BasicPoolInfo<_0, _1>,
					),
				}
				pub enum PoolInitConfiguration<_0, _1> {
					#[codec::codec(index = 0)]
					DualAssetConstantProduct {
						owner: _0,
						assets_weights: ::std::vec::Vec<(
							_1,
							runtime_types::sp_arithmetic::per_things::Permill,
						)>,
						fee: runtime_types::sp_arithmetic::per_things::Permill,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				pub struct PriceCumulative<_0, _1> {
					pub timestamp: _0,
					pub base_price_cumulative: _1,
					pub quote_price_cumulative: _1,
				}
				pub struct TimeWeightedAveragePrice<_0, _1> {
					pub timestamp: _0,
					pub base_price_cumulative: _1,
					pub quote_price_cumulative: _1,
					pub base_twap: runtime_types::sp_arithmetic::fixed_point::FixedU128,
					pub quote_twap: runtime_types::sp_arithmetic::fixed_point::FixedU128,
				}
			}
		}
		pub mod pallet_preimage {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					note_preimage { bytes: ::std::vec::Vec<::core::primitive::u8> },
					#[codec::codec(index = 1)]
					unnote_preimage { hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 2)]
					request_preimage { hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 3)]
					unrequest_preimage { hash: runtime_types::primitive_types::H256 },
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					TooBig,
					#[codec::codec(index = 1)]
					AlreadyNoted,
					#[codec::codec(index = 2)]
					NotAuthorized,
					#[codec::codec(index = 3)]
					NotNoted,
					#[codec::codec(index = 4)]
					Requested,
					#[codec::codec(index = 5)]
					NotRequested,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Noted { hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 1)]
					Requested { hash: runtime_types::primitive_types::H256 },
					#[codec::codec(index = 2)]
					Cleared { hash: runtime_types::primitive_types::H256 },
				}
			}
			pub enum RequestStatus<_0, _1> {
				#[codec::codec(index = 0)]
				Unrequested { deposit: (_0, _1), len: ::core::primitive::u32 },
				#[codec::codec(index = 1)]
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
				pub enum Call {
					#[codec::codec(index = 0)]
					proxy {
						real: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						force_proxy_type: ::core::option::Option<
							runtime_types::composable_traits::account_proxy::ProxyType,
						>,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					},
					#[codec::codec(index = 1)]
					add_proxy {
						delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					remove_proxy {
						delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec::codec(index = 3)]
					remove_proxies,
					#[codec::codec(index = 4)]
					create_pure {
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
						index: ::core::primitive::u16,
					},
					#[codec::codec(index = 5)]
					kill_pure {
						spawner: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						index: ::core::primitive::u16,
						#[codec::codec(compact)]
						height: ::core::primitive::u32,
						#[codec::codec(compact)]
						ext_index: ::core::primitive::u32,
					},
					#[codec::codec(index = 6)]
					announce {
						real: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						call_hash: runtime_types::primitive_types::H256,
					},
					#[codec::codec(index = 7)]
					remove_announcement {
						real: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						call_hash: runtime_types::primitive_types::H256,
					},
					#[codec::codec(index = 8)]
					reject_announcement {
						delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						call_hash: runtime_types::primitive_types::H256,
					},
					#[codec::codec(index = 9)]
					proxy_announced {
						delegate: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						real: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						force_proxy_type: ::core::option::Option<
							runtime_types::composable_traits::account_proxy::ProxyType,
						>,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					TooMany,
					#[codec::codec(index = 1)]
					NotFound,
					#[codec::codec(index = 2)]
					NotProxy,
					#[codec::codec(index = 3)]
					Unproxyable,
					#[codec::codec(index = 4)]
					Duplicate,
					#[codec::codec(index = 5)]
					NoPermission,
					#[codec::codec(index = 6)]
					Unannounced,
					#[codec::codec(index = 7)]
					NoSelfProxy,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					ProxyExecuted {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec::codec(index = 1)]
					PureCreated {
						pure: runtime_types::sp_core::crypto::AccountId32,
						who: runtime_types::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						disambiguation_index: ::core::primitive::u16,
					},
					#[codec::codec(index = 2)]
					Announced {
						real: runtime_types::sp_core::crypto::AccountId32,
						proxy: runtime_types::sp_core::crypto::AccountId32,
						call_hash: runtime_types::primitive_types::H256,
					},
					#[codec::codec(index = 3)]
					ProxyAdded {
						delegator: runtime_types::sp_core::crypto::AccountId32,
						delegatee: runtime_types::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec::codec(index = 4)]
					ProxyRemoved {
						delegator: runtime_types::sp_core::crypto::AccountId32,
						delegatee: runtime_types::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
				}
			}
			pub struct Announcement<_0, _1, _2> {
				pub real: _0,
				pub call_hash: _1,
				pub height: _2,
			}
			pub struct ProxyDefinition<_0, _1, _2> {
				pub delegate: _0,
				pub proxy_type: _1,
				pub delay: _2,
			}
		}
		pub mod pallet_scheduler {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					schedule {
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					},
					#[codec::codec(index = 1)]
					cancel { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec::codec(index = 2)]
					schedule_named {
						id: [::core::primitive::u8; 32usize],
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					},
					#[codec::codec(index = 3)]
					cancel_named { id: [::core::primitive::u8; 32usize] },
					#[codec::codec(index = 4)]
					schedule_after {
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					},
					#[codec::codec(index = 5)]
					schedule_named_after {
						id: [::core::primitive::u8; 32usize],
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					FailedToSchedule,
					#[codec::codec(index = 1)]
					NotFound,
					#[codec::codec(index = 2)]
					TargetBlockNumberInPast,
					#[codec::codec(index = 3)]
					RescheduleNoChange,
					#[codec::codec(index = 4)]
					Named,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Scheduled { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec::codec(index = 1)]
					Canceled { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec::codec(index = 2)]
					Dispatched {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec::codec(index = 3)]
					CallUnavailable {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec::codec(index = 4)]
					PeriodicFailed {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec::codec(index = 5)]
					PermanentlyOverweight {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			pub struct Scheduled<_0, _1, _2, _3, _4> {
				pub maybe_id: ::core::option::Option<_0>,
				pub priority: ::core::primitive::u8,
				pub call: _1,
				pub maybe_periodic: ::core::option::Option<(_2, _2)>,
				pub origin: _3,
				#[codec::codec(skip)]
				pub __subxt_unused_type_params: ::core::marker::PhantomData<_4>,
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					set_keys {
						keys: runtime_types::picasso_runtime::opaque::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec::codec(index = 1)]
					purge_keys,
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					InvalidProof,
					#[codec::codec(index = 1)]
					NoAssociatedValidatorId,
					#[codec::codec(index = 2)]
					DuplicatedKey,
					#[codec::codec(index = 3)]
					NoKeys,
					#[codec::codec(index = 4)]
					NoAccount,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					NewSession { session_index: ::core::primitive::u32 },
				}
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					sudo { call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall> },
					#[codec::codec(index = 1)]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 2)]
					set_key {
						new: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 3)]
					sudo_as {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					RequireSudo,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec::codec(index = 1)]
					KeyChanged {
						old_sudoer:
							::core::option::Option<runtime_types::sp_core::crypto::AccountId32>,
					},
					#[codec::codec(index = 2)]
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
				pub enum Call {
					#[codec::codec(index = 0)]
					set {
						#[codec::codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Event {
					#[codec::codec(index = 0)]
					TransactionFeePaid {
						who: runtime_types::sp_core::crypto::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			pub enum Releases {
				#[codec::codec(index = 0)]
				V1Ancient,
				#[codec::codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_treasury {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					propose_spend {
						#[codec::codec(compact)]
						value: ::core::primitive::u128,
						beneficiary: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 1)]
					reject_proposal {
						#[codec::codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					approve_proposal {
						#[codec::codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec::codec(index = 3)]
					spend {
						#[codec::codec(compact)]
						amount: ::core::primitive::u128,
						beneficiary: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec::codec(index = 4)]
					remove_approval {
						#[codec::codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					InsufficientProposersBalance,
					#[codec::codec(index = 1)]
					InvalidIndex,
					#[codec::codec(index = 2)]
					TooManyApprovals,
					#[codec::codec(index = 3)]
					InsufficientPermission,
					#[codec::codec(index = 4)]
					ProposalNotApproved,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Proposed { proposal_index: ::core::primitive::u32 },
					#[codec::codec(index = 1)]
					Spending { budget_remaining: ::core::primitive::u128 },
					#[codec::codec(index = 2)]
					Awarded {
						proposal_index: ::core::primitive::u32,
						award: ::core::primitive::u128,
						account: runtime_types::sp_core::crypto::AccountId32,
					},
					#[codec::codec(index = 3)]
					Rejected {
						proposal_index: ::core::primitive::u32,
						slashed: ::core::primitive::u128,
					},
					#[codec::codec(index = 4)]
					Burnt { burnt_funds: ::core::primitive::u128 },
					#[codec::codec(index = 5)]
					Rollover { rollover_balance: ::core::primitive::u128 },
					#[codec::codec(index = 6)]
					Deposit { value: ::core::primitive::u128 },
					#[codec::codec(index = 7)]
					SpendApproved {
						proposal_index: ::core::primitive::u32,
						amount: ::core::primitive::u128,
						beneficiary: runtime_types::sp_core::crypto::AccountId32,
					},
					#[codec::codec(index = 8)]
					UpdatedInactive {
						reactivated: ::core::primitive::u128,
						deactivated: ::core::primitive::u128,
					},
				}
			}
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
				pub enum Call {
					#[codec::codec(index = 0)]
					batch { calls: ::std::vec::Vec<runtime_types::picasso_runtime::RuntimeCall> },
					#[codec::codec(index = 1)]
					as_derivative {
						index: ::core::primitive::u16,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					},
					#[codec::codec(index = 2)]
					batch_all {
						calls: ::std::vec::Vec<runtime_types::picasso_runtime::RuntimeCall>,
					},
					#[codec::codec(index = 3)]
					dispatch_as {
						as_origin: ::std::boxed::Box<runtime_types::picasso_runtime::OriginCaller>,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
					},
					#[codec::codec(index = 4)]
					force_batch {
						calls: ::std::vec::Vec<runtime_types::picasso_runtime::RuntimeCall>,
					},
					#[codec::codec(index = 5)]
					with_weight {
						call: ::std::boxed::Box<runtime_types::picasso_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					TooManyCalls,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					BatchInterrupted {
						index: ::core::primitive::u32,
						error: runtime_types::sp_runtime::DispatchError,
					},
					#[codec::codec(index = 1)]
					BatchCompleted,
					#[codec::codec(index = 2)]
					BatchCompletedWithErrors,
					#[codec::codec(index = 3)]
					ItemCompleted,
					#[codec::codec(index = 4)]
					ItemFailed { error: runtime_types::sp_runtime::DispatchError },
					#[codec::codec(index = 5)]
					DispatchedAs {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_vesting {
			use super::runtime_types;
			pub mod module {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					claim {
						asset: runtime_types::primitives::currency::CurrencyId,
						vesting_schedule_ids:
							runtime_types::pallet_vesting::types::VestingScheduleIdSet<
								::core::primitive::u128,
							>,
					},
					#[codec::codec(index = 1)]
					vested_transfer {
						from: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						beneficiary: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						asset: runtime_types::primitives::currency::CurrencyId,
						schedule_info: runtime_types::pallet_vesting::types::VestingScheduleInfo<
							::core::primitive::u32,
							::core::primitive::u64,
							::core::primitive::u128,
						>,
					},
					#[codec::codec(index = 2)]
					update_vesting_schedules {
						who: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						asset: runtime_types::primitives::currency::CurrencyId,
						vesting_schedules: ::std::vec::Vec<
							runtime_types::pallet_vesting::types::VestingScheduleInfo<
								::core::primitive::u32,
								::core::primitive::u64,
								::core::primitive::u128,
							>,
						>,
					},
					#[codec::codec(index = 3)]
					claim_for {
						dest: runtime_types::sp_runtime::multiaddress::MultiAddress<
							runtime_types::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						asset: runtime_types::primitives::currency::CurrencyId,
						vesting_schedule_ids:
							runtime_types::pallet_vesting::types::VestingScheduleIdSet<
								::core::primitive::u128,
							>,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					ZeroVestingPeriod,
					#[codec::codec(index = 1)]
					ZeroVestingPeriodCount,
					#[codec::codec(index = 2)]
					InsufficientBalanceToLock,
					#[codec::codec(index = 3)]
					TooManyVestingSchedules,
					#[codec::codec(index = 4)]
					AmountLow,
					#[codec::codec(index = 5)]
					MaxVestingSchedulesExceeded,
					#[codec::codec(index = 6)]
					TryingToSelfVest,
					#[codec::codec(index = 7)]
					VestingScheduleNotFound,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					VestingScheduleAdded {
						from: runtime_types::sp_core::crypto::AccountId32,
						to: runtime_types::sp_core::crypto::AccountId32,
						asset: runtime_types::primitives::currency::CurrencyId,
						vesting_schedule_id: ::core::primitive::u128,
						schedule: runtime_types::pallet_vesting::types::VestingSchedule<
							::core::primitive::u128,
							::core::primitive::u32,
							::core::primitive::u64,
							::core::primitive::u128,
						>,
						schedule_amount: ::core::primitive::u128,
					},
					#[codec::codec(index = 1)]
					Claimed {
						who: runtime_types::sp_core::crypto::AccountId32,
						asset: runtime_types::primitives::currency::CurrencyId,
						vesting_schedule_ids:
							runtime_types::pallet_vesting::types::VestingScheduleIdSet<
								::core::primitive::u128,
							>,
						locked_amount: ::core::primitive::u128,
						claimed_amount_per_schedule:
							runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
								::core::primitive::u128,
								::core::primitive::u128,
							>,
					},
					#[codec::codec(index = 2)]
					VestingSchedulesUpdated { who: runtime_types::sp_core::crypto::AccountId32 },
				}
			}
			pub mod types {
				use super::runtime_types;
				pub struct VestingSchedule<_0, _1, _2, _3> {
					pub vesting_schedule_id: _0,
					pub window: runtime_types::pallet_vesting::types::VestingWindow<_1, _2>,
					pub period_count: ::core::primitive::u32,
					#[codec::codec(compact)]
					pub per_period: _0,
					pub already_claimed: _3,
				}
				pub enum VestingScheduleIdSet<_0> {
					#[codec::codec(index = 0)]
					All,
					#[codec::codec(index = 1)]
					One(_0),
					#[codec::codec(index = 2)]
					Many(runtime_types::bounded_collections::bounded_vec::BoundedVec<_0>),
				}
				pub struct VestingScheduleInfo<_0, _1, _2> {
					pub window: runtime_types::pallet_vesting::types::VestingWindow<_0, _1>,
					pub period_count: ::core::primitive::u32,
					#[codec::codec(compact)]
					pub per_period: _2,
				}
				pub enum VestingWindow<_0, _1> {
					#[codec::codec(index = 0)]
					MomentBased { start: _1, period: _1 },
					#[codec::codec(index = 1)]
					BlockNumberBased { start: _0, period: _0 },
				}
			}
		}
		pub mod pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {
					#[codec::codec(index = 0)]
					send {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
					},
					#[codec::codec(index = 1)]
					teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec::codec(index = 3)]
					execute {
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm2>,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 4)]
					force_xcm_version {
						location:
							::std::boxed::Box<runtime_types::xcm::v3::multilocation::MultiLocation>,
						xcm_version: ::core::primitive::u32,
					},
					#[codec::codec(index = 5)]
					force_default_xcm_version {
						maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec::codec(index = 6)]
					force_subscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec::codec(index = 7)]
					force_unsubscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec::codec(index = 8)]
					limited_reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec::codec(index = 9)]
					limited_teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
				}
				pub enum Error {
					#[codec::codec(index = 0)]
					Unreachable,
					#[codec::codec(index = 1)]
					SendFailure,
					#[codec::codec(index = 2)]
					Filtered,
					#[codec::codec(index = 3)]
					UnweighableMessage,
					#[codec::codec(index = 4)]
					DestinationNotInvertible,
					#[codec::codec(index = 5)]
					Empty,
					#[codec::codec(index = 6)]
					CannotReanchor,
					#[codec::codec(index = 7)]
					TooManyAssets,
					#[codec::codec(index = 8)]
					InvalidOrigin,
					#[codec::codec(index = 9)]
					BadVersion,
					#[codec::codec(index = 10)]
					BadLocation,
					#[codec::codec(index = 11)]
					NoSubscription,
					#[codec::codec(index = 12)]
					AlreadySubscribed,
					#[codec::codec(index = 13)]
					InvalidAsset,
					#[codec::codec(index = 14)]
					LowBalance,
					#[codec::codec(index = 15)]
					TooManyLocks,
					#[codec::codec(index = 16)]
					AccountNotSovereign,
					#[codec::codec(index = 17)]
					FeesNotMet,
					#[codec::codec(index = 18)]
					LockNotFound,
					#[codec::codec(index = 19)]
					InUse,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					Attempted(runtime_types::xcm::v3::traits::Outcome),
					#[codec::codec(index = 1)]
					Sent(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::Xcm,
					),
					#[codec::codec(index = 2)]
					UnexpectedResponse(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec::codec(index = 3)]
					ResponseReady(::core::primitive::u64, runtime_types::xcm::v3::Response),
					#[codec::codec(index = 4)]
					Notified(::core::primitive::u64, ::core::primitive::u8, ::core::primitive::u8),
					#[codec::codec(index = 5)]
					NotifyOverweight(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
						runtime_types::sp_weights::weight_v2::Weight,
						runtime_types::sp_weights::weight_v2::Weight,
					),
					#[codec::codec(index = 6)]
					NotifyDispatchError(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec::codec(index = 7)]
					NotifyDecodeFailed(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec::codec(index = 8)]
					InvalidResponder(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec::codec(index = 9)]
					InvalidResponderVersion(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec::codec(index = 10)]
					ResponseTaken(::core::primitive::u64),
					#[codec::codec(index = 11)]
					AssetsTrapped(
						runtime_types::primitive_types::H256,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
					#[codec::codec(index = 12)]
					VersionChangeNotified(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u32,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec::codec(index = 13)]
					SupportedVersionChanged(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec::codec(index = 14)]
					NotifyTargetSendFail(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v3::traits::Error,
					),
					#[codec::codec(index = 15)]
					NotifyTargetMigrationFail(
						runtime_types::xcm::VersionedMultiLocation,
						::core::primitive::u64,
					),
					#[codec::codec(index = 16)]
					InvalidQuerierVersion(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec::codec(index = 17)]
					InvalidQuerier(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec::codec(index = 18)]
					VersionNotifyStarted(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec::codec(index = 19)]
					VersionNotifyRequested(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec::codec(index = 20)]
					VersionNotifyUnrequested(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec::codec(index = 21)]
					FeesPaid(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec::codec(index = 22)]
					AssetsClaimed(
						runtime_types::primitive_types::H256,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
				}
				pub enum Origin {
					#[codec::codec(index = 0)]
					Xcm(runtime_types::xcm::v3::multilocation::MultiLocation),
					#[codec::codec(index = 1)]
					Response(runtime_types::xcm::v3::multilocation::MultiLocation),
				}
				pub enum QueryStatus<_0> {
					#[codec::codec(index = 0)]
					Pending {
						responder: runtime_types::xcm::VersionedMultiLocation,
						maybe_match_querier:
							::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
						maybe_notify:
							::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
						timeout: _0,
					},
					#[codec::codec(index = 1)]
					VersionNotifier {
						origin: runtime_types::xcm::VersionedMultiLocation,
						is_active: ::core::primitive::bool,
					},
					#[codec::codec(index = 2)]
					Ready { response: runtime_types::xcm::VersionedResponse, at: _0 },
				}
				pub struct RemoteLockedFungibleRecord {
					pub amount: ::core::primitive::u128,
					pub owner: runtime_types::xcm::VersionedMultiLocation,
					pub locker: runtime_types::xcm::VersionedMultiLocation,
					pub users: ::core::primitive::u32,
				}
				pub enum VersionMigrationStage {
					#[codec::codec(index = 0)]
					MigrateSupportedVersion,
					#[codec::codec(index = 1)]
					MigrateVersionNotifiers,
					#[codec::codec(index = 2)]
					NotifyCurrentTargets(
						::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					),
					#[codec::codec(index = 3)]
					MigrateAndNotifyOldTargets,
				}
			}
		}
		pub mod parachain_info {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {}
			}
		}
		pub mod picasso_runtime {
			use super::runtime_types;
			pub mod opaque {
				use super::runtime_types;
				pub struct SessionKeys {
					pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
				}
			}
			pub enum OriginCaller {
				#[codec::codec(index = 0)]
				system(
					runtime_types::frame_support::dispatch::RawOrigin<
						runtime_types::sp_core::crypto::AccountId32,
					>,
				),
				#[codec::codec(index = 30)]
				Council(
					runtime_types::pallet_collective::RawOrigin<
						runtime_types::sp_core::crypto::AccountId32,
					>,
				),
				#[codec::codec(index = 72)]
				TechnicalCommittee(
					runtime_types::pallet_collective::RawOrigin<
						runtime_types::sp_core::crypto::AccountId32,
					>,
				),
				#[codec::codec(index = 74)]
				ReleaseCommittee(
					runtime_types::pallet_collective::RawOrigin<
						runtime_types::sp_core::crypto::AccountId32,
					>,
				),
				#[codec::codec(index = 41)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Origin),
				#[codec::codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Origin),
				#[codec::codec(index = 6)]
				Void(runtime_types::sp_core::Void),
			}
			pub struct Runtime;
			pub enum RuntimeCall {
				#[codec::codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec::codec(index = 1)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec::codec(index = 2)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
				#[codec::codec(index = 12)]
				AssetTxPayment(runtime_types::pallet_asset_tx_payment::pallet::Call),
				#[codec::codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Call),
				#[codec::codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec::codec(index = 7)]
				Identity(runtime_types::pallet_identity::pallet::Call),
				#[codec::codec(index = 8)]
				Multisig(runtime_types::pallet_multisig::pallet::Call),
				#[codec::codec(index = 10)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
				#[codec::codec(index = 11)]
				ParachainInfo(runtime_types::parachain_info::pallet::Call),
				#[codec::codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
				#[codec::codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec::codec(index = 30)]
				Council(runtime_types::pallet_collective::pallet::Call),
				#[codec::codec(index = 31)]
				CouncilMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec::codec(index = 32)]
				Treasury(runtime_types::pallet_treasury::pallet::Call),
				#[codec::codec(index = 33)]
				Democracy(runtime_types::pallet_democracy::pallet::Call),
				#[codec::codec(index = 72)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Call2),
				#[codec::codec(index = 73)]
				TechnicalCommitteeMembership(runtime_types::pallet_membership::pallet::Call2),
				#[codec::codec(index = 74)]
				ReleaseCommittee(runtime_types::pallet_collective::pallet::Call3),
				#[codec::codec(index = 75)]
				ReleaseMembership(runtime_types::pallet_membership::pallet::Call3),
				#[codec::codec(index = 34)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Call),
				#[codec::codec(index = 35)]
				Utility(runtime_types::pallet_utility::pallet::Call),
				#[codec::codec(index = 36)]
				Preimage(runtime_types::pallet_preimage::pallet::Call),
				#[codec::codec(index = 37)]
				Proxy(runtime_types::pallet_proxy::pallet::Call),
				#[codec::codec(index = 40)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
				#[codec::codec(index = 41)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
				#[codec::codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
				#[codec::codec(index = 43)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
				#[codec::codec(index = 44)]
				XTokens(runtime_types::orml_xtokens::module::Call),
				#[codec::codec(index = 45)]
				UnknownTokens(runtime_types::orml_unknown_tokens::module::Call),
				#[codec::codec(index = 52)]
				Tokens(runtime_types::orml_tokens::module::Call),
				#[codec::codec(index = 53)]
				CurrencyFactory(runtime_types::pallet_currency_factory::pallet::Call),
				#[codec::codec(index = 55)]
				CrowdloanRewards(runtime_types::pallet_crowdloan_rewards::pallet::Call),
				#[codec::codec(index = 56)]
				Vesting(runtime_types::pallet_vesting::module::Call),
				#[codec::codec(index = 57)]
				BondedFinance(runtime_types::pallet_bonded_finance::pallet::Call),
				#[codec::codec(index = 58)]
				AssetsRegistry(runtime_types::pallet_assets_registry::pallet::Call),
				#[codec::codec(index = 59)]
				Pablo(runtime_types::pallet_pablo::pallet::Call),
				#[codec::codec(index = 60)]
				Oracle(runtime_types::pallet_oracle::pallet::Call),
				#[codec::codec(index = 61)]
				AssetsTransactorRouter(
					runtime_types::pallet_assets_transactor_router::pallet::Call,
				),
				#[codec::codec(index = 62)]
				FarmingRewards(runtime_types::reward::pallet::Call),
				#[codec::codec(index = 63)]
				Farming(runtime_types::farming::pallet::Call),
				#[codec::codec(index = 100)]
				CallFilter(runtime_types::pallet_call_filter::pallet::Call),
				#[codec::codec(index = 180)]
				Cosmwasm(runtime_types::pallet_cosmwasm::pallet::Call),
				#[codec::codec(index = 190)]
				Ibc(runtime_types::pallet_ibc::pallet::Call),
				#[codec::codec(index = 191)]
				Ics20Fee(runtime_types::pallet_ibc::ics20_fee::pallet::Call),
				#[codec::codec(index = 192)]
				PalletMultihopXcmIbc(runtime_types::pallet_multihop_xcm_ibc::pallet::Call),
			}
			pub enum RuntimeError {
				#[codec::codec(index = 0)]
				System(runtime_types::frame_system::pallet::Error),
				#[codec::codec(index = 2)]
				Sudo(runtime_types::pallet_sudo::pallet::Error),
				#[codec::codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Error),
				#[codec::codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Error),
				#[codec::codec(index = 7)]
				Identity(runtime_types::pallet_identity::pallet::Error),
				#[codec::codec(index = 8)]
				Multisig(runtime_types::pallet_multisig::pallet::Error),
				#[codec::codec(index = 10)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Error),
				#[codec::codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Error),
				#[codec::codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Error),
				#[codec::codec(index = 30)]
				Council(runtime_types::pallet_collective::pallet::Error),
				#[codec::codec(index = 31)]
				CouncilMembership(runtime_types::pallet_membership::pallet::Error),
				#[codec::codec(index = 32)]
				Treasury(runtime_types::pallet_treasury::pallet::Error),
				#[codec::codec(index = 33)]
				Democracy(runtime_types::pallet_democracy::pallet::Error),
				#[codec::codec(index = 72)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Error2),
				#[codec::codec(index = 73)]
				TechnicalCommitteeMembership(runtime_types::pallet_membership::pallet::Error2),
				#[codec::codec(index = 74)]
				ReleaseCommittee(runtime_types::pallet_collective::pallet::Error3),
				#[codec::codec(index = 75)]
				ReleaseMembership(runtime_types::pallet_membership::pallet::Error3),
				#[codec::codec(index = 34)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Error),
				#[codec::codec(index = 35)]
				Utility(runtime_types::pallet_utility::pallet::Error),
				#[codec::codec(index = 36)]
				Preimage(runtime_types::pallet_preimage::pallet::Error),
				#[codec::codec(index = 37)]
				Proxy(runtime_types::pallet_proxy::pallet::Error),
				#[codec::codec(index = 40)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Error),
				#[codec::codec(index = 41)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Error),
				#[codec::codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Error),
				#[codec::codec(index = 43)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Error),
				#[codec::codec(index = 44)]
				XTokens(runtime_types::orml_xtokens::module::Error),
				#[codec::codec(index = 45)]
				UnknownTokens(runtime_types::orml_unknown_tokens::module::Error),
				#[codec::codec(index = 52)]
				Tokens(runtime_types::orml_tokens::module::Error),
				#[codec::codec(index = 53)]
				CurrencyFactory(runtime_types::pallet_currency_factory::pallet::Error),
				#[codec::codec(index = 55)]
				CrowdloanRewards(runtime_types::pallet_crowdloan_rewards::pallet::Error),
				#[codec::codec(index = 56)]
				Vesting(runtime_types::pallet_vesting::module::Error),
				#[codec::codec(index = 57)]
				BondedFinance(runtime_types::pallet_bonded_finance::pallet::Error),
				#[codec::codec(index = 58)]
				AssetsRegistry(runtime_types::pallet_assets_registry::pallet::Error),
				#[codec::codec(index = 59)]
				Pablo(runtime_types::pallet_pablo::pallet::Error),
				#[codec::codec(index = 60)]
				Oracle(runtime_types::pallet_oracle::pallet::Error),
				#[codec::codec(index = 61)]
				AssetsTransactorRouter(
					runtime_types::pallet_assets_transactor_router::pallet::Error,
				),
				#[codec::codec(index = 62)]
				FarmingRewards(runtime_types::reward::pallet::Error),
				#[codec::codec(index = 63)]
				Farming(runtime_types::farming::pallet::Error),
				#[codec::codec(index = 100)]
				CallFilter(runtime_types::pallet_call_filter::pallet::Error),
				#[codec::codec(index = 180)]
				Cosmwasm(runtime_types::pallet_cosmwasm::pallet::Error),
				#[codec::codec(index = 190)]
				Ibc(runtime_types::pallet_ibc::pallet::Error),
				#[codec::codec(index = 192)]
				PalletMultihopXcmIbc(runtime_types::pallet_multihop_xcm_ibc::pallet::Error),
			}
			pub enum RuntimeEvent {
				#[codec::codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec::codec(index = 2)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
				#[codec::codec(index = 4)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec::codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Event),
				#[codec::codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec::codec(index = 7)]
				Identity(runtime_types::pallet_identity::pallet::Event),
				#[codec::codec(index = 8)]
				Multisig(runtime_types::pallet_multisig::pallet::Event),
				#[codec::codec(index = 10)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
				#[codec::codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
				#[codec::codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec::codec(index = 30)]
				Council(runtime_types::pallet_collective::pallet::Event),
				#[codec::codec(index = 31)]
				CouncilMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec::codec(index = 32)]
				Treasury(runtime_types::pallet_treasury::pallet::Event),
				#[codec::codec(index = 33)]
				Democracy(runtime_types::pallet_democracy::pallet::Event),
				#[codec::codec(index = 72)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Event2),
				#[codec::codec(index = 73)]
				TechnicalCommitteeMembership(runtime_types::pallet_membership::pallet::Event2),
				#[codec::codec(index = 74)]
				ReleaseCommittee(runtime_types::pallet_collective::pallet::Event3),
				#[codec::codec(index = 75)]
				ReleaseMembership(runtime_types::pallet_membership::pallet::Event3),
				#[codec::codec(index = 34)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Event),
				#[codec::codec(index = 35)]
				Utility(runtime_types::pallet_utility::pallet::Event),
				#[codec::codec(index = 36)]
				Preimage(runtime_types::pallet_preimage::pallet::Event),
				#[codec::codec(index = 37)]
				Proxy(runtime_types::pallet_proxy::pallet::Event),
				#[codec::codec(index = 40)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
				#[codec::codec(index = 41)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
				#[codec::codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
				#[codec::codec(index = 43)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
				#[codec::codec(index = 44)]
				XTokens(runtime_types::orml_xtokens::module::Event),
				#[codec::codec(index = 45)]
				UnknownTokens(runtime_types::orml_unknown_tokens::module::Event),
				#[codec::codec(index = 52)]
				Tokens(runtime_types::orml_tokens::module::Event),
				#[codec::codec(index = 53)]
				CurrencyFactory(runtime_types::pallet_currency_factory::pallet::Event),
				#[codec::codec(index = 55)]
				CrowdloanRewards(runtime_types::pallet_crowdloan_rewards::pallet::Event),
				#[codec::codec(index = 56)]
				Vesting(runtime_types::pallet_vesting::module::Event),
				#[codec::codec(index = 57)]
				BondedFinance(runtime_types::pallet_bonded_finance::pallet::Event),
				#[codec::codec(index = 58)]
				AssetsRegistry(runtime_types::pallet_assets_registry::pallet::Event),
				#[codec::codec(index = 59)]
				Pablo(runtime_types::pallet_pablo::pallet::Event),
				#[codec::codec(index = 60)]
				Oracle(runtime_types::pallet_oracle::pallet::Event),
				#[codec::codec(index = 62)]
				FarmingRewards(runtime_types::reward::pallet::Event),
				#[codec::codec(index = 63)]
				Farming(runtime_types::farming::pallet::Event),
				#[codec::codec(index = 100)]
				CallFilter(runtime_types::pallet_call_filter::pallet::Event),
				#[codec::codec(index = 180)]
				Cosmwasm(runtime_types::pallet_cosmwasm::pallet::Event),
				#[codec::codec(index = 190)]
				Ibc(runtime_types::pallet_ibc::pallet::Event),
				#[codec::codec(index = 191)]
				Ics20Fee(runtime_types::pallet_ibc::ics20_fee::pallet::Event),
				#[codec::codec(index = 192)]
				PalletMultihopXcmIbc(runtime_types::pallet_multihop_xcm_ibc::pallet::Event),
			}
		}
		pub mod polkadot_core_primitives {
			use super::runtime_types;
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct OutboundHrmpMessage<_0> {
				pub recipient: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod polkadot_parachain {
			use super::runtime_types;
			pub mod primitives {
				use super::runtime_types;
				pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct Id(pub ::core::primitive::u32);
				pub enum XcmpMessageFormat {
					#[codec::codec(index = 0)]
					ConcatenatedVersionedXcm,
					#[codec::codec(index = 1)]
					ConcatenatedEncodedBlob,
					#[codec::codec(index = 2)]
					Signals,
				}
			}
		}
		pub mod polkadot_primitives {
			use super::runtime_types;
			pub mod v2 {
				use super::runtime_types;
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
				pub struct AbridgedHrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<runtime_types::primitive_types::H256>,
				}
				pub struct PersistedValidationData<_0, _1> {
					pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
					pub relay_parent_number: _1,
					pub relay_parent_storage_root: _0,
					pub max_pov_size: ::core::primitive::u32,
				}
				pub enum UpgradeRestriction {
					#[codec::codec(index = 0)]
					Present,
				}
			}
		}
		pub mod primitive_types {
			use super::runtime_types;
			pub struct H256(pub [::core::primitive::u8; 32usize]);
		}
		pub mod primitives {
			use super::runtime_types;
			pub mod currency {
				use super::runtime_types;
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct CurrencyId(pub ::core::primitive::u128);
				pub enum ForeignAssetId {
					#[codec::codec(index = 0)]
					Xcm(runtime_types::primitives::currency::VersionedMultiLocation),
					#[codec::codec(index = 1)]
					IbcIcs20(runtime_types::primitives::currency::PrefixedDenom),
				}
				pub struct PrefixedDenom(
					pub runtime_types::ibc::applications::transfer::denom::PrefixedDenom,
				);
				pub enum VersionedMultiLocation {
					#[codec::codec(index = 3)]
					V3(runtime_types::xcm::v3::multilocation::MultiLocation),
				}
			}
		}
		pub mod reward {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub enum Call {}
				pub enum Error {
					#[codec::codec(index = 0)]
					TryIntoIntError,
					#[codec::codec(index = 1)]
					InsufficientFunds,
					#[codec::codec(index = 2)]
					ZeroTotalStake,
					#[codec::codec(index = 3)]
					MaxRewardCurrencies,
				}
				pub enum Event {
					#[codec::codec(index = 0)]
					DepositStake {
						pool_id: runtime_types::primitives::currency::CurrencyId,
						stake_id: runtime_types::sp_core::crypto::AccountId32,
						amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
					},
					#[codec::codec(index = 1)]
					DistributeReward {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
					},
					#[codec::codec(index = 2)]
					WithdrawStake {
						pool_id: runtime_types::primitives::currency::CurrencyId,
						stake_id: runtime_types::sp_core::crypto::AccountId32,
						amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
					},
					#[codec::codec(index = 3)]
					WithdrawReward {
						pool_id: runtime_types::primitives::currency::CurrencyId,
						stake_id: runtime_types::sp_core::crypto::AccountId32,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: runtime_types::sp_arithmetic::fixed_point::FixedI128,
					},
				}
			}
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				pub struct FixedI128(pub ::core::primitive::i128);
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct Perbill(pub ::core::primitive::u32);
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct Percent(pub ::core::primitive::u8);
				#[derive(:: subxt :: ext :: codec :: CompactAs)]
				pub struct Permill(pub ::core::primitive::u32);
			}
			pub enum ArithmeticError {
				#[codec::codec(index = 0)]
				Underflow,
				#[codec::codec(index = 1)]
				Overflow,
				#[codec::codec(index = 2)]
				DivisionByZero,
			}
		}
		pub mod sp_consensus_aura {
			use super::runtime_types;
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
				}
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod sr25519 {
				use super::runtime_types;
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub enum Void {}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					pub enum DigestItem {
						#[codec::codec(index = 6)]
						PreRuntime(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec::codec(index = 4)]
						Consensus(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec::codec(index = 5)]
						Seal(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec::codec(index = 0)]
						Other(::std::vec::Vec<::core::primitive::u8>),
						#[codec::codec(index = 8)]
						RuntimeEnvironmentUpdated,
					}
				}
				pub mod era {
					use super::runtime_types;
					pub enum Era {
						#[codec::codec(index = 0)]
						Immortal,
						#[codec::codec(index = 1)]
						Mortal1(::core::primitive::u8),
						#[codec::codec(index = 2)]
						Mortal2(::core::primitive::u8),
						#[codec::codec(index = 3)]
						Mortal3(::core::primitive::u8),
						#[codec::codec(index = 4)]
						Mortal4(::core::primitive::u8),
						#[codec::codec(index = 5)]
						Mortal5(::core::primitive::u8),
						#[codec::codec(index = 6)]
						Mortal6(::core::primitive::u8),
						#[codec::codec(index = 7)]
						Mortal7(::core::primitive::u8),
						#[codec::codec(index = 8)]
						Mortal8(::core::primitive::u8),
						#[codec::codec(index = 9)]
						Mortal9(::core::primitive::u8),
						#[codec::codec(index = 10)]
						Mortal10(::core::primitive::u8),
						#[codec::codec(index = 11)]
						Mortal11(::core::primitive::u8),
						#[codec::codec(index = 12)]
						Mortal12(::core::primitive::u8),
						#[codec::codec(index = 13)]
						Mortal13(::core::primitive::u8),
						#[codec::codec(index = 14)]
						Mortal14(::core::primitive::u8),
						#[codec::codec(index = 15)]
						Mortal15(::core::primitive::u8),
						#[codec::codec(index = 16)]
						Mortal16(::core::primitive::u8),
						#[codec::codec(index = 17)]
						Mortal17(::core::primitive::u8),
						#[codec::codec(index = 18)]
						Mortal18(::core::primitive::u8),
						#[codec::codec(index = 19)]
						Mortal19(::core::primitive::u8),
						#[codec::codec(index = 20)]
						Mortal20(::core::primitive::u8),
						#[codec::codec(index = 21)]
						Mortal21(::core::primitive::u8),
						#[codec::codec(index = 22)]
						Mortal22(::core::primitive::u8),
						#[codec::codec(index = 23)]
						Mortal23(::core::primitive::u8),
						#[codec::codec(index = 24)]
						Mortal24(::core::primitive::u8),
						#[codec::codec(index = 25)]
						Mortal25(::core::primitive::u8),
						#[codec::codec(index = 26)]
						Mortal26(::core::primitive::u8),
						#[codec::codec(index = 27)]
						Mortal27(::core::primitive::u8),
						#[codec::codec(index = 28)]
						Mortal28(::core::primitive::u8),
						#[codec::codec(index = 29)]
						Mortal29(::core::primitive::u8),
						#[codec::codec(index = 30)]
						Mortal30(::core::primitive::u8),
						#[codec::codec(index = 31)]
						Mortal31(::core::primitive::u8),
						#[codec::codec(index = 32)]
						Mortal32(::core::primitive::u8),
						#[codec::codec(index = 33)]
						Mortal33(::core::primitive::u8),
						#[codec::codec(index = 34)]
						Mortal34(::core::primitive::u8),
						#[codec::codec(index = 35)]
						Mortal35(::core::primitive::u8),
						#[codec::codec(index = 36)]
						Mortal36(::core::primitive::u8),
						#[codec::codec(index = 37)]
						Mortal37(::core::primitive::u8),
						#[codec::codec(index = 38)]
						Mortal38(::core::primitive::u8),
						#[codec::codec(index = 39)]
						Mortal39(::core::primitive::u8),
						#[codec::codec(index = 40)]
						Mortal40(::core::primitive::u8),
						#[codec::codec(index = 41)]
						Mortal41(::core::primitive::u8),
						#[codec::codec(index = 42)]
						Mortal42(::core::primitive::u8),
						#[codec::codec(index = 43)]
						Mortal43(::core::primitive::u8),
						#[codec::codec(index = 44)]
						Mortal44(::core::primitive::u8),
						#[codec::codec(index = 45)]
						Mortal45(::core::primitive::u8),
						#[codec::codec(index = 46)]
						Mortal46(::core::primitive::u8),
						#[codec::codec(index = 47)]
						Mortal47(::core::primitive::u8),
						#[codec::codec(index = 48)]
						Mortal48(::core::primitive::u8),
						#[codec::codec(index = 49)]
						Mortal49(::core::primitive::u8),
						#[codec::codec(index = 50)]
						Mortal50(::core::primitive::u8),
						#[codec::codec(index = 51)]
						Mortal51(::core::primitive::u8),
						#[codec::codec(index = 52)]
						Mortal52(::core::primitive::u8),
						#[codec::codec(index = 53)]
						Mortal53(::core::primitive::u8),
						#[codec::codec(index = 54)]
						Mortal54(::core::primitive::u8),
						#[codec::codec(index = 55)]
						Mortal55(::core::primitive::u8),
						#[codec::codec(index = 56)]
						Mortal56(::core::primitive::u8),
						#[codec::codec(index = 57)]
						Mortal57(::core::primitive::u8),
						#[codec::codec(index = 58)]
						Mortal58(::core::primitive::u8),
						#[codec::codec(index = 59)]
						Mortal59(::core::primitive::u8),
						#[codec::codec(index = 60)]
						Mortal60(::core::primitive::u8),
						#[codec::codec(index = 61)]
						Mortal61(::core::primitive::u8),
						#[codec::codec(index = 62)]
						Mortal62(::core::primitive::u8),
						#[codec::codec(index = 63)]
						Mortal63(::core::primitive::u8),
						#[codec::codec(index = 64)]
						Mortal64(::core::primitive::u8),
						#[codec::codec(index = 65)]
						Mortal65(::core::primitive::u8),
						#[codec::codec(index = 66)]
						Mortal66(::core::primitive::u8),
						#[codec::codec(index = 67)]
						Mortal67(::core::primitive::u8),
						#[codec::codec(index = 68)]
						Mortal68(::core::primitive::u8),
						#[codec::codec(index = 69)]
						Mortal69(::core::primitive::u8),
						#[codec::codec(index = 70)]
						Mortal70(::core::primitive::u8),
						#[codec::codec(index = 71)]
						Mortal71(::core::primitive::u8),
						#[codec::codec(index = 72)]
						Mortal72(::core::primitive::u8),
						#[codec::codec(index = 73)]
						Mortal73(::core::primitive::u8),
						#[codec::codec(index = 74)]
						Mortal74(::core::primitive::u8),
						#[codec::codec(index = 75)]
						Mortal75(::core::primitive::u8),
						#[codec::codec(index = 76)]
						Mortal76(::core::primitive::u8),
						#[codec::codec(index = 77)]
						Mortal77(::core::primitive::u8),
						#[codec::codec(index = 78)]
						Mortal78(::core::primitive::u8),
						#[codec::codec(index = 79)]
						Mortal79(::core::primitive::u8),
						#[codec::codec(index = 80)]
						Mortal80(::core::primitive::u8),
						#[codec::codec(index = 81)]
						Mortal81(::core::primitive::u8),
						#[codec::codec(index = 82)]
						Mortal82(::core::primitive::u8),
						#[codec::codec(index = 83)]
						Mortal83(::core::primitive::u8),
						#[codec::codec(index = 84)]
						Mortal84(::core::primitive::u8),
						#[codec::codec(index = 85)]
						Mortal85(::core::primitive::u8),
						#[codec::codec(index = 86)]
						Mortal86(::core::primitive::u8),
						#[codec::codec(index = 87)]
						Mortal87(::core::primitive::u8),
						#[codec::codec(index = 88)]
						Mortal88(::core::primitive::u8),
						#[codec::codec(index = 89)]
						Mortal89(::core::primitive::u8),
						#[codec::codec(index = 90)]
						Mortal90(::core::primitive::u8),
						#[codec::codec(index = 91)]
						Mortal91(::core::primitive::u8),
						#[codec::codec(index = 92)]
						Mortal92(::core::primitive::u8),
						#[codec::codec(index = 93)]
						Mortal93(::core::primitive::u8),
						#[codec::codec(index = 94)]
						Mortal94(::core::primitive::u8),
						#[codec::codec(index = 95)]
						Mortal95(::core::primitive::u8),
						#[codec::codec(index = 96)]
						Mortal96(::core::primitive::u8),
						#[codec::codec(index = 97)]
						Mortal97(::core::primitive::u8),
						#[codec::codec(index = 98)]
						Mortal98(::core::primitive::u8),
						#[codec::codec(index = 99)]
						Mortal99(::core::primitive::u8),
						#[codec::codec(index = 100)]
						Mortal100(::core::primitive::u8),
						#[codec::codec(index = 101)]
						Mortal101(::core::primitive::u8),
						#[codec::codec(index = 102)]
						Mortal102(::core::primitive::u8),
						#[codec::codec(index = 103)]
						Mortal103(::core::primitive::u8),
						#[codec::codec(index = 104)]
						Mortal104(::core::primitive::u8),
						#[codec::codec(index = 105)]
						Mortal105(::core::primitive::u8),
						#[codec::codec(index = 106)]
						Mortal106(::core::primitive::u8),
						#[codec::codec(index = 107)]
						Mortal107(::core::primitive::u8),
						#[codec::codec(index = 108)]
						Mortal108(::core::primitive::u8),
						#[codec::codec(index = 109)]
						Mortal109(::core::primitive::u8),
						#[codec::codec(index = 110)]
						Mortal110(::core::primitive::u8),
						#[codec::codec(index = 111)]
						Mortal111(::core::primitive::u8),
						#[codec::codec(index = 112)]
						Mortal112(::core::primitive::u8),
						#[codec::codec(index = 113)]
						Mortal113(::core::primitive::u8),
						#[codec::codec(index = 114)]
						Mortal114(::core::primitive::u8),
						#[codec::codec(index = 115)]
						Mortal115(::core::primitive::u8),
						#[codec::codec(index = 116)]
						Mortal116(::core::primitive::u8),
						#[codec::codec(index = 117)]
						Mortal117(::core::primitive::u8),
						#[codec::codec(index = 118)]
						Mortal118(::core::primitive::u8),
						#[codec::codec(index = 119)]
						Mortal119(::core::primitive::u8),
						#[codec::codec(index = 120)]
						Mortal120(::core::primitive::u8),
						#[codec::codec(index = 121)]
						Mortal121(::core::primitive::u8),
						#[codec::codec(index = 122)]
						Mortal122(::core::primitive::u8),
						#[codec::codec(index = 123)]
						Mortal123(::core::primitive::u8),
						#[codec::codec(index = 124)]
						Mortal124(::core::primitive::u8),
						#[codec::codec(index = 125)]
						Mortal125(::core::primitive::u8),
						#[codec::codec(index = 126)]
						Mortal126(::core::primitive::u8),
						#[codec::codec(index = 127)]
						Mortal127(::core::primitive::u8),
						#[codec::codec(index = 128)]
						Mortal128(::core::primitive::u8),
						#[codec::codec(index = 129)]
						Mortal129(::core::primitive::u8),
						#[codec::codec(index = 130)]
						Mortal130(::core::primitive::u8),
						#[codec::codec(index = 131)]
						Mortal131(::core::primitive::u8),
						#[codec::codec(index = 132)]
						Mortal132(::core::primitive::u8),
						#[codec::codec(index = 133)]
						Mortal133(::core::primitive::u8),
						#[codec::codec(index = 134)]
						Mortal134(::core::primitive::u8),
						#[codec::codec(index = 135)]
						Mortal135(::core::primitive::u8),
						#[codec::codec(index = 136)]
						Mortal136(::core::primitive::u8),
						#[codec::codec(index = 137)]
						Mortal137(::core::primitive::u8),
						#[codec::codec(index = 138)]
						Mortal138(::core::primitive::u8),
						#[codec::codec(index = 139)]
						Mortal139(::core::primitive::u8),
						#[codec::codec(index = 140)]
						Mortal140(::core::primitive::u8),
						#[codec::codec(index = 141)]
						Mortal141(::core::primitive::u8),
						#[codec::codec(index = 142)]
						Mortal142(::core::primitive::u8),
						#[codec::codec(index = 143)]
						Mortal143(::core::primitive::u8),
						#[codec::codec(index = 144)]
						Mortal144(::core::primitive::u8),
						#[codec::codec(index = 145)]
						Mortal145(::core::primitive::u8),
						#[codec::codec(index = 146)]
						Mortal146(::core::primitive::u8),
						#[codec::codec(index = 147)]
						Mortal147(::core::primitive::u8),
						#[codec::codec(index = 148)]
						Mortal148(::core::primitive::u8),
						#[codec::codec(index = 149)]
						Mortal149(::core::primitive::u8),
						#[codec::codec(index = 150)]
						Mortal150(::core::primitive::u8),
						#[codec::codec(index = 151)]
						Mortal151(::core::primitive::u8),
						#[codec::codec(index = 152)]
						Mortal152(::core::primitive::u8),
						#[codec::codec(index = 153)]
						Mortal153(::core::primitive::u8),
						#[codec::codec(index = 154)]
						Mortal154(::core::primitive::u8),
						#[codec::codec(index = 155)]
						Mortal155(::core::primitive::u8),
						#[codec::codec(index = 156)]
						Mortal156(::core::primitive::u8),
						#[codec::codec(index = 157)]
						Mortal157(::core::primitive::u8),
						#[codec::codec(index = 158)]
						Mortal158(::core::primitive::u8),
						#[codec::codec(index = 159)]
						Mortal159(::core::primitive::u8),
						#[codec::codec(index = 160)]
						Mortal160(::core::primitive::u8),
						#[codec::codec(index = 161)]
						Mortal161(::core::primitive::u8),
						#[codec::codec(index = 162)]
						Mortal162(::core::primitive::u8),
						#[codec::codec(index = 163)]
						Mortal163(::core::primitive::u8),
						#[codec::codec(index = 164)]
						Mortal164(::core::primitive::u8),
						#[codec::codec(index = 165)]
						Mortal165(::core::primitive::u8),
						#[codec::codec(index = 166)]
						Mortal166(::core::primitive::u8),
						#[codec::codec(index = 167)]
						Mortal167(::core::primitive::u8),
						#[codec::codec(index = 168)]
						Mortal168(::core::primitive::u8),
						#[codec::codec(index = 169)]
						Mortal169(::core::primitive::u8),
						#[codec::codec(index = 170)]
						Mortal170(::core::primitive::u8),
						#[codec::codec(index = 171)]
						Mortal171(::core::primitive::u8),
						#[codec::codec(index = 172)]
						Mortal172(::core::primitive::u8),
						#[codec::codec(index = 173)]
						Mortal173(::core::primitive::u8),
						#[codec::codec(index = 174)]
						Mortal174(::core::primitive::u8),
						#[codec::codec(index = 175)]
						Mortal175(::core::primitive::u8),
						#[codec::codec(index = 176)]
						Mortal176(::core::primitive::u8),
						#[codec::codec(index = 177)]
						Mortal177(::core::primitive::u8),
						#[codec::codec(index = 178)]
						Mortal178(::core::primitive::u8),
						#[codec::codec(index = 179)]
						Mortal179(::core::primitive::u8),
						#[codec::codec(index = 180)]
						Mortal180(::core::primitive::u8),
						#[codec::codec(index = 181)]
						Mortal181(::core::primitive::u8),
						#[codec::codec(index = 182)]
						Mortal182(::core::primitive::u8),
						#[codec::codec(index = 183)]
						Mortal183(::core::primitive::u8),
						#[codec::codec(index = 184)]
						Mortal184(::core::primitive::u8),
						#[codec::codec(index = 185)]
						Mortal185(::core::primitive::u8),
						#[codec::codec(index = 186)]
						Mortal186(::core::primitive::u8),
						#[codec::codec(index = 187)]
						Mortal187(::core::primitive::u8),
						#[codec::codec(index = 188)]
						Mortal188(::core::primitive::u8),
						#[codec::codec(index = 189)]
						Mortal189(::core::primitive::u8),
						#[codec::codec(index = 190)]
						Mortal190(::core::primitive::u8),
						#[codec::codec(index = 191)]
						Mortal191(::core::primitive::u8),
						#[codec::codec(index = 192)]
						Mortal192(::core::primitive::u8),
						#[codec::codec(index = 193)]
						Mortal193(::core::primitive::u8),
						#[codec::codec(index = 194)]
						Mortal194(::core::primitive::u8),
						#[codec::codec(index = 195)]
						Mortal195(::core::primitive::u8),
						#[codec::codec(index = 196)]
						Mortal196(::core::primitive::u8),
						#[codec::codec(index = 197)]
						Mortal197(::core::primitive::u8),
						#[codec::codec(index = 198)]
						Mortal198(::core::primitive::u8),
						#[codec::codec(index = 199)]
						Mortal199(::core::primitive::u8),
						#[codec::codec(index = 200)]
						Mortal200(::core::primitive::u8),
						#[codec::codec(index = 201)]
						Mortal201(::core::primitive::u8),
						#[codec::codec(index = 202)]
						Mortal202(::core::primitive::u8),
						#[codec::codec(index = 203)]
						Mortal203(::core::primitive::u8),
						#[codec::codec(index = 204)]
						Mortal204(::core::primitive::u8),
						#[codec::codec(index = 205)]
						Mortal205(::core::primitive::u8),
						#[codec::codec(index = 206)]
						Mortal206(::core::primitive::u8),
						#[codec::codec(index = 207)]
						Mortal207(::core::primitive::u8),
						#[codec::codec(index = 208)]
						Mortal208(::core::primitive::u8),
						#[codec::codec(index = 209)]
						Mortal209(::core::primitive::u8),
						#[codec::codec(index = 210)]
						Mortal210(::core::primitive::u8),
						#[codec::codec(index = 211)]
						Mortal211(::core::primitive::u8),
						#[codec::codec(index = 212)]
						Mortal212(::core::primitive::u8),
						#[codec::codec(index = 213)]
						Mortal213(::core::primitive::u8),
						#[codec::codec(index = 214)]
						Mortal214(::core::primitive::u8),
						#[codec::codec(index = 215)]
						Mortal215(::core::primitive::u8),
						#[codec::codec(index = 216)]
						Mortal216(::core::primitive::u8),
						#[codec::codec(index = 217)]
						Mortal217(::core::primitive::u8),
						#[codec::codec(index = 218)]
						Mortal218(::core::primitive::u8),
						#[codec::codec(index = 219)]
						Mortal219(::core::primitive::u8),
						#[codec::codec(index = 220)]
						Mortal220(::core::primitive::u8),
						#[codec::codec(index = 221)]
						Mortal221(::core::primitive::u8),
						#[codec::codec(index = 222)]
						Mortal222(::core::primitive::u8),
						#[codec::codec(index = 223)]
						Mortal223(::core::primitive::u8),
						#[codec::codec(index = 224)]
						Mortal224(::core::primitive::u8),
						#[codec::codec(index = 225)]
						Mortal225(::core::primitive::u8),
						#[codec::codec(index = 226)]
						Mortal226(::core::primitive::u8),
						#[codec::codec(index = 227)]
						Mortal227(::core::primitive::u8),
						#[codec::codec(index = 228)]
						Mortal228(::core::primitive::u8),
						#[codec::codec(index = 229)]
						Mortal229(::core::primitive::u8),
						#[codec::codec(index = 230)]
						Mortal230(::core::primitive::u8),
						#[codec::codec(index = 231)]
						Mortal231(::core::primitive::u8),
						#[codec::codec(index = 232)]
						Mortal232(::core::primitive::u8),
						#[codec::codec(index = 233)]
						Mortal233(::core::primitive::u8),
						#[codec::codec(index = 234)]
						Mortal234(::core::primitive::u8),
						#[codec::codec(index = 235)]
						Mortal235(::core::primitive::u8),
						#[codec::codec(index = 236)]
						Mortal236(::core::primitive::u8),
						#[codec::codec(index = 237)]
						Mortal237(::core::primitive::u8),
						#[codec::codec(index = 238)]
						Mortal238(::core::primitive::u8),
						#[codec::codec(index = 239)]
						Mortal239(::core::primitive::u8),
						#[codec::codec(index = 240)]
						Mortal240(::core::primitive::u8),
						#[codec::codec(index = 241)]
						Mortal241(::core::primitive::u8),
						#[codec::codec(index = 242)]
						Mortal242(::core::primitive::u8),
						#[codec::codec(index = 243)]
						Mortal243(::core::primitive::u8),
						#[codec::codec(index = 244)]
						Mortal244(::core::primitive::u8),
						#[codec::codec(index = 245)]
						Mortal245(::core::primitive::u8),
						#[codec::codec(index = 246)]
						Mortal246(::core::primitive::u8),
						#[codec::codec(index = 247)]
						Mortal247(::core::primitive::u8),
						#[codec::codec(index = 248)]
						Mortal248(::core::primitive::u8),
						#[codec::codec(index = 249)]
						Mortal249(::core::primitive::u8),
						#[codec::codec(index = 250)]
						Mortal250(::core::primitive::u8),
						#[codec::codec(index = 251)]
						Mortal251(::core::primitive::u8),
						#[codec::codec(index = 252)]
						Mortal252(::core::primitive::u8),
						#[codec::codec(index = 253)]
						Mortal253(::core::primitive::u8),
						#[codec::codec(index = 254)]
						Mortal254(::core::primitive::u8),
						#[codec::codec(index = 255)]
						Mortal255(::core::primitive::u8),
					}
				}
				pub mod unchecked_extrinsic {
					use super::runtime_types;
					pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
						pub ::std::vec::Vec<::core::primitive::u8>,
						#[codec::codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
					);
				}
			}
			pub mod multiaddress {
				use super::runtime_types;
				pub enum MultiAddress<_0, _1> {
					#[codec::codec(index = 0)]
					Id(_0),
					#[codec::codec(index = 1)]
					Index(#[codec::codec(compact)] _1),
					#[codec::codec(index = 2)]
					Raw(::std::vec::Vec<::core::primitive::u8>),
					#[codec::codec(index = 3)]
					Address32([::core::primitive::u8; 32usize]),
					#[codec::codec(index = 4)]
					Address20([::core::primitive::u8; 20usize]),
				}
			}
			pub enum DispatchError {
				#[codec::codec(index = 0)]
				Other,
				#[codec::codec(index = 1)]
				CannotLookup,
				#[codec::codec(index = 2)]
				BadOrigin,
				#[codec::codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
				#[codec::codec(index = 4)]
				ConsumerRemaining,
				#[codec::codec(index = 5)]
				NoProviders,
				#[codec::codec(index = 6)]
				TooManyConsumers,
				#[codec::codec(index = 7)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec::codec(index = 8)]
				Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
				#[codec::codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
				#[codec::codec(index = 10)]
				Exhausted,
				#[codec::codec(index = 11)]
				Corruption,
				#[codec::codec(index = 12)]
				Unavailable,
			}
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			pub enum MultiSignature {
				#[codec::codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Signature),
				#[codec::codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Signature),
				#[codec::codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Signature),
			}
			pub enum TokenError {
				#[codec::codec(index = 0)]
				NoFunds,
				#[codec::codec(index = 1)]
				WouldDie,
				#[codec::codec(index = 2)]
				BelowMinimum,
				#[codec::codec(index = 3)]
				CannotCreate,
				#[codec::codec(index = 4)]
				UnknownAsset,
				#[codec::codec(index = 5)]
				Frozen,
				#[codec::codec(index = 6)]
				Unsupported,
			}
			pub enum TransactionalError {
				#[codec::codec(index = 0)]
				LimitReached,
				#[codec::codec(index = 1)]
				NoLayer,
			}
		}
		pub mod sp_trie {
			use super::runtime_types;
			pub mod storage_proof {
				use super::runtime_types;
				pub struct StorageProof {
					pub trie_nodes:
						::std::collections::BTreeSet<::std::vec::Vec<::core::primitive::u8>>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
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
				pub struct Weight {
					#[codec::codec(compact)]
					pub ref_time: ::core::primitive::u64,
					#[codec::codec(compact)]
					pub proof_size: ::core::primitive::u64,
				}
			}
			#[derive(:: subxt :: ext :: codec :: CompactAs)]
			pub struct OldWeight(pub ::core::primitive::u64);
			pub struct RuntimeDbWeight {
				pub read: ::core::primitive::u64,
				pub write: ::core::primitive::u64,
			}
		}
		pub mod xcm {
			use super::runtime_types;
			pub mod double_encoded {
				use super::runtime_types;
				pub struct DoubleEncoded {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
				pub struct DoubleEncoded2 {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v2 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					pub enum Junction {
						#[codec::codec(index = 0)]
						Parachain(#[codec::codec(compact)] ::core::primitive::u32),
						#[codec::codec(index = 1)]
						AccountId32 {
							network: runtime_types::xcm::v2::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec::codec(index = 2)]
						AccountIndex64 {
							network: runtime_types::xcm::v2::NetworkId,
							#[codec::codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec::codec(index = 3)]
						AccountKey20 {
							network: runtime_types::xcm::v2::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec::codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec::codec(index = 5)]
						GeneralIndex(#[codec::codec(compact)] ::core::primitive::u128),
						#[codec::codec(index = 6)]
						GeneralKey(
							runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec::codec(index = 7)]
						OnlyChild,
						#[codec::codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v2::BodyId,
							part: runtime_types::xcm::v2::BodyPart,
						},
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					pub enum AssetId {
						#[codec::codec(index = 0)]
						Concrete(runtime_types::xcm::v2::multilocation::MultiLocation),
						#[codec::codec(index = 1)]
						Abstract(::std::vec::Vec<::core::primitive::u8>),
					}
					pub enum AssetInstance {
						#[codec::codec(index = 0)]
						Undefined,
						#[codec::codec(index = 1)]
						Index(#[codec::codec(compact)] ::core::primitive::u128),
						#[codec::codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec::codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec::codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec::codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
						#[codec::codec(index = 6)]
						Blob(::std::vec::Vec<::core::primitive::u8>),
					}
					pub enum Fungibility {
						#[codec::codec(index = 0)]
						Fungible(#[codec::codec(compact)] ::core::primitive::u128),
						#[codec::codec(index = 1)]
						NonFungible(runtime_types::xcm::v2::multiasset::AssetInstance),
					}
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v2::multiasset::AssetId,
						pub fun: runtime_types::xcm::v2::multiasset::Fungibility,
					}
					pub enum MultiAssetFilter {
						#[codec::codec(index = 0)]
						Definite(runtime_types::xcm::v2::multiasset::MultiAssets),
						#[codec::codec(index = 1)]
						Wild(runtime_types::xcm::v2::multiasset::WildMultiAsset),
					}
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v2::multiasset::MultiAsset>,
					);
					pub enum WildFungibility {
						#[codec::codec(index = 0)]
						Fungible,
						#[codec::codec(index = 1)]
						NonFungible,
					}
					pub enum WildMultiAsset {
						#[codec::codec(index = 0)]
						All,
						#[codec::codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v2::multiasset::AssetId,
							fun: runtime_types::xcm::v2::multiasset::WildFungibility,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					pub enum Junctions {
						#[codec::codec(index = 0)]
						Here,
						#[codec::codec(index = 1)]
						X1(runtime_types::xcm::v2::junction::Junction),
						#[codec::codec(index = 2)]
						X2(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec::codec(index = 3)]
						X3(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec::codec(index = 4)]
						X4(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec::codec(index = 5)]
						X5(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec::codec(index = 6)]
						X6(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec::codec(index = 7)]
						X7(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec::codec(index = 8)]
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
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v2::multilocation::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					pub enum Error {
						#[codec::codec(index = 0)]
						Overflow,
						#[codec::codec(index = 1)]
						Unimplemented,
						#[codec::codec(index = 2)]
						UntrustedReserveLocation,
						#[codec::codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec::codec(index = 4)]
						MultiLocationFull,
						#[codec::codec(index = 5)]
						MultiLocationNotInvertible,
						#[codec::codec(index = 6)]
						BadOrigin,
						#[codec::codec(index = 7)]
						InvalidLocation,
						#[codec::codec(index = 8)]
						AssetNotFound,
						#[codec::codec(index = 9)]
						FailedToTransactAsset,
						#[codec::codec(index = 10)]
						NotWithdrawable,
						#[codec::codec(index = 11)]
						LocationCannotHold,
						#[codec::codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec::codec(index = 13)]
						DestinationUnsupported,
						#[codec::codec(index = 14)]
						Transport,
						#[codec::codec(index = 15)]
						Unroutable,
						#[codec::codec(index = 16)]
						UnknownClaim,
						#[codec::codec(index = 17)]
						FailedToDecode,
						#[codec::codec(index = 18)]
						MaxWeightInvalid,
						#[codec::codec(index = 19)]
						NotHoldingFees,
						#[codec::codec(index = 20)]
						TooExpensive,
						#[codec::codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec::codec(index = 22)]
						UnhandledXcmVersion,
						#[codec::codec(index = 23)]
						WeightLimitReached(::core::primitive::u64),
						#[codec::codec(index = 24)]
						Barrier,
						#[codec::codec(index = 25)]
						WeightNotComputable,
					}
				}
				pub enum BodyId {
					#[codec::codec(index = 0)]
					Unit,
					#[codec::codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec::codec(index = 2)]
					Index(#[codec::codec(compact)] ::core::primitive::u32),
					#[codec::codec(index = 3)]
					Executive,
					#[codec::codec(index = 4)]
					Technical,
					#[codec::codec(index = 5)]
					Legislative,
					#[codec::codec(index = 6)]
					Judicial,
					#[codec::codec(index = 7)]
					Defense,
					#[codec::codec(index = 8)]
					Administration,
					#[codec::codec(index = 9)]
					Treasury,
				}
				pub enum BodyPart {
					#[codec::codec(index = 0)]
					Voice,
					#[codec::codec(index = 1)]
					Members {
						#[codec::codec(compact)]
						count: ::core::primitive::u32,
					},
					#[codec::codec(index = 2)]
					Fraction {
						#[codec::codec(compact)]
						nom: ::core::primitive::u32,
						#[codec::codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec::codec(index = 3)]
					AtLeastProportion {
						#[codec::codec(compact)]
						nom: ::core::primitive::u32,
						#[codec::codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec::codec(index = 4)]
					MoreThanProportion {
						#[codec::codec(compact)]
						nom: ::core::primitive::u32,
						#[codec::codec(compact)]
						denom: ::core::primitive::u32,
					},
				}
				pub enum Instruction {
					#[codec::codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec::codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec::codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec::codec(index = 3)]
					QueryResponse {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v2::Response,
						#[codec::codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec::codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec::codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec::codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v2::OriginKind,
						#[codec::codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec::codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec::codec(compact)]
						sender: ::core::primitive::u32,
						#[codec::codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec::codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec::codec(index = 8)]
					HrmpChannelAccepted {
						#[codec::codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec::codec(index = 9)]
					HrmpChannelClosing {
						#[codec::codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec::codec(compact)]
						sender: ::core::primitive::u32,
						#[codec::codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec::codec(index = 10)]
					ClearOrigin,
					#[codec::codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
					#[codec::codec(index = 12)]
					ReportError {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						#[codec::codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec::codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec::codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec::codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec::codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec::codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						receive: runtime_types::xcm::v2::multiasset::MultiAssets,
					},
					#[codec::codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec::codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec::codec(index = 18)]
					QueryHolding {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec::codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec::codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v2::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec::codec(index = 20)]
					RefundSurplus,
					#[codec::codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v2::Xcm),
					#[codec::codec(index = 22)]
					SetAppendix(runtime_types::xcm::v2::Xcm),
					#[codec::codec(index = 23)]
					ClearError,
					#[codec::codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec::codec(index = 25)]
					Trap(#[codec::codec(compact)] ::core::primitive::u64),
					#[codec::codec(index = 26)]
					SubscribeVersion {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec::codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec::codec(index = 27)]
					UnsubscribeVersion,
				}
				pub enum Instruction2 {
					#[codec::codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec::codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec::codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec::codec(index = 3)]
					QueryResponse {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v2::Response,
						#[codec::codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec::codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec::codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec::codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v2::OriginKind,
						#[codec::codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded2,
					},
					#[codec::codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec::codec(compact)]
						sender: ::core::primitive::u32,
						#[codec::codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec::codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec::codec(index = 8)]
					HrmpChannelAccepted {
						#[codec::codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec::codec(index = 9)]
					HrmpChannelClosing {
						#[codec::codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec::codec(compact)]
						sender: ::core::primitive::u32,
						#[codec::codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec::codec(index = 10)]
					ClearOrigin,
					#[codec::codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
					#[codec::codec(index = 12)]
					ReportError {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						#[codec::codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec::codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec::codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec::codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec::codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec::codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						receive: runtime_types::xcm::v2::multiasset::MultiAssets,
					},
					#[codec::codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec::codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec::codec(index = 18)]
					QueryHolding {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec::codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec::codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v2::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec::codec(index = 20)]
					RefundSurplus,
					#[codec::codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v2::Xcm2),
					#[codec::codec(index = 22)]
					SetAppendix(runtime_types::xcm::v2::Xcm2),
					#[codec::codec(index = 23)]
					ClearError,
					#[codec::codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec::codec(index = 25)]
					Trap(#[codec::codec(compact)] ::core::primitive::u64),
					#[codec::codec(index = 26)]
					SubscribeVersion {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec::codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec::codec(index = 27)]
					UnsubscribeVersion,
				}
				pub enum NetworkId {
					#[codec::codec(index = 0)]
					Any,
					#[codec::codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec::codec(index = 2)]
					Polkadot,
					#[codec::codec(index = 3)]
					Kusama,
				}
				pub enum OriginKind {
					#[codec::codec(index = 0)]
					Native,
					#[codec::codec(index = 1)]
					SovereignAccount,
					#[codec::codec(index = 2)]
					Superuser,
					#[codec::codec(index = 3)]
					Xcm,
				}
				pub enum Response {
					#[codec::codec(index = 0)]
					Null,
					#[codec::codec(index = 1)]
					Assets(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec::codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v2::traits::Error,
						)>,
					),
					#[codec::codec(index = 3)]
					Version(::core::primitive::u32),
				}
				pub enum WeightLimit {
					#[codec::codec(index = 0)]
					Unlimited,
					#[codec::codec(index = 1)]
					Limited(#[codec::codec(compact)] ::core::primitive::u64),
				}
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
				pub struct Xcm2(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction2>);
			}
			pub mod v3 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					pub enum BodyId {
						#[codec::codec(index = 0)]
						Unit,
						#[codec::codec(index = 1)]
						Moniker([::core::primitive::u8; 4usize]),
						#[codec::codec(index = 2)]
						Index(#[codec::codec(compact)] ::core::primitive::u32),
						#[codec::codec(index = 3)]
						Executive,
						#[codec::codec(index = 4)]
						Technical,
						#[codec::codec(index = 5)]
						Legislative,
						#[codec::codec(index = 6)]
						Judicial,
						#[codec::codec(index = 7)]
						Defense,
						#[codec::codec(index = 8)]
						Administration,
						#[codec::codec(index = 9)]
						Treasury,
					}
					pub enum BodyPart {
						#[codec::codec(index = 0)]
						Voice,
						#[codec::codec(index = 1)]
						Members {
							#[codec::codec(compact)]
							count: ::core::primitive::u32,
						},
						#[codec::codec(index = 2)]
						Fraction {
							#[codec::codec(compact)]
							nom: ::core::primitive::u32,
							#[codec::codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec::codec(index = 3)]
						AtLeastProportion {
							#[codec::codec(compact)]
							nom: ::core::primitive::u32,
							#[codec::codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec::codec(index = 4)]
						MoreThanProportion {
							#[codec::codec(compact)]
							nom: ::core::primitive::u32,
							#[codec::codec(compact)]
							denom: ::core::primitive::u32,
						},
					}
					pub enum Junction {
						#[codec::codec(index = 0)]
						Parachain(#[codec::codec(compact)] ::core::primitive::u32),
						#[codec::codec(index = 1)]
						AccountId32 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec::codec(index = 2)]
						AccountIndex64 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							#[codec::codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec::codec(index = 3)]
						AccountKey20 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec::codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec::codec(index = 5)]
						GeneralIndex(#[codec::codec(compact)] ::core::primitive::u128),
						#[codec::codec(index = 6)]
						GeneralKey {
							length: ::core::primitive::u8,
							data: [::core::primitive::u8; 32usize],
						},
						#[codec::codec(index = 7)]
						OnlyChild,
						#[codec::codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v3::junction::BodyId,
							part: runtime_types::xcm::v3::junction::BodyPart,
						},
						#[codec::codec(index = 9)]
						GlobalConsensus(runtime_types::xcm::v3::junction::NetworkId),
					}
					pub enum NetworkId {
						#[codec::codec(index = 0)]
						ByGenesis([::core::primitive::u8; 32usize]),
						#[codec::codec(index = 1)]
						ByFork {
							block_number: ::core::primitive::u64,
							block_hash: [::core::primitive::u8; 32usize],
						},
						#[codec::codec(index = 2)]
						Polkadot,
						#[codec::codec(index = 3)]
						Kusama,
						#[codec::codec(index = 4)]
						Westend,
						#[codec::codec(index = 5)]
						Rococo,
						#[codec::codec(index = 6)]
						Wococo,
						#[codec::codec(index = 7)]
						Ethereum {
							#[codec::codec(compact)]
							chain_id: ::core::primitive::u64,
						},
						#[codec::codec(index = 8)]
						BitcoinCore,
						#[codec::codec(index = 9)]
						BitcoinCash,
					}
				}
				pub mod junctions {
					use super::runtime_types;
					pub enum Junctions {
						#[codec::codec(index = 0)]
						Here,
						#[codec::codec(index = 1)]
						X1(runtime_types::xcm::v3::junction::Junction),
						#[codec::codec(index = 2)]
						X2(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec::codec(index = 3)]
						X3(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec::codec(index = 4)]
						X4(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec::codec(index = 5)]
						X5(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec::codec(index = 6)]
						X6(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec::codec(index = 7)]
						X7(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec::codec(index = 8)]
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
					pub enum AssetId {
						#[codec::codec(index = 0)]
						Concrete(runtime_types::xcm::v3::multilocation::MultiLocation),
						#[codec::codec(index = 1)]
						Abstract([::core::primitive::u8; 32usize]),
					}
					pub enum AssetInstance {
						#[codec::codec(index = 0)]
						Undefined,
						#[codec::codec(index = 1)]
						Index(#[codec::codec(compact)] ::core::primitive::u128),
						#[codec::codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec::codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec::codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec::codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
					}
					pub enum Fungibility {
						#[codec::codec(index = 0)]
						Fungible(#[codec::codec(compact)] ::core::primitive::u128),
						#[codec::codec(index = 1)]
						NonFungible(runtime_types::xcm::v3::multiasset::AssetInstance),
					}
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v3::multiasset::AssetId,
						pub fun: runtime_types::xcm::v3::multiasset::Fungibility,
					}
					pub enum MultiAssetFilter {
						#[codec::codec(index = 0)]
						Definite(runtime_types::xcm::v3::multiasset::MultiAssets),
						#[codec::codec(index = 1)]
						Wild(runtime_types::xcm::v3::multiasset::WildMultiAsset),
					}
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v3::multiasset::MultiAsset>,
					);
					pub enum WildFungibility {
						#[codec::codec(index = 0)]
						Fungible,
						#[codec::codec(index = 1)]
						NonFungible,
					}
					pub enum WildMultiAsset {
						#[codec::codec(index = 0)]
						All,
						#[codec::codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
						},
						#[codec::codec(index = 2)]
						AllCounted(#[codec::codec(compact)] ::core::primitive::u32),
						#[codec::codec(index = 3)]
						AllOfCounted {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
							#[codec::codec(compact)]
							count: ::core::primitive::u32,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v3::junctions::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					pub enum Error {
						#[codec::codec(index = 0)]
						Overflow,
						#[codec::codec(index = 1)]
						Unimplemented,
						#[codec::codec(index = 2)]
						UntrustedReserveLocation,
						#[codec::codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec::codec(index = 4)]
						LocationFull,
						#[codec::codec(index = 5)]
						LocationNotInvertible,
						#[codec::codec(index = 6)]
						BadOrigin,
						#[codec::codec(index = 7)]
						InvalidLocation,
						#[codec::codec(index = 8)]
						AssetNotFound,
						#[codec::codec(index = 9)]
						FailedToTransactAsset,
						#[codec::codec(index = 10)]
						NotWithdrawable,
						#[codec::codec(index = 11)]
						LocationCannotHold,
						#[codec::codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec::codec(index = 13)]
						DestinationUnsupported,
						#[codec::codec(index = 14)]
						Transport,
						#[codec::codec(index = 15)]
						Unroutable,
						#[codec::codec(index = 16)]
						UnknownClaim,
						#[codec::codec(index = 17)]
						FailedToDecode,
						#[codec::codec(index = 18)]
						MaxWeightInvalid,
						#[codec::codec(index = 19)]
						NotHoldingFees,
						#[codec::codec(index = 20)]
						TooExpensive,
						#[codec::codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec::codec(index = 22)]
						ExpectationFalse,
						#[codec::codec(index = 23)]
						PalletNotFound,
						#[codec::codec(index = 24)]
						NameMismatch,
						#[codec::codec(index = 25)]
						VersionIncompatible,
						#[codec::codec(index = 26)]
						HoldingWouldOverflow,
						#[codec::codec(index = 27)]
						ExportError,
						#[codec::codec(index = 28)]
						ReanchorFailed,
						#[codec::codec(index = 29)]
						NoDeal,
						#[codec::codec(index = 30)]
						FeesNotMet,
						#[codec::codec(index = 31)]
						LockError,
						#[codec::codec(index = 32)]
						NoPermission,
						#[codec::codec(index = 33)]
						Unanchored,
						#[codec::codec(index = 34)]
						NotDepositable,
						#[codec::codec(index = 35)]
						UnhandledXcmVersion,
						#[codec::codec(index = 36)]
						WeightLimitReached(runtime_types::sp_weights::weight_v2::Weight),
						#[codec::codec(index = 37)]
						Barrier,
						#[codec::codec(index = 38)]
						WeightNotComputable,
						#[codec::codec(index = 39)]
						ExceedsStackLimit,
					}
					pub enum Outcome {
						#[codec::codec(index = 0)]
						Complete(runtime_types::sp_weights::weight_v2::Weight),
						#[codec::codec(index = 1)]
						Incomplete(
							runtime_types::sp_weights::weight_v2::Weight,
							runtime_types::xcm::v3::traits::Error,
						),
						#[codec::codec(index = 2)]
						Error(runtime_types::xcm::v3::traits::Error),
					}
				}
				pub enum Instruction {
					#[codec::codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec::codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec::codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec::codec(index = 3)]
					QueryResponse {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v3::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec::codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec::codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v2::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec::codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec::codec(compact)]
						sender: ::core::primitive::u32,
						#[codec::codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec::codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec::codec(index = 8)]
					HrmpChannelAccepted {
						#[codec::codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec::codec(index = 9)]
					HrmpChannelClosing {
						#[codec::codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec::codec(compact)]
						sender: ::core::primitive::u32,
						#[codec::codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec::codec(index = 10)]
					ClearOrigin,
					#[codec::codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
					#[codec::codec(index = 12)]
					ReportError(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec::codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec::codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						want: runtime_types::xcm::v3::multiasset::MultiAssets,
						maximal: ::core::primitive::bool,
					},
					#[codec::codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec::codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec::codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
					},
					#[codec::codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v3::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec::codec(index = 20)]
					RefundSurplus,
					#[codec::codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v3::Xcm),
					#[codec::codec(index = 22)]
					SetAppendix(runtime_types::xcm::v3::Xcm),
					#[codec::codec(index = 23)]
					ClearError,
					#[codec::codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 25)]
					Trap(#[codec::codec(compact)] ::core::primitive::u64),
					#[codec::codec(index = 26)]
					SubscribeVersion {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 27)]
					UnsubscribeVersion,
					#[codec::codec(index = 28)]
					BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec::codec(index = 29)]
					ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec::codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec::codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec::codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec::codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
					},
					#[codec::codec(index = 34)]
					ExpectPallet {
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec::codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec::codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec::codec(index = 35)]
					ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec::codec(index = 36)]
					ClearTransactStatus,
					#[codec::codec(index = 37)]
					UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
					#[codec::codec(index = 38)]
					ExportMessage {
						network: runtime_types::xcm::v3::junction::NetworkId,
						destination: runtime_types::xcm::v3::junctions::Junctions,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec::codec(index = 39)]
					LockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						unlocker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						target: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						owner: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						locker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec::codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec::codec(index = 45)]
					ClearTopic,
					#[codec::codec(index = 46)]
					AliasOrigin(runtime_types::xcm::v3::multilocation::MultiLocation),
					#[codec::codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
				}
				pub enum Instruction2 {
					#[codec::codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec::codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec::codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec::codec(index = 3)]
					QueryResponse {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v3::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec::codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec::codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v2::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded2,
					},
					#[codec::codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec::codec(compact)]
						sender: ::core::primitive::u32,
						#[codec::codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec::codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec::codec(index = 8)]
					HrmpChannelAccepted {
						#[codec::codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec::codec(index = 9)]
					HrmpChannelClosing {
						#[codec::codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec::codec(compact)]
						sender: ::core::primitive::u32,
						#[codec::codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec::codec(index = 10)]
					ClearOrigin,
					#[codec::codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
					#[codec::codec(index = 12)]
					ReportError(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec::codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec::codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						want: runtime_types::xcm::v3::multiasset::MultiAssets,
						maximal: ::core::primitive::bool,
					},
					#[codec::codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec::codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec::codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
					},
					#[codec::codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v3::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec::codec(index = 20)]
					RefundSurplus,
					#[codec::codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v3::Xcm2),
					#[codec::codec(index = 22)]
					SetAppendix(runtime_types::xcm::v3::Xcm2),
					#[codec::codec(index = 23)]
					ClearError,
					#[codec::codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 25)]
					Trap(#[codec::codec(compact)] ::core::primitive::u64),
					#[codec::codec(index = 26)]
					SubscribeVersion {
						#[codec::codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec::codec(index = 27)]
					UnsubscribeVersion,
					#[codec::codec(index = 28)]
					BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec::codec(index = 29)]
					ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec::codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec::codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec::codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec::codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
					},
					#[codec::codec(index = 34)]
					ExpectPallet {
						#[codec::codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec::codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec::codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec::codec(index = 35)]
					ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec::codec(index = 36)]
					ClearTransactStatus,
					#[codec::codec(index = 37)]
					UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
					#[codec::codec(index = 38)]
					ExportMessage {
						network: runtime_types::xcm::v3::junction::NetworkId,
						destination: runtime_types::xcm::v3::junctions::Junctions,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec::codec(index = 39)]
					LockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						unlocker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						target: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						owner: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						locker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec::codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec::codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec::codec(index = 45)]
					ClearTopic,
					#[codec::codec(index = 46)]
					AliasOrigin(runtime_types::xcm::v3::multilocation::MultiLocation),
					#[codec::codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
				}
				pub enum MaybeErrorCode {
					#[codec::codec(index = 0)]
					Success,
					#[codec::codec(index = 1)]
					Error(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec::codec(index = 2)]
					TruncatedError(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
				}
				pub struct PalletInfo {
					#[codec::codec(compact)]
					pub index: ::core::primitive::u32,
					pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					#[codec::codec(compact)]
					pub major: ::core::primitive::u32,
					#[codec::codec(compact)]
					pub minor: ::core::primitive::u32,
					#[codec::codec(compact)]
					pub patch: ::core::primitive::u32,
				}
				pub struct QueryResponseInfo {
					pub destination: runtime_types::xcm::v3::multilocation::MultiLocation,
					#[codec::codec(compact)]
					pub query_id: ::core::primitive::u64,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				pub enum Response {
					#[codec::codec(index = 0)]
					Null,
					#[codec::codec(index = 1)]
					Assets(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec::codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec::codec(index = 3)]
					Version(::core::primitive::u32),
					#[codec::codec(index = 4)]
					PalletsInfo(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::xcm::v3::PalletInfo,
						>,
					),
					#[codec::codec(index = 5)]
					DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
				}
				pub enum WeightLimit {
					#[codec::codec(index = 0)]
					Unlimited,
					#[codec::codec(index = 1)]
					Limited(runtime_types::sp_weights::weight_v2::Weight),
				}
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction>);
				pub struct Xcm2(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction2>);
			}
			pub enum VersionedAssetId {
				#[codec::codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::AssetId),
			}
			pub enum VersionedMultiAsset {
				#[codec::codec(index = 1)]
				V2(runtime_types::xcm::v2::multiasset::MultiAsset),
				#[codec::codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::MultiAsset),
			}
			pub enum VersionedMultiAssets {
				#[codec::codec(index = 1)]
				V2(runtime_types::xcm::v2::multiasset::MultiAssets),
				#[codec::codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::MultiAssets),
			}
			pub enum VersionedMultiLocation {
				#[codec::codec(index = 1)]
				V2(runtime_types::xcm::v2::multilocation::MultiLocation),
				#[codec::codec(index = 3)]
				V3(runtime_types::xcm::v3::multilocation::MultiLocation),
			}
			pub enum VersionedResponse {
				#[codec::codec(index = 2)]
				V2(runtime_types::xcm::v2::Response),
				#[codec::codec(index = 3)]
				V3(runtime_types::xcm::v3::Response),
			}
			pub enum VersionedXcm {
				#[codec::codec(index = 2)]
				V2(runtime_types::xcm::v2::Xcm),
				#[codec::codec(index = 3)]
				V3(runtime_types::xcm::v3::Xcm),
			}
			pub enum VersionedXcm2 {
				#[codec::codec(index = 2)]
				V2(runtime_types::xcm::v2::Xcm2),
				#[codec::codec(index = 3)]
				V3(runtime_types::xcm::v3::Xcm2),
			}
		}
	}
}
