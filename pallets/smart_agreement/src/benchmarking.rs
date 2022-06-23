//! Benchmarking setup for pallet-smart-agreement

use super::*;

use crate::info_types::AgreementType;
#[allow(unused)]
use crate::Pallet as SmartAgreement;
use frame_benchmarking::{
	benchmarks,
	impl_benchmark_test_suite,
	whitelisted_caller
};
use frame_support::{
	sp_runtime::traits::Hash,
};
use frame_system::RawOrigin;

benchmarks! {
	add_agreement_creator {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Root, caller.clone())
	verify {
		assert_eq!(AgreementCreator::<T>::contains_key(&caller), true);
	}

	remove_agreement_creator {
		let caller: T::AccountId = whitelisted_caller();
		SmartAgreement::<T>::add_agreement_creator(RawOrigin::Root.into(), caller.clone()).ok();
	}: _(RawOrigin::Root, caller.clone())
	verify {
		assert_eq!(AgreementCreator::<T>::contains_key(&caller), false);
	}

	create_agreement {
		let c in 1 .. u8::MAX.into();
		let party_a: T::AccountId = whitelisted_caller();
		let party_b: T::AccountId = whitelisted_caller();
		let seed_bytes: [u8; 32] = [c.try_into().unwrap(); 32];
		let agreement_creator: T::AccountId = whitelisted_caller();
		SmartAgreement::<T>::add_agreement_creator(RawOrigin::Root.into(), agreement_creator.clone().into()).ok();
		let agreement_id = <T as frame_system::Config>::Hashing::hash_of(&seed_bytes.to_vec());

	}: _(RawOrigin::Signed(agreement_creator.into()), party_a.clone(), party_b.clone(), AgreementType::ServiceAgreement,
			seed_bytes.to_vec())
	verify {
	  let agreement = InfoForAgreement::<T>::get(&agreement_id).unwrap();
	  assert_eq!(agreement_id, agreement.agreement_id);
	}
}

impl_benchmark_test_suite!(SmartAgreement, crate::mock::new_test_ext(), crate::mock::Test,);
