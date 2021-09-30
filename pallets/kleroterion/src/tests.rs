use crate::{mock::*, Error};
use frame_support::{assert_ok, assert_err};
use frame_system::pallet_prelude::*;


type Event = crate::Event<Test>;

// Fails Probably due to bad definition of expected_event?
#[test]
fn it_check_open_jury_call_event() {
	new_test_ext().execute_with(|| {
		// Dispatch the call
		let tribe_names = vec![b"Tribe1".to_vec()];
		let _res = Kleroterion::open_jury_call(Origin::signed(1), tribe_names, 0, UX_TS_20300101);

		// construct event that should be emitted in the method call
		let expected_event = TestEvent::Kleroterion(Event::JuryCallOpened(1, 1));

		// Check event by iterating through array of `EventRecord`s
		assert!(System::events().iter().any(|a| a.event == expected_event));
	});
}

// THis test fails. The returned error does not match the format of the Test Error. 
// thread 'tests::it_fails_with_no_origin' panicked at 'assertion failed: `(left == right)`
//   left: `Err(BadOrigin)`,
//  right: `Err(Module { index: 2, error: 0, message: Some("BadOrigin") })`',
#[test]
fn it_fails_with_no_origin() {
	new_test_ext().execute_with(|| {

		let tribes = vec![b"Tribe1".to_vec()];

		let res = Kleroterion::open_jury_call(Origin::none(), tribes, 5, UX_TS_20300101);
		// Dispatch a signed open_jury_call extrinsic.
		assert_err!(res,TestError::BadOrigin);
	});
}

#[test]
fn it_works_for_minimum_values() {
	new_test_ext().execute_with(|| {

		let tribes = vec![b"Tribe1".to_vec()];

		// Dispatch a signed open_jury_call extrinsic.
		assert_ok!(Kleroterion::open_jury_call(Origin::signed(1), tribes, 1, UX_TS_20300101));

		// Read pallet storage JuryCallCnt and assert it is 1.
		assert_eq!(Kleroterion::jury_call_cnt(), 1);
	});
}

#[test]
fn it_jury_call_id_increases() {
	new_test_ext().execute_with(|| {

		let tribes = vec![b"Tribe1".to_vec()];

		// Dispatch a signed open_jury_call extrinsic.
		assert_ok!(Kleroterion::open_jury_call(Origin::signed(1), tribes.clone(), 1, UX_TS_20300101));
		// Read pallet storage JuryCallCnt and assert it is 1.
		assert_eq!(Kleroterion::jury_call_cnt(), 1);

		let tribes = vec![b"Tribe1".to_vec()];

		// Dispatch a second signed open_jury_call extrinsic.
		assert_ok!(Kleroterion::open_jury_call(Origin::signed(1), tribes, 1, UX_TS_20300101));
		// Read pallet storage JuryCallCnt and assert it is 2.
		assert_eq!(Kleroterion::jury_call_cnt(), 2);
	});
}

#[test]
fn it_jury_call_check_storage() {
	new_test_ext().execute_with(|| {

		let tribe_names = vec![b"Tribe1".to_vec(), b"Tribe2".to_vec()];

		// Dispatch a signed open_jury_call extrinsic.
		assert_ok!(Kleroterion::open_jury_call(Origin::signed(1), tribe_names.clone(), 1, UX_TS_20300101));

		// Read pallet storage JuryCalls(1) and assert owner is 1
		assert_eq!(Kleroterion::jury_call(1).unwrap().owner, ensure_signed(Origin::signed(1)).unwrap());
	
		// Read pallet storage JuryCalls(1) and assert tribes contains 2 entries
		assert_eq!(Kleroterion::jury_call(1).unwrap().tribes[0].name, b"Tribe1".to_vec());
		assert_eq!(Kleroterion::jury_call(1).unwrap().tribes[1].name, b"Tribe2".to_vec());

	    // Read pallet storage JuryCalls(1) and assert selections is 1
		assert_eq!(Kleroterion::jury_call(1).unwrap().selections, 1);
	
	    // Read pallet storage JuryCalls(1) and assert start_after is UxTs20300101
		assert_eq!(Kleroterion::jury_call(1).unwrap().start_after, UX_TS_20300101);
	});
}

#[test]
fn it_reject_duplicate_tribes() {
	new_test_ext().execute_with(|| {

		let tribe_names = vec![b"Tribe1".to_vec(), b"Tribe1".to_vec()];

		// Dispatch a signed open_jury_call extrinsic.
		assert!(Kleroterion::open_jury_call(Origin::signed(1), tribe_names, 1, UX_TS_20300101).is_err());
	});
}

// Test keeps failing but should pass ???
// Reading the doc on pallet_timestamp, this seems to be dut to the fact that 
// in tests, the initial timestamp of the genesis block is not set. So time_now in open_jury_call returns 0
#[test]
fn it_reject_start_after_in_the_past() {

	new_test_ext().execute_with(|| {
		run_to_block(10);
		let tribe_names = vec![b"Tribe1".to_vec()];
		// Dispatch a signed extrinsic.
		assert_err!(Kleroterion::open_jury_call(Origin::signed(1), tribe_names, 1, UX_TS_20100101),Error::<Test>::StartAfterInThePast);
	});
}


#[test]
fn it_reject_zero_selections() {
	new_test_ext().execute_with(|| {

		let tribe_names = vec![b"Tribe1".to_vec()];
		// Dispatch a signed extrinsic.
		assert!(Kleroterion::open_jury_call(Origin::signed(1), tribe_names, 0, UX_TS_20300101).is_err());
	});
}

