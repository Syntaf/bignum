extern crate bignum;
use bignum::BigNum;

fn main() {
    let a = BigNum::from_u32(347);
    let b = BigNum::from_u32(29);

    println!("{} * {}", a, b);
    let c = &a * &b;
    println!("{}", c);
}
