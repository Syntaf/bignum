use std::ops::{Add, Sub};
use std::fmt::{Display, Formatter, Error};

pub struct BigNum {
    value: String,
    digits: usize
}

impl Display for BigNum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.value)
    }
}


impl<'a> Add for &'a BigNum {
    type Output = BigNum;

    fn add(self, op: &'a BigNum) -> BigNum {
        BigNum::new("123456".to_string())
    }
}

/*
impl<'b> Sub for &'b BigNum {
    type Output = BigNum;

    fn sub(self, op: BigNum) -> BigNum {
        BigNum::new("12345678".to_string())
    }
}
*/

impl BigNum {
    pub fn new<S>(t_num: S) -> BigNum where S: Into<String> {
        let t_value = t_num.into();
        let t_digits = t_value.len();
        BigNum { value: t_value, digits: t_digits }
    }
}