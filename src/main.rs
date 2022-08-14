#![allow(non_camel_case_types, unused_mut, unused_variables, dead_code)]

mod binary;

use binary::byte::Byte;

fn main() {
    let a = Byte::from_dec(3) * Byte::from_dec(63);
    a.log();
}
