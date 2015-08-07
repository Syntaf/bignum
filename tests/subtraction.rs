extern crate bignum;

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
    let a = bignum::BigNum::from_u32(451245);
    let b = bignum::BigNum::from_u32(461245);

    let _c = &a - &b;
}
