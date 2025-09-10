// paillier_rs_close_to_prod.rs
// -----------------------------------------------------------------------------
// Paillier Cryptosystem (close-to-production educational implementation)
// -----------------------------------------------------------------------------
// ⚠️ SECURITY DISCLAIMER
// This implementation is designed to be *educational but practical*. It uses
// strong randomness (OsRng), probabilistic prime generation (Miller–Rabin),
// input validation, and a simple API with error types. However, it is **not**
// constant‑time (uses BigInt), lacks CCA protections, and has not undergone
// formal review. Do not use for production without expert audit and hardening.
//
// Features:
// - 2048+ bit modulus (configurable)
// - KeyGen with probable primes
// - Encrypt/Decrypt with g = n + 1
// - Homomorphic addition and scalar multiplication
// - Ciphertext re-randomization
// - Zeroizes private key on drop (best-effort)
// - Unit tests for correctness
//
// Notes:
// - Paillier is IND‑CPA (randomized). It is malleable; for CCA security, wrap
//   in a KEM+DEM or a CCA transform (e.g., Fujisaki–Okamoto style) or use
//   higher-level protocols with proofs. Not provided here.
// - CRT decryption optimization can be added; the classic λ, μ path is used
//   for clarity and fewer moving parts.
//
// Cargo.toml (add):
// [dependencies]
// num-bigint = "0.4"
// num-integer = "0.1"
// num-traits = "0.2"
// rand = "0.8"
// zeroize = { version = "1", features = ["zeroize_derive"] }
// thiserror = "1"
//
// Optional: serde = { version = "1", features = ["derive" ] }

use num_bigint::{BigInt, RandBigInt, Sign};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::rngs::OsRng;
use rand::RngCore;
use thiserror::Error;

// -----------------------------
// Utilities
// -----------------------------
fn mod_positive(x: BigInt, m: &BigInt) -> BigInt {
    let r = x % m;
    if r.sign() == Sign::Minus { r + m } else { r }
}

fn lcm(a: &BigInt, b: &BigInt) -> BigInt {
    (a * b) / a.gcd(b)
}

fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    // Extended Euclid
    let (mut t, mut new_t) = (BigInt::zero(), BigInt::one());
    let (mut r, mut new_r) = (m.clone(), mod_positive(a.clone(), m));
    while !new_r.is_zero() {
        let q = &r / &new_r;
        t = t - &q * &new_t;
        std::mem::swap(&mut t, &mut new_t);
        r = r - q * &new_r;
        std::mem::swap(&mut r, &mut new_r);
    }
    if r != BigInt::one() { return None; }
    let t = mod_positive(t, m);
    Some(t)
}

fn modexp(mut base: BigInt, mut exp: BigInt, modu: &BigInt) -> BigInt {
    // Fast exponentiation (square-and-multiply)
    let mut result = BigInt::one();
    base = mod_positive(base, modu);
    while exp > BigInt::zero() {
        if (&exp & BigInt::one()) == BigInt::one() {
            result = mod_positive(result * &base, modu);
        }
        base = mod_positive(&base * &base, modu);
        exp >>= 1;
    }
    result
}

// Miller–Rabin probable-prime test
fn is_probable_prime(n: &BigInt, rounds: usize) -> bool {
    if *n < BigInt::from(2u32) { return false; }
    // small primes
    for p in [2u32,3,5,7,11,13,17,19,23,29,31,37].iter() {
        let bp = BigInt::from(*p);
        if n == &bp { return true; }
        if (n % &bp).is_zero() { return false; }
    }
    // write n-1 = d * 2^s
    let one = BigInt::one();
    let two = BigInt::from(2u32);
    let n_minus_1 = n - &one;
    let mut d = n_minus_1.clone();
    let mut s = 0u32;
    while (&d & &one).is_zero() { d >>= 1; s += 1; }

    let mut rng = OsRng;
    'outer: for _ in 0..rounds {
        let a = rng.gen_bigint_range(&two, &(n_minus_1));
        let mut x = modexp(a, d.clone(), n);
        if x == one || x == n_minus_1 { continue 'outer; }
        for _ in 1..s {
            x = mod_positive(&x * &x, n);
            if x == n_minus_1 { continue 'outer; }
        }
        return false;
    }
    true
}

fn random_prime(bits: usize) -> BigInt {
    assert!(bits >= 256, "Use >= 256-bit primes (recommended 1024+)\n");
    let mut rng = OsRng;
    loop {
        let mut p = rng.gen_bigint(bits as u64);
        // force top and bottom bits: ensure exact bit length and odd
        let one = BigInt::one();
        let two = BigInt::from(2u32);
        p.set_bit(bits as u64 - 1, true);
        if (&p & &one).is_zero() { p = p + &one; }
        if is_probable_prime(&p, 64) { return p; }
    }
}

// -----------------------------
// Public/Private keys and API
// -----------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    pub n: BigInt,
    pub n2: BigInt, // n^2 for speed
    pub g: BigInt,  // typically g = n + 1
}

#[derive(Debug)]
pub struct PrivateKey {
    // λ = lcm(p-1, q-1)
    lambda: BigInt,
    // μ = (L(g^λ mod n^2))^{-1} mod n ; with g=n+1 => μ = λ^{-1} mod n
    mu: BigInt,
    // Optional: keep p, q for potential CRT optimizations
    p: BigInt,
    q: BigInt,
}

impl Drop for PrivateKey {
    fn drop(&mut self) {
        // best-effort zeroization: overwrite BigInts with zero values
        // Note: this does not guarantee heap memory is zeroed, but avoids leaving
        // large BigInt values in scope; for stronger guarantees, use a BigInt
        // implementation that supports Zeroize.
        self.lambda = BigInt::zero();
        self.mu = BigInt::zero();
        self.p = BigInt::zero();
        self.q = BigInt::zero();
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ciphertext(pub BigInt);

#[derive(Error, Debug)]
pub enum PaillierError {
    #[error("message out of range (expected 0 <= m < n)")]
    MessageOutOfRange,
    #[error("invalid ciphertext")]
    InvalidCiphertext,
    #[error("randomness not invertible mod n")]
    RNotInvertible,
    #[error("internal error: modular inverse not found")]
    NoInverse,
}

pub struct Keypair {
    pub pk: PublicKey,
    pub sk: PrivateKey,
}

impl Keypair {
    pub fn generate(bits_n: usize) -> Self {
        assert!(bits_n % 2 == 0, "bits_n should be even (p and q similar size)");
        let half = bits_n / 2;
        // Generate distinct primes p, q
        let mut p = random_prime(half);
        let mut q = random_prime(half);
        while p == q { q = random_prime(half); }
        let n = &p * &q;
        let n2 = &n * &n;
        let g = &n + BigInt::one(); // standard choice
        let p1 = &p - BigInt::one();
        let q1 = &q - BigInt::one();
        let lambda = lcm(&p1, &q1);
        // With g = n + 1, L(g^λ mod n^2) = λ (mod n)
        let mu = modinv(&lambda, &n).expect("lambda invertible mod n for primes p,q");

        let pk = PublicKey { n: n.clone(), n2, g };
        let sk = PrivateKey { lambda, mu, p, q };
        Keypair { pk, sk }
    }
}

impl PublicKey {
    /// Encrypt m \in [0, n)
    pub fn encrypt<M: Into<BigInt>>(&self, m: M) -> Result<Ciphertext, PaillierError> {
        let m = m.into();
        if m.sign() == Sign::Minus || m >= self.n {
            return Err(PaillierError::MessageOutOfRange);
        }
        let r = self.sample_zn_star();
        self.encrypt_with_r(m, r)
    }

    /// Encrypt with externally provided randomness r \in Z*_n. Useful for tests or protocols.
    pub fn encrypt_with_r(&self, m: BigInt, r: BigInt) -> Result<Ciphertext, PaillierError> {
        if m.sign() == Sign::Minus || m >= self.n {
            return Err(PaillierError::MessageOutOfRange);
        }
        // require gcd(r, n) = 1
        if r.gcd(&self.n) != BigInt::one() { return Err(PaillierError::RNotInvertible); }
        let c1 = modexp(self.g.clone(), m, &self.n2); // g^m mod n^2
        let c2 = modexp(r.mod_floor(&self.n).pow(1), self.n.clone(), &self.n2); // r^n mod n^2
        Ok(Ciphertext(mod_positive(c1 * c2, &self.n2)))
    }

    pub fn rerandomize(&self, c: &Ciphertext) -> Result<Ciphertext, PaillierError> {
        // multiply by s^n for fresh random s \in Z*_n
        let s = self.sample_zn_star();
        let s_to_n = modexp(s, self.n.clone(), &self.n2);
        Ok(Ciphertext(mod_positive(&c.0 * s_to_n, &self.n2)))
    }

    pub fn add(&self, c1: &Ciphertext, c2: &Ciphertext) -> Ciphertext {
        Ciphertext(mod_positive(&c1.0 * &c2.0, &self.n2))
    }

    pub fn add_plain<M: Into<BigInt>>(&self, c: &Ciphertext, m: M) -> Result<Ciphertext, PaillierError> {
        let m = m.into();
        if m.sign() == Sign::Minus || m >= self.n { return Err(PaillierError::MessageOutOfRange); }
        let g_to_m = modexp(self.g.clone(), m, &self.n2);
        Ok(Ciphertext(mod_positive(&c.0 * g_to_m, &self.n2)))
    }

    pub fn mul_scalar<S: Into<BigInt>>(&self, c: &Ciphertext, k: S) -> Ciphertext {
        let k = k.into();
        Ciphertext(modexp(c.0.clone(), k, &self.n2))
    }

    fn sample_zn_star(&self) -> BigInt {
        // uniform r in Z*_n via rejection sampling
        let mut rng = OsRng;
        loop {
            let r = rng.gen_bigint_range(&BigInt::one(), &self.n);
            if r.gcd(&self.n) == BigInt::one() { return r; }
        }
    }
}

impl PrivateKey {
    /// Decrypt c -> m in [0, n)
    pub fn decrypt(&self, pk: &PublicKey, c: &Ciphertext) -> Result<BigInt, PaillierError> {
        // Classic decryption with λ and μ
        // m = L(c^λ mod n^2) * μ mod n, where L(u) = (u - 1) / n
        let u = modexp(c.0.clone(), self.lambda.clone(), &pk.n2);
        let l = (&u - BigInt::one()) / &pk.n; // safe because u ≡ 1 (mod n)
        let m = mod_positive(l * self.mu.clone(), &pk.n);
        Ok(m)
    }
}

// Convenience: Result conversion for add_plain
trait IntoCipherResult { fn into(self) -> Result<Ciphertext, PaillierError>; }
impl IntoCipherResult for Ciphertext { fn into(self) -> Result<Ciphertext, PaillierError> { Ok(self) } }

// -----------------------------
// Example main (uncomment to run as a bin)
// -----------------------------
// fn main() {
//     // Generate 2048-bit keypair
//     let kp = Keypair::generate(2048);
//     let pk = kp.pk; let sk = kp.sk;
//
//     let m = BigInt::from(123456789u64);
//     let c = pk.encrypt(m.clone()).unwrap();
//     let d = sk.decrypt(&pk, &c).unwrap();
//     assert_eq!(m, d);
//     println!("ok: encrypt/decrypt roundtrip");
//
//     // Homomorphic addition: Enc(m1+m2) = Enc(m1)*Enc(m2)
//     let m1 = BigInt::from(42u32);
//     let m2 = BigInt::from(99u32);
//     let c1 = pk.encrypt(m1.clone()).unwrap();
//     let c2 = pk.encrypt(m2.clone()).unwrap();
//     let c_sum = pk.add(&c1, &c2);
//     let d_sum = sk.decrypt(&pk, &c_sum).unwrap();
//     assert_eq!(d_sum, m1 + m2);
//     println!("ok: homomorphic add");
// }

// -----------------------------
// Tests
// -----------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_2048() {
        let kp = Keypair::generate(2048);
        let pk = kp.pk; let sk = kp.sk;
        let m = BigInt::from(123456789u64);
        let c = pk.encrypt(m.clone()).unwrap();
        let d = sk.decrypt(&pk, &c).unwrap();
        assert_eq!(m, d);
    }

    #[test]
    fn homomorphic_add() {
        let kp = Keypair::generate(1536);
        let pk = kp.pk; let sk = kp.sk;
        let m1 = BigInt::from(42u32);
        let m2 = BigInt::from(99u32);
        let c1 = pk.encrypt(m1.clone()).unwrap();
        let c2 = pk.encrypt(m2.clone()).unwrap();
        let csum = pk.add(&c1, &c2);
        let dsum = sk.decrypt(&pk, &csum).unwrap();
        assert_eq!(dsum, m1 + m2);
    }

    #[test]
    fn homomorphic_scalar() {
        let kp = Keypair::generate(1536);
        let pk = kp.pk; let sk = kp.sk;
        let m = BigInt::from(8888u32);
        let k = BigInt::from(123u32);
        let c = pk.encrypt(m.clone()).unwrap();
        let c2 = pk.mul_scalar(&c, k.clone());
        let d2 = sk.decrypt(&pk, &c2).unwrap();
        assert_eq!(d2, m * k);
    }

    #[test]
    fn rerandomize_changes_ciphertext() {
        let kp = Keypair::generate(1024);
        let pk = kp.pk; let sk = kp.sk;
        let m = BigInt::from(777u32);
        let c = pk.encrypt(m.clone()).unwrap();
        let c_rr = pk.rerandomize(&c).unwrap();
        assert_ne!(c.0, c_rr.0);
        let d = sk.decrypt(&pk, &c_rr).unwrap();
        assert_eq!(d, m);
    }

    #[test]
    fn range_check() {
        let kp = Keypair::generate(1024);
        let pk = kp.pk;
        let mut m = pk.n.clone(); // out of range
        assert!(pk.encrypt(m.clone()).is_err());
        m -= BigInt::one();
        assert!(pk.encrypt(m).is_ok());
    }
}
