extern crate bignum;

fn main() {
    let a = bignum::BigNum::from_u32(900045);
    let b = bignum::BigNum::from_u32(52);

    println!("{}", &a - &b);

}
