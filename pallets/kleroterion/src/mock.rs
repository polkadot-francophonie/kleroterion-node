use crate as pallet_kleroterion;
use crate::Error;

use frame_support::{
	parameter_types,
	traits::{OnFinalize, OnInitialize},
};	
use frame_system as system;
use pallet_timestamp;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

pub const UX_TS_20300101: u64 = 1893452400;
pub const INIT_TIMESTAMP: u64 = 1632873600; //29/09/2021
pub const UX_TS_20100101: u64 = 1262300400;
pub const BLOCK_TIME: u64 = 1000;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type TestError = Error<Test>;
pub type TestEvent = Event;

	
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		Kleroterion: pallet_kleroterion::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ();
    type WeightInfo = ();
}

impl pallet_kleroterion::Config for Test {
	type Event = TestEvent;
	type TimeProvider = pallet_timestamp::Pallet<Test>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

/// Run until a particular block.
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		// let time_now: u64 = Timestamp::now();
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
 		} 
		else {
			Timestamp::set_timestamp(INIT_TIMESTAMP);
		}
		Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
		// let time_now: u64 = Timestamp::now();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
	}
}