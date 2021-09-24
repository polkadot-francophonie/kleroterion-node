//! Benchmarking setup for pallet-kleroterion

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	fn bench_open_jury_call_100(b: &mut Bencher) {
		let tribes = vec![b"Tribe1".to_vec(), b"Tribe2".to_vec()];
		Kleroterion::open_jury_call(Origin::signed(1), tribes.clone(), 1, UX_TS_20300101)
    });
}

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
