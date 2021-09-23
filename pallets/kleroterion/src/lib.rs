#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod types;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	pub use crate::types::JuryCallID;

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	#[derive(Clone, Encode, Decode, PartialEq)]
	pub struct JuryCall<T: Config> {
		// Tribes
		// Tribes+Candidates
		pub start_after: i64,
		pub owner: AccountOf<T>,
	}
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage

	// Properties of a Jury Call
	// id
	// owner
	// not start before date
	// list of tribes
	// map id:canditates to tribe

	#[pallet::storage]
	#[pallet::getter(fn jury_call)]
	/// Stores a Jury Call's unique parameters.
	pub(super) type JuryCalls<T: Config> = StorageMap<_, Twox64Concat, JuryCallID , JuryCall<T>>;



	#[pallet::storage]
	#[pallet::getter(fn jury_call_cnt)]
	pub(super) type JuryCallCnt<T: Config> = StorageValue<_, JuryCallID, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Handles arithemtic overflow when incrementing the Jury Call counter.
		JuryCallCntOverflow,
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]

		// To be improved? the tribe names are passed here as 'str' under the form of Vec<u8>.
		// As multiple tribes can be passed, we pass Vec<Vec<u8>>
		pub fn open_jury_call(origin: OriginFor<T>, tribes: Vec<Vec<u8>>, selections: u8, start_after: i64) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Check that timestamp is in the future compared to current blocks timestamp
			// will need the pallet_timestamp probably
			// Return error if not in the future.

            // Assign a new id
			let new_cnt = Self::jury_call_cnt()
							.checked_add(1)
  							.ok_or(<Error<T>>::JuryCallCntOverflow)?;

			let jury_call = JuryCall::<T> {
				start_after: start_after,
				owner: who.clone(),
			};

			// Update storage.
			<JuryCallCnt<T>>::put(new_cnt);
			<JuryCalls<T>>::insert(new_cnt, jury_call);

			// Emit an event to report the Jury Call.
			// Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		// /// An example dispatchable that may throw a custom error.
		// #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		// pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	// Read a value from storage.
		// 	match <Something<T>>::get() {
		// 		// Return an error if the value has not been set.
		// 		None => Err(Error::<T>::NoneValue)?,
		// 		Some(old) => {
		// 			// Increment the value read from storage; will error in the event of overflow.
		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		// 			// Update the value in storage with the incremented result.
		// 			<Something<T>>::put(new);
		// 			Ok(())
		// 		},
		// 	}
		// }
	}
}
