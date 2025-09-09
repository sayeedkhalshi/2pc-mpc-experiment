//! src/group/4-ecdsa/2-key.rs
use super::params::ECDSAParams;
use super::super::ecc::scalar::scalar_mul;
use super::super::ecc::point::Point;

/// ECDSA keypair
#[derive(Debug)]
pub struct ECDSAKeyPair {
    pub private: i64, // private key d
    pub public: Point, // public key Q = d*G
}

/// Generate ECDSA keypair
pub fn keygen(params: &ECDSAParams, d: i64) -> ECDSAKeyPair {
    let Q = scalar_mul(d, params.G, &params.curve);
    ECDSAKeyPair { private: d, public: Q }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::params::ECDSAParams;

    #[test]
    fn test_keygen() {
        let params = ECDSAParams::example();
        let d = 3;
        let keypair = keygen(&params, d);
        assert_eq!(keypair.private, 3);
        assert!(!keypair.public.infinity);
    }
}
