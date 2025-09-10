// -----------------------------------------------------------------------------
// ClassGroupEngine Implementation (Step 1)
// -----------------------------------------------------------------------------
// Practical prototype of a class-group homomorphic encryption engine
// -----------------------------------------------------------------------------

use num_bigint::{BigInt, RandBigInt, Sign};
use num_traits::{One, Zero};
use rand::rngs::OsRng;

pub struct ClassGroupCiphertext(pub BigInt);

pub struct ClassGroupEngineImpl {
    pub n: BigInt,       // modulus (discriminant representation)
    pub g: BigInt,       // generator in the class group
}

impl ClassGroupEngineImpl {
    pub fn new(n: BigInt, g: BigInt) -> Self {
        Self { n, g }
    }

    fn mod_positive(&self, x: BigInt) -> BigInt {
        let r = x % &self.n;
        if r.sign() == Sign::Minus { r + &self.n } else { r }
    }

    fn modexp(&self, base: BigInt, exp: BigInt) -> BigInt {
        let mut result = BigInt::one();
        let mut base = self.mod_positive(base);
        let mut exp = exp;
        while exp > BigInt::zero() {
            if &exp & BigInt::one() == BigInt::one() {
                result = self.mod_positive(result * &base);
            }
            base = self.mod_positive(&base * &base);
            exp >>= 1;
        }
        result
    }

    fn sample_random(&self) -> BigInt {
        let mut rng = OsRng;
        rng.gen_bigint_range(&BigInt::one(), &self.n)
    }
}

impl super::high_level_structure::class_group_engine::ClassGroupEngine for ClassGroupEngineImpl {
    type Ciphertext = ClassGroupCiphertext;
    type Plaintext = BigInt;

    fn encrypt(&self, m: &Self::Plaintext) -> Self::Ciphertext {
        let r = self.sample_random();
        // c = g^m * r^n mod n^2 (simplified, educational)
        let c_val = self.mod_positive(self.modexp(self.g.clone(), m.clone()) * self.modexp(r, self.n.clone()));
        ClassGroupCiphertext(c_val)
    }

    fn decrypt(&self, c: &Self::Ciphertext) -> Self::Plaintext {
        // Placeholder decryption (to be replaced with proper class group inverse)
        c.0.clone() % &self.n
    }

    fn rerandomize(&self, c: &Self::Ciphertext) -> Self::Ciphertext {
        let r = self.sample_random();
        let c_new = self.mod_positive(&c.0 * self.modexp(r, self.n.clone()));
        ClassGroupCiphertext(c_new)
    }

    fn add(&self, c1: &Self::Ciphertext, c2: &Self::Ciphertext) -> Self::Ciphertext {
        ClassGroupCiphertext(self.mod_positive(&c1.0 * &c2.0))
    }

    fn mul_scalar(&self, c: &Self::Ciphertext, k: &BigInt) -> Self::Ciphertext {
        ClassGroupCiphertext(self.modexp(c.0.clone(), k.clone()))
    }
}
