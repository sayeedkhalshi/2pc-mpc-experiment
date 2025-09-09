//! src/group/2-ecc/2-curve.rs

/// Define an elliptic curve y^2 = x^3 + ax + b mod p
#[derive(Debug, Clone, Copy)]
pub struct Curve {
    pub a: i64,
    pub b: i64,
    pub p: i64, // prime modulus
}

/// Check if a point (x,y) is on the curve
pub fn is_on_curve(x: i64, y: i64, curve: &Curve) -> bool {
    let lhs = super::field::mod_pow(y, 2, curve.p);
    let rhs = (super::field::mod_pow(x, 3, curve.p) + curve.a * x + curve.b) % curve.p;
    (lhs - rhs).rem_euclid(curve.p) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_on_curve() {
        let curve = Curve { a: 2, b: 3, p: 97 };
        assert!(is_on_curve(3, 6, &curve));
        assert!(!is_on_curve(3, 5, &curve));
    }
}
