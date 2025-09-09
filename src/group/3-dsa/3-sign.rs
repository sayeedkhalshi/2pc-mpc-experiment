//! src/group/3-dsa/3-sign.rs
use super::params::DSAParams;
use super::key::DSAKeyPair;
use super::super::rsa::gcd::extended_gcd;
use super::super::ecc::field::{mod_pow, mod_inverse};

/// Sign message m using private key x and random k
pub fn sign(params: &DSAParams, key: &DSAKeyPair, m: i64, k: i64) -> Option<(i64, i64)> {
    if k <= 0 || k >= params.q {
        return None;
    }
    let r = mod_pow(params.g, k, params.p) % params.q;
    if r == 0 { return None; }

    let k_inv = mod_inverse(k, params.q)?;
    let s = (k_inv * (m + key.private * r)).rem_euclid(params.q);
    if s == 0 { return None; }

    Some((r, s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::params::DSAParams;
    use super::super::key::keygen;

    #[test]
    fn test_sign() {
        let params = DSAParams::example();
        let keypair = keygen(&params, 3);
        let sig = sign(&params, &keypair, 7, 2).unwrap();
        assert!(sig.0 > 0 && sig.1 > 0);
    }
}
