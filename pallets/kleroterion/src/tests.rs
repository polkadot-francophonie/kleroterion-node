use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, assert_err	};
use frame_system::pallet_prelude::*;

#[test]
fn it_fails_with_no_origin() {
	new_test_ext().execute_with(|| {

		let tribes = vec![b"Tribe1".to_vec()];
		// Dispatch a signed extrinsic.
		assert!(Kleroterion::open_jury_call(Origin::none(), tribes, 5, 10000).is_err());
	});
}

fn it_works_for_minimum_values() {
	new_test_ext().execute_with(|| {

		let tribes = vec![b"Tribe1".to_vec()];
		// Dispatch a signed extrinsic.
		assert_ok!(Kleroterion::open_jury_call(Origin::signed(1), tribes, 1, 1));
		// Read pallet storage JuryCallCnt and assert it is 1.
		assert_eq!(Kleroterion::jury_call_cnt(), 1);
	});
}

fn it_jury_call_id_increases() {
	new_test_ext().execute_with(|| {

		let tribes = vec![b"Tribe1".to_vec()];

		// Dispatch a signed open_jury_call extrinsic.
		assert_ok!(Kleroterion::open_jury_call(Origin::signed(1), tribes.clone(), 1, 1));

		// Read pallet storage JuryCallCnt and assert it is 1.
		assert_eq!(Kleroterion::jury_call_cnt(), 1);

		// Read pallet storage JuryCalls(1) and assert owner is 1 and stard_after is 1.
		assert_eq!(Kleroterion::jury_call(1).unwrap().owner, ensure_signed(Origin::signed(1)).unwrap());
		assert_eq!(Kleroterion::jury_call(1).unwrap().start_after, 1);

		// Dispatch a second signed open_jury_call extrinsic.
		let tribes = vec![b"Tribe1".to_vec()];
		// Dispatch a signed extrinsic.
		assert_ok!(Kleroterion::open_jury_call(Origin::signed(1), tribes, 1, 1));
		// Read pallet storage JuryCallCnt and assert it is 2.
		assert_eq!(Kleroterion::jury_call_cnt(), 2);

		// Read pallet storage JuryCalls(1) and assert owner is 1 and stard_after is 1.
		assert_eq!(Kleroterion::jury_call(1).unwrap().owner, ensure_signed(Origin::signed(1)).unwrap());
		assert_eq!(Kleroterion::jury_call(1).unwrap().start_after, 1);
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(Kleroterion::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
