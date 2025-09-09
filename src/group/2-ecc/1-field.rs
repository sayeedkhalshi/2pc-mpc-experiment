//! src/group/2-ecc/1-field.rs
use super::super::rsa::gcd::extended_gcd;

/// Modular exponentiation: base^exp mod modulus
pub fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1i64;
    base = ((base % modulus) + modulus) % modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    ((result % modulus) + modulus) % modulus
}

/// Modular inverse using extended GCD
pub fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 && g != -1 {
        None
    } else {
        let mut inv = x % m;
        if inv < 0 {
            inv += m.abs();
        }
        Some(inv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_pow_basic() {
        assert_eq!(mod_pow(2, 10, 1000), 24); // 2^10 = 1024 ≡ 24 mod 1000
        assert_eq!(mod_pow(7, 0, 13), 1);     // a^0 ≡ 1
    }

    #[test]
    fn test_mod_inverse_basic() {
        assert_eq!(mod_inverse(3, 11), Some(4)); // 3*4=12 ≡ 1 mod 11
        assert_eq!(mod_inverse(10, 17), Some(12)); // 10*12=120 ≡ 1 mod 17
    }
}
