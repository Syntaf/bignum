extern crate bignum;
use bignum::BigNum;

fn main() {
    let a = BigNum::from_u32(152);
    println!("{} / 7", a, );
    let c = &a / 7u32;
    println!(" = {}", c);
}
