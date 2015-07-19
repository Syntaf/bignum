use std::ops::{Add, Sub};
use std::fmt::{Display, Formatter, Error};
use std::cmp::max;

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
        let (larger, smaller) = if self.digits > op.digits {
            (&self, &op)
        } else {
            (&op, &self)
        };
        let mut carry = 0;
        for x in larger.raw.iter().zip(
                smaller.raw.iter().
                map(|v| Some(v)).
                chain(::std::iter::repeat(None))).
            collect::<Vec<_>>().iter().rev() {
            println!("{}, {}", x.0, x.1.unwrap());
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