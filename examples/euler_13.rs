/// Solves problem 13 on Project Euler: link - https://projecteuler.net/problem=13
/// Calculates the first ten digits of the sum of one-hundred 50-digit numbers.

extern crate bignum;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let file = File::open(&Path::new("examples/dat/euler_13.dat")).unwrap();
    let reader = BufReader::new(file);

    // read each line and create a vector of BigNum objects
    let numbers: Vec<bignum::BigNum> = reader.lines()
        .filter_map(|x| match x {
            Ok(y) => { Some(bignum::BigNum::from_str(&y).unwrap()) },
            _     => { None }
        }).collect();

    let mut result = bignum::BigNum::new("0");
    for datum in numbers {
        result = &result + &datum;
    }

    println!("The total sum of all one hundred numbers: {}", result);
}
