//! src/group/2-ecc/5-ecc.rs
use super::scalar::scalar_mul;
use super::curve::Curve;
use super::point::Point;

/// ECC keypair struct
#[derive(Debug)]
pub struct ECCKeyPair {
    pub private: i64,
    pub public: Point,
}

/// Generate ECC keypair (private d, public Q = d*G)
pub fn keygen(d: i64, G: Point, curve: &Curve) -> ECCKeyPair {
    let Q = scalar_mul(d, G, curve);
    ECCKeyPair { private: d, public: Q }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::curve::Curve;
    use super::super::point::Point;

    #[test]
    fn test_ecc_keygen() {
        let curve = Curve { a: 2, b: 3, p: 97 };
        let G = Point { x: 3, y: 6, infinity: false };
        let keypair = keygen(20, G, &curve);
        assert!(!keypair.public.infinity);
    }
}
