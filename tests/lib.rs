extern crate bignum;

// #[test]
fn equal_addition() {
    // let a = bignum::BigNum::new("123");
    // assert_eq!(&a + &a, bignum::BigNum::new("246"))
}

#[test]
fn output_testing() {
    let a = bignum::BigNum::new("11111111111111111");
    let b = bignum::BigNum::new("22222222222222222");
    let c = &a + &b;
    println!("{}", c);
}
