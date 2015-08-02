pub mod error;
pub mod inits;
use std::ops::{Add, Sub};
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Error as fmt_Error;
use std::char;
use std::str::FromStr;
use std::iter::repeat;
use error::{Error, ErrorType};
use inits::{Zero, One};

/// `BigNum` takes number of arbitrary size in the form of a `&str`,
/// and allows numerous mathematical operations to be applied to itself.
/// The focus of `BigNum` is to offer enough funcionality to simulate 
/// starndard rust *dtypes*
pub struct BigNum {
    raw: Vec<u32>,
    digits: usize
}


impl Display for BigNum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt_Error> {
        write!(f, "{}", self.raw.iter().
               filter_map(|a| char::from_u32(*a + 0x30)).
               collect::<String>())
    }
}

impl Debug for BigNum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt_Error> {
        write!(f, "{:?}", self.raw)
    }
}

impl PartialEq for BigNum {
    fn eq(&self, other: &BigNum) -> bool {
        self.raw == other.raw
    }

    fn ne(&self, other: &BigNum) -> bool {
        self.raw != other.raw
    }
}

impl Eq for BigNum { }

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
                                   .chain(repeat(None))) {
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
        if self.digits < op.digits {
            panic!("Subtraction of unsigned numbers will result in a negative number");
        }

        if self.digits == op.digits && self.raw[0] < op.raw[0] { 
            panic!("Subtraction of unsigned numbers will result in a negative number");
        }

        let mut op1 = &self.raw;
        let mut op2 = &op.raw;

        let mut result: Vec<u32> = Vec::new();

        for i in 0..op1.len() {

            let local_result = 
                if op1[i].checked_sub(op2[i]) == None {
                    0
                } else {
                    op1[i] - op2[i]
                };
            result.push(local_result);
        }

        BigNum { 
            digits: result.len(), 
            raw: result.into_iter().rev().collect::<Vec<_>>() 
        }
    }
}

impl Zero for BigNum {
    fn zero() -> BigNum {
        BigNum { digits: 0, raw: Vec::new() }
    }
}

impl One for BigNum {
    fn one() -> BigNum { 
        BigNum { digits: 1, raw: vec![1] }
    }
}

impl FromStr for BigNum{
    type Err = Error;

    fn from_str(s: &str) -> Result<BigNum, Error> {
        let mut data: Vec<u32> = Vec::new();
        for ch in s.chars() {
            match ch.to_digit(10) {
                Some(y) => { data.push(y) }
                None    => { return Err(Error::new(ErrorType::NonNumeric,
                                                   "Non digit found while parsing")) }
            }
        }

        Ok(BigNum { digits: data.len(), raw: data })
    }
}

impl BigNum {
    /// Create a new `BigNum` object from a u32 value, useful to setting
    /// the initial value of a BigNum object that lies inside the range of
    /// a u32 value.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = bignum::BigNum::from_u32(12345);
    /// ```
    ///
    pub fn from_u32(num: u32) -> BigNum {
        let mut sig = num;
        let mut cur;
        let mut t_raw: Vec<u32> = Vec::new();
        while sig > 0 {
            cur = sig;
            sig = sig / 10;
            t_raw.push(cur - sig*10);
        }
        BigNum { digits: t_raw.len(), raw: t_raw.into_iter().rev().collect() }
    }
    
    /// Contructs a new `BigNum` object from an existing or passed u32,
    /// useful for setting initial values such as zero, one or any other.
    ///
    /// # Examples
    ///
    /// ```
    /// use bignum::inits::Zero;
    ///
    /// let a = bignum::BigNum::new(Zero::zero());
    /// ```
    ///
    pub fn new(base: BigNum) -> BigNum {
        BigNum { digits: base.digits, raw: base.raw.clone() }
    }

}
