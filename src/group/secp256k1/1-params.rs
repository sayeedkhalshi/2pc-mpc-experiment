// src/group/secp256k1/1-params.rs
use num_bigint::BigUint;
use num_traits::{FromPrimitive, One, Zero};

/// secp256k1 parameters (constants)
pub struct Secp256k1 {
    pub p: BigUint,
    pub a: BigUint,
    pub b: BigUint,
    pub gx: BigUint,
    pub gy: BigUint,
    pub n: BigUint,
}

impl Secp256k1 {
    pub fn new() -> Self {
        let p = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap();
        let a = BigUint::zero(); // a = 0
        let b = BigUint::from_u32(7).unwrap(); // b = 7
        let gx = BigUint::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap();
        let gy = BigUint::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap();
        let n = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap();

        Secp256k1 { p, a, b, gx, gy, n }
    }
}
