extern crate bignum;

use std::str::FromStr;

#[test]
fn subtract_no_borrow() {
    let a = bignum::BigNum::from_u32(666666666);
    let b = bignum::BigNum::from_u32(333333333);
    assert_eq!(&a - &b, bignum::BigNum::from_u32(333333333));
}

#[test]
fn subtract_with_borrow() {
    let a = bignum::BigNum::from_u32(666626);
    let b = bignum::BigNum::from_u32(333333);
    assert_eq!(&a - &b, bignum::BigNum::from_u32(333293));
}

#[test]
fn subtract_with_many_borrow() {
    let a = bignum::BigNum::from_u32(900045);
    let b = bignum::BigNum::from_u32(52);
    assert_eq!(&a - &b, bignum::BigNum::from_u32(899993));
}

#[test]
fn subtract_with_varying_lengths() {
    let a = bignum::BigNum::from_u32(20000);
    let b = bignum::BigNum::from_u32(100);
    assert_eq!(&a - &b, bignum::BigNum::from_u32(19900));

    let c = bignum::BigNum::from_u32(192);
    let d = bignum::BigNum::from_u32(7);
    assert_eq!(&c - &d, bignum::BigNum::from_u32(185));
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
fn negative_result_subtract2() {
    let a = bignum::BigNum::from_u32(10);
    let b = bignum::BigNum::from_u32(20);

    let _c = &a - &b;
}

#[test]
#[should_panic]
fn negative_result_subtract3() {
    let a = bignum::BigNum::from_str("121294838759").unwrap();
    let b = bignum::BigNum::from_str("121394838759").unwrap();

    let _c = &a - &b;
}

#[test]
#[should_panic]
fn negative_result_subtract4() {
    let a = bignum::BigNum::from_u32(451245);
    let b = bignum::BigNum::from_u32(461245);

    let _c = &a - &b;
}
