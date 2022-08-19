use std::{
    fmt::Debug,
    io::BufWriter,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Shl, Shr},
    string,
};

use super::bit::Bit;

#[derive(Clone, Copy)]
pub struct Byte {
    data: [Bit; 32],
}

impl Byte {
    fn empty() -> Byte {
        Byte {
            data: [Bit::empty(); 32],
        }
    }

    fn len(&self) -> usize {
        return self.data.len();
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    pub fn to_bit(&self) -> Bit {
        let mut bit = Bit::empty();

        for i in 1..self.len() {
            bit = self[i] | bit;
        }

        return bit;
    }
}

impl Index<usize> for Byte {
    type Output = Bit;

    fn index(&self, index: usize) -> &Bit {
        &self.data[index]
    }
}

impl IndexMut<usize> for Byte {
    fn index_mut(&mut self, i: usize) -> &mut Bit {
        return &mut self.data[i];
    }
}

// Bit and
impl BitAnd for Byte {
    type Output = Byte;

    fn bitand(self, rhs: Self) -> Byte {
        let mut res = Byte::empty();

        for i in 0..res.size() {
            res[i] = self[i] & rhs[i];
        }

        return res;
    }
}

impl BitAnd<Bit> for Byte {
    type Output = Byte;

    fn bitand(self, rhs: Bit) -> Byte {
        let mut res = Byte::empty();

        for i in 0..res.size() {
            res[i] = self[i] & rhs;
        }

        return res;
    }
}

// Bit or
impl BitOr for Byte {
    type Output = Byte;

    fn bitor(self, rhs: Self) -> Byte {
        let mut res = Byte::empty();

        for i in 0..res.len() {
            res[i] = self[i] | rhs[i];
        }

        return res;
    }
}

// Bit xor
impl BitXor for Byte {
    type Output = Byte;

    fn bitxor(self, rhs: Self) -> Byte {
        let mut res = Byte::empty();

        for i in 0..res.len() {
            res[i] = self[i] ^ rhs[i];
        }

        return res;
    }
}

impl Byte {
    pub fn eq(first: &Byte, second: &Byte) -> Byte {
        let mut res = Byte::empty();
        let len = res.len();
        let diff = *first ^ *second;

        res[len - 1] = Bit::ON;

        for bit in diff.data {
            res[len - 1] = !bit & res[len - 1];
        }

        return res;
    }

    pub fn greq(first: &Byte, second: &Byte) -> Byte {
        let mut res = Byte::empty();
        let len = res.len();
        let diff = *first - *second;

        res[len - 1] = !diff[0] | Byte::eq(first, second)[len - 1];

        return res;
    }
}

// addition (done)
impl Add for Byte {
    type Output = Byte;

    fn add(self, rhs: Self) -> Byte {
        let mut res = Byte::empty();
        let mut carry = Bit::new(0);

        for i in (0..self.size()).rev() {
            res[i] = carry ^ self[i] ^ rhs[i];
            carry = (self.data[i] & rhs.data[i] & carry)
                | (!res.data[i] ^ (!self.data[i] & !rhs.data[i] & !carry));
        }

        return res;
    }
}

// Left bit shift (done)
impl Shl<usize> for Byte {
    type Output = Byte;

    fn shl(self, rhs: usize) -> Byte {
        let mut res = Byte::empty();

        for i in 0..(res.size() - rhs) {
            res[i] = self[i + rhs];
        }

        return res;
    }
}

// Right arithmetic bit shift
impl Shr<usize> for Byte {
    type Output = Byte;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut res = Byte::empty();

        for i in 0..(res.size() - rhs) {
            res[i + rhs] = self[i];
        }
        for i in 0..rhs {
            res[i] = self[0];
        }

        return res;
    }
}

// multiplication (done)
impl Mul for Byte {
    type Output = Byte;

    fn mul(self, rhs: Self) -> Byte {
        let mut res = Byte::empty();
        let mut partial = Byte::empty();

        for i in (0..self.size()).rev() {
            for x in (0..rhs.size()).rev() {
                partial[x] = self[i] & rhs[x];
            }
            res = res + (partial << self.size() - i - 1);
            partial = Byte::empty();
        }

        return res;
    }
}

// division (done)
impl Div for Byte {
    type Output = Byte;

    fn div(mut self, mut rhs: Self) -> Byte {
        let mut res = Byte::empty();
        let mut remainder = Byte::empty();
        let sign = self[0] ^ rhs[0];

        self = (self & !self[0]) | (-self & self[0]);
        rhs = (rhs & !rhs[0]) | (-rhs & rhs[0]);

        for i in 0..self.size() {
            remainder = (remainder << 1) + (Byte::from_dec(1) & self[i]);
            res[i] = Byte::greq(&remainder, &rhs).to_bit();
            remainder = (remainder & !res[i]) | ((remainder - rhs) & res[i]);
        }

        return (res & !sign) | (-res & sign);
    }
}

// Power using square & Multiply Algorithm (done)
// log
impl Byte {
    pub fn pow(self: Self, expo: Byte) -> Byte {
        let mut res = Byte::from_dec(1);

        for bit in expo.data {
            res = res * res;
            res = (res & !bit) | res * (self & bit);
        }

        return res;
    }

    pub fn log(mut self, base: Byte) -> Byte {
        let mut res = Byte::from_dec(-1);

        while self.to_dec() > 0 {
            self = self / base;
            res = res + Byte::from_dec(1);
        }

        return res;
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

        return res;
    }
}

// 2nd compliment (done)
impl Neg for Byte {
    type Output = Byte;

    fn neg(self) -> Byte {
        let mut one = Byte::empty();
        let s = one.size();

        one[s - 1].set(1);

        return !self + one;
    }
}

impl std::ops::Sub for Byte {
    type Output = Byte;

    fn sub(self, rhs: Self) -> Byte {
        return self + -rhs;
    }
}

// Implement a binary array to decimal int converter (done)
impl Byte {
    pub fn to_dec(self) -> i32 {
        let mut res = 0i32;
        let mut byte = (self & !self.data[0]) | ((-self) & self.data[0]);
        let sign = !self.data[0] * 1 + self.data[0] * -1;

        for i in 1..byte.data.len() {
            res += byte.data[i].value() as i32 * 2i32.pow((byte.data.len() - i - 1) as u32);
        }

        return res * sign as i32;
    }
}

// Implement int to binary array converter (done)
impl Byte {
    pub fn from_dec(dec: i32) -> Byte {
        let mut bits = Byte::empty();
        let mut i = 1;
        let mut u = dec.abs();

        // setting value bits
        while u != 0 {
            let x = 2i32.pow((bits.size() - 1 - i) as u32);
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
    pub fn print(&self) {
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
