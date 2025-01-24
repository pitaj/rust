//@ run-pass
//@ compile-flags: -O -C overflow-checks=no

#![feature(new_range_api)]

use std::range::RangeFrom;

fn main() {
    let mut a = RangeFrom::from(u8::MAX..).into_iter();
    assert_eq!(a.next(), Some(u8::MAX));
    assert_eq!(a.next(), Some(0));

    let mut a = RangeFrom::from(0_u8..).into_iter();
    assert_eq!(a.nth(255), Some(255));
    assert_eq!(a.next(), Some(0));
}
