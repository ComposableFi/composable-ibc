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

use crate::{self as pallet_ibc, routing::ModuleRouter};
use cumulus_primitives_core::ParaId;
use frame_support::{
	pallet_prelude::ConstU32,
	parameter_types,
	traits::{
		fungibles::{metadata::Mutate, Create, InspectMetadata},
		ConstU64, Everything,
	},
};
use frame_system as system;
use ibc_primitives::IbcAccount;
use light_client_common::RelayChain;
use orml_traits::parameter_type_with_key;
use sp_core::{
	offchain::{testing::TestOffchainExt, OffchainDbExt, OffchainWorkerExt},
	H256,
};
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentityLookup},
	MultiSignature,
};
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
		IbcPing: pallet_ibc_ping,
		Ibc: pallet_ibc,
	}
);

parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub const SS58Prefix: u8 = 49;
	pub const ExpectedBlockTime: u64 = 1000;
	pub const ExistentialDeposit: u128 = 10000;
}

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u32;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = orml_tokens::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<2>;
}

impl parachain_info::Config for Test {}

impl pallet_ibc_ping::Config for Test {
	type Event = Event;

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
	type Event = Event;
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
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type OnKilledTokenAccount = ();
	type OnNewTokenAccount = ();
}

impl Config for Test {
	type TimeProvider = Timestamp;
	type Event = Event;
	type NativeCurrency = Balances;
	type Balance = Balance;
	type AssetId = AssetId;
	type NativeAssetId = NativeAssetId;
	type IbcDenomToAssetIdConversion = ();
	const PALLET_PREFIX: &'static [u8] = b"ibc/";
	const LIGHT_CLIENT_PROTOCOL: crate::LightClientProtocol = LightClientProtocol::Beefy;
	type AccountIdConversion = IbcAccount<AccountId>;
	type Fungibles = Assets;
	type ExpectedBlockTime = ExpectedBlockTime;
	type Router = Router;
	type MinimumConnectionDelay = MinimumConnectionDelay;
	type ParaId = ParachainId;
	type RelayChain = RelayChainId;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type SentryOrigin = EnsureRoot<AccountId>;
	type SpamProtectionDeposit = SpamProtectionDeposit;
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
	ext
}

impl<T: Config> DenomToAssetId<T> for ()
where
	T::AssetId: From<u128>,
{
	type Error = ();
	fn from_denom_to_asset_id(_denom: &String) -> Result<T::AssetId, Self::Error> {
		if <<Test as Config>::Fungibles as InspectMetadata<AccountId>>::decimals(&2u128) == 0 {
			<<Test as Config>::Fungibles as Create<AccountId>>::create(
				2u128.into(),
				AccountId::new([0; 32]),
				true,
				1000u128.into(),
			)
			.unwrap();

			<<Test as Config>::Fungibles as Mutate<AccountId>>::set(
				2u128.into(),
				&AccountId::new([0; 32]),
				vec![0; 32],
				vec![0; 32],
				8,
			)
			.unwrap();
		};
		Ok(2u128.into())
	}

	fn from_asset_id_to_denom(_id: T::AssetId) -> Option<String> {
		Some("PICA".to_string())
	}

	fn ibc_assets(
		_start_key: Option<T::AssetId>,
		_offset: Option<u32>,
		_limit: u64,
	) -> (Vec<Vec<u8>>, u64, Option<T::AssetId>) {
		(vec![], 0, None)
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Router {
	ibc_ping: pallet_ibc_ping::IbcModule<Test>,
}

impl ModuleRouter for Router {
	fn get_route_mut(
		&mut self,
		module_id: &impl core::borrow::Borrow<ibc::core::ics26_routing::context::ModuleId>,
	) -> Option<&mut dyn ibc::core::ics26_routing::context::Module> {
		match module_id.borrow().to_string().as_str() {
			pallet_ibc_ping::MODULE_ID => Some(&mut self.ibc_ping),
			&_ => None,
		}
	}

	fn has_route(
		module_id: &impl core::borrow::Borrow<ibc::core::ics26_routing::context::ModuleId>,
	) -> bool {
		matches!(module_id.borrow().to_string().as_str(), pallet_ibc_ping::MODULE_ID,)
	}

	fn lookup_module_by_port(
		port_id: &ibc::core::ics24_host::identifier::PortId,
	) -> Option<ibc::core::ics26_routing::context::ModuleId> {
		match port_id.as_str() {
			pallet_ibc_ping::PORT_ID =>
				ibc::core::ics26_routing::context::ModuleId::from_str(pallet_ibc_ping::MODULE_ID)
					.ok(),
			_ => None,
		}
	}
}
