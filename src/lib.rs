use std::ops::{Add, Sub};
use std::fmt::{Display, Formatter, Error};
use std::cmp::max;
use std::char;

/// `BigNum` takes number of arbitrary size in the form of a `&str`,
/// and allows numerous mathematical operations to be applied to itself.
/// The focus of `BigNum` is to offer enough funcionality to simulate 
/// starndard rust *dtypes*
pub struct BigNum {
    raw: Vec<u32>,
    digits: usize
}

impl Display for BigNum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.raw.iter().
               filter_map(|a| char::from_u32(*a + 0x30)).
               collect::<String>())
    }
}


impl<'a> Add for &'a BigNum {
    type Output = BigNum;

    fn add(self, op: &'a BigNum) -> BigNum {
        let (larger, smaller) = 
            if self.digits > op.digits {
                (self.raw.iter().rev(), op.raw.iter().rev())
            } else {
                (op.raw.iter().rev(), self.raw.iter().rev())
            };
        
        let mut carry = 0;
        let mut result: Vec<u32>  = Vec::new();
        
        for x in larger.zip(
                            smaller.map(|v| Some(v))
                                   .chain(::std::iter::repeat(None))) {
            let mut idx_add = match x.1 {
                Some(y) => { x.0 + y + carry},
                None    => { x.0 + carry}
            };

            carry = idx_add / 10;
            idx_add = idx_add % 10;

            result.push(idx_add);
        }

        if carry != 0 {
            result.push(carry);
        }

        BigNum { 
            digits: result.len(),
            raw: result.into_iter().rev().collect::<Vec<_>>() 
        }
        
    }
}


impl<'a> Sub for &'a BigNum {
    type Output = BigNum;

    fn sub(self, op: &'a BigNum) -> BigNum {
        BigNum::new("12345678")
    }
}


impl BigNum {
    /// Constructs a new `BigNum` from a passed `&str`.
    /// Filters all non-digits present in the string
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate bignum;
    ///
    /// let a = bignum::BigNum::new("12345567") // 12345567
    /// let b = bignum::BigNum::new("a123445")  // 123445
    /// ```
    pub fn new(t_num: &str) -> BigNum {
        let filter_vec = t_num.chars().
            filter_map(|a| a.to_digit(10)).
            collect::<Vec<_>>();
        let t_digits = filter_vec.len();
        BigNum { raw: filter_vec, digits: t_digits }
    }
}
