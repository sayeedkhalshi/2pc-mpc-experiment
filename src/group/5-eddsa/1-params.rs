//! src/group/5-eddsa/1-params.rs
use super::super::ecc::field::mod_inverse;
use super::super::ecc::point::Point;

/// Edwards curve parameters for EdDSA
#[derive(Debug, Clone, Copy)]
pub struct EdDSAParams {
    pub p: i64, // prime modulus
    pub a: i64, // curve parameter a
    pub d: i64, // curve parameter d
    pub G: Point, // generator point
    pub n: i64,  // order of G
}

impl EdDSAParams {
    /// Example small curve parameters for testing
    pub fn example() -> Self {
        EdDSAParams {
            p: 97,      // small prime for testing
            a: 1,       // twisted Edwards curve a
            d: 2,       // twisted Edwards curve d
            G: Point { x: 3, y: 6, infinity: false },
            n: 5,       // small order for testing
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_params() {
        let params = EdDSAParams::example();
        assert_eq!(params.p, 97);
        assert_eq!(params.a, 1);
        assert_eq!(params.d, 2);
        assert_eq!(params.G.x, 3);
        assert_eq!(params.n, 5);
    }
}
