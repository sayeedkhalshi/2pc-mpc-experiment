//! src/group/4-ecdsa/1-params.rs
use super::super::ecc::curve::Curve;
use super::super::ecc::point::Point;

/// ECDSA domain parameters
#[derive(Debug, Clone, Copy)]
pub struct ECDSAParams {
    pub curve: Curve, // elliptic curve y^2 = x^3 + ax + b mod p
    pub G: Point,     // generator point
    pub n: i64,       // order of G
}

impl ECDSAParams {
    /// Example parameters for testing (toy curve, do NOT use in production)
    pub fn example() -> Self {
        let curve = Curve { a: 2, b: 3, p: 97 };
        let G = Point { x: 3, y: 6, infinity: false };
        let n = 5; // small order for testing
        ECDSAParams { curve, G, n }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_params() {
        let params = ECDSAParams::example();
        assert_eq!(params.curve.a, 2);
        assert_eq!(params.curve.b, 3);
        assert_eq!(params.curve.p, 97);
        assert_eq!(params.G.x, 3);
        assert_eq!(params.n, 5);
    }
}
