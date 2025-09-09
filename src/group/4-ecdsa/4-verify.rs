//! src/group/4-ecdsa/4-verify.rs
use super::params::ECDSAParams;
use super::super::ecc::scalar::scalar_mul;
use super::super::ecc::point::point_add;
use super::super::ecc::field::mod_inverse;
use super::super::ecc::point::Point;

/// Verify ECDSA signature (r, s) for message m and public key Q
pub fn verify(params: &ECDSAParams, public: Point, m: i64, r: i64, s: i64) -> bool {
    if r <= 0 || r >= params.n || s <= 0 || s >= params.n {
        return false;
    }

    let w = mod_inverse(s, params.n).unwrap();
    let u1 = (m * w).rem_euclid(params.n);
    let u2 = (r * w).rem_euclid(params.n);

    let X1 = scalar_mul(u1, params.G, &params.curve);
    let X2 = scalar_mul(u2, public, &params.curve);
    let X = point_add(X1, X2, &params.curve);

    r == X.x.rem_euclid(params.n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::params::ECDSAParams;
    use super::super::key::keygen;
    use super::super::sign::sign;

    #[test]
    fn test_verify() {
        let params = ECDSAParams::example();
        let keypair = keygen(&params, 3);
        let sig = sign(&params, &keypair, 7, 2).unwrap();
        assert!(verify(&params, keypair.public, 7, sig.0, sig.1));
    }
}
