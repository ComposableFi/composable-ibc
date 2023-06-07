use crate::{self as pallet_ibc, ics20_fee::FlatFeeConverter, routing::ModuleRouter};
use cumulus_primitives_core::ParaId;
use frame_support::{
	pallet_prelude::ConstU32,
	parameter_types,
	traits::{
		fungibles::{metadata::Mutate, Create, InspectMetadata},
		AsEnsureOriginWithArg, ConstU64, Everything,
	},
};
use frame_system as system;
use frame_system::EnsureSigned;
use ibc_primitives::{runtime_interface::ss58_to_account_id_32, IbcAccount};
use light_client_common::RelayChain;
use orml_traits::parameter_type_with_key;
use pallet_membership::Instance2;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{
	offchain::{testing::TestOffchainExt, OffchainDbExt, OffchainWorkerExt},
	H256,
};
use sp_keystore::{testing::KeyStore, KeystoreExt};
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentityLookup},
	MultiSignature, Perbill,
};
use std::sync::Arc;
use system::EnsureRoot;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Header = generic::Header<u32, BlakeTwo256>;
use sp_runtime::traits::{IdentifyAccount, Verify};

pub type AssetId = u128;
pub type Amount = i128;
pub type Balance = u128;
pub type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
use super::*;
use crate::light_clients::{AnyClientMessage, AnyConsensusState};
use ibc::mock::{client_state::MockConsensusState, header::MockClientMessage, host::MockHostBlock};

impl From<MockHostBlock> for AnyClientMessage {
	fn from(block: MockHostBlock) -> Self {
		let MockHostBlock::Mock(header) = block;
		AnyClientMessage::Mock(MockClientMessage::Header(header))
	}
}

impl From<MockHostBlock> for AnyConsensusState {
	fn from(block: MockHostBlock) -> Self {
		let MockHostBlock::Mock(header) = block;
		AnyConsensusState::Mock(MockConsensusState::new(header))
	}
}

impl pallet_aura::Config for Test {
	type AuthorityId = AuraId;
	type MaxAuthorities = MaxAuthorities;
	type DisabledValidators = ();
}

impl pallet_membership::Config<Instance2> for Test {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type SwapOrigin = EnsureRoot<AccountId>;
	type ResetOrigin = EnsureRoot<AccountId>;
	type PrimeOrigin = EnsureRoot<AccountId>;
	type MembershipInitialized = ();
	type MembershipChanged = ();
	type MaxMembers = MaxAuthorities;
	type WeightInfo = ();
}

impl balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub const SS58Prefix: u8 = 49;
	pub const ExpectedBlockTime: u64 = 1000;
	pub const ExistentialDeposit: u128 = 10000;
	pub const MaxAuthorities: u32 = 100_000;
}

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u32;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<2>;
}

impl parachain_info::Config for Test {}

impl pallet_ibc_ping::Config for Test {
	type RuntimeEvent = RuntimeEvent;

	type IbcHandler = Ibc;
}

parameter_types! {
	pub const NativeAssetId: u128 = 1;
	pub const StringLimit: u32 = 32;
	pub const MinimumConnectionDelay: u64 = 1;
}

pub type Balances = orml_tokens::CurrencyAdapter<Test, NativeAssetId>;

impl pallet_assets::Config for Test {
	type AssetId = AssetId;
	type Balance = Balance;
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = ();
	type AssetAccountDeposit = ();
	type MetadataDepositBase = ();
	type MetadataDepositPerByte = ();
	type ApprovalDeposit = ();
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type RemoveItemsLimit = ConstU32<128>;
	type AssetIdParameter = Self::AssetId;

	type CallbackHandle = ();
}

parameter_types! {
	pub const MaxLocks: u32 = 256;
	pub static ParachainId: ParaId = ParaId::from(2087);
	pub static RelayChainId: RelayChain = RelayChain::Rococo;
	pub const SpamProtectionDeposit: u128 = 0;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_a: AssetId| -> Balance {
		0
	};
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type CurrencyHooks = ();
}

parameter_types! {
	pub const GRANDPA: LightClientProtocol = LightClientProtocol::Grandpa;
	pub const IbcTriePrefix : &'static [u8] = b"ibc/";
	pub const ServiceCharge: Perbill = Perbill::from_percent(1);
	pub const PalletId: frame_support::PalletId = frame_support::PalletId(*b"ics20fee");
	pub const FlatFeeAssetId: AssetId = 130;
	pub const FlatFeeAmount: AssetId = 10_000_000;
	pub FeeAccount: <Test as Config>::AccountIdConversion = create_alice_key();
	pub const CleanUpPacketsPeriod: u32 = 10;
}

fn create_alice_key() -> <Test as Config>::AccountIdConversion {
	let alice = "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL";
	let account_id_32 = ss58_to_account_id_32(alice).unwrap().into();
	IbcAccount(account_id_32)
}

impl Config for Test {
	type TimeProvider = Timestamp;
	type RuntimeEvent = RuntimeEvent;
	type NativeCurrency = Balances;
	type Balance = Balance;
	type AssetId = AssetId;
	type NativeAssetId = NativeAssetId;
	type IbcDenomToAssetIdConversion = ();
	type PalletPrefix = IbcTriePrefix;
	type LightClientProtocol = GRANDPA;
	type AccountIdConversion = IbcAccount<AccountId>;
	type Fungibles = Assets;
	type ExpectedBlockTime = ExpectedBlockTime;
	type Router = Router;
	type MinimumConnectionDelay = MinimumConnectionDelay;
	type ParaId = ParachainId;
	type RelayChain = RelayChainId;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type FreezeOrigin = EnsureRoot<AccountId>;
	type SpamProtectionDeposit = SpamProtectionDeposit;
	type IbcAccountId = Self::AccountId;
	type TransferOrigin = EnsureSigned<Self::IbcAccountId>;
	type RelayerOrigin = EnsureSigned<Self::AccountId>;
	type HandleMemo = ();
	type MemoMessage = MemoMessage;
	type IsReceiveEnabled = sp_core::ConstBool<true>;
	type IsSendEnabled = sp_core::ConstBool<true>;
	type Ics20RateLimiter = Everything;
	type FeeAccount = FeeAccount;
	type CleanUpPacketsPeriod = CleanUpPacketsPeriod;
	type ServiceChargeOut = ServiceCharge;
	type FlatFeeConverter = FlatFeeConverterDummy<Test>;
	type FlatFeeAssetId = FlatFeeAssetId;
	type FlatFeeAmount = FlatFeeAmount;
}
#[derive(Debug, Clone)]
pub struct FlatFeeConverterDummy<T: Config>(PhantomData<T>);
impl<T: Config> FlatFeeConverter for FlatFeeConverterDummy<T> {
	type AssetId = u128;
	type Balance = u128;
	fn get_flat_fee(
		asset_id: Self::AssetId,
		_fee_asset_id: Self::AssetId,
		_fee_asset_amount: Self::Balance,
	) -> Option<u128> {
		if asset_id == 3 {
			return Some(1000)
		}
		None
	}
}
impl crate::ics20_fee::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ServiceChargeIn = ServiceCharge;
	type PalletId = PalletId;
}

#[derive(
	Debug, codec::Encode, Clone, codec::Decode, PartialEq, Eq, scale_info::TypeInfo, Default,
)]
pub struct MemoMessage;

impl ToString for MemoMessage {
	fn to_string(&self) -> String {
		Default::default()
	}
}

impl FromStr for MemoMessage {
	type Err = ();

	fn from_str(_s: &str) -> Result<Self, Self::Err> {
		Ok(Default::default())
	}
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

fn register_offchain_ext(ext: &mut sp_io::TestExternalities) {
	let (offchain, _offchain_state) = TestOffchainExt::with_offchain_db(ext.offchain_db());
	ext.register_extension(OffchainDbExt::new(offchain.clone()));
	ext.register_extension(OffchainWorkerExt::new(offchain));
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities =
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	register_offchain_ext(&mut ext);
	ext.register_extension(KeystoreExt(Arc::new(KeyStore::new())));
	ext
}

impl<T: Config> DenomToAssetId<T> for ()
where
	T::AssetId: From<u128>,
{
	type Error = ();
	fn from_denom_to_asset_id(denom: &String) -> Result<T::AssetId, Self::Error> {
		let mut id = 2u128;
		if denom == "PICA" || denom == "1" {
			id = 1;
		}
		if denom.contains("FLATFEE") {
			id = 3;
		}
		if <<Test as Config>::Fungibles as InspectMetadata<AccountId>>::decimals(&id) == 0 {
			<<Test as Config>::Fungibles as Create<AccountId>>::create(
				id.into(),
				AccountId::new([0; 32]),
				true,
				1000u128.into(),
			)
			.unwrap();

			<<Test as Config>::Fungibles as Mutate<AccountId>>::set(
				id.into(),
				&AccountId::new([0; 32]),
				vec![0; 32],
				vec![0; 32],
				8,
			)
			.unwrap();
		};
		Ok(id.into())
	}

	fn from_asset_id_to_denom(id: T::AssetId) -> Option<String> {
		if id == 1u128.into() {
			return Some("PICA".to_string())
		}
		if id == 3u128.into() {
			return Some("PICAFLATFEE".to_string())
		}
		Some("PICA".to_string())
	}

	fn ibc_assets(
		_start_key: Option<Either<T::AssetId, u32>>,
		_limit: u64,
	) -> IbcAssets<T::AssetId> {
		IbcAssets { denoms: vec![], total_count: 0, next_id: None }
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Router {
	ibc_ping: pallet_ibc_ping::IbcModule<Test>,
	ics20: crate::ics20::memo::Memo<
		Test,
		crate::ics20_fee::Ics20ServiceCharge<Test, crate::ics20::IbcModule<Test>>,
	>,
}

impl ModuleRouter for Router {
	fn get_route_mut(
		&mut self,
		module_id: &ibc::core::ics26_routing::context::ModuleId,
	) -> Option<&mut dyn ibc::core::ics26_routing::context::Module> {
		match module_id.as_ref() {
			pallet_ibc_ping::MODULE_ID => Some(&mut self.ibc_ping),
			ibc::applications::transfer::MODULE_ID_STR => Some(&mut self.ics20),
			&_ => None,
		}
	}

	fn has_route(module_id: &ibc::core::ics26_routing::context::ModuleId) -> bool {
		matches!(
			module_id.as_ref(),
			pallet_ibc_ping::MODULE_ID | ibc::applications::transfer::MODULE_ID_STR
		)
	}

	fn lookup_module_by_port(
		port_id: &ibc::core::ics24_host::identifier::PortId,
	) -> Option<ibc::core::ics26_routing::context::ModuleId> {
		match port_id.as_str() {
			pallet_ibc_ping::PORT_ID =>
				ibc::core::ics26_routing::context::ModuleId::from_str(pallet_ibc_ping::MODULE_ID)
					.ok(),
			ibc::applications::transfer::PORT_ID_STR =>
				ibc::core::ics26_routing::context::ModuleId::from_str(
					ibc::applications::transfer::MODULE_ID_STR,
				)
				.ok(),
			_ => None,
		}
	}
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Timestamp: pallet_timestamp,
		ParachainInfo: parachain_info,
		Tokens: orml_tokens,
		Assets: pallet_assets,
		PalletBalances: balances,
		IbcPing: pallet_ibc_ping,
		Ics20Fee: crate::ics20_fee,
		Ibc: pallet_ibc,
		Aura: pallet_aura,
		Membership: pallet_membership::<Instance2>,
	}
);
