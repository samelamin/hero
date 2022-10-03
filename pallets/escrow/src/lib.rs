//! The Escrow pallet is for use only on the Smart Agreement pallet
//! As it expects to be used a single entity, authority is never checked,
//! therefore use of this pallet outside of purview of the Smart Agreement pallet
//! can cause unexpected behavior.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	// Shares the Id type with Smart Agreement Pallet
	use pallet_smart_agreement::AgreementId;

	use codec::MaxEncodedLen;
	use core::fmt::Debug;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement, WithdrawReasons},
	};

	// aliases that allow passing only T
	pub type BalanceIn<Runtime> = <<Runtime as Config>::Currency as Currency<
		<Runtime as frame_system::Config>::AccountId,
	>>::Balance;
	type EscrowFor<Runtime> =
		Escrow<BalanceIn<Runtime>, <Runtime as frame_system::Config>::AccountId>;
	type DebitorCreditorFor<Runtime> =
		DebitorCreditor<BalanceIn<Runtime>, <Runtime as frame_system::Config>::AccountId>;
	type AccountIdFor<Runtime> = <Runtime as frame_system::Config>::AccountId;

	/// The `Escrow` pallet provides the ability for a third party to hold currency when two parties agree on it
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// /// The currency being used in the transaction
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// The heart of the `Escrow` pallet, whats stored on chain
	#[derive(Debug, Clone, Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
	pub struct Escrow<Currency, AccountId: Parameter> {
		/// details of the accounts and their withheld currency is here
		debitor_creditor: DebitorCreditor<Currency, AccountId>,
		/// the eventual sum that is intended to be transferred
		payment: Currency,
	}

	/// Stores the details of the accounts locked in an agreement
	#[derive(PartialEq, Eq, Debug, Clone, Hash, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct DebitorCreditor<Currency, AccountId: scale_info::TypeInfo + Clone> {
		/// the one paying
		pub debitor: AccountId,
		/// withheld from the `debitor`
		pub debitor_withheld: Currency,
		/// the one receiving
		pub creditor: AccountId,
		/// withheld from the `creditor`
		pub creditor_withheld: Currency,
	}

	#[pallet::storage]
	#[pallet::getter(fn escrow_contract)]
	/// A simple store for all `Escrow`s in the pallet
	pub type EscrowStore<T: Config> = StorageMap<
		Hasher = Blake2_128Concat,
		Key = AgreementId<T>,
		Value = EscrowFor<T>,
		QueryKind = OptionQuery,
	>;

	#[pallet::error]
	/// Errors caught by to `Escrow` Pallet
	#[derive(PartialEq, Eq)]
	pub enum Error<T> {
		/// one of the parties' free balance was insufficient
		InsufficientBalance,
		/// The `Escrow` was attempted on false pretenses
		AgreementIdAlreadyInUse,
		/// The `debitor` and the `creditor` had the same account
		DuplicateParty,

		/// the `Escrow` could not be found in storage
		EscrowAbsent,
	}

	/// `Intrinsics` public
	impl<T: Config> Pallet<T> {
		/// Creates an `Escrow` after confirming that the agreement is valid
		pub fn new(
			agreement_id: AgreementId<T>,
			debitor_creditor: DebitorCreditorFor<T>,
			payment: BalanceIn<T>,
		) -> Result<AgreementId<T>, Error<T>> {
			let agreement_id = Self::verify_new_id(agreement_id)?;

			// attempt wthdrawal
			let DebitorCreditor {
				debitor,
				debitor_withheld: debitor_reserve,
				creditor,
				creditor_withheld: creditor_reserve,
			} = &debitor_creditor;

			// ensure that the parties are not the same
			if *debitor == *creditor {
				Err(Error::<T>::DuplicateParty)?
			}

			if payment > *debitor_reserve {
				Err(Error::<T>::InsufficientBalance)?
			}

			// first check if there is sufficient balance
			let balance_check = |who: &AccountIdFor<T>, withhold: &BalanceIn<T>| {
				let balance = <T::Currency as Currency<T::AccountId>>::free_balance(who);
				if balance > *withhold {
					Ok(())
				} else {
					Err(Error::<T>::InsufficientBalance)
				}
			};
			balance_check(debitor, debitor_reserve)?;
			balance_check(creditor, creditor_reserve)?;

			// now withdraw with impunity
			let withdrawal = |who: &AccountIdFor<T>, withhold: &BalanceIn<T>| {
				<T::Currency as Currency<T::AccountId>>::withdraw(
					who,
					*withhold,
					WithdrawReasons::RESERVE,
					ExistenceRequirement::KeepAlive,
				)
			};
			let _ = withdrawal(debitor, debitor_reserve);
			let _ = withdrawal(creditor, creditor_reserve);

			// deposit the `Escrow`
			let escrow: EscrowFor<T> = Escrow { debitor_creditor, payment };
			EscrowStore::<T>::insert(agreement_id, escrow);

			Ok(agreement_id)
		}

		/// Release the `Escrow` via the payment
		/// The `debitor` is refunded their deposit minus the payment ( the amount in the payment field of an `Escrow`),
		/// and the `creditor` is refunded their deposit plus the payment.
		/// If this function succeeds, the `Escrow` is complete, and archived (can no longer be manipulated).
		pub fn payment(agreement_id: AgreementId<T>) -> Result<AgreementId<T>, Error<T>> {
			let Escrow { debitor_creditor, payment, .. } = Self::lookup(agreement_id)?;

			let DebitorCreditor { debitor, debitor_withheld, creditor, creditor_withheld } =
				&debitor_creditor;

			deposit_into_existing::<T>(debitor, *debitor_withheld - payment);
			deposit_into_existing::<T>(creditor, *creditor_withheld + payment);

			EscrowStore::<T>::remove(agreement_id);
			Ok(agreement_id)
		}

		/// Release the `Escrow` via a refund
		/// all funds that were deposited are returned to both parties.
		/// If this function succeeds, the `Escrow` is complete, and archived (can no longer be manipulated).
		pub fn refund(agreement_id: AgreementId<T>) -> Result<AgreementId<T>, Error<T>> {
			let Escrow { debitor_creditor, .. } = Self::lookup(agreement_id)?;

			let DebitorCreditor { debitor, debitor_withheld, creditor, creditor_withheld } =
				&debitor_creditor;

			deposit_into_existing::<T>(debitor, *debitor_withheld);
			deposit_into_existing::<T>(creditor, *creditor_withheld);

			EscrowStore::<T>::remove(agreement_id);
			Ok(agreement_id)
		}
	}

	/// Simply a wrapper for `<T::Currency as Currency<T::AccountId>>::deposit_into_existing`
	pub(crate) fn deposit_into_existing<T: Config + frame_system::Config>(
		who: &AccountIdFor<T>,
		value: BalanceIn<T>,
	) {
		let _ = <T::Currency as Currency<T::AccountId>>::deposit_into_existing(who, value);
	}

	/// `Intrinsics` private
	impl<T: Config> Pallet<T> {
		/// checks if the `AgreementId` is not active already used as a key for escrow pallet storage
		fn verify_new_id(agreement_id: AgreementId<T>) -> Result<AgreementId<T>, Error<T>> {
			// check that it hasn't already been put into `EscrowStore`.
			match EscrowStore::<T>::try_get(agreement_id) {
				Ok(_) => Err(Error::<T>::AgreementIdAlreadyInUse), // occupied => Error
				Err(_) => Ok(agreement_id),                        // vacant => Ok
			}
		}

		/// Should be run every time the escrow is accessed after creation.
		/// attempts to acquire the `Escrow` in storage
		fn lookup(agreement_id: AgreementId<T>) -> Result<EscrowFor<T>, Error<T>> {
			let details: EscrowFor<T> =
				EscrowStore::<T>::get(agreement_id).ok_or(Error::<T>::EscrowAbsent)?;
			Ok(details)
		}
	}
}
