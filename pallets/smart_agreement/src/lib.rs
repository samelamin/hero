#![cfg_attr(not(feature = "std"), no_std)]

pub mod info_types;
pub use info_types::{AgreementInfo, AgreementType};

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use crate::{AgreementInfo, AgreementType};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, sp_runtime::traits::Hash,
		sp_std::vec::Vec, traits::StorageVersion,
	};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Max number of agreements for a particular party
		type MaxAgreementsForUser: Get<u64>;
		/// Max number of agreements for this module
		type MaxAgreements: Get<u128>;
	}

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	pub type AgreementId<T> = <T as frame_system::Config>::Hash;

	#[pallet::storage]
	#[pallet::getter(fn agreement_count)]
	pub type AgreementCount<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn info_for_agreement)]
	pub type InfoForAgreement<T: Config> =
		StorageMap<_, Blake2_128, AgreementId<T>, AgreementInfo<T::AccountId, AgreementId<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn agreement_count_for_user)]
	pub type AgreementCountForUser<T: Config> =
		StorageMap<_, Blake2_128, T::AccountId, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn is_agreement_creator)]
	pub type AgreementCreator<T: Config> =
		StorageMap<_, Blake2_128, T::AccountId, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AgreementInitiated {
			party_a: T::AccountId,
			party_b: T::AccountId,
			agreement_id: AgreementId<T>,
			agreement_type: AgreementType,
		},
		NewAgreementCreator {
			creator: T::AccountId,
		},
		AgreementCreatorRemoved {
			creator: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Agreement is attempting to be duplicated in some way
		AgreementAlreadyExists,
		/// Max Agreements for user exceeded
		MaxAgreementsForUserExceeded,
		/// Max Agreements for pallet exceeded,
		MaxAgreementsExceeded,
		/// Not a trusted creator for service agreements
		NotCertifiedAgreementCreator,
		/// proposed creator is already certified
		AlreadyCertifiedAgreementCreator,
		/// no agreement creator exists to remove
		AgreementCreatorDoesntExist,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_agreement_creator(
			origin: OriginFor<T>,
			creator: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(
				!AgreementCreator::<T>::contains_key(&creator),
				Error::<T>::AlreadyCertifiedAgreementCreator
			);

			AgreementCreator::<T>::insert(&creator, true);
			Self::deposit_event(Event::NewAgreementCreator { creator });
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_agreement_creator(
			origin: OriginFor<T>,
			creator: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(
				AgreementCreator::<T>::contains_key(&creator),
				Error::<T>::AgreementCreatorDoesntExist
			);

			AgreementCreator::<T>::remove(&creator);

			Self::deposit_event(Event::AgreementCreatorRemoved { creator });
			Ok(())
		}

		// Todo: Must ensure that party_a and party_b are different from each other.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_agreement(
			origin: OriginFor<T>,
			party_a: T::AccountId,
			party_b: T::AccountId,
			agreement_type: AgreementType,
			seed_bytes: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;

			ensure!(
				AgreementCreator::<T>::contains_key(creator),
				Error::<T>::NotCertifiedAgreementCreator
			);

			let agreement_id =
				Self::_create_agreement(&party_a, &party_b, &agreement_type, &seed_bytes)?;

			Self::deposit_event(Event::AgreementInitiated {
				party_a,
				party_b,
				agreement_id,
				agreement_type,
			});
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn _create_agreement(
			party_a: &T::AccountId,
			party_b: &T::AccountId,
			agreement_type: &AgreementType,
			seed_bytes: &Vec<u8>,
		) -> Result<AgreementId<T>, DispatchError> {
			let agreement_id = T::Hashing::hash_of(&seed_bytes);

			ensure!(
				!InfoForAgreement::<T>::contains_key(&agreement_id),
				Error::<T>::AgreementAlreadyExists
			);
			ensure!(
				Self::agreement_count() < T::MaxAgreements::get(),
				Error::<T>::MaxAgreementsExceeded
			);
			ensure!(
				Self::agreement_count_for_user(&party_a) < T::MaxAgreementsForUser::get(),
				Error::<T>::MaxAgreementsForUserExceeded
			);
			ensure!(
				Self::agreement_count_for_user(&party_b) < T::MaxAgreementsForUser::get(),
				Error::<T>::MaxAgreementsForUserExceeded
			);

			// Increment Max Agreements for User
			AgreementCount::<T>::mutate(|count| *count = count.saturating_add(1));
			// Increment Max Agreements
			AgreementCountForUser::<T>::mutate(&party_a, |count| *count = count.saturating_add(1));
			AgreementCountForUser::<T>::mutate(&party_b, |count| *count = count.saturating_add(1));

			InfoForAgreement::<T>::insert(
				agreement_id,
				AgreementInfo {
					party_a: party_a.clone(),
					party_b: party_b.clone(),
					agreement_id: agreement_id.clone(),
					agreement_type: agreement_type.clone(),
				},
			);
			Ok(agreement_id)
		}
	}
}
