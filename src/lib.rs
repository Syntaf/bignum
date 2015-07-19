use std::ops::{Add, Sub};

pub struct BigNum {
    value: String,
    digits: usize
}

impl Add for BigNum {
    type Output = BigNum;

    pub fn add(&self, op: &BigNum) -> BigNum {
        BigNum::new("123456".to_string())
    }
}

impl Sub for BigNum {
    type Output = BigNum;

    pub fn sub(&self, op: &BigNum) -> BigNum {
        BigNum::new("12345678".to_string())
    }
}

impl BigNum {
    pub fn new<S>(t_num: S) -> BigNum where S: Into<String> {
        BigNum { value: t_num, digits: t_num.len }
    }
}
