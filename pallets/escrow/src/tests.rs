use crate::{deposit_into_existing, mock::*, DebitorCreditor, Error};
use frame_support::traits::Currency;
// use frame_system::Origin;
use sp_core::H256;

type AccountId = <Test as frame_system::Config>::AccountId;
type Balance = <Balances as Currency<AccountId>>::Balance;
const EXISTENTIAL: Balance = 1_000_000_000;
fn current_issuance() -> Balance {
	<Balances as Currency<AccountId>>::total_issuance()
}
fn free_balance(a: &AccountId) -> Balance {
	<Balances as Currency<AccountId>>::free_balance(a)
}
fn agreement_(x: u8) -> H256 {
	H256::repeat_byte(x)
}

#[test]
fn basic_escrow_test_template() {
	new_test_ext().execute_with(|| {
		let initial_issuance = current_issuance();

		let agreement = agreement_(1);

		let debitor: AccountId = 1;
		let creditor: AccountId = 2;
		let set_balance = |who| {
			Balances::set_balance(
				frame_system::RawOrigin::Root.into(),
				who,
				EXISTENTIAL * 2,
				EXISTENTIAL,
			)
		};
		let _ = set_balance(debitor);
		let _ = set_balance(creditor);
		assert_eq!(current_issuance() - initial_issuance, EXISTENTIAL * 6);

		let debitor_withheld = 5 * EXISTENTIAL / 16;
		let creditor_withheld = EXISTENTIAL / 16;
		let debitor_creditor =
			DebitorCreditor { debitor, debitor_withheld, creditor, creditor_withheld };
		let payment = EXISTENTIAL / 4;

		assert_eq! {agreement, Escrow::new(agreement, debitor_creditor, payment).unwrap()};
		assert_eq! {agreement, Escrow::payment(agreement).unwrap()};

		assert_eq!(free_balance(&debitor), 2 * EXISTENTIAL - EXISTENTIAL / 4);
		assert_eq!(free_balance(&creditor), 2 * EXISTENTIAL + EXISTENTIAL / 4);
		assert_eq!(current_issuance() - initial_issuance, EXISTENTIAL * 6);
	});
}

#[test]
fn test_refund() {
	new_test_ext().execute_with(|| {
		println!();
		let initial_issuance = current_issuance();
		let agreement = agreement_(1);

		let debitor: AccountId = 1;
		let creditor: AccountId = 2;
		let set_balance = |who| {
			Balances::set_balance(
				frame_system::RawOrigin::Root.into(),
				who,
				EXISTENTIAL * 2,
				EXISTENTIAL,
			)
		};
		let _ = set_balance(debitor);
		let _ = set_balance(creditor);
		assert_eq!(current_issuance() - initial_issuance, EXISTENTIAL * 6);

		let debitor_withheld = 5 * EXISTENTIAL / 16;
		let creditor_withheld = EXISTENTIAL / 16;
		let debitor_creditor =
			DebitorCreditor { debitor, debitor_withheld, creditor, creditor_withheld };
		let payment = EXISTENTIAL / 4;

		assert_eq! { Ok(agreement), Escrow::new(agreement, debitor_creditor, payment) };
		assert_eq! { Ok(agreement), Escrow::refund(agreement) };

		assert_eq!(free_balance(&debitor), 2 * EXISTENTIAL);
		assert_eq!(free_balance(&creditor), 2 * EXISTENTIAL);
		assert_eq!(current_issuance() - initial_issuance, EXISTENTIAL * 6);
	});
}

#[test]
fn test_insufficient_funds() {
	new_test_ext().execute_with(|| {
		let initial_issuance = current_issuance();

		let agreement = agreement_(1);

		let debitor: AccountId = 1;
		let creditor: AccountId = 2;
		let set_balance = |who, free, reserved| {
			Balances::set_balance(frame_system::RawOrigin::Root.into(), who, free, reserved)
		};

		let debitor_withheld = 5 * EXISTENTIAL / 16;
		let creditor_withheld = EXISTENTIAL / 16;
		let debitor_creditor =
			DebitorCreditor { debitor, debitor_withheld, creditor, creditor_withheld };
		let payment = EXISTENTIAL / 4;

		// first try to see if insufficient funds are found for the debitor
		let _ = set_balance(debitor, 3 * EXISTENTIAL / 16, EXISTENTIAL);
		let _ = set_balance(creditor, EXISTENTIAL, EXISTENTIAL);
		assert_eq!(current_issuance() - initial_issuance, (EXISTENTIAL + EXISTENTIAL / 16) * 3);

		let attempt_1 = Escrow::new(agreement, debitor_creditor.clone(), payment);
		assert_eq! {Err(Error::<Test>::InsufficientBalance), attempt_1};

		assert_eq!(free_balance(&debitor), 3 * EXISTENTIAL / 16);
		assert_eq!(free_balance(&creditor), EXISTENTIAL);
		assert_eq!(current_issuance() - initial_issuance, (EXISTENTIAL + EXISTENTIAL / 16) * 3);

		// now if the creditor has insufficient funds
		let _ = set_balance(debitor, EXISTENTIAL, EXISTENTIAL);
		let _ = set_balance(creditor, 0, EXISTENTIAL);
		assert_eq!(current_issuance() - initial_issuance, EXISTENTIAL * 3);

		let attempt_2 = Escrow::new(agreement, debitor_creditor, payment);
		assert_eq!(Err(Error::<Test>::InsufficientBalance), attempt_2);

		assert_eq!(free_balance(&debitor), EXISTENTIAL);
		assert_eq!(free_balance(&creditor), 0);
		assert_eq!(current_issuance() - initial_issuance, EXISTENTIAL * 3);
	});
}
#[test]
fn agreement_proof_invalid_test() {
	new_test_ext().execute_with(|| {
		let agreement = agreement_(1);

		let debitor: AccountId = 1;
		let creditor: AccountId = 2;
		let set_balance = |who| {
			Balances::set_balance(
				frame_system::RawOrigin::Root.into(),
				who,
				EXISTENTIAL * 2,
				EXISTENTIAL,
			)
		};
		let _ = set_balance(debitor);
		let _ = set_balance(creditor);

		let debitor_withheld = 5 * EXISTENTIAL / 16;
		let creditor_withheld = EXISTENTIAL / 16;
		let debitor_creditor =
			DebitorCreditor { debitor, debitor_withheld, creditor, creditor_withheld };
		let payment = EXISTENTIAL / 4;

		assert_eq! {Ok(agreement), Escrow::new(agreement, debitor_creditor.clone(), payment)};
		// attempt to use the same agreement Id again
		assert_eq! {Err(Error::<Test>::AgreementIdAlreadyInUse), Escrow::new(agreement, debitor_creditor, payment)};
	})
}

#[test]
fn duplicate_party_test() {
	new_test_ext().execute_with(|| {
		let debitor: AccountId = 1;
		let creditor: AccountId = 1;

		let agreement = agreement_(1);
		let set_balance = |who| {
			Balances::set_balance(
				frame_system::RawOrigin::Root.into(),
				who,
				EXISTENTIAL * 2,
				EXISTENTIAL,
			)
		};
		let _ = set_balance(debitor);
		let _ = set_balance(creditor);

		let debitor_withheld = 5 * EXISTENTIAL / 16;
		let creditor_withheld = EXISTENTIAL / 16;
		let debitor_creditor =
			DebitorCreditor { debitor, debitor_withheld, creditor, creditor_withheld };
		let payment = EXISTENTIAL / 4;

		assert_eq! {Err(Error::<Test>::DuplicateParty), Escrow::new(agreement, debitor_creditor, payment)};
	})
}

#[test]
fn escro_absent_test() {
	new_test_ext().execute_with(|| {
		let agreement = agreement_(1);
		assert_eq!(Err(Error::<Test>::EscrowAbsent), Escrow::refund(agreement));
	})
}

#[test]
fn escrow_absent_test_2() {
	new_test_ext().execute_with(|| {
		let debitor: AccountId = 1;
		let creditor: AccountId = 2;

		let agreement = agreement_(1);
		let set_balance = |who| {
			Balances::set_balance(
				frame_system::RawOrigin::Root.into(),
				who,
				EXISTENTIAL * 2,
				EXISTENTIAL,
			)
		};
		let _ = set_balance(debitor);
		let _ = set_balance(creditor);

		let debitor_withheld = 5 * EXISTENTIAL / 16;
		let creditor_withheld = EXISTENTIAL / 16;
		let debitor_creditor =
			DebitorCreditor { debitor, debitor_withheld, creditor, creditor_withheld };
		let payment = EXISTENTIAL / 4;
		assert_eq! { Ok(agreement), Escrow::new(agreement, debitor_creditor, payment)};
		assert_eq! { Ok(agreement), Escrow::refund(agreement) };
		assert_eq! { Err(Error::<Test>::EscrowAbsent), Escrow::refund(agreement) };
	})
}
