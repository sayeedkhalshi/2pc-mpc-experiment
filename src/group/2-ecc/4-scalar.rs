//! src/group/2-ecc/4-scalar.rs
use super::point::{Point, point_add};
use super::curve::Curve;

/// Scalar multiplication k*P using double-and-add
pub fn scalar_mul(mut k: i64, mut P: Point, curve: &Curve) -> Point {
    let mut result = Point { x: 0, y: 0, infinity: true }; // identity
    let mut addend = P;

    while k > 0 {
        if k % 2 == 1 {
            result = point_add(result, addend, curve);
        }
        addend = point_add(addend, addend, curve);
        k /= 2;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::curve::Curve;
    use super::super::point::Point;

    #[test]
    fn test_scalar_mul() {
        let curve = Curve { a: 2, b: 3, p: 97 };
        let G = Point { x: 3, y: 6, infinity: false };
        let R = scalar_mul(2, G, &curve); // 2G
        assert!(!R.infinity);
    }
}
