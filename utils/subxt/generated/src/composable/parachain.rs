#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
pub mod api {
	#[allow(unused_imports)]
	mod root_mod {
		pub use super::*;
	}
	pub static PALLETS: [&str; 42usize] = [
		"System",
		"Timestamp",
		"Sudo",
		"TransactionPayment",
		"AssetTxPayment",
		"Indices",
		"Balances",
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
		"Assets",
		"GovernanceRegistry",
		"AssetsRegistry",
		"CallFilter",
		"Ibc",
		"Ics20Fee",
	];
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
		#[codec(index = 4)]
		TransactionPayment(transaction_payment::Event),
		#[codec(index = 5)]
		Indices(indices::Event),
		#[codec(index = 6)]
		Balances(balances::Event),
		#[codec(index = 8)]
		Multisig(multisig::Event),
		#[codec(index = 10)]
		ParachainSystem(parachain_system::Event),
		#[codec(index = 21)]
		CollatorSelection(collator_selection::Event),
		#[codec(index = 22)]
		Session(session::Event),
		#[codec(index = 30)]
		Council(council::Event),
		#[codec(index = 31)]
		CouncilMembership(council_membership::Event),
		#[codec(index = 32)]
		Treasury(treasury::Event),
		#[codec(index = 33)]
		Democracy(democracy::Event),
		#[codec(index = 72)]
		TechnicalCommittee(technical_committee::Event),
		#[codec(index = 73)]
		TechnicalCommitteeMembership(technical_committee_membership::Event),
		#[codec(index = 74)]
		ReleaseCommittee(release_committee::Event),
		#[codec(index = 75)]
		ReleaseMembership(release_membership::Event),
		#[codec(index = 34)]
		Scheduler(scheduler::Event),
		#[codec(index = 35)]
		Utility(utility::Event),
		#[codec(index = 36)]
		Preimage(preimage::Event),
		#[codec(index = 37)]
		Proxy(proxy::Event),
		#[codec(index = 40)]
		XcmpQueue(xcmp_queue::Event),
		#[codec(index = 41)]
		PolkadotXcm(polkadot_xcm::Event),
		#[codec(index = 42)]
		CumulusXcm(cumulus_xcm::Event),
		#[codec(index = 43)]
		DmpQueue(dmp_queue::Event),
		#[codec(index = 44)]
		XTokens(x_tokens::Event),
		#[codec(index = 45)]
		UnknownTokens(unknown_tokens::Event),
		#[codec(index = 52)]
		Tokens(tokens::Event),
		#[codec(index = 53)]
		CurrencyFactory(currency_factory::Event),
		#[codec(index = 56)]
		CrowdloanRewards(crowdloan_rewards::Event),
		#[codec(index = 58)]
		GovernanceRegistry(governance_registry::Event),
		#[codec(index = 59)]
		AssetsRegistry(assets_registry::Event),
		#[codec(index = 100)]
		CallFilter(call_filter::Event),
		#[codec(index = 190)]
		Ibc(ibc::Event),
		#[codec(index = 191)]
		Ics20Fee(ics20_fee::Event),
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
			if pallet_name == "TransactionPayment" {
				return Ok(Event::TransactionPayment(
					transaction_payment::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				))
			}
			if pallet_name == "Indices" {
				return Ok(Event::Indices(indices::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Balances" {
				return Ok(Event::Balances(balances::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Multisig" {
				return Ok(Event::Multisig(multisig::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "ParachainSystem" {
				return Ok(Event::ParachainSystem(parachain_system::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "CollatorSelection" {
				return Ok(Event::CollatorSelection(
					collator_selection::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				))
			}
			if pallet_name == "Session" {
				return Ok(Event::Session(session::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Council" {
				return Ok(Event::Council(council::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "CouncilMembership" {
				return Ok(Event::CouncilMembership(
					council_membership::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				))
			}
			if pallet_name == "Treasury" {
				return Ok(Event::Treasury(treasury::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Democracy" {
				return Ok(Event::Democracy(democracy::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "TechnicalCommittee" {
				return Ok(Event::TechnicalCommittee(
					technical_committee::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				))
			}
			if pallet_name == "TechnicalCommitteeMembership" {
				return Ok(Event::TechnicalCommitteeMembership(
					technical_committee_membership::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				))
			}
			if pallet_name == "ReleaseCommittee" {
				return Ok(Event::ReleaseCommittee(release_committee::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "ReleaseMembership" {
				return Ok(Event::ReleaseMembership(
					release_membership::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				))
			}
			if pallet_name == "Scheduler" {
				return Ok(Event::Scheduler(scheduler::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Utility" {
				return Ok(Event::Utility(utility::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Preimage" {
				return Ok(Event::Preimage(preimage::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Proxy" {
				return Ok(Event::Proxy(proxy::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "XcmpQueue" {
				return Ok(Event::XcmpQueue(xcmp_queue::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "PolkadotXcm" {
				return Ok(Event::PolkadotXcm(polkadot_xcm::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "CumulusXcm" {
				return Ok(Event::CumulusXcm(cumulus_xcm::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "DmpQueue" {
				return Ok(Event::DmpQueue(dmp_queue::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "XTokens" {
				return Ok(Event::XTokens(x_tokens::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "UnknownTokens" {
				return Ok(Event::UnknownTokens(unknown_tokens::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "Tokens" {
				return Ok(Event::Tokens(tokens::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "CurrencyFactory" {
				return Ok(Event::CurrencyFactory(currency_factory::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "CrowdloanRewards" {
				return Ok(Event::CrowdloanRewards(crowdloan_rewards::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "GovernanceRegistry" {
				return Ok(Event::GovernanceRegistry(
					governance_registry::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				))
			}
			if pallet_name == "AssetsRegistry" {
				return Ok(Event::AssetsRegistry(assets_registry::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?))
			}
			if pallet_name == "CallFilter" {
				return Ok(Event::CallFilter(call_filter::Event::decode_with_metadata(
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
			if pallet_name == "Ics20Fee" {
				return Ok(Event::Ics20Fee(ics20_fee::Event::decode_with_metadata(
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
		pub fn assets(&self) -> assets::constants::ConstantsApi {
			assets::constants::ConstantsApi
		}
		pub fn call_filter(&self) -> call_filter::constants::ConstantsApi {
			call_filter::constants::ConstantsApi
		}
		pub fn ibc(&self) -> ibc::constants::ConstantsApi {
			ibc::constants::ConstantsApi
		}
		pub fn ics20_fee(&self) -> ics20_fee::constants::ConstantsApi {
			ics20_fee::constants::ConstantsApi
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
		pub fn governance_registry(&self) -> governance_registry::storage::StorageApi {
			governance_registry::storage::StorageApi
		}
		pub fn assets_registry(&self) -> assets_registry::storage::StorageApi {
			assets_registry::storage::StorageApi
		}
		pub fn call_filter(&self) -> call_filter::storage::StorageApi {
			call_filter::storage::StorageApi
		}
		pub fn ibc(&self) -> ibc::storage::StorageApi {
			ibc::storage::StorageApi
		}
		pub fn ics20_fee(&self) -> ics20_fee::storage::StorageApi {
			ics20_fee::storage::StorageApi
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
		pub fn assets(&self) -> assets::calls::TransactionApi {
			assets::calls::TransactionApi
		}
		pub fn governance_registry(&self) -> governance_registry::calls::TransactionApi {
			governance_registry::calls::TransactionApi
		}
		pub fn assets_registry(&self) -> assets_registry::calls::TransactionApi {
			assets_registry::calls::TransactionApi
		}
		pub fn call_filter(&self) -> call_filter::calls::TransactionApi {
			call_filter::calls::TransactionApi
		}
		pub fn ibc(&self) -> ibc::calls::TransactionApi {
			ibc::calls::TransactionApi
		}
		pub fn ics20_fee(&self) -> ics20_fee::calls::TransactionApi {
			ics20_fee::calls::TransactionApi
		}
	}
	#[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
	pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
		client: &C,
	) -> Result<(), ::subxt::error::MetadataError> {
		let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
		if runtime_metadata_hash !=
			[
				234u8, 101u8, 34u8, 163u8, 28u8, 235u8, 15u8, 197u8, 233u8, 99u8, 91u8, 55u8, 0u8,
				93u8, 133u8, 15u8, 142u8, 59u8, 48u8, 142u8, 22u8, 212u8, 24u8, 30u8, 216u8, 212u8,
				93u8, 246u8, 216u8, 17u8, 165u8, 81u8,
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
							149u8, 119u8, 40u8, 118u8, 224u8, 117u8, 80u8, 106u8, 201u8, 24u8,
							74u8, 3u8, 219u8, 221u8, 158u8, 148u8, 130u8, 109u8, 236u8, 150u8,
							222u8, 179u8, 150u8, 144u8, 167u8, 173u8, 42u8, 49u8, 215u8, 242u8,
							95u8, 156u8,
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
							167u8, 164u8, 72u8, 113u8, 250u8, 241u8, 198u8, 204u8, 19u8, 130u8,
							88u8, 172u8, 61u8, 42u8, 132u8, 226u8, 85u8, 31u8, 96u8, 81u8, 218u8,
							142u8, 150u8, 241u8, 186u8, 13u8, 200u8, 150u8, 11u8, 41u8, 66u8,
							246u8,
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
							153u8, 22u8, 4u8, 166u8, 96u8, 251u8, 206u8, 208u8, 130u8, 22u8, 73u8,
							253u8, 177u8, 110u8, 247u8, 16u8, 184u8, 56u8, 38u8, 23u8, 206u8,
							195u8, 193u8, 71u8, 2u8, 204u8, 234u8, 43u8, 88u8, 121u8, 193u8, 228u8,
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
							169u8, 154u8, 217u8, 52u8, 3u8, 240u8, 128u8, 217u8, 234u8, 151u8,
							82u8, 184u8, 170u8, 238u8, 131u8, 105u8, 188u8, 162u8, 229u8, 117u8,
							10u8, 93u8, 170u8, 250u8, 42u8, 154u8, 41u8, 150u8, 210u8, 75u8, 147u8,
							99u8,
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
	pub mod transaction_payment {
		use super::{root_mod, runtime_types};
		pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
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
							210u8, 0u8, 206u8, 165u8, 183u8, 10u8, 206u8, 52u8, 14u8, 90u8, 218u8,
							197u8, 189u8, 125u8, 113u8, 216u8, 52u8, 161u8, 45u8, 24u8, 245u8,
							237u8, 121u8, 41u8, 106u8, 29u8, 45u8, 129u8, 250u8, 203u8, 206u8,
							180u8,
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
			pub struct SetPaymentAsset {
				pub payer: ::subxt::utils::AccountId32,
				pub asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_payment_asset(
					&self,
					payer: ::subxt::utils::AccountId32,
					asset_id: ::core::option::Option<
						runtime_types::primitives::currency::CurrencyId,
					>,
				) -> ::subxt::tx::Payload<SetPaymentAsset> {
					::subxt::tx::Payload::new_static(
						"AssetTxPayment",
						"set_payment_asset",
						SetPaymentAsset { payer, asset_id },
						[
							136u8, 228u8, 139u8, 140u8, 117u8, 3u8, 147u8, 49u8, 43u8, 230u8, 7u8,
							37u8, 202u8, 138u8, 8u8, 109u8, 218u8, 91u8, 42u8, 61u8, 171u8, 147u8,
							19u8, 6u8, 69u8, 224u8, 127u8, 220u8, 127u8, 132u8, 65u8, 138u8,
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
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
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
							150u8, 228u8, 85u8, 159u8, 93u8, 176u8, 168u8, 224u8, 231u8, 60u8,
							49u8, 69u8, 224u8, 78u8, 73u8, 237u8, 193u8, 21u8, 138u8, 57u8, 169u8,
							254u8, 61u8, 212u8, 36u8, 88u8, 253u8, 109u8, 149u8, 229u8, 151u8,
							232u8,
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
							150u8, 228u8, 85u8, 159u8, 93u8, 176u8, 168u8, 224u8, 231u8, 60u8,
							49u8, 69u8, 224u8, 78u8, 73u8, 237u8, 193u8, 21u8, 138u8, 57u8, 169u8,
							254u8, 61u8, 212u8, 36u8, 88u8, 253u8, 109u8, 149u8, 229u8, 151u8,
							232u8,
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
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
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
			pub struct Claim {
				pub index: ::core::primitive::u32,
			}
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
				pub new: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub index: ::core::primitive::u32,
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
			pub struct Free {
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceTransfer {
				pub new: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub index: ::core::primitive::u32,
				pub freeze: ::core::primitive::bool,
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
			pub struct Freeze {
				pub index: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn claim(&self, index: ::core::primitive::u32) -> ::subxt::tx::Payload<Claim> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"claim",
						Claim { index },
						[
							5u8, 24u8, 11u8, 173u8, 226u8, 170u8, 0u8, 30u8, 193u8, 102u8, 214u8,
							59u8, 252u8, 32u8, 221u8, 88u8, 196u8, 189u8, 244u8, 18u8, 233u8, 37u8,
							228u8, 248u8, 76u8, 175u8, 212u8, 233u8, 238u8, 203u8, 162u8, 68u8,
						],
					)
				}
				pub fn transfer(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Transfer> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"transfer",
						Transfer { new, index },
						[
							1u8, 83u8, 197u8, 184u8, 8u8, 96u8, 48u8, 146u8, 116u8, 76u8, 229u8,
							115u8, 226u8, 215u8, 41u8, 154u8, 27u8, 34u8, 205u8, 188u8, 10u8,
							169u8, 203u8, 39u8, 2u8, 236u8, 181u8, 162u8, 115u8, 254u8, 42u8, 28u8,
						],
					)
				}
				pub fn free(&self, index: ::core::primitive::u32) -> ::subxt::tx::Payload<Free> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"free",
						Free { index },
						[
							133u8, 202u8, 225u8, 127u8, 69u8, 145u8, 43u8, 13u8, 160u8, 248u8,
							215u8, 243u8, 232u8, 166u8, 74u8, 203u8, 235u8, 138u8, 255u8, 27u8,
							163u8, 71u8, 254u8, 217u8, 6u8, 208u8, 202u8, 204u8, 238u8, 70u8,
							126u8, 252u8,
						],
					)
				}
				pub fn force_transfer(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
					freeze: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"force_transfer",
						ForceTransfer { new, index, freeze },
						[
							126u8, 8u8, 145u8, 175u8, 177u8, 153u8, 131u8, 123u8, 184u8, 53u8,
							72u8, 207u8, 21u8, 140u8, 87u8, 181u8, 172u8, 64u8, 37u8, 165u8, 121u8,
							111u8, 173u8, 224u8, 181u8, 79u8, 76u8, 134u8, 93u8, 169u8, 65u8,
							131u8,
						],
					)
				}
				pub fn freeze(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Freeze> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"freeze",
						Freeze { index },
						[
							121u8, 45u8, 118u8, 2u8, 72u8, 48u8, 38u8, 7u8, 234u8, 204u8, 68u8,
							20u8, 76u8, 251u8, 205u8, 246u8, 149u8, 31u8, 168u8, 186u8, 208u8,
							90u8, 40u8, 47u8, 100u8, 228u8, 188u8, 33u8, 79u8, 220u8, 105u8, 209u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_indices::pallet::Event;
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
			pub struct IndexAssigned {
				pub who: ::subxt::utils::AccountId32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexAssigned {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexAssigned";
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
			pub struct IndexFreed {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexFreed {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFreed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct IndexFrozen {
				pub index: ::core::primitive::u32,
				pub who: ::subxt::utils::AccountId32,
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
					(::subxt::utils::AccountId32, ::core::primitive::u128, ::core::primitive::bool),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Indices",
						"Accounts",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							211u8, 169u8, 54u8, 254u8, 88u8, 57u8, 22u8, 223u8, 108u8, 27u8, 38u8,
							9u8, 202u8, 209u8, 111u8, 209u8, 144u8, 13u8, 211u8, 114u8, 239u8,
							127u8, 75u8, 166u8, 234u8, 222u8, 225u8, 35u8, 160u8, 163u8, 112u8,
							242u8,
						],
					)
				}
				pub fn accounts_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::subxt::utils::AccountId32, ::core::primitive::u128, ::core::primitive::bool),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Indices",
						"Accounts",
						Vec::new(),
						[
							211u8, 169u8, 54u8, 254u8, 88u8, 57u8, 22u8, 223u8, 108u8, 27u8, 38u8,
							9u8, 202u8, 209u8, 111u8, 209u8, 144u8, 13u8, 211u8, 114u8, 239u8,
							127u8, 75u8, 166u8, 234u8, 222u8, 225u8, 35u8, 160u8, 163u8, 112u8,
							242u8,
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
			pub struct Transfer {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SetBalance {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub new_free: ::core::primitive::u128,
				#[codec(compact)]
				pub new_reserved: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceTransfer {
				pub source: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferKeepAlive {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferAll {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceUnreserve {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub amount: ::core::primitive::u128,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn transfer(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<Transfer> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer",
						Transfer { dest, value },
						[
							255u8, 181u8, 144u8, 248u8, 64u8, 167u8, 5u8, 134u8, 208u8, 20u8,
							223u8, 103u8, 235u8, 35u8, 66u8, 184u8, 27u8, 94u8, 176u8, 60u8, 233u8,
							236u8, 145u8, 218u8, 44u8, 138u8, 240u8, 224u8, 16u8, 193u8, 220u8,
							95u8,
						],
					)
				}
				pub fn set_balance(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<SetBalance> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"set_balance",
						SetBalance { who, new_free, new_reserved },
						[
							174u8, 34u8, 80u8, 252u8, 193u8, 51u8, 228u8, 236u8, 234u8, 16u8,
							173u8, 214u8, 122u8, 21u8, 254u8, 7u8, 49u8, 176u8, 18u8, 128u8, 122u8,
							68u8, 72u8, 181u8, 119u8, 90u8, 167u8, 46u8, 203u8, 220u8, 109u8,
							110u8,
						],
					)
				}
				pub fn force_transfer(
					&self,
					source: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_transfer",
						ForceTransfer { source, dest, value },
						[
							56u8, 80u8, 186u8, 45u8, 134u8, 147u8, 200u8, 19u8, 53u8, 221u8, 213u8,
							32u8, 13u8, 51u8, 130u8, 42u8, 244u8, 85u8, 50u8, 246u8, 189u8, 51u8,
							93u8, 1u8, 108u8, 142u8, 112u8, 245u8, 104u8, 255u8, 15u8, 62u8,
						],
					)
				}
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<TransferKeepAlive> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_keep_alive",
						TransferKeepAlive { dest, value },
						[
							202u8, 239u8, 204u8, 0u8, 52u8, 57u8, 158u8, 8u8, 252u8, 178u8, 91u8,
							197u8, 238u8, 186u8, 205u8, 56u8, 217u8, 250u8, 21u8, 44u8, 239u8,
							66u8, 79u8, 99u8, 25u8, 106u8, 70u8, 226u8, 50u8, 255u8, 176u8, 71u8,
						],
					)
				}
				pub fn transfer_all(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<TransferAll> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_all",
						TransferAll { dest, keep_alive },
						[
							118u8, 215u8, 198u8, 243u8, 4u8, 173u8, 108u8, 224u8, 113u8, 203u8,
							149u8, 23u8, 130u8, 176u8, 53u8, 205u8, 112u8, 147u8, 88u8, 167u8,
							197u8, 32u8, 104u8, 117u8, 201u8, 168u8, 144u8, 230u8, 120u8, 29u8,
							122u8, 159u8,
						],
					)
				}
				pub fn force_unreserve(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<ForceUnreserve> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_unreserve",
						ForceUnreserve { who, amount },
						[
							39u8, 229u8, 111u8, 44u8, 147u8, 80u8, 7u8, 26u8, 185u8, 121u8, 149u8,
							25u8, 151u8, 37u8, 124u8, 46u8, 108u8, 136u8, 167u8, 145u8, 103u8,
							65u8, 33u8, 168u8, 36u8, 214u8, 126u8, 237u8, 180u8, 61u8, 108u8,
							110u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_balances::pallet::Event;
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
			pub struct Endowed {
				pub account: ::subxt::utils::AccountId32,
				pub free_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DustLost {
				pub account: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "DustLost";
			}
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
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Reserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Unreserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Deposit {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Withdraw {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdraw {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
							1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8,
							156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8,
							20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8,
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
							74u8, 203u8, 111u8, 142u8, 225u8, 104u8, 173u8, 51u8, 226u8, 12u8,
							85u8, 135u8, 41u8, 206u8, 177u8, 238u8, 94u8, 246u8, 184u8, 250u8,
							140u8, 213u8, 91u8, 118u8, 163u8, 111u8, 211u8, 46u8, 204u8, 160u8,
							154u8, 21u8,
						],
					)
				}
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
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
							246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
							40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
							217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
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
							246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
							40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
							217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
						],
					)
				}
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
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
							216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
							58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
							136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
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
							216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
							58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
							136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
						],
					)
				}
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
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
							17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
							167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
							183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
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
							17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
							167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
							183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
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
	pub mod multisig {
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
			pub struct AsMultiThreshold1 {
				pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
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
			pub struct AsMulti {
				pub threshold: ::core::primitive::u16,
				pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
				pub maybe_timepoint: ::core::option::Option<
					runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				>,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
			pub struct ApproveAsMulti {
				pub threshold: ::core::primitive::u16,
				pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
				pub maybe_timepoint: ::core::option::Option<
					runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				>,
				pub call_hash: [::core::primitive::u8; 32usize],
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
			pub struct CancelAsMulti {
				pub threshold: ::core::primitive::u16,
				pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn as_multi_threshold_1(
					&self,
					other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<AsMultiThreshold1> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"as_multi_threshold_1",
						AsMultiThreshold1 { other_signatories, call: ::std::boxed::Box::new(call) },
						[
							189u8, 30u8, 253u8, 17u8, 21u8, 196u8, 233u8, 54u8, 135u8, 207u8,
							213u8, 46u8, 205u8, 180u8, 152u8, 175u8, 153u8, 63u8, 229u8, 143u8,
							67u8, 103u8, 213u8, 103u8, 149u8, 188u8, 246u8, 185u8, 14u8, 26u8,
							170u8, 60u8,
						],
					)
				}
				pub fn as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					call: runtime_types::composable_runtime::RuntimeCall,
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<AsMulti> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"as_multi",
						AsMulti {
							threshold,
							other_signatories,
							maybe_timepoint,
							call: ::std::boxed::Box::new(call),
							max_weight,
						},
						[
							205u8, 16u8, 159u8, 5u8, 58u8, 93u8, 13u8, 116u8, 211u8, 21u8, 236u8,
							118u8, 45u8, 209u8, 207u8, 42u8, 143u8, 101u8, 87u8, 145u8, 183u8,
							87u8, 200u8, 230u8, 134u8, 59u8, 77u8, 141u8, 208u8, 238u8, 57u8,
							217u8,
						],
					)
				}
				pub fn approve_as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					call_hash: [::core::primitive::u8; 32usize],
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<ApproveAsMulti> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"approve_as_multi",
						ApproveAsMulti {
							threshold,
							other_signatories,
							maybe_timepoint,
							call_hash,
							max_weight,
						},
						[
							133u8, 113u8, 121u8, 66u8, 218u8, 219u8, 48u8, 64u8, 211u8, 114u8,
							163u8, 193u8, 164u8, 21u8, 140u8, 218u8, 253u8, 237u8, 240u8, 126u8,
							200u8, 213u8, 184u8, 50u8, 187u8, 182u8, 30u8, 52u8, 142u8, 72u8,
							210u8, 101u8,
						],
					)
				}
				pub fn cancel_as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					call_hash: [::core::primitive::u8; 32usize],
				) -> ::subxt::tx::Payload<CancelAsMulti> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"cancel_as_multi",
						CancelAsMulti { threshold, other_signatories, timepoint, call_hash },
						[
							30u8, 25u8, 186u8, 142u8, 168u8, 81u8, 235u8, 164u8, 82u8, 209u8, 66u8,
							129u8, 209u8, 78u8, 172u8, 9u8, 163u8, 222u8, 125u8, 57u8, 2u8, 43u8,
							169u8, 174u8, 159u8, 167u8, 25u8, 226u8, 254u8, 110u8, 80u8, 216u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_multisig::pallet::Event;
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
			pub struct NewMultisig {
				pub approving: ::subxt::utils::AccountId32,
				pub multisig: ::subxt::utils::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for NewMultisig {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "NewMultisig";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MultisigApproval {
				pub approving: ::subxt::utils::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::utils::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for MultisigApproval {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "MultisigApproval";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MultisigExecuted {
				pub approving: ::subxt::utils::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::utils::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MultisigExecuted {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "MultisigExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MultisigCancelled {
				pub cancelling: ::subxt::utils::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::utils::AccountId32,
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
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_multisig::Multisig<
						::core::primitive::u32,
						::core::primitive::u128,
						::subxt::utils::AccountId32,
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
							69u8, 153u8, 186u8, 204u8, 117u8, 95u8, 119u8, 182u8, 220u8, 87u8, 8u8,
							15u8, 123u8, 83u8, 5u8, 188u8, 115u8, 121u8, 163u8, 96u8, 218u8, 3u8,
							106u8, 44u8, 44u8, 187u8, 46u8, 238u8, 80u8, 203u8, 175u8, 155u8,
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
						::subxt::utils::AccountId32,
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
							69u8, 153u8, 186u8, 204u8, 117u8, 95u8, 119u8, 182u8, 220u8, 87u8, 8u8,
							15u8, 123u8, 83u8, 5u8, 188u8, 115u8, 121u8, 163u8, 96u8, 218u8, 3u8,
							106u8, 44u8, 44u8, 187u8, 46u8, 238u8, 80u8, 203u8, 175u8, 155u8,
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
			pub struct SetValidationData {
				pub data:
					runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SudoSendUpwardMessage {
				pub message: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AuthorizeUpgrade {
				pub code_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EnactAuthorizedUpgrade {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_validation_data(
					&self,
					data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
				) -> ::subxt::tx::Payload<SetValidationData> {
					::subxt::tx::Payload::new_static(
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
				) -> ::subxt::tx::Payload<SudoSendUpwardMessage> {
					::subxt::tx::Payload::new_static(
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
				) -> ::subxt::tx::Payload<AuthorizeUpgrade> {
					::subxt::tx::Payload::new_static(
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
				) -> ::subxt::tx::Payload<EnactAuthorizedUpgrade> {
					::subxt::tx::Payload::new_static(
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
		pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
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
			pub struct ValidationFunctionStored;
			impl ::subxt::events::StaticEvent for ValidationFunctionStored {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionStored";
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
			pub struct ValidationFunctionApplied {
				pub relay_chain_block_num: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ValidationFunctionApplied {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionApplied";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ValidationFunctionDiscarded;
			impl ::subxt::events::StaticEvent for ValidationFunctionDiscarded {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionDiscarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DownwardMessagesReceived {
				pub count: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesReceived {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesReceived";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DownwardMessagesProcessed {
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
				pub dmq_head: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesProcessed {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesProcessed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
							162u8, 35u8, 108u8, 76u8, 160u8, 93u8, 215u8, 84u8, 20u8, 249u8, 57u8,
							187u8, 88u8, 161u8, 15u8, 131u8, 213u8, 89u8, 140u8, 20u8, 227u8,
							204u8, 79u8, 176u8, 114u8, 119u8, 8u8, 7u8, 64u8, 15u8, 90u8, 92u8,
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
							224u8, 174u8, 53u8, 106u8, 240u8, 49u8, 48u8, 79u8, 219u8, 74u8, 142u8,
							166u8, 92u8, 204u8, 244u8, 200u8, 43u8, 169u8, 177u8, 207u8, 190u8,
							106u8, 180u8, 65u8, 245u8, 131u8, 134u8, 4u8, 53u8, 45u8, 76u8, 3u8,
						],
					)
				}
				pub fn validation_data(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_primitives::v2::PersistedValidationData<
						::subxt::utils::H256,
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
							112u8, 58u8, 240u8, 81u8, 219u8, 110u8, 244u8, 186u8, 251u8, 90u8,
							195u8, 217u8, 229u8, 102u8, 233u8, 24u8, 109u8, 96u8, 219u8, 72u8,
							139u8, 93u8, 58u8, 140u8, 40u8, 110u8, 167u8, 98u8, 199u8, 12u8, 138u8,
							131u8,
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
							89u8, 83u8, 74u8, 174u8, 234u8, 188u8, 149u8, 78u8, 140u8, 17u8, 92u8,
							165u8, 243u8, 87u8, 59u8, 97u8, 135u8, 81u8, 192u8, 86u8, 193u8, 187u8,
							113u8, 22u8, 108u8, 83u8, 242u8, 208u8, 174u8, 40u8, 49u8, 245u8,
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
							68u8, 121u8, 6u8, 159u8, 181u8, 94u8, 151u8, 215u8, 225u8, 244u8, 4u8,
							158u8, 216u8, 85u8, 55u8, 228u8, 197u8, 35u8, 200u8, 33u8, 29u8, 182u8,
							17u8, 83u8, 59u8, 63u8, 25u8, 180u8, 132u8, 23u8, 97u8, 252u8,
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
							61u8, 3u8, 26u8, 6u8, 88u8, 114u8, 109u8, 63u8, 7u8, 115u8, 245u8,
							198u8, 73u8, 234u8, 28u8, 228u8, 126u8, 27u8, 151u8, 18u8, 133u8, 54u8,
							144u8, 149u8, 246u8, 43u8, 83u8, 47u8, 77u8, 238u8, 10u8, 196u8,
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
							35u8, 124u8, 167u8, 221u8, 162u8, 145u8, 158u8, 186u8, 57u8, 154u8,
							225u8, 6u8, 176u8, 13u8, 178u8, 195u8, 209u8, 122u8, 221u8, 26u8,
							155u8, 126u8, 153u8, 246u8, 101u8, 221u8, 61u8, 145u8, 211u8, 236u8,
							48u8, 130u8,
						],
					)
				}				pub fn relevant_messaging_state (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::Address::new_static(
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
							104u8, 200u8, 30u8, 202u8, 119u8, 204u8, 233u8, 20u8, 67u8, 199u8,
							47u8, 166u8, 254u8, 152u8, 10u8, 187u8, 240u8, 255u8, 148u8, 201u8,
							134u8, 41u8, 130u8, 201u8, 112u8, 65u8, 68u8, 103u8, 56u8, 123u8,
							178u8, 113u8,
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
							176u8, 255u8, 246u8, 125u8, 36u8, 120u8, 24u8, 44u8, 26u8, 64u8, 236u8,
							210u8, 189u8, 237u8, 50u8, 78u8, 45u8, 139u8, 58u8, 141u8, 112u8,
							253u8, 178u8, 198u8, 87u8, 71u8, 77u8, 248u8, 21u8, 145u8, 187u8, 52u8,
						],
					)
				}
				pub fn last_hrmp_mqc_heads(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::KeyedVec<
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
							55u8, 179u8, 35u8, 16u8, 173u8, 0u8, 122u8, 179u8, 236u8, 98u8, 9u8,
							112u8, 11u8, 219u8, 241u8, 89u8, 131u8, 198u8, 64u8, 139u8, 103u8,
							158u8, 77u8, 107u8, 83u8, 236u8, 255u8, 208u8, 47u8, 61u8, 219u8,
							240u8,
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
							48u8, 177u8, 84u8, 228u8, 101u8, 235u8, 181u8, 27u8, 66u8, 55u8, 50u8,
							146u8, 245u8, 223u8, 77u8, 132u8, 178u8, 80u8, 74u8, 90u8, 166u8, 81u8,
							109u8, 25u8, 91u8, 69u8, 5u8, 69u8, 123u8, 197u8, 160u8, 146u8,
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
							189u8, 59u8, 183u8, 195u8, 69u8, 185u8, 241u8, 226u8, 62u8, 204u8,
							230u8, 77u8, 102u8, 75u8, 86u8, 157u8, 249u8, 140u8, 219u8, 72u8, 94u8,
							64u8, 176u8, 72u8, 34u8, 205u8, 114u8, 103u8, 231u8, 233u8, 206u8,
							111u8,
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
							74u8, 86u8, 173u8, 248u8, 90u8, 230u8, 71u8, 225u8, 127u8, 164u8,
							221u8, 62u8, 146u8, 13u8, 73u8, 9u8, 98u8, 168u8, 6u8, 14u8, 97u8,
							166u8, 45u8, 70u8, 62u8, 210u8, 9u8, 32u8, 83u8, 18u8, 4u8, 201u8,
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
							129u8, 208u8, 187u8, 36u8, 48u8, 108u8, 135u8, 56u8, 204u8, 60u8,
							100u8, 158u8, 113u8, 238u8, 46u8, 92u8, 228u8, 41u8, 178u8, 177u8,
							208u8, 195u8, 148u8, 149u8, 127u8, 21u8, 93u8, 92u8, 29u8, 115u8, 10u8,
							248u8,
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
							223u8, 46u8, 224u8, 227u8, 222u8, 119u8, 225u8, 244u8, 59u8, 87u8,
							127u8, 19u8, 217u8, 237u8, 103u8, 61u8, 6u8, 210u8, 107u8, 201u8,
							117u8, 25u8, 85u8, 248u8, 36u8, 231u8, 28u8, 202u8, 41u8, 140u8, 208u8,
							254u8,
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
							132u8, 61u8, 162u8, 129u8, 251u8, 243u8, 20u8, 144u8, 162u8, 73u8,
							237u8, 51u8, 248u8, 41u8, 127u8, 171u8, 180u8, 79u8, 137u8, 23u8, 66u8,
							134u8, 106u8, 222u8, 182u8, 154u8, 0u8, 145u8, 184u8, 156u8, 36u8,
							97u8,
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
							180u8, 90u8, 34u8, 178u8, 1u8, 242u8, 211u8, 97u8, 100u8, 34u8, 39u8,
							42u8, 142u8, 249u8, 236u8, 194u8, 244u8, 164u8, 96u8, 54u8, 98u8, 46u8,
							92u8, 196u8, 185u8, 51u8, 231u8, 234u8, 249u8, 143u8, 244u8, 64u8,
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
							90u8, 122u8, 168u8, 240u8, 95u8, 195u8, 160u8, 109u8, 175u8, 170u8,
							227u8, 44u8, 139u8, 176u8, 32u8, 161u8, 57u8, 233u8, 56u8, 55u8, 123u8,
							168u8, 174u8, 96u8, 159u8, 62u8, 186u8, 186u8, 17u8, 70u8, 57u8, 246u8,
						],
					)
				}
				pub fn authorized_upgrade(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
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
							151u8, 191u8, 241u8, 118u8, 192u8, 47u8, 166u8, 151u8, 217u8, 240u8,
							165u8, 232u8, 51u8, 113u8, 243u8, 1u8, 89u8, 240u8, 11u8, 1u8, 77u8,
							104u8, 12u8, 56u8, 17u8, 135u8, 214u8, 19u8, 114u8, 135u8, 66u8, 76u8,
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
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
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
			}
		}
	}
	pub mod collator_selection {
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
			pub struct SetInvulnerables {
				pub new: ::std::vec::Vec<::subxt::utils::AccountId32>,
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
			pub struct SetDesiredCandidates {
				pub max: ::core::primitive::u32,
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
			pub struct SetCandidacyBond {
				pub bond: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RegisterAsCandidate;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct LeaveIntent;
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_invulnerables(
					&self,
					new: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<SetInvulnerables> {
					::subxt::tx::Payload::new_static(
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
				pub fn set_desired_candidates(
					&self,
					max: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<SetDesiredCandidates> {
					::subxt::tx::Payload::new_static(
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
				pub fn set_candidacy_bond(
					&self,
					bond: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<SetCandidacyBond> {
					::subxt::tx::Payload::new_static(
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
				pub fn register_as_candidate(&self) -> ::subxt::tx::Payload<RegisterAsCandidate> {
					::subxt::tx::Payload::new_static(
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
				pub fn leave_intent(&self) -> ::subxt::tx::Payload<LeaveIntent> {
					::subxt::tx::Payload::new_static(
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
		pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
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
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NewCandidacyBond {
				pub bond_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for NewCandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewCandidacyBond";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CandidateAdded {
				pub account_id: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for CandidateAdded {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				pub fn invulnerables(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::subxt::utils::AccountId32,
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
							215u8, 62u8, 140u8, 81u8, 0u8, 189u8, 182u8, 139u8, 32u8, 42u8, 20u8,
							223u8, 81u8, 212u8, 100u8, 97u8, 146u8, 253u8, 75u8, 123u8, 240u8,
							125u8, 249u8, 62u8, 226u8, 70u8, 57u8, 206u8, 16u8, 74u8, 52u8, 72u8,
						],
					)
				}
				pub fn candidates(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_collator_selection::pallet::CandidateInfo<
							::subxt::utils::AccountId32,
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
							28u8, 116u8, 232u8, 94u8, 147u8, 216u8, 214u8, 30u8, 26u8, 241u8, 68u8,
							108u8, 165u8, 107u8, 89u8, 136u8, 111u8, 239u8, 150u8, 42u8, 210u8,
							214u8, 192u8, 234u8, 29u8, 41u8, 157u8, 169u8, 120u8, 126u8, 192u8,
							32u8,
						],
					)
				}
				pub fn last_authored_block(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
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
							53u8, 30u8, 243u8, 31u8, 228u8, 231u8, 175u8, 153u8, 204u8, 241u8,
							76u8, 147u8, 6u8, 202u8, 255u8, 89u8, 30u8, 129u8, 85u8, 92u8, 10u8,
							97u8, 177u8, 129u8, 88u8, 196u8, 7u8, 255u8, 74u8, 52u8, 28u8, 0u8,
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
							53u8, 30u8, 243u8, 31u8, 228u8, 231u8, 175u8, 153u8, 204u8, 241u8,
							76u8, 147u8, 6u8, 202u8, 255u8, 89u8, 30u8, 129u8, 85u8, 92u8, 10u8,
							97u8, 177u8, 129u8, 88u8, 196u8, 7u8, 255u8, 74u8, 52u8, 28u8, 0u8,
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
							161u8, 170u8, 254u8, 76u8, 112u8, 146u8, 144u8, 7u8, 177u8, 152u8,
							146u8, 60u8, 143u8, 237u8, 1u8, 168u8, 176u8, 33u8, 103u8, 35u8, 39u8,
							233u8, 107u8, 253u8, 47u8, 183u8, 11u8, 86u8, 230u8, 13u8, 127u8,
							133u8,
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
			pub struct SetKeys {
				pub keys: runtime_types::composable_runtime::opaque::SessionKeys,
				pub proof: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PurgeKeys;
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_keys(
					&self,
					keys: runtime_types::composable_runtime::opaque::SessionKeys,
					proof: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<SetKeys> {
					::subxt::tx::Payload::new_static(
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
				pub fn purge_keys(&self) -> ::subxt::tx::Payload<PurgeKeys> {
					::subxt::tx::Payload::new_static(
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
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
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
					::std::vec::Vec<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
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
							148u8, 179u8, 159u8, 15u8, 197u8, 95u8, 214u8, 30u8, 209u8, 251u8,
							183u8, 231u8, 91u8, 25u8, 181u8, 191u8, 143u8, 252u8, 227u8, 80u8,
							159u8, 66u8, 194u8, 67u8, 113u8, 74u8, 111u8, 91u8, 218u8, 187u8,
							130u8, 40u8,
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
							105u8, 140u8, 235u8, 218u8, 96u8, 100u8, 252u8, 10u8, 58u8, 221u8,
							244u8, 251u8, 67u8, 91u8, 80u8, 202u8, 152u8, 42u8, 50u8, 113u8, 200u8,
							247u8, 59u8, 213u8, 77u8, 195u8, 1u8, 150u8, 220u8, 18u8, 245u8, 46u8,
						],
					)
				}
				pub fn queued_keys(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::subxt::utils::AccountId32,
						runtime_types::composable_runtime::opaque::SessionKeys,
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
							42u8, 134u8, 252u8, 233u8, 29u8, 69u8, 168u8, 107u8, 77u8, 70u8, 80u8,
							189u8, 149u8, 227u8, 77u8, 74u8, 100u8, 175u8, 10u8, 162u8, 145u8,
							105u8, 85u8, 196u8, 169u8, 195u8, 116u8, 255u8, 112u8, 122u8, 112u8,
							133u8,
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
							135u8, 22u8, 22u8, 97u8, 82u8, 217u8, 144u8, 141u8, 121u8, 240u8,
							189u8, 16u8, 176u8, 88u8, 177u8, 31u8, 20u8, 242u8, 73u8, 104u8, 11u8,
							110u8, 214u8, 34u8, 52u8, 217u8, 106u8, 33u8, 174u8, 174u8, 198u8,
							84u8,
						],
					)
				}
				pub fn next_keys(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_runtime::opaque::SessionKeys,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"NextKeys",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							21u8, 0u8, 237u8, 42u8, 156u8, 77u8, 229u8, 211u8, 105u8, 8u8, 231u8,
							5u8, 246u8, 188u8, 69u8, 143u8, 202u8, 240u8, 252u8, 253u8, 106u8,
							37u8, 51u8, 244u8, 206u8, 199u8, 249u8, 37u8, 17u8, 102u8, 20u8, 246u8,
						],
					)
				}
				pub fn next_keys_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_runtime::opaque::SessionKeys,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
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
				pub fn key_owner(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
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
							4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
							199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
							0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
						],
					)
				}
				pub fn key_owner_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
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
							199u8, 89u8, 94u8, 48u8, 249u8, 35u8, 105u8, 90u8, 15u8, 86u8, 218u8,
							85u8, 22u8, 236u8, 228u8, 36u8, 137u8, 64u8, 236u8, 171u8, 242u8,
							217u8, 91u8, 240u8, 205u8, 205u8, 226u8, 16u8, 147u8, 235u8, 181u8,
							41u8,
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
	pub mod council {
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
			pub struct SetMembers {
				pub new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				pub prime: ::core::option::Option<::subxt::utils::AccountId32>,
				pub old_count: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Execute {
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Propose {
				#[codec(compact)]
				pub threshold: ::core::primitive::u32,
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Vote {
				pub proposal: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub approve: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CloseOldWeight {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				#[codec(compact)]
				pub proposal_weight_bound: runtime_types::sp_weights::OldWeight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Close {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
					prime: ::core::option::Option<::subxt::utils::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<SetMembers> {
					::subxt::tx::Payload::new_static(
						"Council",
						"set_members",
						SetMembers { new_members, prime, old_count },
						[
							196u8, 103u8, 123u8, 125u8, 226u8, 177u8, 126u8, 37u8, 160u8, 114u8,
							34u8, 136u8, 219u8, 84u8, 199u8, 94u8, 242u8, 20u8, 126u8, 126u8,
							166u8, 190u8, 198u8, 33u8, 162u8, 113u8, 237u8, 222u8, 90u8, 1u8, 2u8,
							234u8,
						],
					)
				}
				pub fn execute(
					&self,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Execute> {
					::subxt::tx::Payload::new_static(
						"Council",
						"execute",
						Execute { proposal: ::std::boxed::Box::new(proposal), length_bound },
						[
							149u8, 198u8, 212u8, 65u8, 131u8, 242u8, 252u8, 178u8, 31u8, 10u8,
							199u8, 72u8, 104u8, 41u8, 174u8, 137u8, 112u8, 63u8, 12u8, 238u8,
							184u8, 149u8, 246u8, 157u8, 36u8, 172u8, 29u8, 56u8, 250u8, 255u8,
							146u8, 137u8,
						],
					)
				}
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Propose> {
					::subxt::tx::Payload::new_static(
						"Council",
						"propose",
						Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							41u8, 137u8, 175u8, 40u8, 83u8, 161u8, 15u8, 132u8, 102u8, 169u8,
							162u8, 20u8, 190u8, 109u8, 97u8, 84u8, 162u8, 83u8, 62u8, 245u8, 65u8,
							190u8, 138u8, 253u8, 155u8, 235u8, 220u8, 39u8, 80u8, 31u8, 244u8,
							175u8,
						],
					)
				}
				pub fn vote(
					&self,
					proposal: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<Vote> {
					::subxt::tx::Payload::new_static(
						"Council",
						"vote",
						Vote { proposal, index, approve },
						[
							108u8, 46u8, 180u8, 148u8, 145u8, 24u8, 173u8, 56u8, 36u8, 100u8,
							216u8, 43u8, 178u8, 202u8, 26u8, 136u8, 93u8, 84u8, 80u8, 134u8, 14u8,
							42u8, 248u8, 205u8, 68u8, 92u8, 79u8, 11u8, 113u8, 115u8, 157u8, 100u8,
						],
					)
				}
				pub fn close_old_weight(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<CloseOldWeight> {
					::subxt::tx::Payload::new_static(
						"Council",
						"close_old_weight",
						CloseOldWeight {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							133u8, 219u8, 90u8, 40u8, 102u8, 95u8, 4u8, 199u8, 45u8, 234u8, 109u8,
							17u8, 162u8, 63u8, 102u8, 186u8, 95u8, 182u8, 13u8, 123u8, 227u8, 20u8,
							186u8, 207u8, 12u8, 47u8, 87u8, 252u8, 244u8, 172u8, 60u8, 206u8,
						],
					)
				}
				pub fn disapprove_proposal(
					&self,
					proposal_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<DisapproveProposal> {
					::subxt::tx::Payload::new_static(
						"Council",
						"disapprove_proposal",
						DisapproveProposal { proposal_hash },
						[
							25u8, 123u8, 1u8, 8u8, 74u8, 37u8, 3u8, 40u8, 97u8, 37u8, 175u8, 224u8,
							72u8, 155u8, 123u8, 109u8, 104u8, 43u8, 91u8, 125u8, 199u8, 51u8, 17u8,
							225u8, 133u8, 38u8, 120u8, 76u8, 164u8, 5u8, 194u8, 201u8,
						],
					)
				}
				pub fn close(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Close> {
					::subxt::tx::Payload::new_static(
						"Council",
						"close",
						Close { proposal_hash, index, proposal_weight_bound, length_bound },
						[
							191u8, 138u8, 89u8, 247u8, 97u8, 51u8, 45u8, 193u8, 76u8, 16u8, 80u8,
							225u8, 197u8, 83u8, 204u8, 133u8, 169u8, 16u8, 86u8, 32u8, 125u8, 16u8,
							116u8, 185u8, 45u8, 20u8, 76u8, 148u8, 206u8, 163u8, 154u8, 30u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_collective::pallet::Event;
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
			pub struct Proposed {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: ::subxt::utils::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Voted {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_hash: ::subxt::utils::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Voted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Approved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Approved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Disapproved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Disapproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Executed {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Executed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MemberExecuted {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "MemberExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Closed {
				pub proposal_hash: ::subxt::utils::H256,
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
						::subxt::utils::H256,
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
							10u8, 133u8, 82u8, 54u8, 193u8, 41u8, 253u8, 159u8, 56u8, 96u8, 249u8,
							148u8, 43u8, 57u8, 116u8, 43u8, 222u8, 243u8, 237u8, 231u8, 238u8,
							60u8, 26u8, 225u8, 19u8, 203u8, 213u8, 220u8, 114u8, 217u8, 100u8,
							27u8,
						],
					)
				}
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_runtime::RuntimeCall,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"ProposalOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							164u8, 89u8, 234u8, 130u8, 84u8, 52u8, 167u8, 211u8, 224u8, 161u8,
							138u8, 7u8, 93u8, 58u8, 187u8, 187u8, 211u8, 209u8, 138u8, 12u8, 140u8,
							59u8, 99u8, 14u8, 230u8, 195u8, 22u8, 237u8, 126u8, 155u8, 43u8, 59u8,
						],
					)
				}
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_runtime::RuntimeCall,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"ProposalOf",
						Vec::new(),
						[
							164u8, 89u8, 234u8, 130u8, 84u8, 52u8, 167u8, 211u8, 224u8, 161u8,
							138u8, 7u8, 93u8, 58u8, 187u8, 187u8, 211u8, 209u8, 138u8, 12u8, 140u8,
							59u8, 99u8, 14u8, 230u8, 195u8, 22u8, 237u8, 126u8, 155u8, 43u8, 59u8,
						],
					)
				}
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						::subxt::utils::AccountId32,
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
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						::subxt::utils::AccountId32,
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
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
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
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						],
					)
				}
				pub fn members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"Members",
						vec![],
						[
							162u8, 72u8, 174u8, 204u8, 140u8, 105u8, 205u8, 176u8, 197u8, 117u8,
							206u8, 134u8, 157u8, 110u8, 139u8, 54u8, 43u8, 233u8, 25u8, 51u8, 36u8,
							238u8, 94u8, 124u8, 221u8, 52u8, 237u8, 71u8, 125u8, 56u8, 129u8,
							222u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Council",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod council_membership {
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
			pub struct AddMember {
				pub who: ::subxt::utils::MultiAddress<
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
			pub struct RemoveMember {
				pub who: ::subxt::utils::MultiAddress<
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
			pub struct SwapMember {
				pub remove: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub add: ::subxt::utils::MultiAddress<
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
			pub struct ResetMembers {
				pub members: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChangeKey {
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
			pub struct SetPrime {
				pub who: ::subxt::utils::MultiAddress<
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
			pub struct ClearPrime;
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<AddMember> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"add_member",
						AddMember { who },
						[
							168u8, 166u8, 6u8, 167u8, 12u8, 109u8, 99u8, 96u8, 240u8, 57u8, 60u8,
							174u8, 57u8, 52u8, 131u8, 16u8, 230u8, 172u8, 23u8, 140u8, 48u8, 131u8,
							73u8, 131u8, 133u8, 217u8, 137u8, 50u8, 165u8, 149u8, 174u8, 188u8,
						],
					)
				}
				pub fn remove_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<RemoveMember> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"remove_member",
						RemoveMember { who },
						[
							33u8, 178u8, 96u8, 158u8, 126u8, 172u8, 0u8, 207u8, 143u8, 144u8,
							219u8, 28u8, 205u8, 197u8, 192u8, 195u8, 141u8, 26u8, 39u8, 101u8,
							140u8, 88u8, 212u8, 26u8, 221u8, 29u8, 187u8, 160u8, 119u8, 101u8,
							45u8, 162u8,
						],
					)
				}
				pub fn swap_member(
					&self,
					remove: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					add: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<SwapMember> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"swap_member",
						SwapMember { remove, add },
						[
							52u8, 10u8, 13u8, 175u8, 35u8, 141u8, 159u8, 135u8, 34u8, 235u8, 117u8,
							146u8, 134u8, 49u8, 76u8, 116u8, 93u8, 209u8, 24u8, 242u8, 123u8, 82u8,
							34u8, 192u8, 147u8, 237u8, 163u8, 167u8, 18u8, 64u8, 196u8, 132u8,
						],
					)
				}
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<ResetMembers> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"reset_members",
						ResetMembers { members },
						[
							9u8, 35u8, 28u8, 59u8, 158u8, 232u8, 89u8, 78u8, 101u8, 53u8, 240u8,
							98u8, 13u8, 104u8, 235u8, 161u8, 201u8, 150u8, 117u8, 32u8, 75u8,
							209u8, 166u8, 252u8, 57u8, 131u8, 96u8, 215u8, 51u8, 81u8, 42u8, 123u8,
						],
					)
				}
				pub fn change_key(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<ChangeKey> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"change_key",
						ChangeKey { new },
						[
							202u8, 114u8, 208u8, 33u8, 254u8, 51u8, 31u8, 220u8, 229u8, 251u8,
							167u8, 149u8, 139u8, 131u8, 252u8, 100u8, 32u8, 20u8, 72u8, 97u8, 5u8,
							8u8, 25u8, 198u8, 95u8, 154u8, 73u8, 220u8, 46u8, 85u8, 162u8, 40u8,
						],
					)
				}
				pub fn set_prime(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<SetPrime> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"set_prime",
						SetPrime { who },
						[
							109u8, 16u8, 35u8, 72u8, 169u8, 141u8, 101u8, 209u8, 241u8, 218u8,
							170u8, 180u8, 37u8, 223u8, 249u8, 37u8, 168u8, 20u8, 130u8, 30u8,
							191u8, 157u8, 230u8, 156u8, 135u8, 73u8, 96u8, 98u8, 193u8, 44u8, 38u8,
							247u8,
						],
					)
				}
				pub fn clear_prime(&self) -> ::subxt::tx::Payload<ClearPrime> {
					::subxt::tx::Payload::new_static(
						"CouncilMembership",
						"clear_prime",
						ClearPrime {},
						[
							186u8, 182u8, 225u8, 90u8, 71u8, 124u8, 69u8, 100u8, 234u8, 25u8, 53u8,
							23u8, 182u8, 32u8, 176u8, 81u8, 54u8, 140u8, 235u8, 126u8, 247u8, 7u8,
							155u8, 62u8, 35u8, 135u8, 48u8, 61u8, 88u8, 160u8, 183u8, 72u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_membership::pallet::Event;
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
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MembersReset";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "CouncilMembership";
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
						::subxt::utils::AccountId32,
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
							56u8, 56u8, 29u8, 90u8, 26u8, 115u8, 252u8, 185u8, 37u8, 108u8, 16u8,
							46u8, 136u8, 139u8, 30u8, 19u8, 235u8, 78u8, 176u8, 129u8, 180u8, 57u8,
							178u8, 239u8, 211u8, 6u8, 64u8, 129u8, 195u8, 46u8, 178u8, 157u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CouncilMembership",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod treasury {
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
			pub struct ProposeSpend {
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub beneficiary: ::subxt::utils::MultiAddress<
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
			pub struct RejectProposal {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ApproveProposal {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Spend {
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub beneficiary: ::subxt::utils::MultiAddress<
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
			pub struct RemoveApproval {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn propose_spend(
					&self,
					value: ::core::primitive::u128,
					beneficiary: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<ProposeSpend> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"propose_spend",
						ProposeSpend { value, beneficiary },
						[
							124u8, 32u8, 83u8, 127u8, 240u8, 169u8, 3u8, 190u8, 235u8, 163u8, 23u8,
							29u8, 88u8, 242u8, 238u8, 187u8, 136u8, 75u8, 193u8, 192u8, 239u8, 2u8,
							54u8, 238u8, 147u8, 42u8, 91u8, 14u8, 244u8, 175u8, 41u8, 14u8,
						],
					)
				}
				pub fn reject_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<RejectProposal> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"reject_proposal",
						RejectProposal { proposal_id },
						[
							106u8, 223u8, 97u8, 22u8, 111u8, 208u8, 128u8, 26u8, 198u8, 140u8,
							118u8, 126u8, 187u8, 51u8, 193u8, 50u8, 193u8, 68u8, 143u8, 144u8,
							34u8, 132u8, 44u8, 244u8, 105u8, 186u8, 223u8, 234u8, 17u8, 145u8,
							209u8, 145u8,
						],
					)
				}
				pub fn approve_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<ApproveProposal> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"approve_proposal",
						ApproveProposal { proposal_id },
						[
							164u8, 229u8, 172u8, 98u8, 129u8, 62u8, 84u8, 128u8, 47u8, 108u8, 33u8,
							120u8, 89u8, 79u8, 57u8, 121u8, 4u8, 197u8, 170u8, 153u8, 156u8, 17u8,
							59u8, 164u8, 123u8, 227u8, 175u8, 195u8, 220u8, 160u8, 60u8, 186u8,
						],
					)
				}
				pub fn spend(
					&self,
					amount: ::core::primitive::u128,
					beneficiary: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<Spend> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"spend",
						Spend { amount, beneficiary },
						[
							208u8, 79u8, 96u8, 218u8, 205u8, 209u8, 165u8, 119u8, 92u8, 208u8,
							54u8, 168u8, 83u8, 190u8, 98u8, 97u8, 6u8, 2u8, 35u8, 249u8, 18u8,
							88u8, 193u8, 51u8, 130u8, 33u8, 28u8, 99u8, 49u8, 194u8, 34u8, 77u8,
						],
					)
				}
				pub fn remove_approval(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<RemoveApproval> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"remove_approval",
						RemoveApproval { proposal_id },
						[
							133u8, 126u8, 181u8, 47u8, 196u8, 243u8, 7u8, 46u8, 25u8, 251u8, 154u8,
							125u8, 217u8, 77u8, 54u8, 245u8, 240u8, 180u8, 97u8, 34u8, 186u8, 53u8,
							225u8, 144u8, 155u8, 107u8, 172u8, 54u8, 250u8, 184u8, 178u8, 86u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_treasury::pallet::Event;
		pub mod events {
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
			pub struct Proposed {
				pub proposal_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Proposed";
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
			pub struct Spending {
				pub budget_remaining: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Spending {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Spending";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Awarded {
				pub proposal_index: ::core::primitive::u32,
				pub award: ::core::primitive::u128,
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Awarded {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Awarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Rejected {
				pub proposal_index: ::core::primitive::u32,
				pub slashed: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rejected {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rejected";
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
			pub struct Burnt {
				pub burnt_funds: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Burnt {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Burnt";
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
			pub struct Rollover {
				pub rollover_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rollover {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rollover";
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
			pub struct Deposit {
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SpendApproved {
				pub proposal_index: ::core::primitive::u32,
				pub amount: ::core::primitive::u128,
				pub beneficiary: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for SpendApproved {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "SpendApproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						],
					)
				}
				pub fn proposals(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_treasury::Proposal<
						::subxt::utils::AccountId32,
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
							62u8, 223u8, 55u8, 209u8, 151u8, 134u8, 122u8, 65u8, 207u8, 38u8,
							113u8, 213u8, 237u8, 48u8, 129u8, 32u8, 91u8, 228u8, 108u8, 91u8, 37u8,
							49u8, 94u8, 4u8, 75u8, 122u8, 25u8, 34u8, 198u8, 224u8, 246u8, 160u8,
						],
					)
				}
				pub fn proposals_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_treasury::Proposal<
						::subxt::utils::AccountId32,
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
							62u8, 223u8, 55u8, 209u8, 151u8, 134u8, 122u8, 65u8, 207u8, 38u8,
							113u8, 213u8, 237u8, 48u8, 129u8, 32u8, 91u8, 228u8, 108u8, 91u8, 37u8,
							49u8, 94u8, 4u8, 75u8, 122u8, 25u8, 34u8, 198u8, 224u8, 246u8, 160u8,
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
							159u8, 57u8, 5u8, 85u8, 136u8, 128u8, 70u8, 43u8, 67u8, 76u8, 123u8,
							206u8, 48u8, 253u8, 51u8, 40u8, 14u8, 35u8, 162u8, 173u8, 127u8, 79u8,
							38u8, 235u8, 9u8, 141u8, 201u8, 37u8, 211u8, 176u8, 119u8, 106u8,
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
							202u8, 106u8, 189u8, 40u8, 127u8, 172u8, 108u8, 50u8, 193u8, 4u8,
							248u8, 226u8, 176u8, 101u8, 212u8, 222u8, 64u8, 206u8, 244u8, 175u8,
							111u8, 106u8, 86u8, 96u8, 19u8, 109u8, 218u8, 152u8, 30u8, 59u8, 96u8,
							1u8,
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
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
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
							84u8, 154u8, 218u8, 83u8, 84u8, 189u8, 32u8, 20u8, 120u8, 194u8, 88u8,
							205u8, 109u8, 216u8, 114u8, 193u8, 120u8, 198u8, 154u8, 237u8, 134u8,
							204u8, 102u8, 247u8, 52u8, 103u8, 231u8, 43u8, 243u8, 122u8, 60u8,
							216u8,
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
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
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
							139u8, 109u8, 228u8, 151u8, 252u8, 32u8, 130u8, 69u8, 112u8, 154u8,
							174u8, 45u8, 83u8, 245u8, 51u8, 132u8, 173u8, 5u8, 186u8, 24u8, 243u8,
							9u8, 12u8, 214u8, 80u8, 74u8, 69u8, 189u8, 30u8, 94u8, 22u8, 39u8,
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
			pub struct Propose {
				pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
					runtime_types::composable_runtime::RuntimeCall,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Second {
				#[codec(compact)]
				pub proposal: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Vote {
				#[codec(compact)]
				pub ref_index: ::core::primitive::u32,
				pub vote:
					runtime_types::pallet_democracy::vote::AccountVote<::core::primitive::u128>,
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
			pub struct EmergencyCancel {
				pub ref_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExternalPropose {
				pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
					runtime_types::composable_runtime::RuntimeCall,
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
			pub struct ExternalProposeMajority {
				pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
					runtime_types::composable_runtime::RuntimeCall,
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
			pub struct ExternalProposeDefault {
				pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
					runtime_types::composable_runtime::RuntimeCall,
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
			pub struct FastTrack {
				pub proposal_hash: ::subxt::utils::H256,
				pub voting_period: ::core::primitive::u32,
				pub delay: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct VetoExternal {
				pub proposal_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CancelReferendum {
				#[codec(compact)]
				pub ref_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Delegate {
				pub to: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub conviction: runtime_types::pallet_democracy::conviction::Conviction,
				pub balance: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Undelegate;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ClearPublicProposals;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Unlock {
				pub target: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
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
			pub struct RemoveVote {
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RemoveOtherVote {
				pub target: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Blacklist {
				pub proposal_hash: ::subxt::utils::H256,
				pub maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CancelProposal {
				#[codec(compact)]
				pub prop_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SetMetadata {
				pub owner: runtime_types::pallet_democracy::types::MetadataOwner,
				pub maybe_hash: ::core::option::Option<::subxt::utils::H256>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn propose(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::composable_runtime::RuntimeCall,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<Propose> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"propose",
						Propose { proposal, value },
						[
							123u8, 3u8, 204u8, 140u8, 194u8, 195u8, 214u8, 39u8, 167u8, 126u8,
							45u8, 4u8, 219u8, 17u8, 143u8, 185u8, 29u8, 224u8, 230u8, 68u8, 253u8,
							15u8, 170u8, 90u8, 232u8, 123u8, 46u8, 255u8, 168u8, 39u8, 204u8, 63u8,
						],
					)
				}
				pub fn second(
					&self,
					proposal: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Second> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"second",
						Second { proposal },
						[
							59u8, 240u8, 183u8, 218u8, 61u8, 93u8, 184u8, 67u8, 10u8, 4u8, 138u8,
							196u8, 168u8, 49u8, 42u8, 69u8, 154u8, 42u8, 90u8, 112u8, 179u8, 69u8,
							51u8, 148u8, 159u8, 212u8, 221u8, 226u8, 132u8, 228u8, 51u8, 83u8,
						],
					)
				}
				pub fn vote(
					&self,
					ref_index: ::core::primitive::u32,
					vote: runtime_types::pallet_democracy::vote::AccountVote<
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::Payload<Vote> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"vote",
						Vote { ref_index, vote },
						[
							138u8, 213u8, 229u8, 111u8, 1u8, 191u8, 73u8, 3u8, 145u8, 28u8, 44u8,
							88u8, 163u8, 188u8, 129u8, 188u8, 64u8, 15u8, 64u8, 103u8, 250u8, 97u8,
							234u8, 188u8, 29u8, 205u8, 51u8, 6u8, 116u8, 58u8, 156u8, 201u8,
						],
					)
				}
				pub fn emergency_cancel(
					&self,
					ref_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<EmergencyCancel> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"emergency_cancel",
						EmergencyCancel { ref_index },
						[
							139u8, 213u8, 133u8, 75u8, 34u8, 206u8, 124u8, 245u8, 35u8, 237u8,
							132u8, 92u8, 49u8, 167u8, 117u8, 80u8, 188u8, 93u8, 198u8, 237u8,
							132u8, 77u8, 195u8, 65u8, 29u8, 37u8, 86u8, 74u8, 214u8, 119u8, 71u8,
							204u8,
						],
					)
				}
				pub fn external_propose(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::composable_runtime::RuntimeCall,
					>,
				) -> ::subxt::tx::Payload<ExternalPropose> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"external_propose",
						ExternalPropose { proposal },
						[
							164u8, 193u8, 14u8, 122u8, 105u8, 232u8, 20u8, 194u8, 99u8, 227u8,
							36u8, 105u8, 218u8, 146u8, 16u8, 208u8, 56u8, 62u8, 100u8, 65u8, 35u8,
							33u8, 51u8, 208u8, 17u8, 43u8, 223u8, 198u8, 202u8, 16u8, 56u8, 75u8,
						],
					)
				}
				pub fn external_propose_majority(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::composable_runtime::RuntimeCall,
					>,
				) -> ::subxt::tx::Payload<ExternalProposeMajority> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"external_propose_majority",
						ExternalProposeMajority { proposal },
						[
							129u8, 124u8, 147u8, 253u8, 69u8, 115u8, 230u8, 186u8, 155u8, 4u8,
							220u8, 103u8, 28u8, 132u8, 115u8, 153u8, 196u8, 88u8, 9u8, 130u8,
							129u8, 234u8, 75u8, 96u8, 202u8, 216u8, 145u8, 189u8, 231u8, 101u8,
							127u8, 11u8,
						],
					)
				}
				pub fn external_propose_default(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::composable_runtime::RuntimeCall,
					>,
				) -> ::subxt::tx::Payload<ExternalProposeDefault> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"external_propose_default",
						ExternalProposeDefault { proposal },
						[
							96u8, 15u8, 108u8, 208u8, 141u8, 247u8, 4u8, 73u8, 2u8, 30u8, 231u8,
							40u8, 184u8, 250u8, 42u8, 161u8, 248u8, 45u8, 217u8, 50u8, 53u8, 13u8,
							181u8, 214u8, 136u8, 51u8, 93u8, 95u8, 165u8, 3u8, 83u8, 190u8,
						],
					)
				}
				pub fn fast_track(
					&self,
					proposal_hash: ::subxt::utils::H256,
					voting_period: ::core::primitive::u32,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<FastTrack> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"fast_track",
						FastTrack { proposal_hash, voting_period, delay },
						[
							125u8, 209u8, 107u8, 120u8, 93u8, 205u8, 129u8, 147u8, 254u8, 126u8,
							45u8, 126u8, 39u8, 0u8, 56u8, 14u8, 233u8, 49u8, 245u8, 220u8, 156u8,
							10u8, 252u8, 31u8, 102u8, 90u8, 163u8, 236u8, 178u8, 85u8, 13u8, 24u8,
						],
					)
				}
				pub fn veto_external(
					&self,
					proposal_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<VetoExternal> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"veto_external",
						VetoExternal { proposal_hash },
						[
							209u8, 18u8, 18u8, 103u8, 186u8, 160u8, 214u8, 124u8, 150u8, 207u8,
							112u8, 90u8, 84u8, 197u8, 95u8, 157u8, 165u8, 65u8, 109u8, 101u8, 75u8,
							201u8, 41u8, 149u8, 75u8, 154u8, 37u8, 178u8, 239u8, 121u8, 124u8,
							23u8,
						],
					)
				}
				pub fn cancel_referendum(
					&self,
					ref_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<CancelReferendum> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"cancel_referendum",
						CancelReferendum { ref_index },
						[
							51u8, 25u8, 25u8, 251u8, 236u8, 115u8, 130u8, 230u8, 72u8, 186u8,
							119u8, 71u8, 165u8, 137u8, 55u8, 167u8, 187u8, 128u8, 55u8, 8u8, 212u8,
							139u8, 245u8, 232u8, 103u8, 136u8, 229u8, 113u8, 125u8, 36u8, 1u8,
							149u8,
						],
					)
				}
				pub fn delegate(
					&self,
					to: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					conviction: runtime_types::pallet_democracy::conviction::Conviction,
					balance: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<Delegate> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"delegate",
						Delegate { to, conviction, balance },
						[
							247u8, 226u8, 242u8, 221u8, 47u8, 161u8, 91u8, 223u8, 6u8, 79u8, 238u8,
							205u8, 41u8, 188u8, 140u8, 56u8, 181u8, 248u8, 102u8, 10u8, 127u8,
							166u8, 90u8, 187u8, 13u8, 124u8, 209u8, 117u8, 16u8, 209u8, 74u8, 29u8,
						],
					)
				}
				pub fn undelegate(&self) -> ::subxt::tx::Payload<Undelegate> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"undelegate",
						Undelegate {},
						[
							165u8, 40u8, 183u8, 209u8, 57u8, 153u8, 111u8, 29u8, 114u8, 109u8,
							107u8, 235u8, 97u8, 61u8, 53u8, 155u8, 44u8, 245u8, 28u8, 220u8, 56u8,
							134u8, 43u8, 122u8, 248u8, 156u8, 191u8, 154u8, 4u8, 121u8, 152u8,
							153u8,
						],
					)
				}
				pub fn clear_public_proposals(&self) -> ::subxt::tx::Payload<ClearPublicProposals> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"clear_public_proposals",
						ClearPublicProposals {},
						[
							59u8, 126u8, 254u8, 223u8, 252u8, 225u8, 75u8, 185u8, 188u8, 181u8,
							42u8, 179u8, 211u8, 73u8, 12u8, 141u8, 243u8, 197u8, 46u8, 130u8,
							215u8, 196u8, 225u8, 88u8, 48u8, 199u8, 231u8, 249u8, 195u8, 53u8,
							184u8, 204u8,
						],
					)
				}
				pub fn unlock(
					&self,
					target: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<Unlock> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"unlock",
						Unlock { target },
						[
							227u8, 6u8, 154u8, 150u8, 253u8, 167u8, 142u8, 6u8, 147u8, 24u8, 124u8,
							51u8, 101u8, 185u8, 184u8, 170u8, 6u8, 223u8, 29u8, 167u8, 73u8, 31u8,
							179u8, 60u8, 156u8, 244u8, 192u8, 233u8, 79u8, 99u8, 248u8, 126u8,
						],
					)
				}
				pub fn remove_vote(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<RemoveVote> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"remove_vote",
						RemoveVote { index },
						[
							148u8, 120u8, 14u8, 172u8, 81u8, 152u8, 159u8, 178u8, 106u8, 244u8,
							36u8, 98u8, 120u8, 189u8, 213u8, 93u8, 119u8, 156u8, 112u8, 34u8,
							241u8, 72u8, 206u8, 113u8, 212u8, 161u8, 164u8, 126u8, 122u8, 82u8,
							160u8, 74u8,
						],
					)
				}
				pub fn remove_other_vote(
					&self,
					target: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<RemoveOtherVote> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"remove_other_vote",
						RemoveOtherVote { target, index },
						[
							251u8, 245u8, 79u8, 229u8, 3u8, 107u8, 66u8, 202u8, 148u8, 31u8, 6u8,
							236u8, 156u8, 202u8, 197u8, 107u8, 100u8, 60u8, 255u8, 213u8, 222u8,
							209u8, 249u8, 61u8, 209u8, 215u8, 82u8, 73u8, 25u8, 73u8, 161u8, 24u8,
						],
					)
				}
				pub fn blacklist(
					&self,
					proposal_hash: ::subxt::utils::H256,
					maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::Payload<Blacklist> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"blacklist",
						Blacklist { proposal_hash, maybe_ref_index },
						[
							48u8, 144u8, 81u8, 164u8, 54u8, 111u8, 197u8, 134u8, 6u8, 98u8, 121u8,
							179u8, 254u8, 191u8, 204u8, 212u8, 84u8, 255u8, 86u8, 110u8, 225u8,
							130u8, 26u8, 65u8, 133u8, 56u8, 231u8, 15u8, 245u8, 137u8, 146u8,
							242u8,
						],
					)
				}
				pub fn cancel_proposal(
					&self,
					prop_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<CancelProposal> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"cancel_proposal",
						CancelProposal { prop_index },
						[
							179u8, 3u8, 198u8, 244u8, 241u8, 124u8, 205u8, 58u8, 100u8, 80u8,
							177u8, 254u8, 98u8, 220u8, 189u8, 63u8, 229u8, 60u8, 157u8, 83u8,
							142u8, 6u8, 236u8, 183u8, 193u8, 235u8, 253u8, 126u8, 153u8, 185u8,
							74u8, 117u8,
						],
					)
				}
				pub fn set_metadata(
					&self,
					owner: runtime_types::pallet_democracy::types::MetadataOwner,
					maybe_hash: ::core::option::Option<::subxt::utils::H256>,
				) -> ::subxt::tx::Payload<SetMetadata> {
					::subxt::tx::Payload::new_static(
						"Democracy",
						"set_metadata",
						SetMetadata { owner, maybe_hash },
						[
							182u8, 2u8, 168u8, 244u8, 247u8, 35u8, 65u8, 9u8, 39u8, 164u8, 30u8,
							141u8, 69u8, 137u8, 75u8, 156u8, 158u8, 107u8, 67u8, 28u8, 145u8, 65u8,
							175u8, 30u8, 254u8, 231u8, 4u8, 77u8, 207u8, 166u8, 157u8, 73u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_democracy::pallet::Event;
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
			pub struct Proposed {
				pub proposal_index: ::core::primitive::u32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Tabled {
				pub proposal_index: ::core::primitive::u32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Tabled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Tabled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExternalTabled;
			impl ::subxt::events::StaticEvent for ExternalTabled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "ExternalTabled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Started {
				pub ref_index: ::core::primitive::u32,
				pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
			}
			impl ::subxt::events::StaticEvent for Started {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Started";
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
			pub struct Passed {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Passed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Passed";
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
			pub struct NotPassed {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NotPassed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "NotPassed";
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
			pub struct Cancelled {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Cancelled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Cancelled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Delegated {
				pub who: ::subxt::utils::AccountId32,
				pub target: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Delegated {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Delegated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Undelegated {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Undelegated {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Undelegated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Vetoed {
				pub who: ::subxt::utils::AccountId32,
				pub proposal_hash: ::subxt::utils::H256,
				pub until: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Vetoed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Vetoed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Blacklisted {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Blacklisted {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Blacklisted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Voted {
				pub voter: ::subxt::utils::AccountId32,
				pub ref_index: ::core::primitive::u32,
				pub vote:
					runtime_types::pallet_democracy::vote::AccountVote<::core::primitive::u128>,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Voted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Seconded {
				pub seconder: ::subxt::utils::AccountId32,
				pub prop_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Seconded {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Seconded";
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
			pub struct ProposalCanceled {
				pub prop_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ProposalCanceled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "ProposalCanceled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MetadataSet {
				pub owner: runtime_types::pallet_democracy::types::MetadataOwner,
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for MetadataSet {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "MetadataSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MetadataCleared {
				pub owner: runtime_types::pallet_democracy::types::MetadataOwner,
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for MetadataCleared {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "MetadataCleared";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MetadataTransferred {
				pub prev_owner: runtime_types::pallet_democracy::types::MetadataOwner,
				pub owner: runtime_types::pallet_democracy::types::MetadataOwner,
				pub hash: ::subxt::utils::H256,
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
							91u8, 14u8, 171u8, 94u8, 37u8, 157u8, 46u8, 157u8, 254u8, 13u8, 68u8,
							144u8, 23u8, 146u8, 128u8, 159u8, 9u8, 174u8, 74u8, 174u8, 218u8,
							197u8, 23u8, 235u8, 152u8, 226u8, 216u8, 4u8, 120u8, 121u8, 27u8,
							138u8,
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
							runtime_types::composable_runtime::RuntimeCall,
						>,
						::subxt::utils::AccountId32,
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
							63u8, 172u8, 211u8, 85u8, 27u8, 14u8, 86u8, 49u8, 133u8, 5u8, 132u8,
							189u8, 138u8, 137u8, 219u8, 37u8, 209u8, 49u8, 172u8, 86u8, 240u8,
							235u8, 42u8, 201u8, 203u8, 12u8, 122u8, 225u8, 0u8, 109u8, 205u8,
							103u8,
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
							::subxt::utils::AccountId32,
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
							9u8, 219u8, 11u8, 58u8, 17u8, 194u8, 248u8, 154u8, 135u8, 119u8, 123u8,
							235u8, 252u8, 176u8, 190u8, 162u8, 236u8, 45u8, 237u8, 125u8, 98u8,
							176u8, 184u8, 160u8, 8u8, 181u8, 213u8, 65u8, 14u8, 84u8, 200u8, 64u8,
						],
					)
				}
				pub fn deposit_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
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
							9u8, 219u8, 11u8, 58u8, 17u8, 194u8, 248u8, 154u8, 135u8, 119u8, 123u8,
							235u8, 252u8, 176u8, 190u8, 162u8, 236u8, 45u8, 237u8, 125u8, 98u8,
							176u8, 184u8, 160u8, 8u8, 181u8, 213u8, 65u8, 14u8, 84u8, 200u8, 64u8,
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
							153u8, 210u8, 106u8, 244u8, 156u8, 70u8, 124u8, 251u8, 123u8, 75u8,
							7u8, 189u8, 199u8, 145u8, 95u8, 119u8, 137u8, 11u8, 240u8, 160u8,
							151u8, 248u8, 229u8, 231u8, 89u8, 222u8, 18u8, 237u8, 144u8, 78u8,
							99u8, 58u8,
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
							4u8, 51u8, 108u8, 11u8, 48u8, 165u8, 19u8, 251u8, 182u8, 76u8, 163u8,
							73u8, 227u8, 2u8, 212u8, 74u8, 128u8, 27u8, 165u8, 164u8, 111u8, 22u8,
							209u8, 190u8, 103u8, 7u8, 116u8, 16u8, 160u8, 144u8, 123u8, 64u8,
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
							runtime_types::composable_runtime::RuntimeCall,
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
							167u8, 58u8, 230u8, 197u8, 185u8, 56u8, 181u8, 32u8, 81u8, 150u8, 29u8,
							138u8, 142u8, 38u8, 255u8, 216u8, 139u8, 93u8, 56u8, 148u8, 196u8,
							169u8, 168u8, 144u8, 193u8, 200u8, 187u8, 5u8, 141u8, 201u8, 254u8,
							248u8,
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
							runtime_types::composable_runtime::RuntimeCall,
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
							167u8, 58u8, 230u8, 197u8, 185u8, 56u8, 181u8, 32u8, 81u8, 150u8, 29u8,
							138u8, 142u8, 38u8, 255u8, 216u8, 139u8, 93u8, 56u8, 148u8, 196u8,
							169u8, 168u8, 144u8, 193u8, 200u8, 187u8, 5u8, 141u8, 201u8, 254u8,
							248u8,
						],
					)
				}
				pub fn voting_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_democracy::vote::Voting<
						::core::primitive::u128,
						::subxt::utils::AccountId32,
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
							125u8, 121u8, 167u8, 170u8, 18u8, 194u8, 183u8, 38u8, 176u8, 48u8,
							30u8, 88u8, 233u8, 196u8, 33u8, 119u8, 160u8, 201u8, 29u8, 183u8, 88u8,
							67u8, 219u8, 137u8, 6u8, 195u8, 11u8, 63u8, 162u8, 181u8, 82u8, 243u8,
						],
					)
				}
				pub fn voting_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_democracy::vote::Voting<
						::core::primitive::u128,
						::subxt::utils::AccountId32,
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
							125u8, 121u8, 167u8, 170u8, 18u8, 194u8, 183u8, 38u8, 176u8, 48u8,
							30u8, 88u8, 233u8, 196u8, 33u8, 119u8, 160u8, 201u8, 29u8, 183u8, 88u8,
							67u8, 219u8, 137u8, 6u8, 195u8, 11u8, 63u8, 162u8, 181u8, 82u8, 243u8,
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
							3u8, 67u8, 106u8, 1u8, 89u8, 204u8, 4u8, 145u8, 121u8, 44u8, 34u8,
							76u8, 18u8, 206u8, 65u8, 214u8, 222u8, 82u8, 31u8, 223u8, 144u8, 169u8,
							17u8, 6u8, 138u8, 36u8, 113u8, 155u8, 241u8, 106u8, 189u8, 218u8,
						],
					)
				}
				pub fn next_external(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::composable_runtime::RuntimeCall,
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
							213u8, 36u8, 235u8, 75u8, 153u8, 33u8, 140u8, 121u8, 191u8, 197u8,
							17u8, 57u8, 234u8, 67u8, 81u8, 55u8, 123u8, 179u8, 207u8, 124u8, 238u8,
							147u8, 243u8, 126u8, 200u8, 2u8, 16u8, 143u8, 165u8, 143u8, 159u8,
							93u8,
						],
					)
				}
				pub fn blacklist(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u32,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
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
							8u8, 227u8, 185u8, 179u8, 192u8, 92u8, 171u8, 125u8, 237u8, 224u8,
							109u8, 207u8, 44u8, 181u8, 78u8, 17u8, 254u8, 183u8, 199u8, 241u8,
							49u8, 90u8, 101u8, 168u8, 46u8, 89u8, 253u8, 155u8, 38u8, 183u8, 112u8,
							35u8,
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
							::subxt::utils::AccountId32,
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
							8u8, 227u8, 185u8, 179u8, 192u8, 92u8, 171u8, 125u8, 237u8, 224u8,
							109u8, 207u8, 44u8, 181u8, 78u8, 17u8, 254u8, 183u8, 199u8, 241u8,
							49u8, 90u8, 101u8, 168u8, 46u8, 89u8, 253u8, 155u8, 38u8, 183u8, 112u8,
							35u8,
						],
					)
				}
				pub fn cancellations(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
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
							154u8, 36u8, 172u8, 46u8, 65u8, 218u8, 30u8, 151u8, 173u8, 186u8,
							166u8, 79u8, 35u8, 226u8, 94u8, 200u8, 67u8, 44u8, 47u8, 7u8, 17u8,
							89u8, 169u8, 166u8, 236u8, 101u8, 68u8, 54u8, 114u8, 141u8, 177u8,
							135u8,
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
							154u8, 36u8, 172u8, 46u8, 65u8, 218u8, 30u8, 151u8, 173u8, 186u8,
							166u8, 79u8, 35u8, 226u8, 94u8, 200u8, 67u8, 44u8, 47u8, 7u8, 17u8,
							89u8, 169u8, 166u8, 236u8, 101u8, 68u8, 54u8, 114u8, 141u8, 177u8,
							135u8,
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
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"MetadataOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							157u8, 252u8, 120u8, 151u8, 76u8, 82u8, 189u8, 77u8, 196u8, 65u8,
							113u8, 138u8, 138u8, 57u8, 199u8, 136u8, 22u8, 35u8, 114u8, 144u8,
							172u8, 42u8, 130u8, 19u8, 19u8, 245u8, 76u8, 177u8, 145u8, 146u8,
							107u8, 23u8,
						],
					)
				}
				pub fn metadata_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Democracy",
						"MetadataOf",
						Vec::new(),
						[
							157u8, 252u8, 120u8, 151u8, 76u8, 82u8, 189u8, 77u8, 196u8, 65u8,
							113u8, 138u8, 138u8, 57u8, 199u8, 136u8, 22u8, 35u8, 114u8, 144u8,
							172u8, 42u8, 130u8, 19u8, 19u8, 245u8, 76u8, 177u8, 145u8, 146u8,
							107u8, 23u8,
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
			pub struct SetMembers {
				pub new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				pub prime: ::core::option::Option<::subxt::utils::AccountId32>,
				pub old_count: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Execute {
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Propose {
				#[codec(compact)]
				pub threshold: ::core::primitive::u32,
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Vote {
				pub proposal: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub approve: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CloseOldWeight {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				#[codec(compact)]
				pub proposal_weight_bound: runtime_types::sp_weights::OldWeight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Close {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
					prime: ::core::option::Option<::subxt::utils::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<SetMembers> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"set_members",
						SetMembers { new_members, prime, old_count },
						[
							196u8, 103u8, 123u8, 125u8, 226u8, 177u8, 126u8, 37u8, 160u8, 114u8,
							34u8, 136u8, 219u8, 84u8, 199u8, 94u8, 242u8, 20u8, 126u8, 126u8,
							166u8, 190u8, 198u8, 33u8, 162u8, 113u8, 237u8, 222u8, 90u8, 1u8, 2u8,
							234u8,
						],
					)
				}
				pub fn execute(
					&self,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Execute> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"execute",
						Execute { proposal: ::std::boxed::Box::new(proposal), length_bound },
						[
							149u8, 198u8, 212u8, 65u8, 131u8, 242u8, 252u8, 178u8, 31u8, 10u8,
							199u8, 72u8, 104u8, 41u8, 174u8, 137u8, 112u8, 63u8, 12u8, 238u8,
							184u8, 149u8, 246u8, 157u8, 36u8, 172u8, 29u8, 56u8, 250u8, 255u8,
							146u8, 137u8,
						],
					)
				}
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Propose> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"propose",
						Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							41u8, 137u8, 175u8, 40u8, 83u8, 161u8, 15u8, 132u8, 102u8, 169u8,
							162u8, 20u8, 190u8, 109u8, 97u8, 84u8, 162u8, 83u8, 62u8, 245u8, 65u8,
							190u8, 138u8, 253u8, 155u8, 235u8, 220u8, 39u8, 80u8, 31u8, 244u8,
							175u8,
						],
					)
				}
				pub fn vote(
					&self,
					proposal: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<Vote> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"vote",
						Vote { proposal, index, approve },
						[
							108u8, 46u8, 180u8, 148u8, 145u8, 24u8, 173u8, 56u8, 36u8, 100u8,
							216u8, 43u8, 178u8, 202u8, 26u8, 136u8, 93u8, 84u8, 80u8, 134u8, 14u8,
							42u8, 248u8, 205u8, 68u8, 92u8, 79u8, 11u8, 113u8, 115u8, 157u8, 100u8,
						],
					)
				}
				pub fn close_old_weight(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<CloseOldWeight> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"close_old_weight",
						CloseOldWeight {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							133u8, 219u8, 90u8, 40u8, 102u8, 95u8, 4u8, 199u8, 45u8, 234u8, 109u8,
							17u8, 162u8, 63u8, 102u8, 186u8, 95u8, 182u8, 13u8, 123u8, 227u8, 20u8,
							186u8, 207u8, 12u8, 47u8, 87u8, 252u8, 244u8, 172u8, 60u8, 206u8,
						],
					)
				}
				pub fn disapprove_proposal(
					&self,
					proposal_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<DisapproveProposal> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"disapprove_proposal",
						DisapproveProposal { proposal_hash },
						[
							25u8, 123u8, 1u8, 8u8, 74u8, 37u8, 3u8, 40u8, 97u8, 37u8, 175u8, 224u8,
							72u8, 155u8, 123u8, 109u8, 104u8, 43u8, 91u8, 125u8, 199u8, 51u8, 17u8,
							225u8, 133u8, 38u8, 120u8, 76u8, 164u8, 5u8, 194u8, 201u8,
						],
					)
				}
				pub fn close(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Close> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"close",
						Close { proposal_hash, index, proposal_weight_bound, length_bound },
						[
							191u8, 138u8, 89u8, 247u8, 97u8, 51u8, 45u8, 193u8, 76u8, 16u8, 80u8,
							225u8, 197u8, 83u8, 204u8, 133u8, 169u8, 16u8, 86u8, 32u8, 125u8, 16u8,
							116u8, 185u8, 45u8, 20u8, 76u8, 148u8, 206u8, 163u8, 154u8, 30u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_collective::pallet::Event;
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
			pub struct Proposed {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: ::subxt::utils::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Voted {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_hash: ::subxt::utils::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Voted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Approved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Approved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Disapproved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Disapproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Executed {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Executed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MemberExecuted {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "MemberExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Closed {
				pub proposal_hash: ::subxt::utils::H256,
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
						::subxt::utils::H256,
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
							10u8, 133u8, 82u8, 54u8, 193u8, 41u8, 253u8, 159u8, 56u8, 96u8, 249u8,
							148u8, 43u8, 57u8, 116u8, 43u8, 222u8, 243u8, 237u8, 231u8, 238u8,
							60u8, 26u8, 225u8, 19u8, 203u8, 213u8, 220u8, 114u8, 217u8, 100u8,
							27u8,
						],
					)
				}
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_runtime::RuntimeCall,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"ProposalOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							164u8, 89u8, 234u8, 130u8, 84u8, 52u8, 167u8, 211u8, 224u8, 161u8,
							138u8, 7u8, 93u8, 58u8, 187u8, 187u8, 211u8, 209u8, 138u8, 12u8, 140u8,
							59u8, 99u8, 14u8, 230u8, 195u8, 22u8, 237u8, 126u8, 155u8, 43u8, 59u8,
						],
					)
				}
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_runtime::RuntimeCall,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"ProposalOf",
						Vec::new(),
						[
							164u8, 89u8, 234u8, 130u8, 84u8, 52u8, 167u8, 211u8, 224u8, 161u8,
							138u8, 7u8, 93u8, 58u8, 187u8, 187u8, 211u8, 209u8, 138u8, 12u8, 140u8,
							59u8, 99u8, 14u8, 230u8, 195u8, 22u8, 237u8, 126u8, 155u8, 43u8, 59u8,
						],
					)
				}
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						::subxt::utils::AccountId32,
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
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						::subxt::utils::AccountId32,
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
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
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
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						],
					)
				}
				pub fn members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Members",
						vec![],
						[
							162u8, 72u8, 174u8, 204u8, 140u8, 105u8, 205u8, 176u8, 197u8, 117u8,
							206u8, 134u8, 157u8, 110u8, 139u8, 54u8, 43u8, 233u8, 25u8, 51u8, 36u8,
							238u8, 94u8, 124u8, 221u8, 52u8, 237u8, 71u8, 125u8, 56u8, 129u8,
							222u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod technical_committee_membership {
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
			pub struct AddMember {
				pub who: ::subxt::utils::MultiAddress<
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
			pub struct RemoveMember {
				pub who: ::subxt::utils::MultiAddress<
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
			pub struct SwapMember {
				pub remove: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub add: ::subxt::utils::MultiAddress<
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
			pub struct ResetMembers {
				pub members: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChangeKey {
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
			pub struct SetPrime {
				pub who: ::subxt::utils::MultiAddress<
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
			pub struct ClearPrime;
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<AddMember> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"add_member",
						AddMember { who },
						[
							168u8, 166u8, 6u8, 167u8, 12u8, 109u8, 99u8, 96u8, 240u8, 57u8, 60u8,
							174u8, 57u8, 52u8, 131u8, 16u8, 230u8, 172u8, 23u8, 140u8, 48u8, 131u8,
							73u8, 131u8, 133u8, 217u8, 137u8, 50u8, 165u8, 149u8, 174u8, 188u8,
						],
					)
				}
				pub fn remove_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<RemoveMember> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"remove_member",
						RemoveMember { who },
						[
							33u8, 178u8, 96u8, 158u8, 126u8, 172u8, 0u8, 207u8, 143u8, 144u8,
							219u8, 28u8, 205u8, 197u8, 192u8, 195u8, 141u8, 26u8, 39u8, 101u8,
							140u8, 88u8, 212u8, 26u8, 221u8, 29u8, 187u8, 160u8, 119u8, 101u8,
							45u8, 162u8,
						],
					)
				}
				pub fn swap_member(
					&self,
					remove: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					add: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<SwapMember> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"swap_member",
						SwapMember { remove, add },
						[
							52u8, 10u8, 13u8, 175u8, 35u8, 141u8, 159u8, 135u8, 34u8, 235u8, 117u8,
							146u8, 134u8, 49u8, 76u8, 116u8, 93u8, 209u8, 24u8, 242u8, 123u8, 82u8,
							34u8, 192u8, 147u8, 237u8, 163u8, 167u8, 18u8, 64u8, 196u8, 132u8,
						],
					)
				}
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<ResetMembers> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"reset_members",
						ResetMembers { members },
						[
							9u8, 35u8, 28u8, 59u8, 158u8, 232u8, 89u8, 78u8, 101u8, 53u8, 240u8,
							98u8, 13u8, 104u8, 235u8, 161u8, 201u8, 150u8, 117u8, 32u8, 75u8,
							209u8, 166u8, 252u8, 57u8, 131u8, 96u8, 215u8, 51u8, 81u8, 42u8, 123u8,
						],
					)
				}
				pub fn change_key(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<ChangeKey> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"change_key",
						ChangeKey { new },
						[
							202u8, 114u8, 208u8, 33u8, 254u8, 51u8, 31u8, 220u8, 229u8, 251u8,
							167u8, 149u8, 139u8, 131u8, 252u8, 100u8, 32u8, 20u8, 72u8, 97u8, 5u8,
							8u8, 25u8, 198u8, 95u8, 154u8, 73u8, 220u8, 46u8, 85u8, 162u8, 40u8,
						],
					)
				}
				pub fn set_prime(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<SetPrime> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"set_prime",
						SetPrime { who },
						[
							109u8, 16u8, 35u8, 72u8, 169u8, 141u8, 101u8, 209u8, 241u8, 218u8,
							170u8, 180u8, 37u8, 223u8, 249u8, 37u8, 168u8, 20u8, 130u8, 30u8,
							191u8, 157u8, 230u8, 156u8, 135u8, 73u8, 96u8, 98u8, 193u8, 44u8, 38u8,
							247u8,
						],
					)
				}
				pub fn clear_prime(&self) -> ::subxt::tx::Payload<ClearPrime> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommitteeMembership",
						"clear_prime",
						ClearPrime {},
						[
							186u8, 182u8, 225u8, 90u8, 71u8, 124u8, 69u8, 100u8, 234u8, 25u8, 53u8,
							23u8, 182u8, 32u8, 176u8, 81u8, 54u8, 140u8, 235u8, 126u8, 247u8, 7u8,
							155u8, 62u8, 35u8, 135u8, 48u8, 61u8, 88u8, 160u8, 183u8, 72u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_membership::pallet::Event;
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
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MembersReset";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
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
						::subxt::utils::AccountId32,
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
							56u8, 56u8, 29u8, 90u8, 26u8, 115u8, 252u8, 185u8, 37u8, 108u8, 16u8,
							46u8, 136u8, 139u8, 30u8, 19u8, 235u8, 78u8, 176u8, 129u8, 180u8, 57u8,
							178u8, 239u8, 211u8, 6u8, 64u8, 129u8, 195u8, 46u8, 178u8, 157u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommitteeMembership",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod release_committee {
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
			pub struct SetMembers {
				pub new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				pub prime: ::core::option::Option<::subxt::utils::AccountId32>,
				pub old_count: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Execute {
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Propose {
				#[codec(compact)]
				pub threshold: ::core::primitive::u32,
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Vote {
				pub proposal: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub approve: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CloseOldWeight {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				#[codec(compact)]
				pub proposal_weight_bound: runtime_types::sp_weights::OldWeight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Close {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
					prime: ::core::option::Option<::subxt::utils::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<SetMembers> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"set_members",
						SetMembers { new_members, prime, old_count },
						[
							196u8, 103u8, 123u8, 125u8, 226u8, 177u8, 126u8, 37u8, 160u8, 114u8,
							34u8, 136u8, 219u8, 84u8, 199u8, 94u8, 242u8, 20u8, 126u8, 126u8,
							166u8, 190u8, 198u8, 33u8, 162u8, 113u8, 237u8, 222u8, 90u8, 1u8, 2u8,
							234u8,
						],
					)
				}
				pub fn execute(
					&self,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Execute> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"execute",
						Execute { proposal: ::std::boxed::Box::new(proposal), length_bound },
						[
							149u8, 198u8, 212u8, 65u8, 131u8, 242u8, 252u8, 178u8, 31u8, 10u8,
							199u8, 72u8, 104u8, 41u8, 174u8, 137u8, 112u8, 63u8, 12u8, 238u8,
							184u8, 149u8, 246u8, 157u8, 36u8, 172u8, 29u8, 56u8, 250u8, 255u8,
							146u8, 137u8,
						],
					)
				}
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Propose> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"propose",
						Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							41u8, 137u8, 175u8, 40u8, 83u8, 161u8, 15u8, 132u8, 102u8, 169u8,
							162u8, 20u8, 190u8, 109u8, 97u8, 84u8, 162u8, 83u8, 62u8, 245u8, 65u8,
							190u8, 138u8, 253u8, 155u8, 235u8, 220u8, 39u8, 80u8, 31u8, 244u8,
							175u8,
						],
					)
				}
				pub fn vote(
					&self,
					proposal: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<Vote> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"vote",
						Vote { proposal, index, approve },
						[
							108u8, 46u8, 180u8, 148u8, 145u8, 24u8, 173u8, 56u8, 36u8, 100u8,
							216u8, 43u8, 178u8, 202u8, 26u8, 136u8, 93u8, 84u8, 80u8, 134u8, 14u8,
							42u8, 248u8, 205u8, 68u8, 92u8, 79u8, 11u8, 113u8, 115u8, 157u8, 100u8,
						],
					)
				}
				pub fn close_old_weight(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<CloseOldWeight> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"close_old_weight",
						CloseOldWeight {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							133u8, 219u8, 90u8, 40u8, 102u8, 95u8, 4u8, 199u8, 45u8, 234u8, 109u8,
							17u8, 162u8, 63u8, 102u8, 186u8, 95u8, 182u8, 13u8, 123u8, 227u8, 20u8,
							186u8, 207u8, 12u8, 47u8, 87u8, 252u8, 244u8, 172u8, 60u8, 206u8,
						],
					)
				}
				pub fn disapprove_proposal(
					&self,
					proposal_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<DisapproveProposal> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"disapprove_proposal",
						DisapproveProposal { proposal_hash },
						[
							25u8, 123u8, 1u8, 8u8, 74u8, 37u8, 3u8, 40u8, 97u8, 37u8, 175u8, 224u8,
							72u8, 155u8, 123u8, 109u8, 104u8, 43u8, 91u8, 125u8, 199u8, 51u8, 17u8,
							225u8, 133u8, 38u8, 120u8, 76u8, 164u8, 5u8, 194u8, 201u8,
						],
					)
				}
				pub fn close(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Close> {
					::subxt::tx::Payload::new_static(
						"ReleaseCommittee",
						"close",
						Close { proposal_hash, index, proposal_weight_bound, length_bound },
						[
							191u8, 138u8, 89u8, 247u8, 97u8, 51u8, 45u8, 193u8, 76u8, 16u8, 80u8,
							225u8, 197u8, 83u8, 204u8, 133u8, 169u8, 16u8, 86u8, 32u8, 125u8, 16u8,
							116u8, 185u8, 45u8, 20u8, 76u8, 148u8, 206u8, 163u8, 154u8, 30u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_collective::pallet::Event;
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
			pub struct Proposed {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: ::subxt::utils::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Voted {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_hash: ::subxt::utils::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Voted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Approved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Approved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Disapproved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Disapproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Executed {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Executed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MemberExecuted {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "MemberExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Closed {
				pub proposal_hash: ::subxt::utils::H256,
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
						::subxt::utils::H256,
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
							10u8, 133u8, 82u8, 54u8, 193u8, 41u8, 253u8, 159u8, 56u8, 96u8, 249u8,
							148u8, 43u8, 57u8, 116u8, 43u8, 222u8, 243u8, 237u8, 231u8, 238u8,
							60u8, 26u8, 225u8, 19u8, 203u8, 213u8, 220u8, 114u8, 217u8, 100u8,
							27u8,
						],
					)
				}
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_runtime::RuntimeCall,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"ProposalOf",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							164u8, 89u8, 234u8, 130u8, 84u8, 52u8, 167u8, 211u8, 224u8, 161u8,
							138u8, 7u8, 93u8, 58u8, 187u8, 187u8, 211u8, 209u8, 138u8, 12u8, 140u8,
							59u8, 99u8, 14u8, 230u8, 195u8, 22u8, 237u8, 126u8, 155u8, 43u8, 59u8,
						],
					)
				}
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_runtime::RuntimeCall,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"ProposalOf",
						Vec::new(),
						[
							164u8, 89u8, 234u8, 130u8, 84u8, 52u8, 167u8, 211u8, 224u8, 161u8,
							138u8, 7u8, 93u8, 58u8, 187u8, 187u8, 211u8, 209u8, 138u8, 12u8, 140u8,
							59u8, 99u8, 14u8, 230u8, 195u8, 22u8, 237u8, 126u8, 155u8, 43u8, 59u8,
						],
					)
				}
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						::subxt::utils::AccountId32,
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
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						::subxt::utils::AccountId32,
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
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
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
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						],
					)
				}
				pub fn members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"Members",
						vec![],
						[
							162u8, 72u8, 174u8, 204u8, 140u8, 105u8, 205u8, 176u8, 197u8, 117u8,
							206u8, 134u8, 157u8, 110u8, 139u8, 54u8, 43u8, 233u8, 25u8, 51u8, 36u8,
							238u8, 94u8, 124u8, 221u8, 52u8, 237u8, 71u8, 125u8, 56u8, 129u8,
							222u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseCommittee",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod release_membership {
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
			pub struct AddMember {
				pub who: ::subxt::utils::MultiAddress<
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
			pub struct RemoveMember {
				pub who: ::subxt::utils::MultiAddress<
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
			pub struct SwapMember {
				pub remove: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub add: ::subxt::utils::MultiAddress<
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
			pub struct ResetMembers {
				pub members: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChangeKey {
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
			pub struct SetPrime {
				pub who: ::subxt::utils::MultiAddress<
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
			pub struct ClearPrime;
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<AddMember> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"add_member",
						AddMember { who },
						[
							168u8, 166u8, 6u8, 167u8, 12u8, 109u8, 99u8, 96u8, 240u8, 57u8, 60u8,
							174u8, 57u8, 52u8, 131u8, 16u8, 230u8, 172u8, 23u8, 140u8, 48u8, 131u8,
							73u8, 131u8, 133u8, 217u8, 137u8, 50u8, 165u8, 149u8, 174u8, 188u8,
						],
					)
				}
				pub fn remove_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<RemoveMember> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"remove_member",
						RemoveMember { who },
						[
							33u8, 178u8, 96u8, 158u8, 126u8, 172u8, 0u8, 207u8, 143u8, 144u8,
							219u8, 28u8, 205u8, 197u8, 192u8, 195u8, 141u8, 26u8, 39u8, 101u8,
							140u8, 88u8, 212u8, 26u8, 221u8, 29u8, 187u8, 160u8, 119u8, 101u8,
							45u8, 162u8,
						],
					)
				}
				pub fn swap_member(
					&self,
					remove: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					add: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<SwapMember> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"swap_member",
						SwapMember { remove, add },
						[
							52u8, 10u8, 13u8, 175u8, 35u8, 141u8, 159u8, 135u8, 34u8, 235u8, 117u8,
							146u8, 134u8, 49u8, 76u8, 116u8, 93u8, 209u8, 24u8, 242u8, 123u8, 82u8,
							34u8, 192u8, 147u8, 237u8, 163u8, 167u8, 18u8, 64u8, 196u8, 132u8,
						],
					)
				}
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<ResetMembers> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"reset_members",
						ResetMembers { members },
						[
							9u8, 35u8, 28u8, 59u8, 158u8, 232u8, 89u8, 78u8, 101u8, 53u8, 240u8,
							98u8, 13u8, 104u8, 235u8, 161u8, 201u8, 150u8, 117u8, 32u8, 75u8,
							209u8, 166u8, 252u8, 57u8, 131u8, 96u8, 215u8, 51u8, 81u8, 42u8, 123u8,
						],
					)
				}
				pub fn change_key(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<ChangeKey> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"change_key",
						ChangeKey { new },
						[
							202u8, 114u8, 208u8, 33u8, 254u8, 51u8, 31u8, 220u8, 229u8, 251u8,
							167u8, 149u8, 139u8, 131u8, 252u8, 100u8, 32u8, 20u8, 72u8, 97u8, 5u8,
							8u8, 25u8, 198u8, 95u8, 154u8, 73u8, 220u8, 46u8, 85u8, 162u8, 40u8,
						],
					)
				}
				pub fn set_prime(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<SetPrime> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"set_prime",
						SetPrime { who },
						[
							109u8, 16u8, 35u8, 72u8, 169u8, 141u8, 101u8, 209u8, 241u8, 218u8,
							170u8, 180u8, 37u8, 223u8, 249u8, 37u8, 168u8, 20u8, 130u8, 30u8,
							191u8, 157u8, 230u8, 156u8, 135u8, 73u8, 96u8, 98u8, 193u8, 44u8, 38u8,
							247u8,
						],
					)
				}
				pub fn clear_prime(&self) -> ::subxt::tx::Payload<ClearPrime> {
					::subxt::tx::Payload::new_static(
						"ReleaseMembership",
						"clear_prime",
						ClearPrime {},
						[
							186u8, 182u8, 225u8, 90u8, 71u8, 124u8, 69u8, 100u8, 234u8, 25u8, 53u8,
							23u8, 182u8, 32u8, 176u8, 81u8, 54u8, 140u8, 235u8, 126u8, 247u8, 7u8,
							155u8, 62u8, 35u8, 135u8, 48u8, 61u8, 88u8, 160u8, 183u8, 72u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_membership::pallet::Event;
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
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MembersReset";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "ReleaseMembership";
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
						::subxt::utils::AccountId32,
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
							56u8, 56u8, 29u8, 90u8, 26u8, 115u8, 252u8, 185u8, 37u8, 108u8, 16u8,
							46u8, 136u8, 139u8, 30u8, 19u8, 235u8, 78u8, 176u8, 129u8, 180u8, 57u8,
							178u8, 239u8, 211u8, 6u8, 64u8, 129u8, 195u8, 46u8, 178u8, 157u8,
						],
					)
				}
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ReleaseMembership",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod scheduler {
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
			pub struct Schedule {
				pub when: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
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
			pub struct Cancel {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ScheduleNamed {
				pub id: [::core::primitive::u8; 32usize],
				pub when: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
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
			pub struct CancelNamed {
				pub id: [::core::primitive::u8; 32usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ScheduleAfter {
				pub after: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
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
			pub struct ScheduleNamedAfter {
				pub id: [::core::primitive::u8; 32usize],
				pub after: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
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
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<Schedule> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule",
						Schedule {
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							105u8, 87u8, 79u8, 178u8, 171u8, 123u8, 163u8, 35u8, 223u8, 54u8,
							231u8, 253u8, 36u8, 6u8, 214u8, 70u8, 159u8, 65u8, 5u8, 195u8, 200u8,
							29u8, 94u8, 61u8, 25u8, 149u8, 216u8, 240u8, 29u8, 41u8, 170u8, 12u8,
						],
					)
				}
				pub fn cancel(
					&self,
					when: ::core::primitive::u32,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<Cancel> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"cancel",
						Cancel { when, index },
						[
							81u8, 251u8, 234u8, 17u8, 214u8, 75u8, 19u8, 59u8, 19u8, 30u8, 89u8,
							74u8, 6u8, 216u8, 238u8, 165u8, 7u8, 19u8, 153u8, 253u8, 161u8, 103u8,
							178u8, 227u8, 152u8, 180u8, 80u8, 156u8, 82u8, 126u8, 132u8, 120u8,
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
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<ScheduleNamed> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule_named",
						ScheduleNamed {
							id,
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							46u8, 102u8, 120u8, 29u8, 195u8, 224u8, 144u8, 67u8, 228u8, 6u8, 214u8,
							103u8, 77u8, 107u8, 47u8, 1u8, 190u8, 167u8, 94u8, 149u8, 169u8, 247u8,
							89u8, 178u8, 242u8, 44u8, 13u8, 80u8, 17u8, 176u8, 130u8, 177u8,
						],
					)
				}
				pub fn cancel_named(
					&self,
					id: [::core::primitive::u8; 32usize],
				) -> ::subxt::tx::Payload<CancelNamed> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"cancel_named",
						CancelNamed { id },
						[
							51u8, 3u8, 140u8, 50u8, 214u8, 211u8, 50u8, 4u8, 19u8, 43u8, 230u8,
							114u8, 18u8, 108u8, 138u8, 67u8, 99u8, 24u8, 255u8, 11u8, 246u8, 37u8,
							192u8, 207u8, 90u8, 157u8, 171u8, 93u8, 233u8, 189u8, 64u8, 180u8,
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
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<ScheduleAfter> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule_after",
						ScheduleAfter {
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							2u8, 248u8, 219u8, 180u8, 8u8, 20u8, 207u8, 150u8, 231u8, 169u8, 169u8,
							212u8, 233u8, 52u8, 161u8, 129u8, 210u8, 22u8, 124u8, 132u8, 252u8,
							204u8, 22u8, 205u8, 163u8, 215u8, 208u8, 125u8, 128u8, 109u8, 54u8,
							106u8,
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
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<ScheduleNamedAfter> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule_named_after",
						ScheduleNamedAfter {
							id,
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							107u8, 182u8, 196u8, 154u8, 119u8, 118u8, 156u8, 20u8, 219u8, 23u8,
							127u8, 146u8, 98u8, 192u8, 232u8, 23u8, 128u8, 140u8, 230u8, 211u8,
							222u8, 59u8, 65u8, 87u8, 52u8, 240u8, 111u8, 51u8, 209u8, 17u8, 44u8,
							0u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_scheduler::pallet::Event;
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
			pub struct Scheduled {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Scheduled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Scheduled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Canceled {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Canceled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Canceled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Dispatched {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Dispatched {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Dispatched";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CallUnavailable {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for CallUnavailable {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "CallUnavailable";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PeriodicFailed {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for PeriodicFailed {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "PeriodicFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
							149u8, 66u8, 239u8, 67u8, 235u8, 219u8, 101u8, 182u8, 145u8, 56u8,
							252u8, 150u8, 253u8, 221u8, 125u8, 57u8, 38u8, 152u8, 153u8, 31u8,
							92u8, 238u8, 66u8, 246u8, 104u8, 163u8, 94u8, 73u8, 222u8, 168u8,
							193u8, 227u8,
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
									runtime_types::composable_runtime::RuntimeCall,
								>,
								::core::primitive::u32,
								runtime_types::composable_runtime::OriginCaller,
								::subxt::utils::AccountId32,
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
							56u8, 240u8, 156u8, 126u8, 152u8, 97u8, 37u8, 238u8, 109u8, 30u8, 7u8,
							138u8, 71u8, 68u8, 78u8, 66u8, 114u8, 31u8, 182u8, 93u8, 210u8, 155u8,
							83u8, 35u8, 52u8, 107u8, 72u8, 12u8, 138u8, 202u8, 80u8, 214u8,
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
									runtime_types::composable_runtime::RuntimeCall,
								>,
								::core::primitive::u32,
								runtime_types::composable_runtime::OriginCaller,
								::subxt::utils::AccountId32,
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
							56u8, 240u8, 156u8, 126u8, 152u8, 97u8, 37u8, 238u8, 109u8, 30u8, 7u8,
							138u8, 71u8, 68u8, 78u8, 66u8, 114u8, 31u8, 182u8, 93u8, 210u8, 155u8,
							83u8, 35u8, 52u8, 107u8, 72u8, 12u8, 138u8, 202u8, 80u8, 214u8,
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
							82u8, 20u8, 178u8, 101u8, 108u8, 198u8, 71u8, 99u8, 16u8, 175u8, 15u8,
							187u8, 229u8, 243u8, 140u8, 200u8, 99u8, 77u8, 248u8, 178u8, 45u8,
							121u8, 193u8, 67u8, 165u8, 43u8, 234u8, 211u8, 158u8, 250u8, 103u8,
							243u8,
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
							82u8, 20u8, 178u8, 101u8, 108u8, 198u8, 71u8, 99u8, 16u8, 175u8, 15u8,
							187u8, 229u8, 243u8, 140u8, 200u8, 99u8, 77u8, 248u8, 178u8, 45u8,
							121u8, 193u8, 67u8, 165u8, 43u8, 234u8, 211u8, 158u8, 250u8, 103u8,
							243u8,
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
							206u8, 61u8, 253u8, 247u8, 163u8, 40u8, 161u8, 52u8, 134u8, 140u8,
							206u8, 83u8, 44u8, 166u8, 226u8, 115u8, 181u8, 14u8, 227u8, 130u8,
							210u8, 32u8, 85u8, 29u8, 230u8, 97u8, 130u8, 165u8, 147u8, 134u8,
							106u8, 76u8,
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
			pub struct Batch {
				pub calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AsDerivative {
				pub index: ::core::primitive::u16,
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
			pub struct BatchAll {
				pub calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DispatchAs {
				pub as_origin: ::std::boxed::Box<runtime_types::composable_runtime::OriginCaller>,
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
			pub struct ForceBatch {
				pub calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct WithWeight {
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
				) -> ::subxt::tx::Payload<Batch> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"batch",
						Batch { calls },
						[
							248u8, 174u8, 110u8, 53u8, 224u8, 173u8, 159u8, 186u8, 190u8, 102u8,
							205u8, 146u8, 128u8, 72u8, 3u8, 91u8, 97u8, 116u8, 143u8, 121u8, 140u8,
							216u8, 78u8, 29u8, 113u8, 195u8, 129u8, 114u8, 118u8, 176u8, 56u8,
							61u8,
						],
					)
				}
				pub fn as_derivative(
					&self,
					index: ::core::primitive::u16,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<AsDerivative> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"as_derivative",
						AsDerivative { index, call: ::std::boxed::Box::new(call) },
						[
							72u8, 140u8, 6u8, 133u8, 185u8, 192u8, 7u8, 242u8, 82u8, 68u8, 1u8,
							252u8, 5u8, 92u8, 177u8, 143u8, 93u8, 21u8, 203u8, 231u8, 188u8, 248u8,
							132u8, 136u8, 96u8, 246u8, 120u8, 98u8, 28u8, 221u8, 20u8, 21u8,
						],
					)
				}
				pub fn batch_all(
					&self,
					calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
				) -> ::subxt::tx::Payload<BatchAll> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"batch_all",
						BatchAll { calls },
						[
							160u8, 60u8, 74u8, 140u8, 181u8, 130u8, 211u8, 8u8, 126u8, 45u8, 118u8,
							3u8, 7u8, 46u8, 218u8, 216u8, 180u8, 176u8, 139u8, 59u8, 176u8, 140u8,
							255u8, 176u8, 113u8, 93u8, 221u8, 219u8, 237u8, 197u8, 63u8, 32u8,
						],
					)
				}
				pub fn dispatch_as(
					&self,
					as_origin: runtime_types::composable_runtime::OriginCaller,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<DispatchAs> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"dispatch_as",
						DispatchAs {
							as_origin: ::std::boxed::Box::new(as_origin),
							call: ::std::boxed::Box::new(call),
						},
						[
							46u8, 214u8, 179u8, 45u8, 249u8, 210u8, 72u8, 252u8, 119u8, 174u8,
							152u8, 30u8, 29u8, 230u8, 110u8, 30u8, 83u8, 193u8, 92u8, 75u8, 205u8,
							60u8, 134u8, 110u8, 124u8, 58u8, 182u8, 243u8, 88u8, 217u8, 54u8,
							118u8,
						],
					)
				}
				pub fn force_batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
				) -> ::subxt::tx::Payload<ForceBatch> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"force_batch",
						ForceBatch { calls },
						[
							185u8, 70u8, 95u8, 179u8, 74u8, 197u8, 230u8, 226u8, 175u8, 223u8,
							68u8, 128u8, 222u8, 219u8, 30u8, 32u8, 94u8, 217u8, 46u8, 189u8, 248u8,
							21u8, 197u8, 82u8, 149u8, 219u8, 14u8, 45u8, 34u8, 225u8, 219u8, 102u8,
						],
					)
				}
				pub fn with_weight(
					&self,
					call: runtime_types::composable_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<WithWeight> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"with_weight",
						WithWeight { call: ::std::boxed::Box::new(call), weight },
						[
							13u8, 156u8, 62u8, 4u8, 253u8, 172u8, 81u8, 141u8, 96u8, 62u8, 112u8,
							213u8, 178u8, 78u8, 162u8, 109u8, 94u8, 51u8, 118u8, 215u8, 187u8, 0u8,
							166u8, 16u8, 88u8, 202u8, 148u8, 42u8, 211u8, 199u8, 245u8, 80u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_utility::pallet::Event;
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
			pub struct BatchInterrupted {
				pub index: ::core::primitive::u32,
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for BatchInterrupted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchInterrupted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BatchCompleted;
			impl ::subxt::events::StaticEvent for BatchCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BatchCompletedWithErrors;
			impl ::subxt::events::StaticEvent for BatchCompletedWithErrors {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompletedWithErrors";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ItemCompleted;
			impl ::subxt::events::StaticEvent for ItemCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ItemFailed {
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for ItemFailed {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
			pub struct NotePreimage {
				pub bytes: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnnotePreimage {
				pub hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RequestPreimage {
				pub hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnrequestPreimage {
				pub hash: ::subxt::utils::H256,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn note_preimage(
					&self,
					bytes: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<NotePreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"note_preimage",
						NotePreimage { bytes },
						[
							77u8, 48u8, 104u8, 3u8, 254u8, 65u8, 106u8, 95u8, 204u8, 89u8, 149u8,
							29u8, 144u8, 188u8, 99u8, 23u8, 146u8, 142u8, 35u8, 17u8, 125u8, 130u8,
							31u8, 206u8, 106u8, 83u8, 163u8, 192u8, 81u8, 23u8, 232u8, 230u8,
						],
					)
				}
				pub fn unnote_preimage(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<UnnotePreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"unnote_preimage",
						UnnotePreimage { hash },
						[
							211u8, 204u8, 205u8, 58u8, 33u8, 179u8, 68u8, 74u8, 149u8, 138u8,
							213u8, 45u8, 140u8, 27u8, 106u8, 81u8, 68u8, 212u8, 147u8, 116u8, 27u8,
							130u8, 84u8, 34u8, 231u8, 197u8, 135u8, 8u8, 19u8, 242u8, 207u8, 17u8,
						],
					)
				}
				pub fn request_preimage(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<RequestPreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"request_preimage",
						RequestPreimage { hash },
						[
							195u8, 26u8, 146u8, 255u8, 79u8, 43u8, 73u8, 60u8, 115u8, 78u8, 99u8,
							197u8, 137u8, 95u8, 139u8, 141u8, 79u8, 213u8, 170u8, 169u8, 127u8,
							30u8, 236u8, 65u8, 38u8, 16u8, 118u8, 228u8, 141u8, 83u8, 162u8, 233u8,
						],
					)
				}
				pub fn unrequest_preimage(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<UnrequestPreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"unrequest_preimage",
						UnrequestPreimage { hash },
						[
							143u8, 225u8, 239u8, 44u8, 237u8, 83u8, 18u8, 105u8, 101u8, 68u8,
							111u8, 116u8, 66u8, 212u8, 63u8, 190u8, 38u8, 32u8, 105u8, 152u8, 69u8,
							177u8, 193u8, 15u8, 60u8, 26u8, 95u8, 130u8, 11u8, 113u8, 187u8, 108u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_preimage::pallet::Event;
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
			pub struct Noted {
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Noted {
				const PALLET: &'static str = "Preimage";
				const EVENT: &'static str = "Noted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Requested {
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Requested {
				const PALLET: &'static str = "Preimage";
				const EVENT: &'static str = "Requested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Cleared {
				pub hash: ::subxt::utils::H256,
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
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_preimage::RequestStatus<
						::subxt::utils::AccountId32,
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
							103u8, 208u8, 88u8, 167u8, 244u8, 198u8, 129u8, 134u8, 182u8, 80u8,
							71u8, 192u8, 73u8, 92u8, 190u8, 15u8, 20u8, 132u8, 37u8, 108u8, 88u8,
							233u8, 18u8, 145u8, 9u8, 235u8, 5u8, 132u8, 42u8, 17u8, 227u8, 56u8,
						],
					)
				}
				pub fn status_for_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_preimage::RequestStatus<
						::subxt::utils::AccountId32,
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
							103u8, 208u8, 88u8, 167u8, 244u8, 198u8, 129u8, 134u8, 182u8, 80u8,
							71u8, 192u8, 73u8, 92u8, 190u8, 15u8, 20u8, 132u8, 37u8, 108u8, 88u8,
							233u8, 18u8, 145u8, 9u8, 235u8, 5u8, 132u8, 42u8, 17u8, 227u8, 56u8,
						],
					)
				}
				pub fn preimage_for(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
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
							96u8, 74u8, 30u8, 112u8, 120u8, 41u8, 52u8, 187u8, 252u8, 68u8, 42u8,
							5u8, 61u8, 228u8, 250u8, 192u8, 224u8, 61u8, 53u8, 222u8, 95u8, 148u8,
							6u8, 53u8, 43u8, 152u8, 88u8, 58u8, 185u8, 234u8, 131u8, 124u8,
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
							96u8, 74u8, 30u8, 112u8, 120u8, 41u8, 52u8, 187u8, 252u8, 68u8, 42u8,
							5u8, 61u8, 228u8, 250u8, 192u8, 224u8, 61u8, 53u8, 222u8, 95u8, 148u8,
							6u8, 53u8, 43u8, 152u8, 88u8, 58u8, 185u8, 234u8, 131u8, 124u8,
						],
					)
				}
			}
		}
	}
	pub mod proxy {
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
			pub struct Proxy {
				pub real: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub force_proxy_type: ::core::option::Option<
					runtime_types::composable_traits::account_proxy::ProxyType,
				>,
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
			pub struct AddProxy {
				pub delegate: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RemoveProxy {
				pub delegate: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RemoveProxies;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CreatePure {
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub delay: ::core::primitive::u32,
				pub index: ::core::primitive::u16,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct KillPure {
				pub spawner: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub index: ::core::primitive::u16,
				#[codec(compact)]
				pub height: ::core::primitive::u32,
				#[codec(compact)]
				pub ext_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Announce {
				pub real: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub call_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RemoveAnnouncement {
				pub real: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub call_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RejectAnnouncement {
				pub delegate: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub call_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ProxyAnnounced {
				pub delegate: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub real: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub force_proxy_type: ::core::option::Option<
					runtime_types::composable_traits::account_proxy::ProxyType,
				>,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn proxy(
					&self,
					real: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					force_proxy_type: ::core::option::Option<
						runtime_types::composable_traits::account_proxy::ProxyType,
					>,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<Proxy> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"proxy",
						Proxy { real, force_proxy_type, call: ::std::boxed::Box::new(call) },
						[
							115u8, 38u8, 81u8, 155u8, 184u8, 192u8, 225u8, 239u8, 100u8, 96u8,
							218u8, 139u8, 248u8, 190u8, 103u8, 202u8, 146u8, 130u8, 76u8, 235u8,
							69u8, 146u8, 213u8, 76u8, 106u8, 88u8, 212u8, 41u8, 87u8, 252u8, 208u8,
							189u8,
						],
					)
				}
				pub fn add_proxy(
					&self,
					delegate: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<AddProxy> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"add_proxy",
						AddProxy { delegate, proxy_type, delay },
						[
							84u8, 231u8, 178u8, 40u8, 111u8, 129u8, 6u8, 186u8, 103u8, 1u8, 100u8,
							183u8, 27u8, 62u8, 55u8, 233u8, 37u8, 110u8, 151u8, 3u8, 218u8, 230u8,
							65u8, 56u8, 68u8, 21u8, 58u8, 240u8, 183u8, 116u8, 218u8, 226u8,
						],
					)
				}
				pub fn remove_proxy(
					&self,
					delegate: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<RemoveProxy> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"remove_proxy",
						RemoveProxy { delegate, proxy_type, delay },
						[
							174u8, 24u8, 162u8, 43u8, 182u8, 210u8, 225u8, 238u8, 244u8, 157u8,
							39u8, 150u8, 29u8, 53u8, 191u8, 91u8, 171u8, 231u8, 45u8, 118u8, 172u8,
							151u8, 162u8, 31u8, 95u8, 145u8, 72u8, 167u8, 128u8, 195u8, 151u8,
							83u8,
						],
					)
				}
				pub fn remove_proxies(&self) -> ::subxt::tx::Payload<RemoveProxies> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"remove_proxies",
						RemoveProxies {},
						[
							15u8, 237u8, 27u8, 166u8, 254u8, 218u8, 92u8, 5u8, 213u8, 239u8, 99u8,
							59u8, 1u8, 26u8, 73u8, 252u8, 81u8, 94u8, 214u8, 227u8, 169u8, 58u8,
							40u8, 253u8, 187u8, 225u8, 192u8, 26u8, 19u8, 23u8, 121u8, 129u8,
						],
					)
				}
				pub fn create_pure(
					&self,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					delay: ::core::primitive::u32,
					index: ::core::primitive::u16,
				) -> ::subxt::tx::Payload<CreatePure> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"create_pure",
						CreatePure { proxy_type, delay, index },
						[
							176u8, 173u8, 191u8, 144u8, 58u8, 237u8, 247u8, 46u8, 166u8, 28u8,
							169u8, 154u8, 90u8, 117u8, 158u8, 124u8, 143u8, 139u8, 156u8, 68u8,
							144u8, 117u8, 153u8, 233u8, 254u8, 138u8, 66u8, 66u8, 79u8, 74u8, 64u8,
							247u8,
						],
					)
				}
				pub fn kill_pure(
					&self,
					spawner: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					index: ::core::primitive::u16,
					height: ::core::primitive::u32,
					ext_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<KillPure> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"kill_pure",
						KillPure { spawner, proxy_type, index, height, ext_index },
						[
							150u8, 48u8, 9u8, 78u8, 163u8, 204u8, 124u8, 1u8, 89u8, 156u8, 226u8,
							61u8, 110u8, 20u8, 133u8, 234u8, 212u8, 40u8, 191u8, 76u8, 16u8, 114u8,
							245u8, 169u8, 188u8, 181u8, 130u8, 42u8, 128u8, 73u8, 179u8, 222u8,
						],
					)
				}
				pub fn announce(
					&self,
					real: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					call_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<Announce> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"announce",
						Announce { real, call_hash },
						[
							226u8, 252u8, 69u8, 50u8, 248u8, 212u8, 209u8, 225u8, 201u8, 236u8,
							51u8, 136u8, 56u8, 85u8, 36u8, 130u8, 233u8, 84u8, 44u8, 192u8, 174u8,
							119u8, 245u8, 62u8, 150u8, 78u8, 217u8, 90u8, 167u8, 154u8, 228u8,
							141u8,
						],
					)
				}
				pub fn remove_announcement(
					&self,
					real: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					call_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<RemoveAnnouncement> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"remove_announcement",
						RemoveAnnouncement { real, call_hash },
						[
							251u8, 236u8, 113u8, 182u8, 125u8, 244u8, 31u8, 144u8, 66u8, 28u8,
							65u8, 97u8, 67u8, 94u8, 225u8, 210u8, 46u8, 143u8, 242u8, 124u8, 120u8,
							93u8, 23u8, 165u8, 83u8, 177u8, 250u8, 171u8, 58u8, 66u8, 70u8, 64u8,
						],
					)
				}
				pub fn reject_announcement(
					&self,
					delegate: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					call_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<RejectAnnouncement> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"reject_announcement",
						RejectAnnouncement { delegate, call_hash },
						[
							122u8, 165u8, 114u8, 85u8, 209u8, 197u8, 11u8, 96u8, 211u8, 93u8,
							201u8, 42u8, 1u8, 131u8, 254u8, 177u8, 191u8, 212u8, 229u8, 13u8, 28u8,
							163u8, 133u8, 200u8, 113u8, 28u8, 132u8, 45u8, 105u8, 177u8, 82u8,
							206u8,
						],
					)
				}
				pub fn proxy_announced(
					&self,
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
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<ProxyAnnounced> {
					::subxt::tx::Payload::new_static(
						"Proxy",
						"proxy_announced",
						ProxyAnnounced {
							delegate,
							real,
							force_proxy_type,
							call: ::std::boxed::Box::new(call),
						},
						[
							140u8, 115u8, 52u8, 194u8, 69u8, 167u8, 86u8, 69u8, 100u8, 17u8, 25u8,
							223u8, 233u8, 16u8, 170u8, 97u8, 137u8, 144u8, 209u8, 76u8, 17u8,
							233u8, 179u8, 171u8, 245u8, 78u8, 35u8, 18u8, 69u8, 251u8, 69u8, 146u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_proxy::pallet::Event;
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
			pub struct ProxyExecuted {
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for ProxyExecuted {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "ProxyExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PureCreated {
				pub pure: ::subxt::utils::AccountId32,
				pub who: ::subxt::utils::AccountId32,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub disambiguation_index: ::core::primitive::u16,
			}
			impl ::subxt::events::StaticEvent for PureCreated {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "PureCreated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Announced {
				pub real: ::subxt::utils::AccountId32,
				pub proxy: ::subxt::utils::AccountId32,
				pub call_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Announced {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "Announced";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ProxyAdded {
				pub delegator: ::subxt::utils::AccountId32,
				pub delegatee: ::subxt::utils::AccountId32,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ProxyAdded {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "ProxyAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ProxyRemoved {
				pub delegator: ::subxt::utils::AccountId32,
				pub delegatee: ::subxt::utils::AccountId32,
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
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::ProxyDefinition<
								::subxt::utils::AccountId32,
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
							4u8, 179u8, 233u8, 7u8, 107u8, 36u8, 245u8, 75u8, 67u8, 135u8, 44u8,
							205u8, 95u8, 235u8, 58u8, 25u8, 179u8, 49u8, 12u8, 204u8, 133u8, 2u8,
							211u8, 42u8, 182u8, 92u8, 220u8, 247u8, 53u8, 168u8, 243u8, 236u8,
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
								::subxt::utils::AccountId32,
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
							4u8, 179u8, 233u8, 7u8, 107u8, 36u8, 245u8, 75u8, 67u8, 135u8, 44u8,
							205u8, 95u8, 235u8, 58u8, 25u8, 179u8, 49u8, 12u8, 204u8, 133u8, 2u8,
							211u8, 42u8, 182u8, 92u8, 220u8, 247u8, 53u8, 168u8, 243u8, 236u8,
						],
					)
				}
				pub fn announcements(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::Announcement<
								::subxt::utils::AccountId32,
								::subxt::utils::H256,
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
							233u8, 38u8, 249u8, 89u8, 103u8, 87u8, 64u8, 52u8, 140u8, 228u8, 110u8,
							37u8, 8u8, 92u8, 48u8, 7u8, 46u8, 99u8, 179u8, 83u8, 232u8, 171u8,
							160u8, 45u8, 37u8, 23u8, 151u8, 198u8, 237u8, 103u8, 217u8, 53u8,
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
								::subxt::utils::AccountId32,
								::subxt::utils::H256,
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
							233u8, 38u8, 249u8, 89u8, 103u8, 87u8, 64u8, 52u8, 140u8, 228u8, 110u8,
							37u8, 8u8, 92u8, 48u8, 7u8, 46u8, 99u8, 179u8, 83u8, 232u8, 171u8,
							160u8, 45u8, 37u8, 23u8, 151u8, 198u8, 237u8, 103u8, 217u8, 53u8,
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
			pub struct ServiceOverweight {
				pub index: ::core::primitive::u64,
				pub weight_limit: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SuspendXcmExecution;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ResumeXcmExecution;
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
			pub struct UpdateSuspendThreshold {
				pub new: ::core::primitive::u32,
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
			pub struct UpdateDropThreshold {
				pub new: ::core::primitive::u32,
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
			pub struct UpdateResumeThreshold {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpdateThresholdWeight {
				pub new: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpdateWeightRestrictDecay {
				pub new: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpdateXcmpMaxIndividualWeight {
				pub new: runtime_types::sp_weights::weight_v2::Weight,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<ServiceOverweight> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"service_overweight",
						ServiceOverweight { index, weight_limit },
						[
							121u8, 236u8, 235u8, 23u8, 210u8, 238u8, 238u8, 122u8, 15u8, 86u8,
							34u8, 119u8, 105u8, 100u8, 214u8, 236u8, 117u8, 39u8, 254u8, 235u8,
							189u8, 15u8, 72u8, 74u8, 225u8, 134u8, 148u8, 126u8, 31u8, 203u8,
							144u8, 106u8,
						],
					)
				}
				pub fn suspend_xcm_execution(&self) -> ::subxt::tx::Payload<SuspendXcmExecution> {
					::subxt::tx::Payload::new_static(
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
				pub fn resume_xcm_execution(&self) -> ::subxt::tx::Payload<ResumeXcmExecution> {
					::subxt::tx::Payload::new_static(
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
				pub fn update_suspend_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<UpdateSuspendThreshold> {
					::subxt::tx::Payload::new_static(
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
				pub fn update_drop_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<UpdateDropThreshold> {
					::subxt::tx::Payload::new_static(
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
				pub fn update_resume_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<UpdateResumeThreshold> {
					::subxt::tx::Payload::new_static(
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
				pub fn update_threshold_weight(
					&self,
					new: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<UpdateThresholdWeight> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_threshold_weight",
						UpdateThresholdWeight { new },
						[
							14u8, 144u8, 112u8, 207u8, 195u8, 208u8, 184u8, 164u8, 94u8, 41u8, 8u8,
							58u8, 180u8, 80u8, 239u8, 39u8, 210u8, 159u8, 114u8, 169u8, 152u8,
							176u8, 26u8, 161u8, 32u8, 43u8, 250u8, 156u8, 56u8, 21u8, 43u8, 159u8,
						],
					)
				}
				pub fn update_weight_restrict_decay(
					&self,
					new: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<UpdateWeightRestrictDecay> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_weight_restrict_decay",
						UpdateWeightRestrictDecay { new },
						[
							42u8, 53u8, 83u8, 191u8, 51u8, 227u8, 210u8, 193u8, 142u8, 218u8,
							244u8, 177u8, 19u8, 87u8, 148u8, 177u8, 231u8, 197u8, 196u8, 255u8,
							41u8, 130u8, 245u8, 139u8, 107u8, 212u8, 90u8, 161u8, 82u8, 248u8,
							160u8, 223u8,
						],
					)
				}
				pub fn update_xcmp_max_individual_weight(
					&self,
					new: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<UpdateXcmpMaxIndividualWeight> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_xcmp_max_individual_weight",
						UpdateXcmpMaxIndividualWeight { new },
						[
							148u8, 185u8, 89u8, 36u8, 152u8, 220u8, 248u8, 233u8, 236u8, 82u8,
							170u8, 111u8, 225u8, 142u8, 25u8, 211u8, 72u8, 248u8, 250u8, 14u8,
							45u8, 72u8, 78u8, 95u8, 92u8, 196u8, 245u8, 104u8, 112u8, 128u8, 27u8,
							109u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
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
			pub struct Success {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for Success {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Success";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Fail {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
				pub error: runtime_types::xcm::v3::traits::Error,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for Fail {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Fail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BadVersion {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for BadVersion {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BadFormat {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for BadFormat {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct XcmpMessageSent {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for XcmpMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "XcmpMessageSent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
							183u8, 198u8, 237u8, 153u8, 132u8, 201u8, 87u8, 182u8, 121u8, 164u8,
							129u8, 241u8, 58u8, 192u8, 115u8, 152u8, 7u8, 33u8, 95u8, 51u8, 2u8,
							176u8, 144u8, 12u8, 125u8, 83u8, 92u8, 198u8, 211u8, 101u8, 28u8, 50u8,
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
							157u8, 232u8, 222u8, 97u8, 218u8, 96u8, 96u8, 90u8, 216u8, 205u8, 39u8,
							130u8, 109u8, 152u8, 127u8, 57u8, 54u8, 63u8, 104u8, 135u8, 33u8,
							175u8, 197u8, 166u8, 238u8, 22u8, 137u8, 162u8, 226u8, 199u8, 87u8,
							25u8,
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
							157u8, 232u8, 222u8, 97u8, 218u8, 96u8, 96u8, 90u8, 216u8, 205u8, 39u8,
							130u8, 109u8, 152u8, 127u8, 57u8, 54u8, 63u8, 104u8, 135u8, 33u8,
							175u8, 197u8, 166u8, 238u8, 22u8, 137u8, 162u8, 226u8, 199u8, 87u8,
							25u8,
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
							238u8, 120u8, 185u8, 141u8, 82u8, 159u8, 41u8, 68u8, 204u8, 15u8, 46u8,
							152u8, 144u8, 74u8, 250u8, 83u8, 71u8, 105u8, 54u8, 53u8, 226u8, 87u8,
							14u8, 202u8, 58u8, 160u8, 54u8, 162u8, 239u8, 248u8, 227u8, 116u8,
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
							50u8, 182u8, 237u8, 191u8, 106u8, 67u8, 54u8, 1u8, 17u8, 107u8, 70u8,
							90u8, 202u8, 8u8, 63u8, 184u8, 171u8, 111u8, 192u8, 196u8, 7u8, 31u8,
							186u8, 68u8, 31u8, 63u8, 71u8, 61u8, 83u8, 223u8, 79u8, 200u8,
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
							50u8, 182u8, 237u8, 191u8, 106u8, 67u8, 54u8, 1u8, 17u8, 107u8, 70u8,
							90u8, 202u8, 8u8, 63u8, 184u8, 171u8, 111u8, 192u8, 196u8, 7u8, 31u8,
							186u8, 68u8, 31u8, 63u8, 71u8, 61u8, 83u8, 223u8, 79u8, 200u8,
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
							156u8, 242u8, 186u8, 89u8, 177u8, 195u8, 90u8, 121u8, 94u8, 106u8,
							222u8, 78u8, 19u8, 162u8, 179u8, 96u8, 38u8, 113u8, 209u8, 148u8, 29u8,
							110u8, 106u8, 167u8, 162u8, 96u8, 221u8, 20u8, 33u8, 179u8, 168u8,
							142u8,
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
							156u8, 242u8, 186u8, 89u8, 177u8, 195u8, 90u8, 121u8, 94u8, 106u8,
							222u8, 78u8, 19u8, 162u8, 179u8, 96u8, 38u8, 113u8, 209u8, 148u8, 29u8,
							110u8, 106u8, 167u8, 162u8, 96u8, 221u8, 20u8, 33u8, 179u8, 168u8,
							142u8,
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
							154u8, 172u8, 227u8, 208u8, 130u8, 93u8, 173u8, 129u8, 33u8, 75u8,
							180u8, 100u8, 35u8, 154u8, 40u8, 188u8, 86u8, 53u8, 74u8, 118u8, 131u8,
							159u8, 240u8, 159u8, 185u8, 45u8, 165u8, 6u8, 90u8, 125u8, 77u8, 253u8,
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
							222u8, 249u8, 232u8, 110u8, 117u8, 229u8, 165u8, 164u8, 219u8, 219u8,
							149u8, 204u8, 25u8, 78u8, 204u8, 116u8, 111u8, 114u8, 120u8, 222u8,
							56u8, 77u8, 122u8, 147u8, 108u8, 15u8, 94u8, 161u8, 212u8, 50u8, 7u8,
							7u8,
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
							222u8, 249u8, 232u8, 110u8, 117u8, 229u8, 165u8, 164u8, 219u8, 219u8,
							149u8, 204u8, 25u8, 78u8, 204u8, 116u8, 111u8, 114u8, 120u8, 222u8,
							56u8, 77u8, 122u8, 147u8, 108u8, 15u8, 94u8, 161u8, 212u8, 50u8, 7u8,
							7u8,
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
							148u8, 226u8, 248u8, 107u8, 165u8, 97u8, 218u8, 160u8, 127u8, 48u8,
							185u8, 251u8, 35u8, 137u8, 119u8, 251u8, 151u8, 167u8, 189u8, 66u8,
							80u8, 74u8, 134u8, 129u8, 222u8, 180u8, 51u8, 182u8, 50u8, 110u8, 10u8,
							43u8,
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
							102u8, 180u8, 196u8, 148u8, 115u8, 62u8, 46u8, 238u8, 97u8, 116u8,
							117u8, 42u8, 14u8, 5u8, 72u8, 237u8, 230u8, 46u8, 150u8, 126u8, 89u8,
							64u8, 233u8, 166u8, 180u8, 137u8, 52u8, 233u8, 252u8, 255u8, 36u8,
							20u8,
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
			pub struct Send {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TeleportAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ReserveTransferAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Execute {
				pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
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
			pub struct ForceXcmVersion {
				pub location:
					::std::boxed::Box<runtime_types::xcm::v3::multilocation::MultiLocation>,
				pub xcm_version: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceDefaultXcmVersion {
				pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceSubscribeVersionNotify {
				pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceUnsubscribeVersionNotify {
				pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct LimitedReserveTransferAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v3::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct LimitedTeleportAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v3::WeightLimit,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn send(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					message: runtime_types::xcm::VersionedXcm,
				) -> ::subxt::tx::Payload<Send> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"send",
						Send {
							dest: ::std::boxed::Box::new(dest),
							message: ::std::boxed::Box::new(message),
						},
						[
							246u8, 35u8, 227u8, 112u8, 223u8, 7u8, 44u8, 186u8, 60u8, 225u8, 153u8,
							249u8, 104u8, 51u8, 123u8, 227u8, 143u8, 65u8, 232u8, 209u8, 178u8,
							104u8, 70u8, 56u8, 230u8, 14u8, 75u8, 83u8, 250u8, 160u8, 9u8, 39u8,
						],
					)
				}
				pub fn teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<TeleportAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"teleport_assets",
						TeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							187u8, 42u8, 2u8, 96u8, 105u8, 125u8, 74u8, 53u8, 2u8, 21u8, 31u8,
							160u8, 201u8, 197u8, 157u8, 190u8, 40u8, 145u8, 5u8, 99u8, 194u8, 41u8,
							114u8, 60u8, 165u8, 186u8, 15u8, 226u8, 85u8, 113u8, 159u8, 136u8,
						],
					)
				}
				pub fn reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<ReserveTransferAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"reserve_transfer_assets",
						ReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							249u8, 177u8, 76u8, 204u8, 186u8, 165u8, 16u8, 186u8, 129u8, 239u8,
							65u8, 252u8, 9u8, 132u8, 32u8, 164u8, 117u8, 177u8, 40u8, 21u8, 196u8,
							246u8, 147u8, 2u8, 95u8, 110u8, 68u8, 162u8, 148u8, 9u8, 59u8, 170u8,
						],
					)
				}
				pub fn execute(
					&self,
					message: runtime_types::xcm::VersionedXcm,
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<Execute> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"execute",
						Execute { message: ::std::boxed::Box::new(message), max_weight },
						[
							102u8, 41u8, 146u8, 29u8, 241u8, 205u8, 95u8, 153u8, 228u8, 141u8,
							11u8, 228u8, 13u8, 44u8, 75u8, 204u8, 174u8, 35u8, 155u8, 104u8, 204u8,
							82u8, 239u8, 98u8, 249u8, 187u8, 193u8, 1u8, 122u8, 88u8, 162u8, 200u8,
						],
					)
				}
				pub fn force_xcm_version(
					&self,
					location: runtime_types::xcm::v3::multilocation::MultiLocation,
					xcm_version: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<ForceXcmVersion> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_xcm_version",
						ForceXcmVersion { location: ::std::boxed::Box::new(location), xcm_version },
						[
							68u8, 48u8, 95u8, 61u8, 152u8, 95u8, 213u8, 126u8, 209u8, 176u8, 230u8,
							160u8, 164u8, 42u8, 128u8, 62u8, 175u8, 3u8, 161u8, 170u8, 20u8, 31u8,
							216u8, 122u8, 31u8, 77u8, 64u8, 182u8, 121u8, 41u8, 23u8, 80u8,
						],
					)
				}
				pub fn force_default_xcm_version(
					&self,
					maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::Payload<ForceDefaultXcmVersion> {
					::subxt::tx::Payload::new_static(
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
				pub fn force_subscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::tx::Payload<ForceSubscribeVersionNotify> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_subscribe_version_notify",
						ForceSubscribeVersionNotify { location: ::std::boxed::Box::new(location) },
						[
							236u8, 37u8, 153u8, 26u8, 174u8, 187u8, 154u8, 38u8, 179u8, 223u8,
							130u8, 32u8, 128u8, 30u8, 148u8, 229u8, 7u8, 185u8, 174u8, 9u8, 96u8,
							215u8, 189u8, 178u8, 148u8, 141u8, 249u8, 118u8, 7u8, 238u8, 1u8, 49u8,
						],
					)
				}
				pub fn force_unsubscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::tx::Payload<ForceUnsubscribeVersionNotify> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_unsubscribe_version_notify",
						ForceUnsubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						},
						[
							154u8, 169u8, 145u8, 211u8, 185u8, 71u8, 9u8, 63u8, 3u8, 158u8, 187u8,
							173u8, 115u8, 166u8, 100u8, 66u8, 12u8, 40u8, 198u8, 40u8, 213u8,
							104u8, 95u8, 183u8, 215u8, 53u8, 94u8, 158u8, 106u8, 56u8, 149u8, 52u8,
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
				) -> ::subxt::tx::Payload<LimitedReserveTransferAssets> {
					::subxt::tx::Payload::new_static(
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
							131u8, 191u8, 89u8, 27u8, 236u8, 142u8, 130u8, 129u8, 245u8, 95u8,
							159u8, 96u8, 252u8, 80u8, 28u8, 40u8, 128u8, 55u8, 41u8, 123u8, 22u8,
							18u8, 0u8, 236u8, 77u8, 68u8, 135u8, 181u8, 40u8, 47u8, 92u8, 240u8,
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
				) -> ::subxt::tx::Payload<LimitedTeleportAssets> {
					::subxt::tx::Payload::new_static(
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
							234u8, 19u8, 104u8, 174u8, 98u8, 159u8, 205u8, 110u8, 240u8, 78u8,
							186u8, 138u8, 236u8, 116u8, 104u8, 215u8, 57u8, 178u8, 166u8, 208u8,
							197u8, 113u8, 101u8, 56u8, 23u8, 56u8, 84u8, 14u8, 173u8, 70u8, 211u8,
							201u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_xcm::pallet::Event;
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
			pub struct Attempted(pub runtime_types::xcm::v3::traits::Outcome);
			impl ::subxt::events::StaticEvent for Attempted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Attempted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Sent(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::Xcm,
			);
			impl ::subxt::events::StaticEvent for Sent {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Sent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnexpectedResponse(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for UnexpectedResponse {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "UnexpectedResponse";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ResponseReady(
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v3::Response,
			);
			impl ::subxt::events::StaticEvent for ResponseReady {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseReady";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InvalidResponder(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub ::core::option::Option<runtime_types::xcm::v3::multilocation::MultiLocation>,
			);
			impl ::subxt::events::StaticEvent for InvalidResponder {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponder";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InvalidResponderVersion(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
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
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ResponseTaken(pub ::core::primitive::u64);
			impl ::subxt::events::StaticEvent for ResponseTaken {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseTaken";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetsTrapped(
				pub ::subxt::utils::H256,
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::VersionedMultiAssets,
			);
			impl ::subxt::events::StaticEvent for AssetsTrapped {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AssetsTrapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct VersionChangeNotified(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u32,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionChangeNotified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionChangeNotified";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SupportedVersionChanged(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for SupportedVersionChanged {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "SupportedVersionChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NotifyTargetSendFail(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v3::traits::Error,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetSendFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetSendFail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NotifyTargetMigrationFail(
				pub runtime_types::xcm::VersionedMultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetMigrationFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetMigrationFail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InvalidQuerierVersion(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for InvalidQuerierVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidQuerierVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct VersionNotifyStarted(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionNotifyStarted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyStarted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct VersionNotifyRequested(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionNotifyRequested {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyRequested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct VersionNotifyUnrequested(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for VersionNotifyUnrequested {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyUnrequested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct FeesPaid(
				pub runtime_types::xcm::v3::multilocation::MultiLocation,
				pub runtime_types::xcm::v3::multiasset::MultiAssets,
			);
			impl ::subxt::events::StaticEvent for FeesPaid {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "FeesPaid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AssetsClaimed(
				pub ::subxt::utils::H256,
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
							137u8, 58u8, 184u8, 88u8, 247u8, 22u8, 151u8, 64u8, 50u8, 77u8, 49u8,
							10u8, 234u8, 84u8, 213u8, 156u8, 26u8, 200u8, 214u8, 225u8, 125u8,
							231u8, 42u8, 93u8, 159u8, 168u8, 86u8, 201u8, 116u8, 153u8, 41u8,
							127u8,
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
							72u8, 239u8, 157u8, 117u8, 200u8, 28u8, 80u8, 70u8, 205u8, 253u8,
							147u8, 30u8, 130u8, 72u8, 154u8, 95u8, 183u8, 162u8, 165u8, 203u8,
							128u8, 98u8, 216u8, 172u8, 98u8, 220u8, 16u8, 236u8, 216u8, 68u8, 33u8,
							184u8,
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
							72u8, 239u8, 157u8, 117u8, 200u8, 28u8, 80u8, 70u8, 205u8, 253u8,
							147u8, 30u8, 130u8, 72u8, 154u8, 95u8, 183u8, 162u8, 165u8, 203u8,
							128u8, 98u8, 216u8, 172u8, 98u8, 220u8, 16u8, 236u8, 216u8, 68u8, 33u8,
							184u8,
						],
					)
				}
				pub fn asset_traps(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
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
							4u8, 185u8, 92u8, 4u8, 7u8, 71u8, 214u8, 1u8, 141u8, 59u8, 87u8, 55u8,
							149u8, 26u8, 125u8, 8u8, 88u8, 31u8, 240u8, 138u8, 133u8, 28u8, 37u8,
							131u8, 107u8, 218u8, 86u8, 152u8, 147u8, 44u8, 19u8, 239u8,
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
							4u8, 185u8, 92u8, 4u8, 7u8, 71u8, 214u8, 1u8, 141u8, 59u8, 87u8, 55u8,
							149u8, 26u8, 125u8, 8u8, 88u8, 31u8, 240u8, 138u8, 133u8, 28u8, 37u8,
							131u8, 107u8, 218u8, 86u8, 152u8, 147u8, 44u8, 19u8, 239u8,
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
							1u8, 223u8, 218u8, 204u8, 222u8, 129u8, 137u8, 237u8, 197u8, 142u8,
							233u8, 66u8, 229u8, 153u8, 138u8, 222u8, 113u8, 164u8, 135u8, 213u8,
							233u8, 34u8, 24u8, 23u8, 215u8, 59u8, 40u8, 188u8, 45u8, 244u8, 205u8,
							199u8,
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
							16u8, 172u8, 183u8, 14u8, 63u8, 199u8, 42u8, 204u8, 218u8, 197u8,
							129u8, 40u8, 32u8, 213u8, 50u8, 170u8, 231u8, 123u8, 188u8, 83u8,
							250u8, 148u8, 133u8, 78u8, 249u8, 33u8, 122u8, 55u8, 22u8, 179u8, 98u8,
							113u8,
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
							16u8, 172u8, 183u8, 14u8, 63u8, 199u8, 42u8, 204u8, 218u8, 197u8,
							129u8, 40u8, 32u8, 213u8, 50u8, 170u8, 231u8, 123u8, 188u8, 83u8,
							250u8, 148u8, 133u8, 78u8, 249u8, 33u8, 122u8, 55u8, 22u8, 179u8, 98u8,
							113u8,
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
							137u8, 87u8, 59u8, 219u8, 207u8, 188u8, 145u8, 38u8, 197u8, 219u8,
							197u8, 179u8, 102u8, 25u8, 184u8, 83u8, 31u8, 63u8, 251u8, 21u8, 211u8,
							124u8, 23u8, 40u8, 4u8, 43u8, 113u8, 158u8, 233u8, 192u8, 38u8, 177u8,
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
							137u8, 87u8, 59u8, 219u8, 207u8, 188u8, 145u8, 38u8, 197u8, 219u8,
							197u8, 179u8, 102u8, 25u8, 184u8, 83u8, 31u8, 63u8, 251u8, 21u8, 211u8,
							124u8, 23u8, 40u8, 4u8, 43u8, 113u8, 158u8, 233u8, 192u8, 38u8, 177u8,
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
							138u8, 26u8, 26u8, 108u8, 21u8, 255u8, 143u8, 241u8, 15u8, 163u8, 22u8,
							155u8, 221u8, 63u8, 58u8, 104u8, 4u8, 186u8, 66u8, 178u8, 67u8, 178u8,
							220u8, 78u8, 1u8, 77u8, 45u8, 214u8, 98u8, 240u8, 120u8, 254u8,
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
							138u8, 26u8, 26u8, 108u8, 21u8, 255u8, 143u8, 241u8, 15u8, 163u8, 22u8,
							155u8, 221u8, 63u8, 58u8, 104u8, 4u8, 186u8, 66u8, 178u8, 67u8, 178u8,
							220u8, 78u8, 1u8, 77u8, 45u8, 214u8, 98u8, 240u8, 120u8, 254u8,
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
							134u8, 60u8, 255u8, 145u8, 139u8, 29u8, 38u8, 47u8, 209u8, 218u8,
							127u8, 123u8, 2u8, 196u8, 52u8, 99u8, 143u8, 112u8, 0u8, 133u8, 99u8,
							218u8, 187u8, 171u8, 175u8, 153u8, 149u8, 1u8, 57u8, 45u8, 118u8, 79u8,
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
							137u8, 144u8, 168u8, 185u8, 158u8, 90u8, 127u8, 243u8, 227u8, 134u8,
							150u8, 73u8, 15u8, 99u8, 23u8, 47u8, 68u8, 18u8, 39u8, 16u8, 24u8,
							43u8, 161u8, 56u8, 66u8, 111u8, 16u8, 7u8, 252u8, 125u8, 100u8, 225u8,
						],
					)
				}
				pub fn remote_locked_fungibles(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
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
							26u8, 71u8, 1u8, 2u8, 214u8, 3u8, 65u8, 62u8, 133u8, 85u8, 151u8,
							180u8, 225u8, 180u8, 40u8, 49u8, 133u8, 107u8, 190u8, 102u8, 1u8,
							111u8, 144u8, 240u8, 0u8, 209u8, 198u8, 76u8, 143u8, 121u8, 2u8, 118u8,
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
							26u8, 71u8, 1u8, 2u8, 214u8, 3u8, 65u8, 62u8, 133u8, 85u8, 151u8,
							180u8, 225u8, 180u8, 40u8, 49u8, 133u8, 107u8, 190u8, 102u8, 1u8,
							111u8, 144u8, 240u8, 0u8, 209u8, 198u8, 76u8, 143u8, 121u8, 2u8, 118u8,
						],
					)
				}
				pub fn locked_fungibles(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
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
							158u8, 103u8, 153u8, 216u8, 19u8, 122u8, 251u8, 183u8, 15u8, 143u8,
							161u8, 105u8, 168u8, 100u8, 76u8, 220u8, 56u8, 129u8, 185u8, 251u8,
							220u8, 166u8, 3u8, 100u8, 48u8, 147u8, 123u8, 94u8, 226u8, 112u8, 59u8,
							171u8,
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
							158u8, 103u8, 153u8, 216u8, 19u8, 122u8, 251u8, 183u8, 15u8, 143u8,
							161u8, 105u8, 168u8, 100u8, 76u8, 220u8, 56u8, 129u8, 185u8, 251u8,
							220u8, 166u8, 3u8, 100u8, 48u8, 147u8, 123u8, 94u8, 226u8, 112u8, 59u8,
							171u8,
						],
					)
				}
			}
		}
	}
	pub mod cumulus_xcm {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
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
			pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
			pub struct ServiceOverweight {
				pub index: ::core::primitive::u64,
				pub weight_limit: runtime_types::sp_weights::weight_v2::Weight,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<ServiceOverweight> {
					::subxt::tx::Payload::new_static(
						"DmpQueue",
						"service_overweight",
						ServiceOverweight { index, weight_limit },
						[
							121u8, 236u8, 235u8, 23u8, 210u8, 238u8, 238u8, 122u8, 15u8, 86u8,
							34u8, 119u8, 105u8, 100u8, 214u8, 236u8, 117u8, 39u8, 254u8, 235u8,
							189u8, 15u8, 72u8, 74u8, 225u8, 134u8, 148u8, 126u8, 31u8, 203u8,
							144u8, 106u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_dmp_queue::pallet::Event;
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
			pub struct InvalidFormat {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnsupportedVersion {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExecutedDownward {
				pub message_id: [::core::primitive::u8; 32usize],
				pub outcome: runtime_types::xcm::v3::traits::Outcome,
			}
			impl ::subxt::events::StaticEvent for ExecutedDownward {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "ExecutedDownward";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OverweightServiced {
				pub overweight_index: ::core::primitive::u64,
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightServiced {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightServiced";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
							133u8, 113u8, 115u8, 164u8, 128u8, 145u8, 234u8, 106u8, 150u8, 54u8,
							247u8, 135u8, 181u8, 197u8, 178u8, 30u8, 204u8, 46u8, 6u8, 137u8, 82u8,
							1u8, 75u8, 171u8, 7u8, 157u8, 3u8, 19u8, 92u8, 10u8, 234u8, 66u8,
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
							94u8, 132u8, 34u8, 67u8, 10u8, 22u8, 235u8, 96u8, 168u8, 26u8, 57u8,
							200u8, 130u8, 218u8, 37u8, 71u8, 28u8, 119u8, 78u8, 107u8, 209u8,
							120u8, 190u8, 2u8, 101u8, 215u8, 122u8, 187u8, 94u8, 38u8, 255u8,
							234u8,
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
							228u8, 86u8, 33u8, 107u8, 248u8, 4u8, 223u8, 175u8, 222u8, 25u8, 204u8,
							42u8, 235u8, 21u8, 215u8, 91u8, 167u8, 14u8, 133u8, 151u8, 190u8, 57u8,
							138u8, 208u8, 79u8, 244u8, 132u8, 14u8, 48u8, 247u8, 171u8, 108u8,
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
							228u8, 86u8, 33u8, 107u8, 248u8, 4u8, 223u8, 175u8, 222u8, 25u8, 204u8,
							42u8, 235u8, 21u8, 215u8, 91u8, 167u8, 14u8, 133u8, 151u8, 190u8, 57u8,
							138u8, 208u8, 79u8, 244u8, 132u8, 14u8, 48u8, 247u8, 171u8, 108u8,
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
							222u8, 85u8, 143u8, 49u8, 42u8, 248u8, 138u8, 163u8, 46u8, 199u8,
							188u8, 61u8, 137u8, 135u8, 127u8, 146u8, 210u8, 254u8, 121u8, 42u8,
							112u8, 114u8, 22u8, 228u8, 207u8, 207u8, 245u8, 175u8, 152u8, 140u8,
							225u8, 237u8,
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
							222u8, 85u8, 143u8, 49u8, 42u8, 248u8, 138u8, 163u8, 46u8, 199u8,
							188u8, 61u8, 137u8, 135u8, 127u8, 146u8, 210u8, 254u8, 121u8, 42u8,
							112u8, 114u8, 22u8, 228u8, 207u8, 207u8, 245u8, 175u8, 152u8, 140u8,
							225u8, 237u8,
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
							148u8, 226u8, 248u8, 107u8, 165u8, 97u8, 218u8, 160u8, 127u8, 48u8,
							185u8, 251u8, 35u8, 137u8, 119u8, 251u8, 151u8, 167u8, 189u8, 66u8,
							80u8, 74u8, 134u8, 129u8, 222u8, 180u8, 51u8, 182u8, 50u8, 110u8, 10u8,
							43u8,
						],
					)
				}
			}
		}
	}
	pub mod x_tokens {
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
			pub struct Transfer {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferMultiasset {
				pub asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferWithFee {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
				pub fee: ::core::primitive::u128,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferMultiassetWithFee {
				pub asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
				pub fee: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferMulticurrencies {
				pub currencies: ::std::vec::Vec<(
					runtime_types::primitives::currency::CurrencyId,
					::core::primitive::u128,
				)>,
				pub fee_item: ::core::primitive::u32,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferMultiassets {
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_item: ::core::primitive::u32,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn transfer(
					&self,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<Transfer> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer",
						Transfer {
							currency_id,
							amount,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							240u8, 96u8, 64u8, 224u8, 162u8, 49u8, 108u8, 225u8, 196u8, 98u8, 96u8,
							120u8, 69u8, 133u8, 7u8, 215u8, 123u8, 32u8, 40u8, 224u8, 139u8, 249u8,
							95u8, 161u8, 113u8, 157u8, 130u8, 239u8, 133u8, 92u8, 157u8, 43u8,
						],
					)
				}
				pub fn transfer_multiasset(
					&self,
					asset: runtime_types::xcm::VersionedMultiAsset,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<TransferMultiasset> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer_multiasset",
						TransferMultiasset {
							asset: ::std::boxed::Box::new(asset),
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							248u8, 230u8, 0u8, 234u8, 150u8, 141u8, 112u8, 244u8, 133u8, 122u8,
							119u8, 121u8, 98u8, 230u8, 119u8, 126u8, 132u8, 175u8, 23u8, 143u8,
							81u8, 77u8, 184u8, 161u8, 235u8, 192u8, 186u8, 189u8, 136u8, 200u8,
							115u8, 116u8,
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
				) -> ::subxt::tx::Payload<TransferWithFee> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer_with_fee",
						TransferWithFee {
							currency_id,
							amount,
							fee,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							107u8, 201u8, 86u8, 135u8, 250u8, 253u8, 249u8, 5u8, 67u8, 12u8, 243u8,
							220u8, 23u8, 232u8, 136u8, 245u8, 107u8, 1u8, 110u8, 67u8, 102u8, 47u8,
							224u8, 139u8, 157u8, 200u8, 144u8, 237u8, 247u8, 185u8, 202u8, 118u8,
						],
					)
				}
				pub fn transfer_multiasset_with_fee(
					&self,
					asset: runtime_types::xcm::VersionedMultiAsset,
					fee: runtime_types::xcm::VersionedMultiAsset,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<TransferMultiassetWithFee> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer_multiasset_with_fee",
						TransferMultiassetWithFee {
							asset: ::std::boxed::Box::new(asset),
							fee: ::std::boxed::Box::new(fee),
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							69u8, 13u8, 138u8, 127u8, 184u8, 190u8, 236u8, 39u8, 208u8, 34u8,
							159u8, 241u8, 85u8, 85u8, 116u8, 37u8, 85u8, 221u8, 102u8, 125u8,
							159u8, 1u8, 70u8, 111u8, 113u8, 24u8, 40u8, 252u8, 128u8, 51u8, 113u8,
							144u8,
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
				) -> ::subxt::tx::Payload<TransferMulticurrencies> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer_multicurrencies",
						TransferMulticurrencies {
							currencies,
							fee_item,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							111u8, 160u8, 26u8, 215u8, 5u8, 203u8, 10u8, 102u8, 56u8, 145u8, 190u8,
							10u8, 19u8, 7u8, 4u8, 235u8, 183u8, 167u8, 169u8, 23u8, 120u8, 110u8,
							102u8, 71u8, 230u8, 239u8, 70u8, 255u8, 91u8, 76u8, 78u8, 185u8,
						],
					)
				}
				pub fn transfer_multiassets(
					&self,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_item: ::core::primitive::u32,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<TransferMultiassets> {
					::subxt::tx::Payload::new_static(
						"XTokens",
						"transfer_multiassets",
						TransferMultiassets {
							assets: ::std::boxed::Box::new(assets),
							fee_item,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							231u8, 125u8, 18u8, 214u8, 38u8, 20u8, 181u8, 151u8, 118u8, 43u8, 23u8,
							174u8, 169u8, 43u8, 84u8, 106u8, 236u8, 159u8, 19u8, 22u8, 245u8,
							252u8, 93u8, 181u8, 3u8, 52u8, 53u8, 82u8, 117u8, 78u8, 178u8, 88u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::orml_xtokens::module::Event;
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
			pub struct TransferredMultiAssets {
				pub sender: ::subxt::utils::AccountId32,
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
							184u8, 248u8, 100u8, 185u8, 98u8, 151u8, 232u8, 35u8, 100u8, 73u8,
							205u8, 51u8, 174u8, 76u8, 123u8, 182u8, 28u8, 14u8, 181u8, 186u8,
							146u8, 226u8, 224u8, 177u8, 18u8, 21u8, 112u8, 19u8, 134u8, 180u8,
							159u8, 238u8,
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
							206u8, 61u8, 253u8, 247u8, 163u8, 40u8, 161u8, 52u8, 134u8, 140u8,
							206u8, 83u8, 44u8, 166u8, 226u8, 115u8, 181u8, 14u8, 227u8, 130u8,
							210u8, 32u8, 85u8, 29u8, 230u8, 97u8, 130u8, 165u8, 147u8, 134u8,
							106u8, 76u8,
						],
					)
				}
			}
		}
	}
	pub mod unknown_tokens {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub type Event = runtime_types::orml_unknown_tokens::module::Event;
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
			pub struct Deposited {
				pub asset: runtime_types::xcm::v3::multiasset::MultiAsset,
				pub who: runtime_types::xcm::v3::multilocation::MultiLocation,
			}
			impl ::subxt::events::StaticEvent for Deposited {
				const PALLET: &'static str = "UnknownTokens";
				const EVENT: &'static str = "Deposited";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
							203u8, 8u8, 76u8, 85u8, 77u8, 70u8, 223u8, 249u8, 194u8, 27u8, 201u8,
							103u8, 162u8, 20u8, 85u8, 235u8, 168u8, 225u8, 167u8, 95u8, 131u8,
							141u8, 121u8, 80u8, 120u8, 82u8, 133u8, 221u8, 169u8, 80u8, 17u8, 78u8,
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
							203u8, 8u8, 76u8, 85u8, 77u8, 70u8, 223u8, 249u8, 194u8, 27u8, 201u8,
							103u8, 162u8, 20u8, 85u8, 235u8, 168u8, 225u8, 167u8, 95u8, 131u8,
							141u8, 121u8, 80u8, 120u8, 82u8, 133u8, 221u8, 169u8, 80u8, 17u8, 78u8,
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
							122u8, 0u8, 43u8, 27u8, 253u8, 102u8, 65u8, 175u8, 240u8, 26u8, 138u8,
							1u8, 44u8, 229u8, 170u8, 135u8, 180u8, 116u8, 104u8, 207u8, 132u8,
							48u8, 131u8, 144u8, 249u8, 17u8, 5u8, 92u8, 107u8, 38u8, 64u8, 89u8,
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
							122u8, 0u8, 43u8, 27u8, 253u8, 102u8, 65u8, 175u8, 240u8, 26u8, 138u8,
							1u8, 44u8, 229u8, 170u8, 135u8, 180u8, 116u8, 104u8, 207u8, 132u8,
							48u8, 131u8, 144u8, 249u8, 17u8, 5u8, 92u8, 107u8, 38u8, 64u8, 89u8,
						],
					)
				}
			}
		}
	}
	pub mod tokens {
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
			pub struct Transfer {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferAll {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferKeepAlive {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceTransfer {
				pub source: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SetBalance {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub new_free: ::core::primitive::u128,
				#[codec(compact)]
				pub new_reserved: ::core::primitive::u128,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn transfer(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<Transfer> {
					::subxt::tx::Payload::new_static(
						"Tokens",
						"transfer",
						Transfer { dest, currency_id, amount },
						[
							206u8, 83u8, 17u8, 48u8, 58u8, 130u8, 54u8, 103u8, 110u8, 163u8, 160u8,
							138u8, 162u8, 221u8, 65u8, 125u8, 126u8, 44u8, 210u8, 48u8, 212u8,
							83u8, 229u8, 173u8, 146u8, 129u8, 21u8, 111u8, 45u8, 85u8, 160u8,
							167u8,
						],
					)
				}
				pub fn transfer_all(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<TransferAll> {
					::subxt::tx::Payload::new_static(
						"Tokens",
						"transfer_all",
						TransferAll { dest, currency_id, keep_alive },
						[
							3u8, 187u8, 164u8, 247u8, 255u8, 219u8, 96u8, 91u8, 177u8, 2u8, 216u8,
							236u8, 119u8, 10u8, 114u8, 150u8, 166u8, 218u8, 88u8, 232u8, 119u8,
							132u8, 9u8, 185u8, 181u8, 40u8, 54u8, 107u8, 162u8, 201u8, 100u8,
							116u8,
						],
					)
				}
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<TransferKeepAlive> {
					::subxt::tx::Payload::new_static(
						"Tokens",
						"transfer_keep_alive",
						TransferKeepAlive { dest, currency_id, amount },
						[
							220u8, 153u8, 159u8, 112u8, 205u8, 69u8, 215u8, 153u8, 140u8, 202u8,
							205u8, 131u8, 61u8, 239u8, 33u8, 15u8, 133u8, 230u8, 2u8, 31u8, 61u8,
							4u8, 103u8, 190u8, 69u8, 89u8, 171u8, 57u8, 136u8, 54u8, 112u8, 194u8,
						],
					)
				}
				pub fn force_transfer(
					&self,
					source: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Tokens",
						"force_transfer",
						ForceTransfer { source, dest, currency_id, amount },
						[
							201u8, 63u8, 141u8, 47u8, 68u8, 174u8, 30u8, 110u8, 86u8, 85u8, 129u8,
							234u8, 133u8, 0u8, 122u8, 248u8, 80u8, 160u8, 1u8, 5u8, 194u8, 197u8,
							125u8, 162u8, 45u8, 116u8, 198u8, 249u8, 200u8, 108u8, 175u8, 22u8,
						],
					)
				}
				pub fn set_balance(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<SetBalance> {
					::subxt::tx::Payload::new_static(
						"Tokens",
						"set_balance",
						SetBalance { who, currency_id, new_free, new_reserved },
						[
							163u8, 191u8, 141u8, 39u8, 221u8, 176u8, 17u8, 59u8, 148u8, 120u8,
							11u8, 122u8, 195u8, 81u8, 44u8, 216u8, 33u8, 205u8, 119u8, 59u8, 77u8,
							92u8, 1u8, 107u8, 206u8, 78u8, 100u8, 51u8, 155u8, 122u8, 228u8, 24u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::orml_tokens::module::Event;
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
			pub struct Endowed {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DustLost {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "DustLost";
			}
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
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Reserved {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Unreserved {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ReserveRepatriated {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
				pub status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BalanceSet {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub free: ::core::primitive::u128,
				pub reserved: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TotalIssuanceSet {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TotalIssuanceSet {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "TotalIssuanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Withdrawn {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdrawn {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Withdrawn";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Slashed {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub free_amount: ::core::primitive::u128,
				pub reserved_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Slashed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Deposited {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposited {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Deposited";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct LockSet {
				pub lock_id: [::core::primitive::u8; 8usize],
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for LockSet {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "LockSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct LockRemoved {
				pub lock_id: [::core::primitive::u8; 8usize],
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for LockRemoved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "LockRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Locked {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Locked {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Locked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Unlocked {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
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
							203u8, 177u8, 143u8, 200u8, 190u8, 210u8, 147u8, 46u8, 47u8, 202u8,
							42u8, 57u8, 168u8, 246u8, 168u8, 203u8, 190u8, 234u8, 253u8, 130u8,
							72u8, 19u8, 2u8, 227u8, 115u8, 21u8, 106u8, 71u8, 239u8, 148u8, 192u8,
							106u8,
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
							203u8, 177u8, 143u8, 200u8, 190u8, 210u8, 147u8, 46u8, 47u8, 202u8,
							42u8, 57u8, 168u8, 246u8, 168u8, 203u8, 190u8, 234u8, 253u8, 130u8,
							72u8, 19u8, 2u8, 227u8, 115u8, 21u8, 106u8, 71u8, 239u8, 148u8, 192u8,
							106u8,
						],
					)
				}
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
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
							102u8, 209u8, 145u8, 141u8, 113u8, 97u8, 120u8, 28u8, 130u8, 122u8,
							139u8, 193u8, 38u8, 34u8, 146u8, 166u8, 222u8, 97u8, 193u8, 137u8,
							116u8, 56u8, 3u8, 118u8, 192u8, 249u8, 74u8, 17u8, 224u8, 53u8, 209u8,
							195u8,
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
							102u8, 209u8, 145u8, 141u8, 113u8, 97u8, 120u8, 28u8, 130u8, 122u8,
							139u8, 193u8, 38u8, 34u8, 146u8, 166u8, 222u8, 97u8, 193u8, 137u8,
							116u8, 56u8, 3u8, 118u8, 192u8, 249u8, 74u8, 17u8, 224u8, 53u8, 209u8,
							195u8,
						],
					)
				}
				pub fn accounts(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
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
							80u8, 235u8, 135u8, 10u8, 123u8, 40u8, 79u8, 225u8, 219u8, 128u8,
							105u8, 19u8, 7u8, 57u8, 131u8, 239u8, 221u8, 8u8, 122u8, 212u8, 191u8,
							186u8, 232u8, 221u8, 196u8, 10u8, 150u8, 219u8, 132u8, 161u8, 60u8,
							247u8,
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
							80u8, 235u8, 135u8, 10u8, 123u8, 40u8, 79u8, 225u8, 219u8, 128u8,
							105u8, 19u8, 7u8, 57u8, 131u8, 239u8, 221u8, 8u8, 122u8, 212u8, 191u8,
							186u8, 232u8, 221u8, 196u8, 10u8, 150u8, 219u8, 132u8, 161u8, 60u8,
							247u8,
						],
					)
				}
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
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
							68u8, 199u8, 14u8, 150u8, 65u8, 10u8, 132u8, 26u8, 235u8, 92u8, 5u8,
							96u8, 117u8, 28u8, 179u8, 113u8, 214u8, 210u8, 136u8, 183u8, 137u8,
							23u8, 64u8, 12u8, 181u8, 8u8, 239u8, 215u8, 57u8, 78u8, 237u8, 94u8,
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
							68u8, 199u8, 14u8, 150u8, 65u8, 10u8, 132u8, 26u8, 235u8, 92u8, 5u8,
							96u8, 117u8, 28u8, 179u8, 113u8, 214u8, 210u8, 136u8, 183u8, 137u8,
							23u8, 64u8, 12u8, 181u8, 8u8, 239u8, 215u8, 57u8, 78u8, 237u8, 94u8,
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
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
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
			pub struct AddRange {
				pub length: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SetMetadata {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub metadata: runtime_types::composable_traits::assets::BasicAssetMetadata,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_range(
					&self,
					length: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<AddRange> {
					::subxt::tx::Payload::new_static(
						"CurrencyFactory",
						"add_range",
						AddRange { length },
						[
							239u8, 242u8, 170u8, 252u8, 41u8, 195u8, 156u8, 238u8, 196u8, 166u8,
							6u8, 228u8, 202u8, 48u8, 230u8, 140u8, 228u8, 214u8, 157u8, 67u8, 81u8,
							9u8, 215u8, 113u8, 199u8, 238u8, 2u8, 163u8, 239u8, 192u8, 155u8, 38u8,
						],
					)
				}
				pub fn set_metadata(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					metadata: runtime_types::composable_traits::assets::BasicAssetMetadata,
				) -> ::subxt::tx::Payload<SetMetadata> {
					::subxt::tx::Payload::new_static(
						"CurrencyFactory",
						"set_metadata",
						SetMetadata { asset_id, metadata },
						[
							247u8, 186u8, 131u8, 92u8, 172u8, 202u8, 106u8, 118u8, 77u8, 255u8,
							150u8, 218u8, 247u8, 1u8, 131u8, 42u8, 160u8, 162u8, 191u8, 154u8,
							150u8, 65u8, 23u8, 188u8, 183u8, 58u8, 102u8, 64u8, 16u8, 229u8, 234u8,
							32u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_currency_factory::pallet::Event;
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
							22u8, 227u8, 15u8, 251u8, 150u8, 72u8, 61u8, 107u8, 142u8, 193u8,
							253u8, 199u8, 241u8, 219u8, 138u8, 28u8, 59u8, 177u8, 155u8, 80u8,
							26u8, 245u8, 85u8, 141u8, 122u8, 161u8, 215u8, 147u8, 202u8, 168u8,
							149u8, 156u8,
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
							157u8, 246u8, 89u8, 3u8, 170u8, 111u8, 221u8, 215u8, 106u8, 78u8, 11u8,
							245u8, 15u8, 218u8, 143u8, 173u8, 188u8, 148u8, 224u8, 153u8, 82u8,
							54u8, 242u8, 102u8, 164u8, 129u8, 100u8, 119u8, 69u8, 227u8, 144u8,
							62u8,
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
							157u8, 246u8, 89u8, 3u8, 170u8, 111u8, 221u8, 215u8, 106u8, 78u8, 11u8,
							245u8, 15u8, 218u8, 143u8, 173u8, 188u8, 148u8, 224u8, 153u8, 82u8,
							54u8, 242u8, 102u8, 164u8, 129u8, 100u8, 119u8, 69u8, 227u8, 144u8,
							62u8,
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
							185u8, 114u8, 217u8, 111u8, 55u8, 241u8, 125u8, 51u8, 95u8, 89u8, 39u8,
							166u8, 183u8, 208u8, 129u8, 214u8, 56u8, 6u8, 0u8, 44u8, 134u8, 242u8,
							45u8, 238u8, 61u8, 41u8, 155u8, 137u8, 166u8, 53u8, 130u8, 28u8,
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
							185u8, 114u8, 217u8, 111u8, 55u8, 241u8, 125u8, 51u8, 95u8, 89u8, 39u8,
							166u8, 183u8, 208u8, 129u8, 214u8, 56u8, 6u8, 0u8, 44u8, 134u8, 242u8,
							45u8, 238u8, 61u8, 41u8, 155u8, 137u8, 166u8, 53u8, 130u8, 28u8,
						],
					)
				}
			}
		}
	}
	pub mod crowdloan_rewards {
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
			pub struct Initialize;
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
			pub struct InitializeAt {
				pub at: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Populate {
				pub rewards: ::std::vec::Vec<(
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						[::core::primitive::u8; 32usize],
					>,
					::core::primitive::u128,
					::core::primitive::u64,
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
			pub struct Associate {
				pub reward_account: ::subxt::utils::AccountId32,
				pub proof: runtime_types::pallet_crowdloan_rewards::models::Proof<
					[::core::primitive::u8; 32usize],
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
			pub struct Claim;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnlockRewardsFor {
				pub reward_accounts: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Add {
				pub additions: ::std::vec::Vec<(
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						[::core::primitive::u8; 32usize],
					>,
					::core::primitive::u128,
					::core::primitive::u64,
				)>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn initialize(&self) -> ::subxt::tx::Payload<Initialize> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"initialize",
						Initialize {},
						[
							210u8, 6u8, 171u8, 194u8, 188u8, 76u8, 163u8, 192u8, 223u8, 241u8,
							194u8, 189u8, 221u8, 190u8, 28u8, 191u8, 208u8, 85u8, 140u8, 167u8,
							160u8, 29u8, 155u8, 216u8, 185u8, 27u8, 109u8, 39u8, 4u8, 82u8, 50u8,
							180u8,
						],
					)
				}
				pub fn initialize_at(
					&self,
					at: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<InitializeAt> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"initialize_at",
						InitializeAt { at },
						[
							213u8, 36u8, 13u8, 147u8, 34u8, 81u8, 248u8, 154u8, 70u8, 189u8, 57u8,
							225u8, 107u8, 84u8, 25u8, 18u8, 160u8, 135u8, 118u8, 251u8, 223u8,
							204u8, 43u8, 65u8, 50u8, 130u8, 31u8, 80u8, 16u8, 158u8, 173u8, 20u8,
						],
					)
				}
				pub fn populate(
					&self,
					rewards: ::std::vec::Vec<(
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							[::core::primitive::u8; 32usize],
						>,
						::core::primitive::u128,
						::core::primitive::u64,
					)>,
				) -> ::subxt::tx::Payload<Populate> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"populate",
						Populate { rewards },
						[
							218u8, 90u8, 77u8, 124u8, 0u8, 48u8, 38u8, 89u8, 185u8, 77u8, 126u8,
							131u8, 109u8, 112u8, 52u8, 19u8, 65u8, 60u8, 0u8, 81u8, 45u8, 173u8,
							161u8, 245u8, 167u8, 248u8, 246u8, 147u8, 244u8, 181u8, 23u8, 138u8,
						],
					)
				}
				pub fn associate(
					&self,
					reward_account: ::subxt::utils::AccountId32,
					proof: runtime_types::pallet_crowdloan_rewards::models::Proof<
						[::core::primitive::u8; 32usize],
					>,
				) -> ::subxt::tx::Payload<Associate> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"associate",
						Associate { reward_account, proof },
						[
							172u8, 177u8, 98u8, 224u8, 100u8, 149u8, 103u8, 198u8, 237u8, 49u8,
							31u8, 231u8, 222u8, 173u8, 28u8, 233u8, 20u8, 109u8, 135u8, 134u8,
							170u8, 40u8, 244u8, 28u8, 174u8, 139u8, 51u8, 229u8, 120u8, 251u8,
							73u8, 5u8,
						],
					)
				}
				pub fn claim(&self) -> ::subxt::tx::Payload<Claim> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"claim",
						Claim {},
						[
							45u8, 97u8, 229u8, 222u8, 255u8, 43u8, 179u8, 22u8, 163u8, 231u8, 33u8,
							96u8, 167u8, 206u8, 213u8, 116u8, 80u8, 254u8, 184u8, 3u8, 96u8, 5u8,
							160u8, 81u8, 148u8, 30u8, 117u8, 255u8, 107u8, 177u8, 200u8, 78u8,
						],
					)
				}
				pub fn unlock_rewards_for(
					&self,
					reward_accounts: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<UnlockRewardsFor> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"unlock_rewards_for",
						UnlockRewardsFor { reward_accounts },
						[
							116u8, 71u8, 22u8, 93u8, 198u8, 85u8, 61u8, 147u8, 75u8, 125u8, 232u8,
							122u8, 54u8, 186u8, 142u8, 244u8, 235u8, 65u8, 164u8, 187u8, 11u8,
							90u8, 72u8, 111u8, 104u8, 109u8, 239u8, 164u8, 148u8, 43u8, 248u8,
							187u8,
						],
					)
				}
				pub fn add(
					&self,
					additions: ::std::vec::Vec<(
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							[::core::primitive::u8; 32usize],
						>,
						::core::primitive::u128,
						::core::primitive::u64,
					)>,
				) -> ::subxt::tx::Payload<Add> {
					::subxt::tx::Payload::new_static(
						"CrowdloanRewards",
						"add",
						Add { additions },
						[
							253u8, 134u8, 219u8, 228u8, 49u8, 178u8, 201u8, 105u8, 69u8, 236u8,
							199u8, 179u8, 132u8, 113u8, 73u8, 110u8, 181u8, 233u8, 215u8, 145u8,
							27u8, 32u8, 221u8, 185u8, 168u8, 129u8, 56u8, 102u8, 94u8, 172u8, 59u8,
							129u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_crowdloan_rewards::pallet::Event;
		pub mod events {
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
			pub struct Initialized {
				pub at: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for Initialized {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "Initialized";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Claimed {
				pub remote_account: runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
					[::core::primitive::u8; 32usize],
				>,
				pub reward_account: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Claimed {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "Claimed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Associated {
				pub remote_account: runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
					[::core::primitive::u8; 32usize],
				>,
				pub reward_account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Associated {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "Associated";
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
			pub struct OverFunded {
				pub excess_funds: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for OverFunded {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "OverFunded";
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
			pub struct RewardsUnlocked {
				pub at: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for RewardsUnlocked {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "RewardsUnlocked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RewardsAdded {
				pub additions: ::std::vec::Vec<(
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						[::core::primitive::u8; 32usize],
					>,
					::core::primitive::u128,
					::core::primitive::u64,
				)>,
			}
			impl ::subxt::events::StaticEvent for RewardsAdded {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "RewardsAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RewardsDeleted {
				pub deletions: ::std::vec::Vec<
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						[::core::primitive::u8; 32usize],
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
							[::core::primitive::u8; 32usize],
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
							82u8, 149u8, 137u8, 45u8, 253u8, 248u8, 147u8, 196u8, 112u8, 11u8,
							148u8, 238u8, 132u8, 84u8, 242u8, 5u8, 204u8, 201u8, 41u8, 27u8, 252u8,
							42u8, 94u8, 250u8, 127u8, 169u8, 230u8, 130u8, 193u8, 10u8, 252u8,
							145u8,
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
							82u8, 149u8, 137u8, 45u8, 253u8, 248u8, 147u8, 196u8, 112u8, 11u8,
							148u8, 238u8, 132u8, 84u8, 242u8, 5u8, 204u8, 201u8, 41u8, 27u8, 252u8,
							42u8, 94u8, 250u8, 127u8, 169u8, 230u8, 130u8, 193u8, 10u8, 252u8,
							145u8,
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
							37u8, 36u8, 124u8, 79u8, 45u8, 126u8, 177u8, 179u8, 118u8, 125u8,
							178u8, 245u8, 125u8, 208u8, 201u8, 248u8, 51u8, 5u8, 202u8, 199u8,
							82u8, 75u8, 64u8, 150u8, 40u8, 196u8, 223u8, 17u8, 32u8, 105u8, 208u8,
							126u8,
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
							250u8, 96u8, 206u8, 11u8, 109u8, 190u8, 255u8, 1u8, 24u8, 244u8, 7u8,
							255u8, 93u8, 85u8, 138u8, 87u8, 165u8, 25u8, 154u8, 246u8, 135u8,
							210u8, 89u8, 170u8, 227u8, 236u8, 123u8, 161u8, 77u8, 214u8, 44u8,
							240u8,
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
							236u8, 88u8, 207u8, 169u8, 18u8, 55u8, 31u8, 213u8, 140u8, 154u8,
							142u8, 214u8, 66u8, 114u8, 157u8, 35u8, 172u8, 205u8, 122u8, 169u8,
							45u8, 64u8, 132u8, 177u8, 180u8, 21u8, 208u8, 12u8, 20u8, 23u8, 13u8,
							30u8,
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
							93u8, 101u8, 112u8, 233u8, 17u8, 239u8, 82u8, 207u8, 167u8, 62u8,
							181u8, 104u8, 114u8, 195u8, 132u8, 255u8, 106u8, 152u8, 75u8, 200u8,
							76u8, 193u8, 89u8, 137u8, 224u8, 62u8, 225u8, 206u8, 157u8, 28u8,
							126u8, 48u8,
						],
					)
				}
				pub fn associations(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						[::core::primitive::u8; 32usize],
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
							85u8, 12u8, 50u8, 120u8, 143u8, 116u8, 152u8, 188u8, 100u8, 72u8, 80u8,
							64u8, 16u8, 169u8, 122u8, 10u8, 221u8, 178u8, 231u8, 78u8, 151u8, 31u8,
							216u8, 254u8, 118u8, 243u8, 237u8, 37u8, 127u8, 238u8, 206u8, 101u8,
						],
					)
				}
				pub fn associations_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						[::core::primitive::u8; 32usize],
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
							85u8, 12u8, 50u8, 120u8, 143u8, 116u8, 152u8, 188u8, 100u8, 72u8, 80u8,
							64u8, 16u8, 169u8, 122u8, 10u8, 221u8, 178u8, 231u8, 78u8, 151u8, 31u8,
							216u8, 254u8, 118u8, 243u8, 237u8, 37u8, 127u8, 238u8, 206u8, 101u8,
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
							88u8, 210u8, 233u8, 161u8, 138u8, 199u8, 210u8, 0u8, 71u8, 237u8,
							189u8, 204u8, 252u8, 44u8, 191u8, 207u8, 81u8, 76u8, 220u8, 222u8,
							13u8, 236u8, 71u8, 55u8, 224u8, 246u8, 57u8, 31u8, 58u8, 191u8, 158u8,
							13u8,
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
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
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
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
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
							106u8, 50u8, 57u8, 116u8, 43u8, 202u8, 37u8, 248u8, 102u8, 22u8, 62u8,
							22u8, 242u8, 54u8, 152u8, 168u8, 107u8, 64u8, 72u8, 172u8, 124u8, 40u8,
							42u8, 110u8, 104u8, 145u8, 31u8, 144u8, 242u8, 189u8, 145u8, 208u8,
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
							139u8, 109u8, 228u8, 151u8, 252u8, 32u8, 130u8, 69u8, 112u8, 154u8,
							174u8, 45u8, 83u8, 245u8, 51u8, 132u8, 173u8, 5u8, 186u8, 24u8, 243u8,
							9u8, 12u8, 214u8, 80u8, 74u8, 69u8, 189u8, 30u8, 94u8, 22u8, 39u8,
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
							224u8, 197u8, 247u8, 125u8, 62u8, 180u8, 69u8, 91u8, 226u8, 36u8, 82u8,
							148u8, 70u8, 147u8, 209u8, 40u8, 210u8, 229u8, 181u8, 191u8, 170u8,
							205u8, 138u8, 97u8, 127u8, 59u8, 124u8, 244u8, 252u8, 30u8, 213u8,
							179u8,
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
				) -> ::subxt::constants::Address<::subxt::utils::AccountId32> {
					::subxt::constants::Address::new_static(
						"CrowdloanRewards",
						"account_id",
						[
							167u8, 71u8, 0u8, 47u8, 217u8, 107u8, 29u8, 163u8, 157u8, 187u8, 110u8,
							219u8, 88u8, 213u8, 82u8, 107u8, 46u8, 199u8, 41u8, 110u8, 102u8,
							187u8, 45u8, 201u8, 247u8, 66u8, 33u8, 228u8, 33u8, 99u8, 242u8, 80u8,
						],
					)
				}
			}
		}
	}
	pub mod assets {
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
			pub struct Transfer {
				pub asset: runtime_types::primitives::currency::CurrencyId,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferNative {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceTransfer {
				pub asset: runtime_types::primitives::currency::CurrencyId,
				pub source: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ForceTransferNative {
				pub source: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferAll {
				pub asset: runtime_types::primitives::currency::CurrencyId,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct TransferAllNative {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MintInitialize {
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub dest: ::subxt::utils::MultiAddress<
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
			pub struct MintInitializeWithGovernance {
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub governance_origin: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::utils::MultiAddress<
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
			pub struct MintInto {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BurnFrom {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn transfer(
					&self,
					asset: runtime_types::primitives::currency::CurrencyId,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<Transfer> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"transfer",
						Transfer { asset, dest, amount, keep_alive },
						[
							191u8, 249u8, 227u8, 177u8, 227u8, 30u8, 137u8, 210u8, 170u8, 186u8,
							138u8, 181u8, 23u8, 51u8, 178u8, 172u8, 107u8, 134u8, 163u8, 172u8,
							190u8, 202u8, 127u8, 160u8, 205u8, 98u8, 205u8, 39u8, 15u8, 68u8,
							165u8, 80u8,
						],
					)
				}
				pub fn transfer_native(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<TransferNative> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"transfer_native",
						TransferNative { dest, value, keep_alive },
						[
							203u8, 255u8, 186u8, 102u8, 209u8, 83u8, 227u8, 118u8, 11u8, 209u8,
							70u8, 190u8, 67u8, 158u8, 173u8, 231u8, 41u8, 137u8, 127u8, 209u8,
							160u8, 160u8, 59u8, 226u8, 154u8, 116u8, 108u8, 210u8, 87u8, 108u8,
							141u8, 18u8,
						],
					)
				}
				pub fn force_transfer(
					&self,
					asset: runtime_types::primitives::currency::CurrencyId,
					source: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"force_transfer",
						ForceTransfer { asset, source, dest, value, keep_alive },
						[
							123u8, 143u8, 36u8, 52u8, 57u8, 12u8, 209u8, 44u8, 106u8, 69u8, 200u8,
							38u8, 79u8, 3u8, 59u8, 128u8, 242u8, 132u8, 83u8, 22u8, 13u8, 7u8,
							185u8, 221u8, 193u8, 73u8, 242u8, 55u8, 109u8, 194u8, 15u8, 163u8,
						],
					)
				}
				pub fn force_transfer_native(
					&self,
					source: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<ForceTransferNative> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"force_transfer_native",
						ForceTransferNative { source, dest, value, keep_alive },
						[
							109u8, 219u8, 2u8, 9u8, 154u8, 57u8, 173u8, 220u8, 132u8, 248u8, 31u8,
							203u8, 185u8, 230u8, 252u8, 89u8, 92u8, 152u8, 87u8, 44u8, 21u8, 209u8,
							202u8, 159u8, 229u8, 5u8, 156u8, 252u8, 219u8, 9u8, 138u8, 135u8,
						],
					)
				}
				pub fn transfer_all(
					&self,
					asset: runtime_types::primitives::currency::CurrencyId,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<TransferAll> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"transfer_all",
						TransferAll { asset, dest, keep_alive },
						[
							252u8, 242u8, 56u8, 229u8, 110u8, 245u8, 215u8, 78u8, 248u8, 237u8,
							202u8, 143u8, 219u8, 104u8, 121u8, 75u8, 53u8, 234u8, 134u8, 214u8,
							73u8, 250u8, 151u8, 124u8, 247u8, 60u8, 230u8, 36u8, 26u8, 222u8,
							240u8, 108u8,
						],
					)
				}
				pub fn transfer_all_native(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<TransferAllNative> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"transfer_all_native",
						TransferAllNative { dest, keep_alive },
						[
							199u8, 166u8, 244u8, 2u8, 74u8, 109u8, 252u8, 7u8, 251u8, 242u8, 80u8,
							154u8, 164u8, 73u8, 144u8, 79u8, 83u8, 188u8, 208u8, 23u8, 127u8, 19u8,
							234u8, 226u8, 111u8, 93u8, 176u8, 171u8, 178u8, 132u8, 74u8, 63u8,
						],
					)
				}
				pub fn mint_initialize(
					&self,
					amount: ::core::primitive::u128,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<MintInitialize> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"mint_initialize",
						MintInitialize { amount, dest },
						[
							46u8, 118u8, 244u8, 196u8, 195u8, 185u8, 222u8, 58u8, 151u8, 155u8,
							118u8, 131u8, 134u8, 226u8, 8u8, 155u8, 76u8, 98u8, 92u8, 157u8, 133u8,
							62u8, 166u8, 172u8, 200u8, 39u8, 11u8, 184u8, 87u8, 73u8, 62u8, 36u8,
						],
					)
				}
				pub fn mint_initialize_with_governance(
					&self,
					amount: ::core::primitive::u128,
					governance_origin: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<MintInitializeWithGovernance> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"mint_initialize_with_governance",
						MintInitializeWithGovernance { amount, governance_origin, dest },
						[
							112u8, 237u8, 174u8, 228u8, 234u8, 128u8, 152u8, 223u8, 18u8, 220u8,
							251u8, 233u8, 136u8, 177u8, 214u8, 237u8, 151u8, 115u8, 86u8, 68u8,
							220u8, 98u8, 98u8, 101u8, 94u8, 55u8, 195u8, 248u8, 233u8, 20u8, 186u8,
							45u8,
						],
					)
				}
				pub fn mint_into(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<MintInto> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"mint_into",
						MintInto { asset_id, dest, amount },
						[
							67u8, 51u8, 185u8, 110u8, 243u8, 173u8, 151u8, 175u8, 141u8, 214u8,
							194u8, 139u8, 176u8, 25u8, 49u8, 248u8, 121u8, 103u8, 178u8, 128u8,
							5u8, 52u8, 66u8, 232u8, 182u8, 57u8, 192u8, 55u8, 136u8, 90u8, 60u8,
							32u8,
						],
					)
				}
				pub fn burn_from(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<BurnFrom> {
					::subxt::tx::Payload::new_static(
						"Assets",
						"burn_from",
						BurnFrom { asset_id, dest, amount },
						[
							97u8, 142u8, 84u8, 209u8, 163u8, 111u8, 93u8, 46u8, 152u8, 84u8, 142u8,
							82u8, 3u8, 128u8, 43u8, 26u8, 148u8, 160u8, 230u8, 48u8, 239u8, 34u8,
							174u8, 88u8, 52u8, 149u8, 146u8, 77u8, 139u8, 31u8, 225u8, 102u8,
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
						"Assets",
						"NativeAssetId",
						[
							150u8, 207u8, 49u8, 178u8, 254u8, 209u8, 81u8, 36u8, 235u8, 117u8,
							62u8, 166u8, 4u8, 173u8, 64u8, 189u8, 19u8, 182u8, 131u8, 166u8, 234u8,
							145u8, 83u8, 23u8, 246u8, 20u8, 47u8, 34u8, 66u8, 162u8, 146u8, 49u8,
						],
					)
				}
			}
		}
	}
	pub mod governance_registry {
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
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub value: ::subxt::utils::AccountId32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct GrantRoot {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Remove {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					value: ::subxt::utils::AccountId32,
				) -> ::subxt::tx::Payload<Set> {
					::subxt::tx::Payload::new_static(
						"GovernanceRegistry",
						"set",
						Set { asset_id, value },
						[
							38u8, 114u8, 100u8, 157u8, 78u8, 228u8, 102u8, 210u8, 55u8, 88u8, 52u8,
							18u8, 121u8, 34u8, 119u8, 225u8, 203u8, 174u8, 141u8, 25u8, 51u8,
							148u8, 40u8, 130u8, 222u8, 173u8, 49u8, 174u8, 41u8, 106u8, 3u8, 4u8,
						],
					)
				}
				pub fn grant_root(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
				) -> ::subxt::tx::Payload<GrantRoot> {
					::subxt::tx::Payload::new_static(
						"GovernanceRegistry",
						"grant_root",
						GrantRoot { asset_id },
						[
							108u8, 52u8, 102u8, 38u8, 197u8, 192u8, 164u8, 84u8, 15u8, 112u8,
							103u8, 138u8, 152u8, 15u8, 42u8, 241u8, 242u8, 96u8, 189u8, 37u8, 49u8,
							133u8, 70u8, 97u8, 144u8, 98u8, 115u8, 78u8, 151u8, 73u8, 45u8, 71u8,
						],
					)
				}
				pub fn remove(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
				) -> ::subxt::tx::Payload<Remove> {
					::subxt::tx::Payload::new_static(
						"GovernanceRegistry",
						"remove",
						Remove { asset_id },
						[
							44u8, 136u8, 86u8, 109u8, 250u8, 117u8, 227u8, 246u8, 40u8, 114u8,
							204u8, 219u8, 208u8, 23u8, 103u8, 113u8, 249u8, 99u8, 199u8, 86u8,
							25u8, 165u8, 160u8, 59u8, 187u8, 67u8, 192u8, 111u8, 177u8, 80u8,
							254u8, 69u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_governance_registry::pallet::Event;
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
			pub struct Set {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub value: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Set {
				const PALLET: &'static str = "GovernanceRegistry";
				const EVENT: &'static str = "Set";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct GrantRoot {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			impl ::subxt::events::StaticEvent for GrantRoot {
				const PALLET: &'static str = "GovernanceRegistry";
				const EVENT: &'static str = "GrantRoot";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Remove {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			impl ::subxt::events::StaticEvent for Remove {
				const PALLET: &'static str = "GovernanceRegistry";
				const EVENT: &'static str = "Remove";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn origins_by_asset_id(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_traits::governance::SignedRawOrigin<
						::subxt::utils::AccountId32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"GovernanceRegistry",
						"OriginsByAssetId",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							188u8, 140u8, 55u8, 118u8, 91u8, 24u8, 34u8, 167u8, 239u8, 89u8, 100u8,
							77u8, 158u8, 53u8, 227u8, 26u8, 137u8, 42u8, 189u8, 220u8, 210u8, 52u8,
							4u8, 178u8, 33u8, 226u8, 30u8, 231u8, 168u8, 115u8, 189u8, 238u8,
						],
					)
				}
				pub fn origins_by_asset_id_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::composable_traits::governance::SignedRawOrigin<
						::subxt::utils::AccountId32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"GovernanceRegistry",
						"OriginsByAssetId",
						Vec::new(),
						[
							188u8, 140u8, 55u8, 118u8, 91u8, 24u8, 34u8, 167u8, 239u8, 89u8, 100u8,
							77u8, 158u8, 53u8, 227u8, 26u8, 137u8, 42u8, 189u8, 220u8, 210u8, 52u8,
							4u8, 178u8, 33u8, 226u8, 30u8, 231u8, 168u8, 115u8, 189u8, 238u8,
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
				pub protocol_id: [::core::primitive::u8; 8usize],
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
					protocol_id: [::core::primitive::u8; 8usize],
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
							74u8, 117u8, 89u8, 87u8, 93u8, 136u8, 210u8, 115u8, 212u8, 242u8,
							196u8, 224u8, 185u8, 130u8, 226u8, 83u8, 141u8, 146u8, 138u8, 164u8,
							128u8, 226u8, 122u8, 143u8, 208u8, 5u8, 189u8, 245u8, 160u8, 13u8,
							23u8, 86u8,
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
	}
	pub mod call_filter {
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
			pub struct Disable {
				pub entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
					runtime_types::common::MaxStringSize,
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
			pub struct Enable {
				pub entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
					runtime_types::common::MaxStringSize,
				>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn disable(
					&self,
					entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
						runtime_types::common::MaxStringSize,
					>,
				) -> ::subxt::tx::Payload<Disable> {
					::subxt::tx::Payload::new_static(
						"CallFilter",
						"disable",
						Disable { entry },
						[
							135u8, 13u8, 52u8, 184u8, 86u8, 89u8, 77u8, 78u8, 232u8, 107u8, 230u8,
							67u8, 122u8, 192u8, 193u8, 4u8, 59u8, 44u8, 175u8, 209u8, 194u8, 49u8,
							73u8, 116u8, 232u8, 227u8, 56u8, 254u8, 72u8, 54u8, 206u8, 27u8,
						],
					)
				}
				pub fn enable(
					&self,
					entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
						runtime_types::common::MaxStringSize,
					>,
				) -> ::subxt::tx::Payload<Enable> {
					::subxt::tx::Payload::new_static(
						"CallFilter",
						"enable",
						Enable { entry },
						[
							24u8, 54u8, 83u8, 13u8, 223u8, 77u8, 229u8, 162u8, 164u8, 107u8, 208u8,
							132u8, 0u8, 252u8, 176u8, 125u8, 236u8, 185u8, 128u8, 209u8, 252u8,
							116u8, 112u8, 242u8, 25u8, 76u8, 69u8, 22u8, 4u8, 205u8, 227u8, 207u8,
						],
					)
				}
			}
		}
		pub type Event = runtime_types::pallet_call_filter::pallet::Event;
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
			pub struct Disabled {
				pub entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
					runtime_types::common::MaxStringSize,
				>,
			}
			impl ::subxt::events::StaticEvent for Disabled {
				const PALLET: &'static str = "CallFilter";
				const EVENT: &'static str = "Disabled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
							54u8, 93u8, 201u8, 230u8, 97u8, 19u8, 95u8, 130u8, 213u8, 155u8, 135u8,
							208u8, 52u8, 55u8, 238u8, 157u8, 6u8, 135u8, 46u8, 136u8, 82u8, 53u8,
							1u8, 182u8, 38u8, 172u8, 140u8, 63u8, 56u8, 88u8, 173u8, 16u8,
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
							54u8, 93u8, 201u8, 230u8, 97u8, 19u8, 95u8, 130u8, 213u8, 155u8, 135u8,
							208u8, 52u8, 55u8, 238u8, 157u8, 6u8, 135u8, 46u8, 136u8, 82u8, 53u8,
							1u8, 182u8, 38u8, 172u8, 140u8, 63u8, 56u8, 88u8, 173u8, 16u8,
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
				pub memo:
					::core::option::Option<runtime_types::composable_runtime::ibc::MemoMessage>,
			}
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
					memo: ::core::option::Option<
						runtime_types::composable_runtime::ibc::MemoMessage,
					>,
				) -> ::subxt::tx::Payload<Transfer> {
					::subxt::tx::Payload::new_static(
						"Ibc",
						"transfer",
						Transfer { params, asset_id, amount, memo },
						[
							104u8, 136u8, 36u8, 50u8, 105u8, 241u8, 120u8, 243u8, 74u8, 48u8,
							173u8, 124u8, 56u8, 78u8, 20u8, 193u8, 27u8, 73u8, 16u8, 127u8, 125u8,
							113u8, 70u8, 115u8, 43u8, 2u8, 103u8, 109u8, 208u8, 40u8, 10u8, 36u8,
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
	pub mod ics20_fee {
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
			pub struct SetCharge {
				pub charge: runtime_types::sp_arithmetic::per_things::Perbill,
			}
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
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn set_charge(
					&self,
					charge: runtime_types::sp_arithmetic::per_things::Perbill,
				) -> ::subxt::tx::Payload<SetCharge> {
					::subxt::tx::Payload::new_static(
						"Ics20Fee",
						"set_charge",
						SetCharge { charge },
						[
							170u8, 50u8, 193u8, 188u8, 61u8, 223u8, 45u8, 155u8, 32u8, 252u8, 90u8,
							233u8, 31u8, 114u8, 226u8, 200u8, 227u8, 169u8, 87u8, 114u8, 26u8,
							158u8, 86u8, 40u8, 133u8, 240u8, 28u8, 120u8, 187u8, 26u8, 87u8, 11u8,
						],
					)
				}
				pub fn add_channels_to_feeless_channel_list(
					&self,
					source_channel: ::core::primitive::u64,
					destination_channel: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<AddChannelsToFeelessChannelList> {
					::subxt::tx::Payload::new_static(
						"Ics20Fee",
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
						"Ics20Fee",
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
			}
		}
		pub type Event = runtime_types::pallet_ibc::ics20_fee::pallet::Event;
		pub mod events {
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
			pub struct IbcTransferFeeCollected {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for IbcTransferFeeCollected {
				const PALLET: &'static str = "Ics20Fee";
				const EVENT: &'static str = "IbcTransferFeeCollected";
			}
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
				const PALLET: &'static str = "Ics20Fee";
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
							129u8, 223u8, 79u8, 233u8, 108u8, 171u8, 157u8, 84u8, 38u8, 149u8,
							146u8, 33u8, 178u8, 49u8, 125u8, 207u8, 11u8, 191u8, 152u8, 53u8,
							223u8, 241u8, 199u8, 102u8, 198u8, 0u8, 71u8, 166u8, 10u8, 211u8, 1u8,
							13u8,
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
						"Ics20Fee",
						"FeeLessChannelIds",
						Vec::new(),
						[
							190u8, 190u8, 60u8, 41u8, 38u8, 36u8, 108u8, 181u8, 192u8, 34u8, 88u8,
							107u8, 188u8, 195u8, 107u8, 232u8, 197u8, 153u8, 16u8, 234u8, 161u8,
							255u8, 37u8, 78u8, 4u8, 7u8, 40u8, 52u8, 9u8, 110u8, 150u8, 216u8,
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
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
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
							139u8, 109u8, 228u8, 151u8, 252u8, 32u8, 130u8, 69u8, 112u8, 154u8,
							174u8, 45u8, 83u8, 245u8, 51u8, 132u8, 173u8, 5u8, 186u8, 24u8, 243u8,
							9u8, 12u8, 214u8, 80u8, 74u8, 69u8, 189u8, 30u8, 94u8, 22u8, 39u8,
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
			pub mod ibc {
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
				pub struct MemoMessage;
			}
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
				#[codec(index = 6)]
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
				#[codec(index = 58)]
				GovernanceRegistry(runtime_types::pallet_governance_registry::pallet::Call),
				#[codec(index = 59)]
				AssetsRegistry(runtime_types::pallet_assets_registry::pallet::Call),
				#[codec(index = 100)]
				CallFilter(runtime_types::pallet_call_filter::pallet::Call),
				#[codec(index = 190)]
				Ibc(runtime_types::pallet_ibc::pallet::Call),
				#[codec(index = 191)]
				Ics20Fee(runtime_types::pallet_ibc::ics20_fee::pallet::Call),
			}
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
				#[codec(index = 58)]
				GovernanceRegistry(runtime_types::pallet_governance_registry::pallet::Event),
				#[codec(index = 59)]
				AssetsRegistry(runtime_types::pallet_assets_registry::pallet::Event),
				#[codec(index = 100)]
				CallFilter(runtime_types::pallet_call_filter::pallet::Event),
				#[codec(index = 190)]
				Ibc(runtime_types::pallet_ibc::pallet::Event),
				#[codec(index = 191)]
				Ics20Fee(runtime_types::pallet_ibc::ics20_fee::pallet::Event),
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
			pub mod governance {
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
				pub enum SignedRawOrigin<_0> {
					#[codec(index = 0)]
					Root,
					#[codec(index = 1)]
					Signed(_0),
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
						protocol_id: [::core::primitive::u8; 8usize],
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
		pub mod pallet_governance_registry {
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
						asset_id: runtime_types::primitives::currency::CurrencyId,
						value: ::subxt::utils::AccountId32,
					},
					#[codec(index = 1)]
					grant_root { asset_id: runtime_types::primitives::currency::CurrencyId },
					#[codec(index = 2)]
					remove { asset_id: runtime_types::primitives::currency::CurrencyId },
				}
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
					NoneError,
				}
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
					Set {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						value: ::subxt::utils::AccountId32,
					},
					#[codec(index = 1)]
					GrantRoot { asset_id: runtime_types::primitives::currency::CurrencyId },
					#[codec(index = 2)]
					Remove { asset_id: runtime_types::primitives::currency::CurrencyId },
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
						IbcTransferFeeCollected { amount: ::core::primitive::u128 },
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
						memo: ::core::option::Option<
							runtime_types::composable_runtime::ibc::MemoMessage,
						>,
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
				}
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
