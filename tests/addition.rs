extern crate bignum;
use std::str::FromStr;

#[test]
fn adding_no_carry() {
    let a = bignum::BigNum::from_str("3333333333").unwrap();
    assert_eq!(&a + &a, bignum::BigNum::from_str("6666666666").unwrap());
}

#[test]
fn adding_with_carry() {
    let a = bignum::BigNum::from_str("192845").unwrap();
    let b = bignum::BigNum::from_u32(384729u32);
    assert_eq!(&a + &b, bignum::BigNum::from_str("577574").unwrap());
}
