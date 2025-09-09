//! src/group/4-ecdsa/3-sign.rs
use super::params::ECDSAParams;
use super::key::ECDSAKeyPair;
use super::super::ecc::scalar::scalar_mul;
use super::super::ecc::field::mod_inverse;

/// Sign a message `m` using private key d and nonce k
pub fn sign(params: &ECDSAParams, key: &ECDSAKeyPair, m: i64, k: i64) -> Option<(i64, i64)> {
    if k <= 0 || k >= params.n {
        return None;
    }

    let R = scalar_mul(k, params.G, &params.curve);
    let r = R.x.rem_euclid(params.n);
    if r == 0 { return None; }

    let k_inv = mod_inverse(k, params.n)?;
    let s = (k_inv * (m + key.private * r)).rem_euclid(params.n);
    if s == 0 { return None; }

    Some((r, s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::params::ECDSAParams;
    use super::super::key::keygen;

    #[test]
    fn test_sign() {
        let params = ECDSAParams::example();
        let keypair = keygen(&params, 3);
        let sig = sign(&params, &keypair, 7, 2).unwrap();
        assert!(sig.0 > 0 && sig.1 > 0);
    }
}
