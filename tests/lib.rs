extern crate bignum;
use std::str::FromStr;

#[test]
fn create_from_string() {
    let test = "123456";
    let test2 = "123452";

    let a = bignum::BigNum::from_str(&test).unwrap();
    let b = bignum::BigNum::from_str(&test2).unwrap();

    assert_eq!(&a + &b, bignum::BigNum::new("246908"));
}

#[test]
fn adding_no_carry() {
    let a = bignum::BigNum::from_str("3333333333").unwrap();
    assert_eq!(&a + &a, bignum::BigNum::new("6666666666"));
}

