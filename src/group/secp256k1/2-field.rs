// src/group/secp256k1/2-field.rs
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Zero, ToPrimitive};
use std::ops::{Add, Sub, Mul};

/// Field element mod p (p provided per operation)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    pub n: BigUint, // representative in [0, p-1]
    pub p: BigUint,
}

impl FieldElement {
    pub fn new(n: BigUint, p: &BigUint) -> Self {
        let n = n % p;
        FieldElement { n, p: p.clone() }
    }

    pub fn zero(p: &BigUint) -> Self {
        FieldElement { n: BigUint::zero(), p: p.clone() }
    }

    pub fn one(p: &BigUint) -> Self {
        FieldElement { n: BigUint::one(), p: p.clone() }
    }

    pub fn add(&self, other: &FieldElement) -> FieldElement {
        assert_eq!(self.p, other.p);
        FieldElement::new((&self.n + &other.n) % &self.p, &self.p)
    }

    pub fn sub(&self, other: &FieldElement) -> FieldElement {
        assert_eq!(self.p, other.p);
        let mut a = self.n.clone();
        let b = other.n.clone();
        let p = &self.p;
        let res = if a >= b { a - b } else { (a + p) - b };
        FieldElement::new(res % p, p)
    }

    pub fn mul(&self, other: &FieldElement) -> FieldElement {
        assert_eq!(self.p, other.p);
        FieldElement::new((&self.n * &other.n) % &self.p, &self.p)
    }

    pub fn neg(&self) -> FieldElement {
        if self.n.is_zero() {
            FieldElement::zero(&self.p)
        } else {
            FieldElement::new((&self.p - &self.n) % &self.p, &self.p)
        }
    }

    /// modular inverse using extended euclidean algorithm on signed BigInt
    pub fn inv(&self) -> Option<FieldElement> {
        // Convert to BigInt for negative possibilities
        let p_bi = BigInt::from_biguint(Sign::Plus, self.p.clone());
        let a_bi = BigInt::from_biguint(Sign::Plus, self.n.clone());
        let (g, x, _y) = extended_gcd_bigint(&a_bi, &p_bi);
        if g != BigInt::one() {
            return None;
        }
        let mut x = x;
        // ensure positive
        if x.sign() == Sign::Minus {
            x += &p_bi;
        }
        // convert back to BigUint
        match x.to_biguint() {
            Some(xu) => Some(FieldElement::new(xu % &self.p, &self.p)),
            None => None,
        }
    }

    pub fn pow(&self, exp: &BigUint) -> FieldElement {
        // modular exponentiation using BigUint::modpow
        FieldElement::new(self.n.modpow(exp, &self.p), &self.p)
    }
}

/// Extended GCD for BigInt: returns (g, x, y) such that a*x + b*y = g
pub fn extended_gcd_bigint(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        return (b.clone(), BigInt::zero(), BigInt::one());
    }
    let (g, x1, y1) = extended_gcd_bigint(&(b % a), a);
    let q = b / a;
    let x = y1 - &q * &x1;
    let y = x1;
    (g, x, y)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use num_bigint::BigUint;
//     #[test]
//     fn test_field_inv() {
//         let secp = super::super::secp256k1::params::Secp256k1::new();
//         let p = secp.p;
//         let a = FieldElement::new(BigUint::from(3u32), &p);
//         let inv = a.inv().expect("inv");
//         let prod = a.mul(&inv);
//         assert_eq!(prod.n, BigUint::one());
//     }
// }
