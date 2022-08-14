use std::{
    fmt::Debug,
    ops::{Neg, Not},
};

use super::bit::Bit;

#[derive(Clone, Copy)]
pub struct Byte {
    data: [Bit; 8],
}

impl Byte {
    fn empty() -> Byte {
        Byte {
            data: [Bit::empty(); 8],
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

impl std::ops::Index<usize> for Byte {
    type Output = Bit;
    // fn index<'a>(&'a self, i: usize) -> &'a Bit {
    //     return &self.data[i];
    // }
    fn index(&self, index: usize) -> &Bit {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for Byte {
    fn index_mut(&mut self, i: usize) -> &mut Bit {
        return &mut self.data[i];
    }
}

// Binary addition (done)
impl std::ops::Add for Byte {
    type Output = Byte;
    fn add(self, rhs: Self) -> Byte {
        let mut res = Byte::empty();
        let mut carry = Bit::new(0);

        for i in (0..self.size()).rev() {
            // xor
            res[i] = carry ^ self[i] ^ rhs[i];

            carry = (self.data[i] & rhs.data[i] & carry)
                | (!res.data[i] ^ (!self.data[i] & !rhs.data[i] & !carry));
        }

        if self[0] == rhs[0] && self[0] != carry {
            panic!("Overflow at binary addition");
        }
        res
    }
}

// Negation / 1st compliment (done)
impl Not for Byte {
    type Output = Byte;
    fn not(self) -> Byte {
        let mut res = Byte::empty();
        for i in 0..self.size() {
            res[i] = !self[i];
        }
        res
    }
}

// 2nd compliment (done)
impl Neg for Byte {
    type Output = Byte;
    fn neg(self) -> Byte {
        let mut one = Byte::empty();
        let s = one.size();
        one[s - 1].set(1);
        !self + one
    }
}

impl std::ops::Sub for Byte {
    type Output = Byte;
    fn sub(self, rhs: Self) -> Byte {
        self + -rhs
    }
}

// Impliment a binary array to decimal int converter (done)
impl Byte {
    pub fn to_dec(&self) -> i8 {
        let mut res = 0i8;
        let mut byte = if self.data[0].is_true() {
            -*self
        } else {
            *self
        };

        let sign = !self.data[0] * 1 + self.data[0] * -1;

        for i in 1..byte.data.len() {
            res += byte.data[i].value() * 2i8.pow((byte.data.len() - i - 1) as u32);
        }

        return res * sign;
    }
}

// Impliment int to binary array converter (done)
impl Byte {
    pub fn from_dec(dec: i8) -> Byte {
        // if dec > 127 || dec < -128 {
        //     panic!("Overflow");
        // }

        let mut bits = Byte::empty();

        let mut i = 1;
        let mut u = dec.abs();

        // setting value bits
        while u != 0 {
            let x = 2i8.pow((bits.size() - 1 - i) as u32);
            if u - x >= 0 {
                bits[i].set(1);
                u -= x;
            } else {
                bits[i].set(0);
            }
            i += 1;
        }

        // 2's compliment if negative
        if dec < 0 {
            bits = -bits;
        }

        return bits;
    }
}

impl Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.to_dec().to_string())
    }
}

impl Byte {
    pub fn log(&self) {
        print!("{}: 0b", self.to_dec().to_string());
        for bit in self.data {
            print!("{}", bit.value().to_string());
        }
        print!("\n");
    }

    pub fn log_bin(&self) {
        for bit in self.data {
            print!("{}", bit.value().to_string());
        }
        print!("\n");
    }
}
