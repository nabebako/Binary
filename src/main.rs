#![allow(non_camel_case_types, unused_mut, unused_variables, dead_code, unused)]

mod binary;

use binary::byte::Byte;
use std::time::{Duration, Instant};

fn main() {
    let x = Byte::from_dec(100) / Byte::from_dec(20);
    x.print();
}
