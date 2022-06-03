use crate::{mock::*, Error};
use crate::info_types::AgreementType;
use frame_support::{assert_noop, assert_ok, Hashable};
use sp_core::H256;

const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId 	 = 1;
const PARTY_A_ACCOUNT: <Test as frame_system::Config>::AccountId = 2;
const PARTY_B_ACCOUNT: <Test as frame_system::Config>::AccountId = 3;

#[test]
fn add_agreement_creator() {
	new_test_ext().execute_with(|| {
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), TEST_ACCOUNT));
		assert_eq!(SmartAgreement::is_agreement_creator(TEST_ACCOUNT), true);
	});
}

#[test]
fn add_agreement_creator_when_not_permissioned() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			SmartAgreement::add_agreement_creator(Origin::signed(TEST_ACCOUNT), TEST_ACCOUNT),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn add_agreement_already_agreement_creator() {
	new_test_ext().execute_with(|| {
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), TEST_ACCOUNT));
		assert_noop!(
			SmartAgreement::add_agreement_creator(Origin::root(), TEST_ACCOUNT),
			Error::<Test>::AlreadyCertifiedAgreementCreator
		);
	});
}

#[test]
fn remove_agreement_creator() {
	new_test_ext().execute_with(|| {
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), TEST_ACCOUNT));
		assert_eq!(SmartAgreement::is_agreement_creator(TEST_ACCOUNT), true);
		assert_ok!(SmartAgreement::remove_agreement_creator(Origin::root(), TEST_ACCOUNT));
		assert_noop!(
			SmartAgreement::remove_agreement_creator(Origin::root(), TEST_ACCOUNT),
			Error::<Test>::AgreementCreatorDoesntExist
		);
	});
}

#[test]
fn remove_agreement_creator_when_not_permissioned() {
	new_test_ext().execute_with(|| {
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), TEST_ACCOUNT));
		assert_noop!(
			SmartAgreement::remove_agreement_creator(Origin::signed(TEST_ACCOUNT), TEST_ACCOUNT),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn create_agreement() {
	new_test_ext().execute_with(|| {
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), TEST_ACCOUNT));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(TEST_ACCOUNT),
			PARTY_A_ACCOUNT,
			PARTY_B_ACCOUNT,
			AgreementType::ServiceAgreement,
			Vec::<u8>::default()
		));
		let agreement_id: H256 = Vec::<u8>::default().blake2_256().into();
		let agreement_info = SmartAgreement::info_for_agreement(agreement_id).unwrap();
		assert_eq!(agreement_info.party_a, PARTY_A_ACCOUNT);
		assert_eq!(agreement_info.party_b, PARTY_B_ACCOUNT);
		assert_eq!(agreement_info.agreement_type, AgreementType::ServiceAgreement);
		assert_eq!(SmartAgreement::agreement_count_for_user(PARTY_A_ACCOUNT), 1);
		assert_eq!(SmartAgreement::agreement_count_for_user(PARTY_B_ACCOUNT), 1);
		assert_eq!(SmartAgreement::agreement_count(), 1);
	});
}

#[test]
fn create_agreement_not_certified_agreement_creator() {
	new_test_ext().execute_with(|| {
		assert_noop!(SmartAgreement::create_agreement(
			Origin::signed(TEST_ACCOUNT),
			PARTY_A_ACCOUNT,
			PARTY_B_ACCOUNT,
			AgreementType::ServiceAgreement,
			Vec::<u8>::default()),
		Error::<Test>::NotCertifiedAgreementCreator);
	});
}

#[test]
fn create_agreement_agreement_already_exists() {
	new_test_ext().execute_with(|| {
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), TEST_ACCOUNT));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(TEST_ACCOUNT),
			PARTY_A_ACCOUNT,
			PARTY_B_ACCOUNT,
			AgreementType::ServiceAgreement,
			Vec::<u8>::default()
		));
		assert_noop!(SmartAgreement::create_agreement(
			Origin::signed(TEST_ACCOUNT),
			PARTY_A_ACCOUNT,
			PARTY_B_ACCOUNT,
			AgreementType::ServiceAgreement,
			Vec::<u8>::default()),
		Error::<Test>::AgreementAlreadyExists);
	});
}

#[test]
fn create_agreement_max_agreements_exceeded() {
	new_test_ext().execute_with(|| {
		const PARTY_A2_ACCOUNT: <Test as frame_system::Config>::AccountId = 4;
		const PARTY_B2_ACCOUNT: <Test as frame_system::Config>::AccountId = 5;
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), TEST_ACCOUNT));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(TEST_ACCOUNT),
			PARTY_A_ACCOUNT,
			PARTY_B_ACCOUNT,
			AgreementType::ServiceAgreement,
			Vec::<u8>::default()
		));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(TEST_ACCOUNT),
			PARTY_A_ACCOUNT,
			PARTY_B_ACCOUNT,
			AgreementType::ServiceAgreement,
			vec![1]
		));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(TEST_ACCOUNT),
			PARTY_A2_ACCOUNT,
			PARTY_B2_ACCOUNT,
			AgreementType::ServiceAgreement,
			vec![2]
		));
		assert_noop!(SmartAgreement::create_agreement(
			Origin::signed(TEST_ACCOUNT),
			PARTY_A2_ACCOUNT,
			PARTY_B2_ACCOUNT,
			AgreementType::ServiceAgreement,
			vec![3]),
		Error::<Test>::MaxAgreementsExceeded);
	});
}

#[test]
fn create_agreement_max_agreements_exceeded_for_user() {
	new_test_ext().execute_with(|| {
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), TEST_ACCOUNT));
			assert_ok!(SmartAgreement::create_agreement(
				Origin::signed(TEST_ACCOUNT),
				PARTY_A_ACCOUNT,
				PARTY_B_ACCOUNT,
				AgreementType::ServiceAgreement,
				Vec::<u8>::default()
			));
			assert_ok!(SmartAgreement::create_agreement(
				Origin::signed(TEST_ACCOUNT),
				PARTY_A_ACCOUNT,
				PARTY_B_ACCOUNT,
				AgreementType::ServiceAgreement,
				vec![1]
			));
			assert_noop!(SmartAgreement::create_agreement(
				Origin::signed(TEST_ACCOUNT),
				PARTY_A_ACCOUNT,
				PARTY_B_ACCOUNT,
				AgreementType::ServiceAgreement,
				vec![2]),
			Error::<Test>::MaxAgreementsForUserExceeded);
	});
}