// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

extern crate alloc;

use alloc::string::String;
use asset_registry::{AssetMetadata, DefaultAssetMetadata};

mod weights;
pub mod xcm_config;

use codec::Encode;
use core::str::FromStr;
use cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
use ibc::core::{
	ics24_host::identifier::PortId,
	ics26_routing::context::{Module, ModuleId},
};
use orml_traits::asset_registry::AssetProcessor;
use pallet_ibc::{light_client_common::RelayChain, LightClientProtocol};
use smallvec::smallvec;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, Get, IdentifyAccount, Verify},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, DispatchError, Either,
	Either::*,
	MultiSignature,
};

use frame_support::{
	construct_runtime,
	dispatch::DispatchClass,
	parameter_types,
	traits::{fungibles::InspectMetadata, AsEnsureOriginWithArg, Everything},
	weights::{
		constants::WEIGHT_REF_TIME_PER_SECOND, ConstantMultiplier, Weight, WeightToFeeCoefficient,
		WeightToFeeCoefficients, WeightToFeePolynomial,
	},
	PalletId,
};
use frame_system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot, EnsureSigned,
};
use pallet_ibc::{DenomToAssetId, IbcAssetIds, IbcAssets, IbcDenoms};
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
pub use sp_runtime::{MultiAddress, Perbill, Permill};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
use xcm_config::{XcmConfig, XcmOriginToTransactDispatchOrigin};

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

// Polkadot imports
use polkadot_runtime_common::{BlockHashCount, SlowAdjustingFeeUpdate};
use sp_runtime::traits::AccountIdConversion;

use weights::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight};

// XCM Imports
use pallet_ibc::routing::ModuleRouter;
use xcm::latest::prelude::BodyId;
use xcm_executor::XcmExecutor;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

pub type AssetId = u128;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
/// node's balance type.
///
/// This should typically create a mapping between the following ranges:
///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
///   - `[Balance::min, Balance::max]`
///
/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
///   - Setting it to `0` will essentially disable the weight fee.
///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		// in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNIT:
		// in our template, we map to 1/10 of that, or 1/10 MILLIUNIT
		let p = MILLIUNIT / 10;
		let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time() as u128);
		smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;
	use sp_runtime::{generic, traits::BlakeTwo256};

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("centauri"),
	impl_name: create_runtime_str!("centauri"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLIUNIT: Balance = 1_000_000_000;
pub const MICROUNIT: Balance = 1_000_000;

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We allow for 0.5 of a second of compute with a 12 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight =
	Weight::from_ref_time(WEIGHT_REF_TIME_PER_SECOND.saturating_div(2))
		.set_proof_size(cumulus_primitives_core::relay_chain::v2::MAX_POV_SIZE as u64);

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;

	// This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
	//  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
	// `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
	// the lazy contract deletion.
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub const SS58Prefix: u16 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// Runtime version.
	type Version = Version;
	/// Converts a module to an index of this module in the runtime.
	type PalletInfo = PalletInfo;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = Everything;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The action to take on a Runtime Upgrade
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const UncleGenerations: u32 = 0;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = (CollatorSelection,);
}

parameter_types! {
	pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	/// Relay Chain `TransactionByteFee` / 10
	pub const TransactionByteFee: Balance = 10 * MICROUNIT;
	pub const OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = WeightToFee;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.div(4);
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnSystemEvent = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type OutboundXcmpMessageSource = XcmpQueue;
	type DmpMessageHandler = DmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
}

impl parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = ();
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = ();
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

parameter_types! {
	pub const Period: u32 = 6 * HOURS;
	pub const Offset: u32 = 0;
	pub const MaxAuthorities: u32 = 100_000;
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager = CollatorSelection;
	// Essentially just Aura, but lets be pedantic.
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = ();
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
	pub const PotId: PalletId = PalletId(*b"PotStake");
	pub const MaxCandidates: u32 = 1000;
	pub const MinCandidates: u32 = 5;
	pub const SessionLength: BlockNumber = 6 * HOURS;
	pub const MaxInvulnerables: u32 = 100;
	pub const ExecutiveBody: BodyId = BodyId::Executive;
}

// We allow root only to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = EnsureRoot<AccountId>;

impl pallet_collator_selection::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UpdateOrigin = CollatorSelectionUpdateOrigin;
	type PotId = PotId;
	type MaxCandidates = MaxCandidates;
	type MinCandidates = MinCandidates;
	type MaxInvulnerables = MaxInvulnerables;
	// should be a multiple of session or things will get inconsistent
	type KickThreshold = Period;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ValidatorRegistration = Session;
	type WeightInfo = ();
}

impl pallet_ibc_ping::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type IbcHandler = Ibc;
}

impl asset_registry::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = AssetId;
	type AssetProcessor = asset_registry::SequentialId<Self>;
	type AuthorityOrigin = AsEnsureOriginWithArg<EnsureRoot<AccountId>>;
	type Balance = Balance;
	type CustomMetadata = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const StringLimit: u32 = 150;
}

impl pallet_assets::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = AssetId;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = ();
	type AssetAccountDeposit = ();
	type MetadataDepositBase = ();
	type MetadataDepositPerByte = ();
	type ApprovalDeposit = ();
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type RemoveItemsLimit = sp_core::ConstU32<128>;
	type AssetIdParameter = Self::AssetId;
}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}

parameter_types! {
	pub const ExpectedBlockTime: u64 = MILLISECS_PER_BLOCK as u64;
	pub const RelayChainId: RelayChain = RelayChain::Rococo;
	pub const SpamProtectionDeposit: Balance = 1_000_000_000_000;
	pub const NativeAssetId: AssetId = 1;
	pub const MinimumConnectionDelay: u64 = 0; // well we don't need the security tbh.
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Router {
	pallet_ibc_ping: pallet_ibc_ping::IbcModule<Runtime>,
}

impl ModuleRouter for Router {
	fn get_route_mut(&mut self, module_id: &ModuleId) -> Option<&mut dyn Module> {
		match module_id.as_ref() {
			pallet_ibc_ping::MODULE_ID => Some(&mut self.pallet_ibc_ping),
			_ => None,
		}
	}

	fn has_route(module_id: &ModuleId) -> bool {
		matches!(module_id.as_ref(), pallet_ibc_ping::MODULE_ID)
	}

	fn lookup_module_by_port(port_id: &PortId) -> Option<ModuleId> {
		match port_id.as_str() {
			pallet_ibc_ping::PORT_ID => ModuleId::from_str(pallet_ibc_ping::MODULE_ID).ok(),
			_ => None,
		}
	}
}

pub struct IbcDenomToAssetIdConversion;

// generate new asset id
fn generate_asset_id() -> Result<AssetId, DispatchError> {
	let (asset_id, ..) = <asset_registry::SequentialId<Runtime> as AssetProcessor<
		AssetId,
		DefaultAssetMetadata<Runtime>,
	>>::pre_register(
		None,
		// Metadata is not useful to this call so we can use default values
		AssetMetadata {
			decimals: Default::default(),
			name: Default::default(),
			symbol: Default::default(),
			existential_deposit: Default::default(),
			location: None,
			additional: (),
		},
	)
	.map_err(|_| DispatchError::Other("Failed to generate asset id"))?;
	let asset_id = if asset_id == 1 {
		let (asset_id, ..) = <asset_registry::SequentialId<Runtime> as AssetProcessor<
			AssetId,
			DefaultAssetMetadata<Runtime>,
		>>::pre_register(
			None,
			// Metadata is not useful to this call so we can use default values
			AssetMetadata {
				decimals: Default::default(),
				name: Default::default(),
				symbol: Default::default(),
				existential_deposit: Default::default(),
				location: None,
				additional: (),
			},
		)
		.map_err(|_| DispatchError::Other("Failed to generate asset id"))?;
		asset_id
	} else {
		asset_id
	};

	Ok(asset_id)
}

impl DenomToAssetId<Runtime> for IbcDenomToAssetIdConversion {
	type Error = DispatchError;

	fn from_denom_to_asset_id(denom: &String) -> Result<AssetId, Self::Error> {
		use frame_support::traits::fungibles::{metadata::Mutate, Create};

		let denom_bytes = denom.as_bytes().to_vec();
		if let Some(id) = IbcDenoms::<Runtime>::get(&denom_bytes) {
			return Ok(id)
		}

		let pallet_id: AccountId = PalletId(*b"pall-ibc").into_account_truncating();

		let symbol = denom
			.split("/")
			.last()
			.ok_or_else(|| DispatchError::Other("denom missing a name"))?
			.as_bytes()
			.to_vec();
		let asset_id = generate_asset_id()?;

		IbcDenoms::<Runtime>::insert(denom_bytes.clone(), asset_id);
		IbcAssetIds::<Runtime>::insert(asset_id, denom_bytes.clone());

		<pallet_assets::Pallet<Runtime> as Create<AccountId>>::create(
			asset_id,
			pallet_id.clone(),
			true,
			1,
		)?;

		<pallet_assets::Pallet<Runtime> as Mutate<AccountId>>::set(
			asset_id,
			&pallet_id,
			denom_bytes,
			symbol,
			12,
		)?;

		Ok(asset_id)
	}

	fn from_asset_id_to_denom(id: AssetId) -> Option<String> {
		let name = <pallet_assets::Pallet<Runtime> as InspectMetadata<AccountId>>::name(&id);
		String::from_utf8(name).ok()
	}

	fn ibc_assets(start_key: Option<Either<AssetId, u32>>, limit: u64) -> IbcAssets<AssetId> {
		let mut iterator = match start_key {
			None => IbcAssetIds::<Runtime>::iter().skip(0),
			Some(Left(asset_id)) => {
				let raw_key = asset_id.encode();
				IbcAssetIds::<Runtime>::iter_from(raw_key).skip(0)
			},
			Some(Right(offset)) => IbcAssetIds::<Runtime>::iter().skip(offset as usize),
		};

		let denoms = iterator.by_ref().take(limit as usize).map(|(_, denom)| denom).collect();
		let maybe_currency_id = iterator.next().map(|(id, ..)| id);
		IbcAssets {
			denoms,
			total_count: IbcAssetIds::<Runtime>::count() as u64,
			next_id: maybe_currency_id,
		}
	}
}

#[derive(
	Debug, codec::Encode, Clone, codec::Decode, PartialEq, Eq, scale_info::TypeInfo, Default,
)]
pub struct MemoMessage;

impl alloc::string::ToString for MemoMessage {
	fn to_string(&self) -> String {
		Default::default()
	}
}

impl core::str::FromStr for MemoMessage {
	type Err = ();

	fn from_str(_s: &str) -> Result<Self, Self::Err> {
		Ok(Default::default())
	}
}

parameter_types! {
	pub const GRANDPA: LightClientProtocol = LightClientProtocol::Grandpa;
	pub const IbcTriePrefix : &'static [u8] = b"ibc/";
}

impl pallet_ibc::Config for Runtime {
	type TimeProvider = Timestamp;
	type RuntimeEvent = RuntimeEvent;
	type NativeCurrency = Balances;
	type Balance = Balance;
	type AssetId = AssetId;
	type NativeAssetId = NativeAssetId;
	type IbcDenomToAssetIdConversion = IbcDenomToAssetIdConversion;
	type AccountIdConversion = ibc_primitives::IbcAccount<AccountId>;
	type Fungibles = Assets;
	type ExpectedBlockTime = ExpectedBlockTime;
	type Router = Router;
	type MinimumConnectionDelay = MinimumConnectionDelay;
	type ParaId = parachain_info::Pallet<Runtime>;
	type RelayChain = RelayChainId;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type FreezeOrigin = EnsureRoot<AccountId>;
	type SpamProtectionDeposit = SpamProtectionDeposit;
	type TransferOrigin = EnsureSigned<Self::IbcAccountId>;
	type RelayerOrigin = EnsureSigned<Self::AccountId>;
	type MemoMessage = MemoMessage;
	type HandleMemo = ();
	type PalletPrefix = IbcTriePrefix;
	type LightClientProtocol = GRANDPA;
	type IbcAccountId = Self::AccountId;
	type Ics20RateLimiter = Everything;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		// System support stuff.
		System: frame_system = 0,
		Timestamp: pallet_timestamp = 1,
		ParachainSystem: cumulus_pallet_parachain_system = 2,
		ParachainInfo: parachain_info = 3,

		// Monetary stuff.
		Balances: pallet_balances = 10,
		TransactionPayment: pallet_transaction_payment = 11,

		// Collator support. The order of these 4 are important and shall not change.
		Authorship: pallet_authorship = 20,
		CollatorSelection: pallet_collator_selection = 21,
		Session: pallet_session = 22,
		Aura: pallet_aura = 23,
		AuraExt: cumulus_pallet_aura_ext = 24,

		// XCM helpers.
		XcmpQueue: cumulus_pallet_xcmp_queue = 30,
		PolkadotXcm: pallet_xcm = 31,
		CumulusXcm: cumulus_pallet_xcm = 32,
		DmpQueue: cumulus_pallet_dmp_queue = 33,

		Sudo: pallet_sudo = 35,
		IbcPing: pallet_ibc_ping = 36,
		Assets: pallet_assets = 37,
		AssetRegistry: asset_registry = 38,
		// pallet-ibc, should be the last module in your runtime
		Ibc: pallet_ibc = 255,
	}
);

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[frame_system, SystemBench::<Runtime>]
		[pallet_balances, Balances]
		[pallet_session, SessionBench::<Runtime>]
		[pallet_timestamp, Timestamp]
		[pallet_collator_selection, CollatorSelection]
		[cumulus_pallet_xcmp_queue, XcmpQueue]
		[pallet_ibc, Ibc]
	);
}

impl_runtime_apis! {
	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
		fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
			ParachainSystem::collect_collation_info(header)
		}
	}

	impl ibc_runtime_api::IbcRuntimeApi<Block, AssetId> for Runtime {
		fn para_id() -> u32 {
			<Runtime as cumulus_pallet_parachain_system::Config>::SelfParaId::get().into()
		}

		fn child_trie_key() -> Vec<u8> {
			<Runtime as pallet_ibc::Config>::PalletPrefix::get().to_vec()
		}

		fn query_balance_with_address(addr: Vec<u8>, asset_id: AssetId) -> Option<u128> {
			Ibc::query_balance_with_address(addr, asset_id).ok()
		}

		fn query_send_packet_info(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<ibc_primitives::PacketInfo>> {
			Ibc::get_send_packet_info(channel_id, port_id, seqs).ok()
		}

		fn query_recv_packet_info(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<ibc_primitives::PacketInfo>> {
			Ibc::get_recv_packet_info(channel_id, port_id, seqs).ok()
		}

		fn client_update_time_and_height(client_id: Vec<u8>, revision_number: u64, revision_height: u64) -> Option<(u64, u64)>{
			Ibc::client_update_time_and_height(client_id, revision_number, revision_height).ok()
		}

		fn client_state(client_id: Vec<u8>) -> Option<ibc_primitives::QueryClientStateResponse> {
			Ibc::client(client_id).ok()
		}

		fn client_consensus_state(client_id: Vec<u8>, revision_number: u64, revision_height: u64, latest_cs: bool) -> Option<ibc_primitives::QueryConsensusStateResponse> {
			Ibc::consensus_state(client_id, revision_number, revision_height, latest_cs).ok()
		}

		fn clients() -> Option<Vec<(Vec<u8>, Vec<u8>)>> {
			Some(Ibc::clients())
		}

		fn connection(connection_id: Vec<u8>) -> Option<ibc_primitives::QueryConnectionResponse>{
			Ibc::connection(connection_id).ok()
		}

		fn connections() -> Option<ibc_primitives::QueryConnectionsResponse> {
			Ibc::connections().ok()
		}

		fn connection_using_client(client_id: Vec<u8>) -> Option<Vec<ibc_primitives::IdentifiedConnection>>{
			Ibc::connection_using_client(client_id).ok()
		}

		fn connection_handshake(client_id: Vec<u8>, connection_id: Vec<u8>) -> Option<ibc_primitives::ConnectionHandshake> {
			Ibc::connection_handshake(client_id, connection_id).ok()
		}

		fn channel(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryChannelResponse> {
			Ibc::channel(channel_id, port_id).ok()
		}

		fn channel_client(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::IdentifiedClientState> {
			Ibc::channel_client(channel_id, port_id).ok()
		}

		fn connection_channels(connection_id: Vec<u8>) -> Option<ibc_primitives::QueryChannelsResponse> {
			Ibc::connection_channels(connection_id).ok()
		}

		fn channels() -> Option<ibc_primitives::QueryChannelsResponse> {
			Ibc::channels().ok()
		}

		fn packet_commitments(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryPacketCommitmentsResponse> {
			Ibc::packet_commitments(channel_id, port_id).ok()
		}

		fn packet_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryPacketAcknowledgementsResponse>{
			Ibc::packet_acknowledgements(channel_id, port_id).ok()
		}

		fn unreceived_packets(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<u64>> {
			Ibc::unreceived_packets(channel_id, port_id, seqs).ok()
		}

		fn unreceived_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<u64>> {
			Ibc::unreceived_acknowledgements(channel_id, port_id, seqs).ok()
		}

		fn next_seq_recv(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryNextSequenceReceiveResponse> {
			Ibc::next_seq_recv(channel_id, port_id).ok()
		}

		fn packet_commitment(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<ibc_primitives::QueryPacketCommitmentResponse> {
			Ibc::packet_commitment(channel_id, port_id, seq).ok()
		}

		fn packet_acknowledgement(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<ibc_primitives::QueryPacketAcknowledgementResponse> {
			Ibc::packet_acknowledgement(channel_id, port_id, seq).ok()
		}

		fn packet_receipt(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<ibc_primitives::QueryPacketReceiptResponse> {
			Ibc::packet_receipt(channel_id, port_id, seq).ok()
		}

		fn denom_trace(asset_id: AssetId) -> Option<ibc_primitives::QueryDenomTraceResponse> {
			Ibc::get_denom_trace(asset_id)
		}

		fn denom_traces(key: Option<AssetId>, offset: Option<u32>, limit: u64, count_total: bool) -> ibc_primitives::QueryDenomTracesResponse {
			let key = key.map(|k| Either::Left(k)).or_else(|| offset.map(|o| Either::Right(o)));
			Ibc::get_denom_traces(key, limit, count_total)
		}

		fn block_events(extrinsic_index: Option<u32>) -> Vec<Result<pallet_ibc::events::IbcEvent, pallet_ibc::errors::IbcError>> {
			let mut raw_events = frame_system::Pallet::<Self>::read_events_no_consensus().into_iter();
			if let Some(idx) = extrinsic_index {
				raw_events.find_map(|e| {
					let frame_system::EventRecord{ event, phase, ..} = *e;
					match (event, phase) {
						(RuntimeEvent::Ibc(pallet_ibc::Event::Events{ events }), frame_system::Phase::ApplyExtrinsic(index)) if index == idx => Some(events),
						_ => None
					}
				}).unwrap_or_default()
			}
			else {
				raw_events.filter_map(|e| {
					let frame_system::EventRecord{ event, ..} = *e;

					match event {
						RuntimeEvent::Ibc(pallet_ibc::Event::Events{ events }) => {
								Some(events)
							},
						_ => None
					}
				}).flatten().collect()
			}
		}
	}

	impl<RuntimeCall, AccountId> simnode_apis::CreateTransactionApi<Block, AccountId, RuntimeCall> for Runtime
		where
			RuntimeCall: codec::Codec,
			AccountId: codec::Codec + codec::EncodeLike<sp_runtime::AccountId32> + Into<sp_runtime::AccountId32> + Clone+ PartialEq + scale_info::TypeInfo + core::fmt::Debug,
	{
		fn create_transaction(call: RuntimeCall, signer: AccountId) -> Vec<u8> {
			use sp_runtime::{
				generic::Era, MultiSignature,
				traits::StaticLookup,
			};
			use sp_core::sr25519;
			let nonce = frame_system::Pallet::<Runtime>::account_nonce(signer.clone());
			let extra = (
				frame_system::CheckNonZeroSender::<Runtime>::new(),
				frame_system::CheckSpecVersion::<Runtime>::new(),
				frame_system::CheckTxVersion::<Runtime>::new(),
				frame_system::CheckGenesis::<Runtime>::new(),
				frame_system::CheckEra::<Runtime>::from(Era::Immortal),
				frame_system::CheckNonce::<Runtime>::from(nonce),
				frame_system::CheckWeight::<Runtime>::new(),
				pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(0),
			);
			let signature = MultiSignature::from(sr25519::Signature([0_u8;64]));
			let address = AccountIdLookup::unlookup(signer.into());
			let ext = generic::UncheckedExtrinsic::<Address, RuntimeCall, Signature, SignedExtra>::new_signed(
				call,
				address,
				signature,
				extra,
			);
			ext.encode()
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade() -> (Weight, Weight) {
			log::info!("try-runtime::on_runtime_upgrade parachain-template.");
			let weight = Executive::try_runtime_upgrade().unwrap();
			(weight, RuntimeBlockWeights::get().max_block)
		}

		fn execute_block_no_check(block: Block) -> Weight {
			Executive::execute_block_no_check(block)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use cumulus_pallet_session_benchmarking::Pallet as SessionBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();
			return (list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			impl frame_system_benchmarking::Config for Runtime {}

			use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
			impl cumulus_pallet_session_benchmarking::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
	fn check_inherents(
		block: &Block,
		relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
	) -> sp_inherents::CheckInherentsResult {
		let relay_chain_slot = relay_state_proof
			.read_slot()
			.expect("Could not read the relay chain slot from the proof");

		let inherent_data =
			cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
				relay_chain_slot,
				sp_std::time::Duration::from_secs(6),
			)
			.create_inherent_data()
			.expect("Could not create the timestamp inherent data");

		inherent_data.check_extrinsics(block)
	}
}

cumulus_pallet_parachain_system::register_validate_block! {
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
	CheckInherents = CheckInherents,
}
