use super::gcd::gcd;
use super::fermat::mod_pow;

/// Compute Euler's Totient function φ(n)
pub fn phi(n: i64) -> i64 {
    let mut result = n;
    let mut p = 2;
    let mut nn = n;

    while p * p <= nn {
        if nn % p == 0 {
            while nn % p == 0 {
                nn /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if nn > 1 {
        result -= result / nn;
    }
    result
}

/// Euler's theorem check: a^φ(n) ≡ 1 (mod n)
pub fn euler_theorem(a: i64, n: i64) -> bool {
    if gcd(a, n) != 1 {
        return false;
    }
    let phi_n = phi(n);
    mod_pow(a, phi_n, n) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi() {
        assert_eq!(phi(9), 6);   // {1,2,4,5,7,8}
        assert_eq!(phi(10), 4);  // {1,3,7,9}
    }

    #[test]
    fn test_euler_theorem() {
        assert!(euler_theorem(3, 10)); // 3^φ(10)=3^4=81 ≡ 1 mod 10
        assert!(euler_theorem(2, 9));  // 2^6=64 ≡ 1 mod 9
    }
}
