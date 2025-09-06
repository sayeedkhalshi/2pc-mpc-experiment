use super::gcd::extended_gcd;

/// Compute modular inverse of a modulo m using Extended GCD
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        None // inverse does not exist
    } else {
        Some((x % m + m) % m) // normalize to positive
    }
}

/// Chinese Remainder Theorem
/// Given remainders `a` and moduli `n`, returns solution modulo N = product(n)
pub fn crt(a: &[i64], n: &[i64]) -> Option<(i64, i64)> {
    assert!(a.len() == n.len(), "Input arrays must have same length");
    let k = a.len();

    let big_n: i64 = n.iter().product();
    let mut result = 0i64;

    for i in 0..k {
        let ni = n[i];
        let ai = a[i];
        let ni_hat = big_n / ni;

        if let Some(mi) = mod_inverse(ni_hat, ni) {
            result += ai * ni_hat * mi;
        } else {
            return None; // no inverse → CRT fails
        }
    }

    Some(((result % big_n + big_n) % big_n, big_n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt_two_congruences() {
        let a = [2, 3];
        let n = [3, 5];
        let (x, m) = crt(&a, &n).unwrap();
        assert_eq!(m, 15);
        assert_eq!(x, 8); // solution
    }

    #[test]
    fn test_crt_three_congruences() {
        let a = [2, 3, 2];
        let n = [3, 5, 7];
        let (x, m) = crt(&a, &n).unwrap();
        assert_eq!(m, 105);
        assert_eq!(x, 23);
    }
}
