use super::gcd::gcd;
use super::euler::phi;
use super::fermat::mod_pow;

/// Compute order of an element `a` in multiplicative group mod n
pub fn order_of_element(a: i64, n: i64) -> i64 {
    if gcd(a, n) != 1 {
        return 0; // not in multiplicative group
    }
    let phi_n = phi(n);
    for k in 1..=phi_n {
        if mod_pow(a, k, n) == 1 {
            return k;
        }
    }
    phi_n
}

/// Check Lagrange’s theorem: order(a) | φ(n)
pub fn lagrange_theorem(a: i64, n: i64) -> bool {
    let order = order_of_element(a, n);
    if order == 0 {
        return false;
    }
    phi(n) % order == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_of_element() {
        assert_eq!(order_of_element(2, 7), 3); // 2^3 ≡ 1 mod 7
        assert_eq!(order_of_element(3, 7), 6); // 3^6 ≡ 1 mod 7
    }

    #[test]
    fn test_lagrange() {
        assert!(lagrange_theorem(2, 7)); // order(2)=3 divides φ(7)=6
        assert!(lagrange_theorem(3, 7)); // order(3)=6 divides 6
    }
}
