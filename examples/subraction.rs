extern crate bignum;

fn main() {
    let a = bignum::BigNum::from_u32(666662);
    let b = bignum::BigNum::from_u32(333333);

    println!("{}", &a - &b);

}
