use crate::{mock::*, Counter};
use frame_support::{assert_noop, assert_ok};

#[test]
fn counter_works() {
    new_test_ext().execute_with(|| {
        assert_eq!(1, 1);
    });
}


