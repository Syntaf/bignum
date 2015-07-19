// Performs a simple addition between two BigNum
// objects, 'a' and 'b'. The result is placed in
// 'c' which is also a BigNum object. They are 
// printed with the default formatter

extern crate bignum;

fn main() {
    let a = bignum::BigNum::new("111111111111");
    let b = bignum::BigNum::new("888888888888");

    let c = &a + &b;
    println!("Adding {a} + {b} = {c}", a=a, b=b, c=c);

    let n = bignum::BigNum::new("1631015");
    let p = bignum::BigNum::new("19950716");

    let o = &n + &p;
    println!("Adding {n} + {p} = {o}", n=n, p=p, o=o);
}
