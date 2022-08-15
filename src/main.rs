#![allow(non_camel_case_types, unused_mut, unused_variables, dead_code)]

mod binary;

use binary::byte::Byte;

fn main() {
    let mut a = Byte::from_dec(9);
    a.log();
    a = a.pow(Byte::from_dec(0));
    a.log();
}
