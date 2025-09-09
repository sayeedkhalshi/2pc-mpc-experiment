//! src/group/5-eddsa/3-sign.rs
use super::params::EdDSAParams;
use super::key::EdDSAKeyPair;
use super::super::ecc::scalar::scalar_mul;
use super::super::ecc::point::point_add;

/// Simple deterministic "hash" function for testing
fn hash_message(m: i64) -> i64 {
    m % 97 // replace with real hash in production
}

/// Sign a message m
pub fn sign(params: &EdDSAParams, key: &EdDSAKeyPair, m: i64) -> (i64, i64) {
    let r = hash_message(m); // deterministic nonce
    let curve = super::super::ecc::curve::Curve { a: params.a, b: params.d, p: params.p };
    let R = scalar_mul(r, params.G, &curve);
    let s = (r + key.private * R.x).rem_euclid(params.n); // simplified
    (R.x, s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::params::EdDSAParams;
    use super::super::key::{keygen, EdDSAKeyPair};

    #[test]
    fn test_sign() {
        let params = EdDSAParams::example();
        let keypair = keygen(&params, 3);
        let sig = sign(&params, &keypair, 7);
        assert!(sig.0 > 0 && sig.1 > 0);
    }
}
