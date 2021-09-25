use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, assert_err	};
use frame_system::pallet_prelude::*;

const UX_TS_20300101: u64 = 1893452400;
const UX_TS_20100101: u64 = 1262300400;

#[test]
fn it_fails_with_no_origin() {
	new_test_ext().execute_with(|| {

		let tribes = vec![b"Tribe1".to_vec()];
		// Dispatch a signed extrinsic.
		assert!(Kleroterion::open_jury_call(Origin::none(), tribes, 5, 10000).is_err());
	});
}

#[test]
fn it_works_for_minimum_values() {
	new_test_ext().execute_with(|| {

		let tribes = vec![b"Tribe1".to_vec()];
		// Dispatch a signed extrinsic.
		assert_ok!(Kleroterion::open_jury_call(Origin::signed(1), tribes, 1, 1));
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

		let tribes = vec![b"Tribe1".to_vec(), b"Tribe2".to_vec()];

		// Dispatch a signed open_jury_call extrinsic.
		assert_ok!(Kleroterion::open_jury_call(Origin::signed(1), tribes.clone(), 1, UX_TS_20300101));

		// Read pallet storage JuryCalls(1) and assert owner is 1
		assert_eq!(Kleroterion::jury_call(1).unwrap().owner, ensure_signed(Origin::signed(1)).unwrap());
	
		// Read pallet storage JuryCalls(1) and assert tribes contains 2 entries
		assert_eq!(Kleroterion::jury_call(1).unwrap().tribes[0], b"Tribe1".to_vec());
		assert_eq!(Kleroterion::jury_call(1).unwrap().tribes[1], b"Tribe2".to_vec());

	    // Read pallet storage JuryCalls(1) and assert selections is 1
		assert_eq!(Kleroterion::jury_call(1).unwrap().selections, 1);
	
	    // Read pallet storage JuryCalls(1) and assert start_after is UxTs20300101
		assert_eq!(Kleroterion::jury_call(1).unwrap().start_after, UX_TS_20300101);
	});
}

#[test]
fn it_reject_duplicate_tribes() {
	new_test_ext().execute_with(|| {

		let tribes = vec![b"Tribe1".to_vec(), b"Tribe1".to_vec()];
		// Dispatch a signed extrinsic.
		assert!(Kleroterion::open_jury_call(Origin::signed(1), tribes, 1, UX_TS_20300101).is_err());
	});
}

#[test]
fn it_reject_zero_selections() {
	new_test_ext().execute_with(|| {

		let tribes = vec![b"Tribe1".to_vec()];
		// Dispatch a signed extrinsic.
		assert!(Kleroterion::open_jury_call(Origin::signed(1), tribes, 0, UX_TS_20300101).is_err());
	});
}

#[test]
fn it_reject_start_after_in_the_past() {
	new_test_ext().execute_with(|| {

		let tribes = vec![b"Tribe1".to_vec()];
		// Dispatch a signed extrinsic.
		assert!(Kleroterion::open_jury_call(Origin::signed(1), tribes, 1, UX_TS_20100101).is_err());
	});
}

