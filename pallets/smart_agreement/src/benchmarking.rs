//! Benchmarking setup for pallet-smart-agreement

use super::*;

#[allow(unused)]
use crate::Pallet as SmartAgreement;
use crate::info_types::AgreementType;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use frame_support::sp_runtime::traits::Hash;

benchmarks! {
    where_clause {
        where T::AccountId: From<u32>
    }

    add_agreement_creator {
        let c in 1 .. u32::MAX;
        let temp: T::AccountId = c.into();
    }: _(RawOrigin::Root, temp.clone())
    verify {
        assert_eq!(AgreementCreator::<T>::contains_key(&temp), true);
    }

    remove_agreement_creator {
        let c in 1 .. u32::MAX;
        let temp: T::AccountId = c.into();
        SmartAgreement::<T>::add_agreement_creator(RawOrigin::Root.into(), temp.clone()).ok();
    }: _(RawOrigin::Root, temp.clone())
    verify {
        assert_eq!(AgreementCreator::<T>::contains_key(&temp), false);
    }

    create_agreement {
        let a in 1 .. (u32::MAX / 2);
        let b in ((u32::MAX / 2) + 1) .. u32::MAX;
        let c in 1 .. u8::MAX.into();
        let party_a: T::AccountId = a.into();
        let party_b: T::AccountId = b.into();
        let seed_bytes: [u8; 32] = [c.try_into().unwrap(); 32];
        let agreement_creator = 0;
        SmartAgreement::<T>::add_agreement_creator(RawOrigin::Root.into(), agreement_creator.into()).ok();
        let agreement_id = <T as frame_system::Config>::Hashing::hash_of(&seed_bytes.to_vec());

    }: _(RawOrigin::Signed(agreement_creator.into()), party_a.clone(), party_b.clone(), AgreementType::ServiceAgreement,
            seed_bytes.to_vec())
    verify {
      let agreement = InfoForAgreement::<T>::get(&agreement_id).unwrap();
      assert_eq!(agreement_id, agreement.agreement_id);
    }
}

impl_benchmark_test_suite!(SmartAgreement, crate::mock::new_test_ext(), crate::mock::Test,);
