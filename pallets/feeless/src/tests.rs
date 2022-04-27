use crate::{mock::*, Error, FeelessInfo};
use frame_support::{assert_noop, assert_ok};

#[test]
fn make_transaction_feeless() {
	new_test_ext().execute_with(|| {
		const DEST_ACCOUNT: u64 = 3;
		const VALUE: u64 = 1000;
		let call_transfer = Box::new(call_transfer(DEST_ACCOUNT, VALUE));

		// Making feeless transaction call.
		assert_ok!(FeelessModule::make_feeless(Origin::signed(1), call_transfer));
	});
}

#[test]
fn feeless_transaction_limit_exceeds() {
	new_test_ext().execute_with(|| {
		const DEST_ACCOUNT: u64 = 3;
		const VALUE: u64 = 1000;
		let call_transfer = Box::new(call_transfer(DEST_ACCOUNT, VALUE));

		// Feeless transaction attempt.
		assert_ok!(FeelessModule::make_feeless(Origin::signed(1), call_transfer.clone()));

		// Ensure the expected error is thrown when the feeless limit exceeds.
		assert_noop!(FeelessModule::make_feeless(Origin::signed(1), call_transfer), Error::<Test>::ExceedMaxCalls);
	});
}

#[test]
fn feeless_tracker() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const DEST_ACCOUNT: u64 = 3;
		const VALUE: u64 = 1000;
		let call_transfer = Box::new(call_transfer(DEST_ACCOUNT, VALUE));
		// Dispatch a signed extrinsic.
		assert_ok!(FeelessModule::make_feeless(Origin::signed(TEST_ACCOUNT), call_transfer));
		// Read pallet storage and assert an expected result.
		assert_eq!(FeelessModule::tracker(TEST_ACCOUNT), FeelessInfo{last_user_session: 0, user_calls: 1});
	});
}
