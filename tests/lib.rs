extern crate bignum;
use std::str::FromStr;

#[test]
fn create_from_string() {
    let test = "123456";
    let test2 = "123452";

    let a = bignum::BigNum::from_str(&test).unwrap();
    let b = bignum::BigNum::from_str(&test2).unwrap();

    assert_eq!(&a + &b, bignum::BigNum::from_u32(246908));
}

#[test]
fn crate_from_u32() {
    let test = 12345;

    let a = bignum::BigNum::from_u32(test);
    let b = bignum::BigNum::from_u32(99999);

    assert_eq!(&a + &b, bignum::BigNum::from_u32(112344));
}

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
fn bad_string() {
    let _a = bignum::BigNum::from_str("123a456").unwrap();
}


