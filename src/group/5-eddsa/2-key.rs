//! src/group/5-eddsa/2-key.rs
use super::params::EdDSAParams;
use super::super::ecc::scalar::scalar_mul;
use super::super::ecc::curve::Curve; // import Curve
use super::super::ecc::point::Point;

/// EdDSA keypair
#[derive(Debug)]
pub struct EdDSAKeyPair {
    pub private: i64, // private key d
    pub public: Point, // public key A = d*G
}

/// Generate EdDSA keypair
pub fn keygen(params: &EdDSAParams, d: i64) -> EdDSAKeyPair {
    // create a Curve struct just for scalar_mul
    let curve = Curve { a: params.a, b: params.d, p: params.p }; 
    let A = scalar_mul(d, params.G, &curve);
    EdDSAKeyPair { private: d, public: A }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::params::EdDSAParams;

    #[test]
    fn test_keygen() {
        let params = EdDSAParams::example();
        let d = 3;
        let keypair = keygen(&params, d);
        assert_eq!(keypair.private, 3);
        assert!(!keypair.public.infinity);
    }
}
