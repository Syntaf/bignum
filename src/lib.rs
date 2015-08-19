pub mod error;
pub mod inits;
pub mod arithmatic;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Error as fmt_Error;
use std::char;
use std::str::FromStr;
use std::iter::repeat;
use error::{Error, ErrorType};
use inits::{Zero, One};
use arithmatic::{vector_add, vector_sub};

/// BigNum is designed to wrap a simple struct containing 
/// a vector and offer the user to create these objects
/// just as they would a standard type, but `BigNum` allows
/// operations on an arbitrary size and is not limited to a 
/// limit.
///
/// # Examples
///
/// ```
/// use bignum::BigNum;
/// use bignum::inits::{One, Zero};
///
/// let a = BigNum::from_u32(5);    // 5
/// let b = &a + &One::one();       // 6
///
/// let c = &a + &b;                // 11
/// let d = &c - &b;                // 5
/// ```
///
pub struct BigNum {
    raw: Vec<u32>,
    digits: usize
}


impl Display for BigNum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt_Error> {
        // Simply display raw as if it were a number
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
        let result = vector_add(&self.raw, &op.raw);
        // return a BigNum representation of the result of addition between 
        // the two passed vectors
        BigNum { 
            digits: result.len(),
            raw: result
        } 
    }
}


impl<'a> Sub for &'a BigNum {
    type Output = BigNum;

    fn sub(self, op: &'a BigNum) -> BigNum {
        match vector_sub(&self.raw, &op.raw) {
            Ok(a) => {
                BigNum { digits: a.len(),
                         raw: a
                }
            }
            Err(e) => { panic!(e) }
        }
    }
}

impl<'a> Mul for &'a BigNum {
    type Output = BigNum;

    fn mul(self, rhs: &'a BigNum) -> BigNum {
        let mut result: Vec<u32> = Vec::new();
        // for each digit in the rhs of the statement, loop
        // through the lhs and apply a multiplication. future
        // iterations also insert '0' as the normal method would
        for (index, rhs_val) in rhs.raw.iter().enumerate() {
            let mut carry = 0;
            let mut intermediate = (0..index).map(|_| 0).collect::<Vec<u32>>();
            for lhs_val in &self.raw {
                let mut idx_mul = lhs_val * rhs_val;
                carry = idx_mul / 10;
                idx_mul = idx_mul % 10;

                intermediate.push(idx_mul);
            }
        }

        BigNum { digits: 0, raw: Vec::new() }
    }
}

impl<'a> Div for &'a BigNum {
    type Output = BigNum;

    fn div(self, rhs: &'a BigNum) -> BigNum {
        let mut result: Vec<u32> = Vec::new();
        let mut current = 0;
        for (index, lhs_val) in self.raw.iter().enumerate() {
            current = 0;            
        }
        BigNum { digits: 0, raw: result }
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
        // for each character in the string, if the given character
        // is a digit proceed to push it into `data`, otherwise 
        // return with an Error since from_str only accepts strings
        // with valid digits
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
    
    /// Create a new `BigNum` object from a vector of u32 values
    ///
    /// # Examples
    ///
    /// ```
    /// let a = bignum::BigNum::from_vec(vec![1,1,1,1,1]);
    /// ```
    ///
    pub fn from_vec(t_raw: Vec<u32>) -> BigNum {
        BigNum { digits: t_raw.len(), raw: t_raw }
    }

    //pub fn from_slice(t_raw: &[u32]) -> BigNum {
    //    
    //}

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
