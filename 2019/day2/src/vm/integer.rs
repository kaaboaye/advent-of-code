use crate::vm::value_type::ValueType;
use std::ops::{Add, Mul};

pub struct Integer(pub usize);

impl ValueType for Integer {
    fn new(val: usize) -> Self {
        Integer(val)
    }

    fn to_usize(self) -> usize {
        let Integer(val) = self;
        val
    }
}

impl Add for Integer {
    type Output = Integer;

    fn add(self, rhs: Self) -> Self::Output {
        let Integer(lhs) = self;
        let Integer(rhs) = rhs;
        Integer(lhs + rhs)
    }
}

impl Mul for Integer {
    type Output = Integer;

    fn mul(self, rhs: Self) -> Self::Output {
        let Integer(lhs) = self;
        let Integer(rhs) = rhs;
        Integer(lhs * rhs)
    }
}
