use std::ops::{Add, Sub};
use std::fmt::{Display, Formatter, Error};
use std::cmp::max;
use std::iter::Zip;

pub struct BigNum {
    raw: Vec<u32>,
    digits: usize
}

impl Display for BigNum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self.raw)
    }
}


impl<'a> Add for &'a BigNum {
    type Output = BigNum;

    fn add(self, op: &'a BigNum) -> BigNum {
        let ciel = max(self.digits, op.digits);
        let mut carry = 0;
        for (x, y) in self.raw.iter().zip(op.raw.iter()) {
            println!("{}, {}", x, y);
        }

        BigNum::new("12345")
    }
}


impl<'a> Sub for &'a BigNum {
    type Output = BigNum;

    fn sub(self, op: &'a BigNum) -> BigNum {
        BigNum::new("12345678")
    }
}


impl BigNum {
    pub fn new(t_num: &str) -> BigNum {
        let filter_vec = t_num.chars().
            filter_map(|a| a.to_digit(10)).
            collect::<Vec<_>>();
        let t_digits = filter_vec.len();
        BigNum { raw: filter_vec, digits: t_digits }
    }
}