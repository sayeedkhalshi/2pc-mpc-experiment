//! src/group/5-eddsa/4-verify.rs
use super::params::EdDSAParams;
use super::super::ecc::scalar::scalar_mul;
use super::super::ecc::point::point_add;
use super::super::ecc::curve::Curve;
use super::super::ecc::point::Point;

/// Verify EdDSA signature (R_x, s)
pub fn verify(params: &EdDSAParams, public: Point, m: i64, r: i64, s: i64) -> bool {
    let curve = Curve { a: params.a, b: params.d, p: params.p };
    let R1 = scalar_mul(s, params.G, &curve);
    let R2 = scalar_mul(r, public, &curve);
    let R_check = point_add(R1, R2, &curve);
    R_check.x.rem_euclid(params.n) == r
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::params::EdDSAParams;
    use super::super::key::{keygen};
    use super::super::sign::sign;

    #[test]
    fn test_verify() {
        let params = EdDSAParams::example();
        let keypair = keygen(&params, 3);
        let (r, s) = sign(&params, &keypair, 7);
        assert!(verify(&params, keypair.public, 7, r, s));
    }
}
