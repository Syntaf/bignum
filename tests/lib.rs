extern crate bignum;

#[test]
fn create_from_string() {
    let test = "123456";
    let test2 = "abs12345bh2";

    let a = bignum::BigNum::new(&test);
    let b = bignum::BigNum::new(&test2);

    assert_eq!(&a + &b, bignum::BigNum::new("246908"));
}

#[test]
fn adding_no_carry() {
    let a = bignum::BigNum::new("3333333333");
    assert_eq!(&a + &a, bignum::BigNum::new("6666666666"));
}

