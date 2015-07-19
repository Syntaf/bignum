use bignum::BigNum;

mod Zero {
    const zero: BigNum = BigNum::from_u32(0).unwrap();
}

mod One {
    const one: BigNum = BigNum::from_32(1).unwrap();
}
