extern crate bignum;
use std::str::FromStr;

#[test]
fn subtract_no_carry() {
    let a = bignum::BigNum::from_str("666666666").unwrap();
    let b = bignum::BigNum::from_str("333333333").unwrap();
    assert_eq!(&a - &b, bignum::BigNum::from_str("333333333").unwrap());
}

#[test]
#[should_panic]
fn negative_result_subtract() {
    let a = bignum::BigNum::from_u32(100);
    let b = bignum::BigNum::from_u32(1000);

    let _c = &a - &b;
}

#[test]
#[should_panic]
fn negative_result_subract2() {
    let a = bignum::BigNum::from_u32(10);
    let b = bignum::BigNum::from_u32(20);

    let _c = &a - &b;
}
