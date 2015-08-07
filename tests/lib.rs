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
#[should_panic]
fn bad_string() {
    let _a = bignum::BigNum::from_str("123a456").unwrap();
}


