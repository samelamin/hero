// Copyright 2022 Hero
// This file is part of Hero project.

// Hero is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Hero is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! Bridge pallet's unit test cases

// ----------------------------------------------------------------------------
// Module imports
// ----------------------------------------------------------------------------

use crate::{
	self as pallet_bridge,
	mock::{
		helpers::*, Balances, Bridge, ChainBridge, Event, MockRuntime, NativeTokenId, Origin,
		ProposalLifetime, TestExternalitiesBuilder, ENDOWED_BALANCE, RELAYER_A, RELAYER_B,
		RELAYER_B_INITIAL_BALANCE, RELAYER_C, TEST_RELAYER_VOTE_THRESHOLD,
	},
	Error,
};

use codec::Encode;

use frame_support::{
	assert_err, assert_noop, assert_ok,
	traits::{LockableCurrency, WithdrawReasons},
};

pub use runtime_common::{
  constants::{
    HERO, NATIVE_TOKEN_TRANSFER_FEE, NFT_TOKEN_TRANSFER_FEE, Balance
  },
};

use sp_core::{blake2_256, H256};

use sp_runtime::DispatchError;

// ----------------------------------------------------------------------------
// Test cases
// ----------------------------------------------------------------------------

#[test]
fn transfer_native() {
	TestExternalitiesBuilder::default()
		.build()
		.execute_with(|| {
			let dest_chain = 0;
			let resource_id = NativeTokenId::get();
			let amount: u128 = 20 * HERO;
			let recipient = vec![99];

			assert_ok!(ChainBridge::whitelist_chain(
				Origin::root(),
				dest_chain.clone()
			));

			let balance_relayer_c = Balances::free_balance(RELAYER_C);
      assert_eq!(balance_relayer_c, 0);
      println!("balance_relayer_c: {}", balance_relayer_c);
			
      // Using account with not enough balance for fee should fail when requesting transfer
			assert_err!(
				Bridge::transfer_native(
					Origin::signed(RELAYER_C),
					amount.clone(),
					recipient.clone(),
					dest_chain,
				),
				Error::<MockRuntime>::InsufficientBalance
			);

			// Using account with enough balance for fee but not for transfer amount
			let mut account_current_balance = Balances::free_balance(RELAYER_B);
			assert_eq!(account_current_balance, RELAYER_B_INITIAL_BALANCE);

			assert_err!(
				Bridge::transfer_native(
					Origin::signed(RELAYER_B),
					amount.clone(),
					recipient.clone(),
					dest_chain,
				),
				Error::<MockRuntime>::InsufficientBalance
			);

			// Account balance of relayer B should be reverted to original balance
			account_current_balance = Balances::free_balance(RELAYER_B);
			assert_eq!(account_current_balance, RELAYER_B_INITIAL_BALANCE);

      // Check RelayA balance
      let mut balance_relayer_a = Balances::free_balance(RELAYER_A);
			assert_eq!(balance_relayer_a, ENDOWED_BALANCE);

			// Using account with enough balance for fee, but transfer blocked by a lock
			let lock_amount = 7990 * HERO;
			Balances::set_lock(
				*b"testlock",
				&RELAYER_A,
				lock_amount,
				WithdrawReasons::all(),
			);
			assert_err!(
				Bridge::transfer_native(
					Origin::signed(RELAYER_A),
					amount.clone(),
					recipient.clone(),
					dest_chain,
				),
				Error::<MockRuntime>::InsufficientBalance
			);

			// Account balance of relayer A should be tantamount to the initial endowed value
			Balances::remove_lock(*b"testlock", &RELAYER_A);
			balance_relayer_a = Balances::free_balance(RELAYER_A);
      println!("balance_relayer_a: {}", balance_relayer_a);
			assert_eq!(balance_relayer_a, ENDOWED_BALANCE);

			// Successful transfer with relayer A account, which has enough funds
			// for the requested amount plus transfer fees
			assert_ok!(Bridge::transfer_native(
				Origin::signed(RELAYER_A),
				amount.clone(),
				recipient.clone(),
				dest_chain,
			));

			expect_event(chainbridge::Event::FungibleTransfer(
				dest_chain,
				1,
				resource_id,
				amount.into(),
				recipient,
			));

			// Current Relay A account balance is initial value (i.e. ENDOWED_BALANCE) less transfer fees (i.e. NATIVE_TOKEN_TRANSFER_FEE)
      println!("NATIVE_TOKEN_TRANSFER_FEE: {}, amount: {}", NATIVE_TOKEN_TRANSFER_FEE, amount);

      // and amount (i.e. 20 * HERO), that is, (10000 * HERO) - (2000 * HERO) - (20 * HERO) = 7980 * HERO
			account_current_balance = Balances::free_balance(RELAYER_A);
			let amount_and_fees = amount + NATIVE_TOKEN_TRANSFER_FEE;
			let account_expected_balance = ENDOWED_BALANCE - amount_and_fees;
      assert_eq!(account_current_balance, account_expected_balance);
		})
}

#[test]
fn create_successful_remark_proposal() {
	TestExternalitiesBuilder::default()
		.build()
		.execute_with(|| {
			let hash: H256 = "ABC".using_encoded(blake2_256).into();
			let proposal_id = 1;
			let src_id = 1;
			let resource_id = chainbridge::derive_resource_id(src_id, b"hero_nft_hash");
			let proposal = mock_remark_proposal(hash.clone(), resource_id);
			let resource = b"Bridge.remark".to_vec();

			assert_ok!(ChainBridge::set_threshold(
				Origin::root(),
				TEST_RELAYER_VOTE_THRESHOLD
			));
			assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_A));
			assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_B));
			assert_eq!(ChainBridge::get_relayer_count(), 2);
			assert_ok!(ChainBridge::whitelist_chain(Origin::root(), src_id));
			assert_ok!(ChainBridge::set_resource(Origin::root(), resource_id, resource));

			assert_ok!(ChainBridge::acknowledge_proposal(
				Origin::signed(RELAYER_A),
				proposal_id,
				src_id,
				resource_id,
				Box::new(proposal.clone())
			));

			assert_ok!(ChainBridge::acknowledge_proposal(
				Origin::signed(RELAYER_B),
				proposal_id,
				src_id,
				resource_id,
				Box::new(proposal.clone())
			));

			event_exists(pallet_bridge::Event::<MockRuntime>::Remark(hash, resource_id));
		})
}

#[test]
fn create_invalid_remark_proposal_with_bad_origin() {
	TestExternalitiesBuilder::default()
		.build()
		.execute_with(|| {
			let hash: H256 = "ABC".using_encoded(blake2_256).into();
			let resource_id = chainbridge::derive_resource_id(1, b"hero_nft_hash");

			// Add a new relayer account to the chainbridge
			assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_A));

			// Chain bridge account is a valid origin for a remark proposal
			assert_ok!(Bridge::remark(
				Origin::signed(ChainBridge::account_id()),
				hash,
				resource_id
			));

			// Don't allow any signed origin except from chainbridge addr,
			// even if the relayer is listed on the chain bridge
			assert_noop!(
				Bridge::remark(Origin::signed(RELAYER_A), hash, resource_id),
				DispatchError::BadOrigin
			);

			// Don't allow root calls
			assert_noop!(
				Bridge::remark(Origin::root(), hash, resource_id),
				DispatchError::BadOrigin
			);
		})
}

#[test]
fn transfer() {
	TestExternalitiesBuilder::default()
		.build()
		.execute_with(|| {
			// Check inital state
			let bridge_id: u64 = ChainBridge::account_id();
			let resource_id = NativeTokenId::get();
			let current_balance = Balances::free_balance(&bridge_id);

			assert_eq!(current_balance, ENDOWED_BALANCE);

			// Transfer and check result
			assert_ok!(Bridge::transfer(
				Origin::signed(ChainBridge::account_id()),
				RELAYER_A,
				10,
				resource_id
			));
			assert_eq!(Balances::free_balance(&bridge_id), ENDOWED_BALANCE - 10);
			assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);

			assert_events(vec![Event::Balances(pallet_balances::Event::Transfer {
				from: ChainBridge::account_id(),
				to: RELAYER_A,
				amount: 10,
			})]);
		})
}

#[test]
fn create_successful_transfer_proposal() {
	TestExternalitiesBuilder::default()
		.build()
		.execute_with(|| {
			let proposal_id = 1;
			let src_id = 1;
			let resource_id = chainbridge::derive_resource_id(src_id, b"transfer");
			let resource = b"Bridge.transfer".to_vec();

			// Create dummy transfer proposal for an amount of 10 transfered to RELAYER A
			let transfer_proposal = mock_transfer_proposal(RELAYER_A, 10, resource_id);

			assert_ok!(ChainBridge::set_threshold(
				Origin::root(),
				TEST_RELAYER_VOTE_THRESHOLD
			));
			assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_A));
			assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_B));
			assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_C));
			assert_ok!(ChainBridge::whitelist_chain(Origin::root(), src_id));
			assert_ok!(ChainBridge::set_resource(Origin::root(), resource_id, resource));

			// First relayer (i.e. RELAYER_A) creates a new transfer proposal (so that an amount of 10 is transfered to his account)
			assert_ok!(ChainBridge::acknowledge_proposal(
				Origin::signed(RELAYER_A),
				proposal_id,
				src_id,
				resource_id,
				Box::new(transfer_proposal.clone())
			));
			let actual_votes =
				ChainBridge::get_votes(src_id, (proposal_id.clone(), transfer_proposal.clone()))
					.unwrap();
			let expected_votes = chainbridge::types::ProposalVotes {
				votes_for: vec![RELAYER_A],
				votes_against: vec![],
				status: chainbridge::types::ProposalStatus::Initiated,
				expiry: ProposalLifetime::get() + 1,
			};
			assert_eq!(actual_votes, expected_votes);

			// Second relayer (i.e. RELAYER_B) votes against
			assert_ok!(ChainBridge::reject_proposal(
				Origin::signed(RELAYER_B),
				proposal_id,
				src_id,
				resource_id,
				Box::new(transfer_proposal.clone())
			));
			let actual_votes =
				ChainBridge::get_votes(src_id, (proposal_id.clone(), transfer_proposal.clone()))
					.unwrap();
			let expected_votes = chainbridge::types::ProposalVotes {
				votes_for: vec![RELAYER_A],
				votes_against: vec![RELAYER_B],
				status: chainbridge::types::ProposalStatus::Initiated,
				expiry: ProposalLifetime::get() + 1,
			};
			assert_eq!(actual_votes, expected_votes);

			// Third relayer (i.e. RELAYER_C) votes in favour
			assert_ok!(ChainBridge::acknowledge_proposal(
				Origin::signed(RELAYER_C),
				proposal_id,
				src_id,
				resource_id,
				Box::new(transfer_proposal.clone())
			));
			let actual_votes =
				ChainBridge::get_votes(src_id, (proposal_id.clone(), transfer_proposal.clone()))
					.unwrap();
			let expected_votes = chainbridge::types::ProposalVotes {
				votes_for: vec![RELAYER_A, RELAYER_C],
				votes_against: vec![RELAYER_B],
				status: chainbridge::types::ProposalStatus::Approved,
				expiry: ProposalLifetime::get() + 1,
			};
			assert_eq!(actual_votes, expected_votes);

			// First relayer's (i.e. RELAYER_A) account balance is increased of 10 as there were 2 votes for (i.e. RELAYER_A and RELAYER_B)
			assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);

			//The chainbridge pallet's account balance must now be decreased by 10 after the transfer proposal was accepted
			assert_eq!(
				Balances::free_balance(ChainBridge::account_id()),
				ENDOWED_BALANCE - 10
			);

			assert_events(vec![
				Event::ChainBridge(chainbridge::Event::VoteFor(src_id, proposal_id, RELAYER_A)),
				Event::ChainBridge(chainbridge::Event::VoteAgainst(src_id, proposal_id, RELAYER_B)),
				Event::ChainBridge(chainbridge::Event::VoteFor(src_id, proposal_id, RELAYER_C)),
				Event::ChainBridge(chainbridge::Event::ProposalApproved(src_id, proposal_id)),
				Event::Balances(pallet_balances::Event::Transfer {
					from: ChainBridge::account_id(),
					to: RELAYER_A,
					amount: 10,
				}),
				Event::ChainBridge(chainbridge::Event::ProposalSucceeded(src_id, proposal_id)),
			]);
		})
}

#[test]
fn modify_native_token_transfer_fees() {
	TestExternalitiesBuilder::default()
		.build()
		.execute_with(|| {
			let current_fee = Bridge::get_native_token_transfer_fee();
			assert_eq!(current_fee, NATIVE_TOKEN_TRANSFER_FEE);
			let new_fee = 3000 * HERO;
			assert_ok!(Bridge::set_native_token_transfer_fee(
				Origin::signed(1),
				new_fee
			));
			assert_eq!(new_fee, Bridge::get_native_token_transfer_fee());
		})
}

#[test]
fn modify_nft_token_transfer_fees() {
	TestExternalitiesBuilder::default()
		.build()
		.execute_with(|| {
			let current_fee = Bridge::get_nft_token_transfer_fee();
			assert_eq!(current_fee, NFT_TOKEN_TRANSFER_FEE);
			let new_fee = 3000 * HERO;
			assert_ok!(Bridge::set_nft_token_transfer_fee(
				Origin::signed(1),
				new_fee
			));
			assert_eq!(new_fee, Bridge::get_nft_token_transfer_fee());
		})
}
