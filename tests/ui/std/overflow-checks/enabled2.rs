//@ run-fail
//@ compile-flags: -O -C overflow-checks=yes

#![feature(new_range_api)]

use std::range::RangeFrom;

fn main() {
    let mut a = RangeFrom::from(0_u8..).into_iter();
    assert_eq!(a.nth(255), Some(255));
    a.next(); // panic
}
