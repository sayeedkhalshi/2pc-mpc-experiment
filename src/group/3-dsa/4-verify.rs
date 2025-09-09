//! src/group/3-dsa/4-verify.rs
use super::params::DSAParams;
use super::key::DSAKeyPair;
use super::super::ecc::field::{mod_pow, mod_inverse};

/// Verify DSA signature (r,s) for message m and public key y
pub fn verify(params: &DSAParams, public: i64, m: i64, r: i64, s: i64) -> bool {
    if r <= 0 || r >= params.q || s <= 0 || s >= params.q {
        return false;
    }

    let w = mod_inverse(s, params.q).unwrap();
    let u1 = (m * w).rem_euclid(params.q);
    let u2 = (r * w).rem_euclid(params.q);

    let v = ((mod_pow(params.g, u1, params.p) * mod_pow(public, u2, params.p)) % params.p) % params.q;
    v == r
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::params::DSAParams;
    use super::super::key::keygen;
    use super::super::sign::sign;

    #[test]
    fn test_verify() {
        let params = DSAParams::example();
        let keypair = keygen(&params, 3);
        let sig = sign(&params, &keypair, 7, 2).unwrap();
        assert!(verify(&params, keypair.public, 7, sig.0, sig.1));
    }
}
