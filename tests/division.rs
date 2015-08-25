extern crate bignum;
use std::str::FromStr;
use bignum::BigNum;

#[test]
fn div_simple () {
    let a = BigNum::from_u32(879034);
    assert_eq!(&a / 7, BigNum::from_u32(125576));
}

#[test]
fn div_large() {
    let a = BigNum::from_str("999999999999999999999999").unwrap();
    assert_eq!(&a / 3, BigNum::from_str("333333333333333333333333").unwrap());
}

#[test]
#[should_panic]
fn div_by_zero() {
    let a = BigNum::from_u32(15);
    let _b = &a / 0;
}

#[test]
fn div_by_larger_rhs() {
    let a = BigNum::from_u32(10);
    assert_eq!(&a / 15, BigNum::new(bignum::inits::Zero::zero()));
}
