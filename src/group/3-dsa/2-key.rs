//! src/group/3-dsa/2-key.rs
use super::params::DSAParams;
use super::super::rsa::gcd::extended_gcd;
use super::super::ecc::field::mod_pow;

/// Private/public key pair for DSA
#[derive(Debug)]
pub struct DSAKeyPair {
    pub private: i64,
    pub public: i64,
}

/// Generate DSA keypair
pub fn keygen(params: &DSAParams, x: i64) -> DSAKeyPair {
    // x = private key ∈ [1, q-1]
    let y = mod_pow(params.g, x, params.p); // public key
    DSAKeyPair { private: x, public: y }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::params::DSAParams;

    #[test]
    fn test_keygen() {
        let params = DSAParams::example();
        let keypair = keygen(&params, 3);
        assert_eq!(keypair.private, 3);
        assert_eq!(keypair.public, 8); // 2^3 mod 23 = 8
    }
}
