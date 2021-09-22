use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, assert_err	};

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
		// // Read pallet storage and assert an expected result.
		// assert_eq!(Kleroterion::something(), Some(42));
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(Kleroterion::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
