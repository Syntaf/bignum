extern crate bignum;
use std::str::FromStr;
use bignum::error::{ErrorType, Error};

fn main() { 
    let a = bignum::BigNum::from_str("12345").unwrap();
    let b = bignum::BigNum::from_u32(12345);

    println!("Bignum a -> Orig: {}, Debug: {:?}", a, a);
    println!("Bignum b -> Orig: {}, Debug: {:?}", b, b);
    println!("Error -> Debug: {:?}", Error::new(ErrorType::NonNumeric,
                                                "Non numeric string detected"));
}
