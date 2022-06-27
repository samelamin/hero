//! traits for parachain-staking

pub trait OnCollatorPayout<AccountId, Balance> {
	fn on_collator_payout(
		for_round: crate::RoundIndex,
		collator_id: AccountId,
		amount: Balance,
	) -> frame_support::pallet_prelude::Weight;
}
impl<AccountId, Balance> OnCollatorPayout<AccountId, Balance> for () {
	fn on_collator_payout(
		_for_round: crate::RoundIndex,
		_collator_id: AccountId,
		_amount: Balance,
	) -> frame_support::pallet_prelude::Weight {
		0
	}
}

pub trait OnNewRound {
	fn on_new_round(round_index: crate::RoundIndex) -> frame_support::pallet_prelude::Weight;
}
impl OnNewRound for () {
	fn on_new_round(_round_index: crate::RoundIndex) -> frame_support::pallet_prelude::Weight {
		0
	}
}
