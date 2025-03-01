#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;

#[benchmarks]
mod benchmarks {
	use super::*;
	#[cfg(test)]
	use crate::pallet::Pallet as Derivative;
	use frame_system::RawOrigin;

	#[benchmark]
	fn increment() {
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		increment(RawOrigin::Signed(caller));
	}

	impl_benchmark_test_suite!(Derivative, crate::mock::new_test_ext(), crate::mock::Test);
}

