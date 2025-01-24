//@ run-fail
//@ compile-flags: -O -C overflow-checks=yes

#![feature(new_range_api)]

use std::range::RangeFrom;

fn main() {
    let mut a = RangeFrom::from(u8::MAX..).into_iter();
    assert_eq!(a.next(), Some(u8::MAX));
    a.next(); // panic
}
