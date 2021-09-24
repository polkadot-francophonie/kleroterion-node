//! Benchmarking setup for pallet-kleroterion
#[cfg(feature = "runtime-benchmarks")]

use super::*;
const UX_TS_20300101: u64 = 1893452400;


#[allow(unused)]
use crate::Pallet as Kleroterion;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec;


benchmarks! {
	// Trigger 100 Open Jury Calls
	bench_open_jury_call_100  {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let tribes = vec![b"Tribe1".to_vec(), b"Tribe2".to_vec()];
	}: 	open_jury_call(RawOrigin::Signed(caller), tribes.clone(), 1, UX_TS_20300101)
}

impl_benchmark_test_suite!(Kleroterion, crate::mock::new_test_ext(), crate::mock::Test);
