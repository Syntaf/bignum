extern crate bignum;
use std::str::FromStr;

#[test]
fn adding_no_carry() {
    let a = bignum::BigNum::from_u32(333333);
    assert_eq!(&a + &a, bignum::BigNum::from_u32(666666));
}

#[test]
fn adding_with_carry() {
    let a = bignum::BigNum::from_u32(192845);
    let b = bignum::BigNum::from_u32(384729u32);
    assert_eq!(&a + &b, bignum::BigNum::from_u32(577574));
}

#[test]
fn adding_large() {
    let a = bignum::BigNum::from_str("888888888888888").unwrap();
    let b = bignum::BigNum::from_str("111111111111111").unwrap();
    assert_eq!(&a + &b, bignum::BigNum::from_str("999999999999999").unwrap());
}
