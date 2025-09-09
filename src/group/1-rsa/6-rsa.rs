// src/group/1-rsa/5-rsa.rs

//! Basic RSA implementation (educational).
//! Reuses:
//! - super::gcd::extended_gcd for modular inverse
//! - super::fermat::mod_pow for modular exponentiation
//! - super::euler::phi when needed (but here we compute phi directly from p,q)

use super::gcd::extended_gcd;
use super::fermat::mod_pow;

/// Simple struct to hold RSA public/private values (small-demo)
#[derive(Debug, Clone, Copy)]
pub struct KeyPair {
    pub n: i64, // modulus
    pub e: i64, // public exponent
    pub d: i64, // private exponent
    pub p: i64, // prime 1
    pub q: i64, // prime 2
}

/// Compute modular inverse of `a` modulo `m` using extended_gcd.
/// Returns Some(inv) where 0 <= inv < m, or None if inverse doesn't exist.
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 && g != -1 {
        None
    } else {
        // normalize x to positive modulo m
        let mut inv = x % m;
        if inv < 0 {
            inv += m.abs();
        }
        Some(inv)
    }
}

/// Very small and simple primality check (trial division).
/// Good enough for tiny demo primes; replace with Miller-Rabin for real use.
fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

/// Generate an RSA keypair given two primes p and q and a public exponent e.
/// Returns None if inputs invalid (non-prime p/q or gcd(e, phi) != 1).
pub fn keygen_from_primes(p: i64, q: i64, e: i64) -> Option<KeyPair> {
    if !is_prime(p) || !is_prime(q) || p == q {
        return None;
    }

    let n = p.checked_mul(q)?; // avoid overflow (still demo)
    let phi = (p - 1).checked_mul(q - 1)?;

    // gcd check: e must be coprime with phi
    let (g, _, _) = extended_gcd(e, phi);
    if g != 1 && g != -1 {
        return None;
    }

    // compute d = e^{-1} mod phi
    let d = mod_inverse(e, phi)?;
    Some(KeyPair { n, e, d, p, q })
}

/// Encrypt message `m` with public key (e, n). `m` must be in [0, n-1].
pub fn encrypt(m: i64, e: i64, n: i64) -> i64 {
    mod_pow(m, e, n)
}

/// Decrypt ciphertext `c` with private exponent `d` and modulus `n`.
pub fn decrypt(c: i64, d: i64, n: i64) -> i64 {
    mod_pow(c, d, n)
}

/// CRT-accelerated decryption using p, q and private exponent d.
/// Uses:
///   dp = d mod (p-1)
///   dq = d mod (q-1)
///   qinv = q^{-1} mod p
/// Then recombine as:
///   m1 = c^{dp} mod p
///   m2 = c^{dq} mod q
///   h = (qinv * (m1 - m2)) mod p
///   m = m2 + h * q
pub fn crt_decrypt(c: i64, p: i64, q: i64, d: i64) -> Option<i64> {
    // sanity checks
    if !is_prime(p) || !is_prime(q) {
        return None;
    }

    let dp = (d % (p - 1) + (p - 1)) % (p - 1);
    let dq = (d % (q - 1) + (q - 1)) % (q - 1);

    let m1 = mod_pow(c % p, dp, p);
    let m2 = mod_pow(c % q, dq, q);

    // compute q^{-1} mod p
    let qinv = mod_inverse(q % p, p)?;
    // h = qinv * (m1 - m2) mod p
    let mut h = (m1 - m2) % p;
    if h < 0 {
        h += p;
    }
    h = (qinv * h) % p;
    let m = m2 + h * q;
    Some(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa_basic_encrypt_decrypt() {
        // Classic small RSA example
        let p = 61;
        let q = 53;
        let e = 17;
        let kp = keygen_from_primes(p, q, e).expect("keygen failed");
        assert_eq!(kp.n, 61 * 53);
        assert_eq!(kp.e, 17);

        // message must be less than n
        let m: i64 = 65;
        let c = encrypt(m, kp.e, kp.n);
        let m_decrypted = decrypt(c, kp.d, kp.n);
        assert_eq!(m, m_decrypted, "standard decrypt should recover message");
    }

    #[test]
    fn test_rsa_crt_decrypt_matches() {
        let p = 61;
        let q = 53;
        let e = 17;
        let kp = keygen_from_primes(p, q, e).unwrap();

        let m: i64 = 123;
        let c = encrypt(m, kp.e, kp.n);

        // normal decrypt
        let plain1 = decrypt(c, kp.d, kp.n);

        // crt decrypt
        let plain2 = crt_decrypt(c, kp.p, kp.q, kp.d).unwrap();

        assert_eq!(plain1, plain2);
        assert_eq!(plain1, m);
    }

    #[test]
    fn test_mod_inverse_and_extended_gcd_usage() {
        // check that modular inverse via extended_gcd works in a small example
        let a = 17;
        let m = 3120; // phi(61*53)
        let inv = mod_inverse(a, m).expect("inverse should exist");
        // inv should satisfy a*inv ≡ 1 (mod m)
        assert_eq!((a * inv) % m, 1 % m);
    }

    #[test]
    fn keygen_rejects_bad_inputs() {
        // non-prime inputs
        assert!(keygen_from_primes(4, 9, 3).is_none());
        // p == q
        assert!(keygen_from_primes(11, 11, 3).is_none());
        // e not coprime with phi
        assert!(keygen_from_primes(7, 11, 6).is_none()); // phi=60, gcd(6,60)=6
    }
}
