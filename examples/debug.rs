extern crate bignum;
use std::str::FromStr;
use bignum::error::{ErrorType, Error};

fn main() { 
    let a = bignum::BigNum::from_str("12345").unwrap();

    println!("Bignum -> Orig: {}, Debug: {:?}", a, a);
    println!("Error -> Debug: {:?}", Error::new(ErrorType::NonNumeric,
                                                "Non numeric string detected"));
}
