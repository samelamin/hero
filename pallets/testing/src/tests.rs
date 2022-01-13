use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, assert_err};
// use super::*;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TestingPallet::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TestingPallet::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TestingPallet::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn error_works() {
	new_test_ext().execute_with( || {
		assert_err!(
			TestingPallet::add_value(Origin::signed(1), 51),
			"value must be <= maximum add amount constant"
		);
	});
}

#[test]
fn test_should_work() {
	new_test_ext().execute_with( || {
		assert_ok!(TestingPallet::do_something(Origin::signed(1), 42));
		assert_ok!(
			TestingPallet::add_value(Origin::signed(1), 8)
		);
	});
}

#[test]
fn testing_greater_than_max_fails() {
	new_test_ext().execute_with( || {
		assert_ok!(TestingPallet::do_something(Origin::signed(1), 42));
		assert_noop!( TestingPallet::add_value(Origin::signed(1), 42),
			Error::<Test>::StorageOverflow
		);
	});
}
