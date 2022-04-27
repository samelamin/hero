#![cfg_attr(not(feature = "std"), no_std)]

pub mod info_types;
pub use info_types::FeelessInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::{DispatchResult, Dispatchable, DispatchResultWithPostInfo},
		pallet_prelude::*,
		Parameter,
		weights::{Pays, GetDispatchInfo},
		traits::Get,
		sp_runtime::traits::BlockNumberProvider,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::boxed::Box;
	use crate::{FeelessInfo};

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The call type from the runtime which has all the calls available in your runtime.
		type Call: Parameter + GetDispatchInfo + Dispatchable<Origin=Self::Origin>;

		/// The maximum amount of calls an account can make in a session.
		#[pallet::constant]
		type MaxCalls: Get<u32>;

		/// The length of a session in no of blocks.
		#[pallet::constant]
		type SessionLength: Get<<Self as frame_system::Config>::BlockNumber>;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// An account cannot make more Calls than 'MaxCalls'.
		ExceedMaxCalls,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		ExtrinsicResult {
			/// Account id of the sender/transaction maker.
			tx_sender: T::AccountId,

			/// Feeless code result.
			feeless_result: DispatchResult,
		},
	}

	#[pallet::storage]
	#[pallet::getter(fn tracker)]
	/// Track how many calls each user has done for the latest session
	pub(super) type Tracker<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, FeelessInfo<T::BlockNumber>, ValueQuery>;


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight({
		let dispatch_info = call.get_dispatch_info();
		(dispatch_info.weight.saturating_add(10_000), dispatch_info.class, Pays::Yes)
		})]
		pub fn make_feeless(
			origin: OriginFor<T>,
			call: Box<<T as Config>::Call>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin.clone())?;

			// Get the relevant storage data.
			let max_calls = T::MaxCalls::get();
			let mut feeless_info = Tracker::<T>::get(&sender);
			let current_blocknumber: T::BlockNumber = frame_system::Pallet::<T>::current_block_number();
			let session_length = T::SessionLength::get();

			// Calculate the current session.
			let current_session = current_blocknumber / session_length;

			// If this is a new session for the user, reset their count.
			if feeless_info.last_user_session < current_session {
				feeless_info.user_calls = 0;
			}

			// Checking whether the account is eligible for feeless payment.
			ensure!(feeless_info.user_calls < max_calls , Error::<T>::ExceedMaxCalls);

			// Update the tracker count.
			feeless_info.user_calls = feeless_info.user_calls.saturating_add(1);

			Tracker::<T>::insert(
				&sender,
				feeless_info
			);

			// Dispatch the call.
			let result = call.dispatch(origin);

			Self::deposit_event(
				Event::ExtrinsicResult
				{
					tx_sender: sender,
					feeless_result: result.map(|_| ()).map_err(|e| e.error)
				}
			);

			// Making the tx feeless.
			Ok(Pays::No.into())
		}
	}
}
