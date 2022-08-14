use std::ops::{BitAnd, BitOr, BitXor, Mul, Not};

pub struct Bit {
    data: i8,
}

impl Bit {
    pub fn new(v: u8) -> Self {
        return Bit {
            data: if v == 0 { 0 } else { 1 },
        };
    }
    pub fn empty() -> Self {
        return Bit { data: 0 };
    }

    pub fn value(&self) -> i8 {
        return self.data;
    }

    pub fn is_true(&self) -> bool {
        return if self.data == 0 { false } else { true };
    }

    pub fn set(&mut self, v: isize) -> &Self {
        self.data = if v == 0 { 0 } else { 1 };
        self
    }
}

impl Default for Bit {
    fn default() -> Self {
        Bit { data: 0 }
    }
}

impl Copy for Bit {}

impl Clone for Bit {
    fn clone(&self) -> Bit {
        return *self;
    }
}

impl PartialEq for Bit {
    fn eq(&self, other: &Self) -> bool {
        return self.data == other.data;
    }
    fn ne(&self, other: &Self) -> bool {
        return self.data != other.data;
    }
}

impl BitAnd for Bit {
    type Output = Bit;
    fn bitand(self, rhs: Self) -> Bit {
        return Bit {
            data: self.data & rhs.data,
        };
    }
}

impl BitOr for Bit {
    type Output = Bit;
    fn bitor(self, rhs: Self) -> Bit {
        return Bit {
            data: self.data | rhs.data,
        };
    }
}

impl BitXor for Bit {
    type Output = Bit;
    fn bitxor(self, rhs: Self) -> Bit {
        return Bit {
            data: self.data ^ rhs.data,
        };
    }
}

impl Not for Bit {
    type Output = Bit;
    fn not(self) -> Bit {
        return Bit {
            data: !self.data & 1,
        };
    }
}

impl Mul<i8> for Bit {
    type Output = i8;
    fn mul(self, rhs: i8) -> i8 {
        return self.data * rhs;
    }
}
