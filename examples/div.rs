extern crate bignum;
use bignum::BigNum;

fn main() {
    let a = BigNum::from_u32(152);
    let b = BigNum::from_u32(7);
    println!("{} / {}", a, b);
    let c = &a / &b;
    println!(" = {}", c);
}
