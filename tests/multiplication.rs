extern crate bignum;
use std::str::FromStr;

#[test]
fn mult_no_carry() {
    let a = bignum::BigNum::from_u32(4);
    let b = bignum::BigNum::from_u32(2);
    assert_eq!(&a * &b, bignum::BigNum::from_u32(8));
}

#[test]
fn mult_small_carry() {
    let a = bignum::BigNum::from_u32(15);
    let b = bignum::BigNum::from_u32(2);
    assert_eq!(&a * &b, bignum::BigNum::from_u32(30));
}

#[test]
fn mult_big_num() {
    let a = bignum::BigNum::from_str("888888888888").unwrap();
    let b = bignum::BigNum::from_u32(2);
    assert_eq!(&a * &b, bignum::BigNum::from_str("1777777777776").unwrap());
}

#[test]
fn mult_zero() {
    let a = bignum::BigNum::from_u32(456);
    let b = bignum::BigNum::new(bignum::inits::Zero::zero());
    assert_eq!(&a * &b, bignum::BigNum::new(bignum::inits::Zero::zero()));
}
