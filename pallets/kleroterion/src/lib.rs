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

	use frame_support::{
			dispatch::DispatchResult, 
			pallet_prelude::*,
			traits::UnixTime,
		};
	use frame_support::sp_runtime::traits::Printable;
	use frame_support::sp_runtime::print;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	pub use crate::types::{
		JuryCallID,
		Selections,
		Candidates,
	};

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	#[derive(Clone, Encode, Decode, PartialEq)]
	pub struct Tribe<T: Config> {
		pub name:  Vec<u8>,
		pub candidate_count: Candidates, //indicates the current number of candidates 
		pub candidates: Vec<AccountOf<T>>, // Vector containing the candidates
	}

	#[derive(Clone, Encode, Decode, PartialEq)]
	pub struct JuryCall<T: Config> {
		pub tribes:  Vec<Tribe<T>>,
		pub selections: Selections,
		// Tribes+Candidates
		pub start_after: u64,
		pub owner: AccountOf<T>,
	}
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TimeProvider: UnixTime;  //Cf https://stackoverflow.com/questions/68262293/substrate-frame-v2-how-to-use-pallet-timestamp
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
		/// A Jury Call has been opened. [jury_call_id, who]
		JuryCallOpened(JuryCallID, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Invalid Signer
		BadOrigin,
		/// Arithemtic overflow when incrementing the Jury Call counter.
		JuryCallCntOverflow,
		/// Cannot use tTwo identical tribe names.
		DuplicateTribes,
		/// Selections must be greater than zero
		ZeroSelections,
		/// Start_After should be in the future
		StartAfterInThePast,
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	impl<T: Config> Printable for Error<T> {
		fn print(&self) {
		  match self {
			Error::BadOrigin => "BadOrigin".print(),  
			Error::JuryCallCntOverflow => "Jury Call Value Overflowed".print(),
			Error::DuplicateTribes => "Duplicate tribe names entered".print(),
			Error::ZeroSelections => "Zero selections not allowed".print(),
			Error::StartAfterInThePast => "Start After must be in the future".print(),
			_ => "Invalid Error Case".print(),
		  }
		}
	  }

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		// The tribe names are passed here as 'str' under the form of Vec<u8>.
		// As multiple tribes can be passed, we pass Vec<Vec<u8>>
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn open_jury_call(origin: OriginFor<T>, tribe_names: Vec<Vec<u8>>, selections: Selections, start_after: u64) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Check tribes are distinct
			if check_duplicate_tribes(&tribe_names) { Err(Error::<T>::DuplicateTribes)? }

			// Check selections > 0
			if selections == 0 { Err(Error::<T>::ZeroSelections)? }
			

			// Check that timestamp is in the future compared to current blocks timestamp
			let time_now: u64 = T::TimeProvider::now().as_secs();
			print(time_now);
			print(start_after);
			if time_now >= start_after {
				print(Error::<T>::StartAfterInThePast);
				Err(Error::<T>::StartAfterInThePast)? 
			}

            // Assign a new id
			let new_cnt = Self::jury_call_cnt()
							.checked_add(1)
  							.ok_or(<Error<T>>::JuryCallCntOverflow)?;


            // Initialize the tribes
			let mut tribes: Vec<Tribe<T>> = Default::default();
			for tribe_name in tribe_names {
				let tribe = Tribe::<T> {
					name: tribe_name,
					candidate_count: 0,
					candidates: Default::default(),
				};
				tribes.push(tribe);
			}

			let jury_call = JuryCall::<T> {
				tribes: tribes,
				selections: selections,
				start_after: start_after,
				owner: who.clone(),
			};



			// Update storage. 
            //keep track of how many jury_calls have been created
			<JuryCallCnt<T>>::put(new_cnt);

			// Store the new jury_call
			// The ID of a jury_call is the new_cnt number (do we need to store it inside the struct?)
			<JuryCalls<T>>::insert(new_cnt, jury_call);

			// Emit an event to report the Jury Call.
			Self::deposit_event(Event::JuryCallOpened(new_cnt, who));

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn test_time(origin: OriginFor<T>,  start_after: u64) -> DispatchResult {
		// 	// Check that the extrinsic was signed and get the signer.
		// 	// This function will return an error if the extrinsic is not signed.
		// 	// https://substrate.dev/docs/en/knowledgebase/runtime/origin
		// 	let who = ensure_signed(origin)?;

		// 	// Check that timestamp is in the future compared to current blocks timestamp
		// 	let time_now: u64 = T::TimeProvider::now().as_secs();
		// 	print("Time now");
		// 	print(time_now);
		// 	print(Error::<T>::StorageOverflow);
		// 	if time_now >= start_after {
		// 		print(Error::<T>::StartAfterInThePast);
		// 		//  Err(Error::<T>::StartAfterInThePast)? 

		// 	}

		// 	// Return a successful DispatchResultWithPostInfo
		// 	Ok(())
		// }


	}

	/// Returns true if the tribes vector contains duplicates
    fn check_duplicate_tribes( tribes: &Vec<Vec<u8>> ) -> bool {
		let mut tribes_distinct = Vec::<Vec<u8>>::new();
		for tribe in tribes {
			for tribe_distinct in &tribes_distinct {
				if compare_tribes(Some(tribe.to_vec()), Some(&tribe_distinct)) {
					return true;
				}
			}
			tribes_distinct.push(tribe.clone());	
		}
		false
	} 

	/// Returns true if the 2 strings are the same
	fn compare_tribes(a: Option<Vec<u8>>, b: Option<&[u8]>) -> bool {
		a.as_ref().map(|x| &x[..]) == b
	}
}
