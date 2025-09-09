use super::gcd::gcd;

/// Fast modular exponentiation
pub fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1i64;
    base = ((base % modulus) + modulus) % modulus; // normalize base

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    ((result % modulus) + modulus) % modulus
}

/// Fermat's Little Theorem
/// Returns true if a^(p-1) ≡ 1 (mod p), given p is prime and gcd(a,p)=1
pub fn fermat_theorem(a: i64, p: i64) -> bool {
    if gcd(a, p) != 1 {
        return false;
    }
    mod_pow(a, p - 1, p) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fermat_small_primes() {
        assert!(fermat_theorem(2, 7)); // 2^6 ≡ 1 mod 7
        assert!(fermat_theorem(3, 5)); // 3^4 ≡ 1 mod 5
        assert!(fermat_theorem(10, 13)); // 10^12 ≡ 1 mod 13
    }

    #[test]
    fn test_fermat_non_coprime() {
        assert!(!fermat_theorem(6, 9)); // gcd(6,9) != 1
    }
}
