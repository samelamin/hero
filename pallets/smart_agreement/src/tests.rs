use crate::{info_types::AgreementType, mock::*, Error};
use frame_support::{assert_noop, assert_ok, Hashable};
use sp_core::{H256, sr25519::Public};

#[test]
fn add_agreement_creator() {
	new_test_ext().execute_with(|| {
		let test_account: Public = get_account_id_from_seed::<Public>("Alice");
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), test_account));
		assert_eq!(SmartAgreement::is_agreement_creator(test_account), true);
	});
}

#[test]
fn add_agreement_creator_when_not_permissioned() {
	new_test_ext().execute_with(|| {
		let test_account: Public = get_account_id_from_seed::<Public>("Alice");
		assert_noop!(
			SmartAgreement::add_agreement_creator(Origin::signed(test_account), test_account),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn add_agreement_already_agreement_creator() {
	new_test_ext().execute_with(|| {
		let test_account: Public = get_account_id_from_seed::<Public>("Alice");
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), test_account));
		assert_noop!(
			SmartAgreement::add_agreement_creator(Origin::root(), test_account),
			Error::<Test>::AlreadyCertifiedAgreementCreator
		);
	});
}

#[test]
fn remove_agreement_creator() {
	new_test_ext().execute_with(|| {
		let test_account: Public = get_account_id_from_seed::<Public>("Alice");
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), test_account));
		assert_eq!(SmartAgreement::is_agreement_creator(test_account), true);
		assert_ok!(SmartAgreement::remove_agreement_creator(Origin::root(), test_account));
		assert_noop!(
			SmartAgreement::remove_agreement_creator(Origin::root(), test_account),
			Error::<Test>::AgreementCreatorDoesntExist
		);
	});
}

#[test]
fn remove_agreement_creator_when_not_permissioned() {
	new_test_ext().execute_with(|| {
		let test_account: Public = get_account_id_from_seed::<Public>("Alice");
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), test_account));
		assert_noop!(
			SmartAgreement::remove_agreement_creator(Origin::signed(test_account), test_account),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn create_agreement() {
	new_test_ext().execute_with(|| {
		let test_account: Public = get_account_id_from_seed::<Public>("Alice");
		let party_a_account: Public = get_account_id_from_seed::<Public>("Bob");
		let party_b_account: Public = get_account_id_from_seed::<Public>("Charlie");
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), test_account));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(test_account),
			party_a_account,
			party_b_account,
			AgreementType::ServiceAgreement,
			Vec::<u8>::default()
		));
		let agreement_id: H256 = Vec::<u8>::default().blake2_256().into();
		let agreement_info = SmartAgreement::info_for_agreement(agreement_id).unwrap();
		assert_eq!(agreement_info.party_a, party_a_account);
		assert_eq!(agreement_info.party_b, party_b_account);
		assert_eq!(agreement_info.agreement_type, AgreementType::ServiceAgreement);
		assert_eq!(SmartAgreement::agreement_count_for_user(party_a_account), 1);
		assert_eq!(SmartAgreement::agreement_count_for_user(party_b_account), 1);
		assert_eq!(SmartAgreement::agreement_count(), 1);
	});
}

#[test]
fn create_agreement_not_certified_agreement_creator() {
	new_test_ext().execute_with(|| {
		let test_account: Public = get_account_id_from_seed::<Public>("Alice");
		let party_a_account: Public = get_account_id_from_seed::<Public>("Bob");
		let party_b_account: Public = get_account_id_from_seed::<Public>("Charlie");
		assert_noop!(
			SmartAgreement::create_agreement(
				Origin::signed(test_account),
				party_a_account,
				party_b_account,
				AgreementType::ServiceAgreement,
				Vec::<u8>::default()
			),
			Error::<Test>::NotCertifiedAgreementCreator
		);
	});
}

#[test]
fn create_agreement_agreement_already_exists() {
	new_test_ext().execute_with(|| {
		let test_account: Public = get_account_id_from_seed::<Public>("Alice");
		let party_a_account: Public = get_account_id_from_seed::<Public>("Bob");
		let party_b_account: Public = get_account_id_from_seed::<Public>("Charlie");
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), test_account));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(test_account),
			party_a_account,
			party_b_account,
			AgreementType::ServiceAgreement,
			Vec::<u8>::default()
		));
		assert_noop!(
			SmartAgreement::create_agreement(
				Origin::signed(test_account),
				party_a_account,
				party_b_account,
				AgreementType::ServiceAgreement,
				Vec::<u8>::default()
			),
			Error::<Test>::AgreementAlreadyExists
		);
	});
}

#[test]
fn create_agreement_max_agreements_exceeded() {
	new_test_ext().execute_with(|| {
		let test_account: Public = get_account_id_from_seed::<Public>("Alice");
		let party_a_account: Public = get_account_id_from_seed::<Public>("Bob");
		let party_b_account: Public = get_account_id_from_seed::<Public>("Charlie");
		let party_a2_account: Public = get_account_id_from_seed::<Public>("Sam");
		let party_b2_account: Public = get_account_id_from_seed::<Public>("Jacob");
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), test_account));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(test_account),
			party_a_account,
			party_b_account,
			AgreementType::ServiceAgreement,
			Vec::<u8>::default()
		));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(test_account),
			party_a_account,
			party_b_account,
			AgreementType::ServiceAgreement,
			vec![1]
		));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(test_account),
			party_a2_account,
			party_b2_account,
			AgreementType::ServiceAgreement,
			vec![2]
		));
		assert_noop!(
			SmartAgreement::create_agreement(
				Origin::signed(test_account),
				party_a2_account,
				party_b2_account,
				AgreementType::ServiceAgreement,
				vec![3]
			),
			Error::<Test>::MaxAgreementsExceeded
		);
	});
}

#[test]
fn create_agreement_max_agreements_exceeded_for_user() {
	new_test_ext().execute_with(|| {
		let test_account: Public = get_account_id_from_seed::<Public>("Alice");
		let party_a_account: Public = get_account_id_from_seed::<Public>("Bob");
		let party_b_account: Public = get_account_id_from_seed::<Public>("Charlie");
		assert_ok!(SmartAgreement::add_agreement_creator(Origin::root(), test_account));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(test_account),
			party_a_account,
			party_b_account,
			AgreementType::ServiceAgreement,
			Vec::<u8>::default()
		));
		assert_ok!(SmartAgreement::create_agreement(
			Origin::signed(test_account),
			party_a_account,
			party_b_account,
			AgreementType::ServiceAgreement,
			vec![1]
		));
		assert_noop!(
			SmartAgreement::create_agreement(
				Origin::signed(test_account),
				party_a_account,
				party_b_account,
				AgreementType::ServiceAgreement,
				vec![2]
			),
			Error::<Test>::MaxAgreementsForUserExceeded
		);
	});
}
