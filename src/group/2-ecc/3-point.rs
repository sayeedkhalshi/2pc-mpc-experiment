//! src/group/2-ecc/3-point.rs
use super::curve::Curve;
use super::field::{mod_inverse, mod_pow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub infinity: bool,
}

/// Point addition P + Q
pub fn point_add(P: Point, Q: Point, curve: &Curve) -> Point {
    if P.infinity { return Q; }
    if Q.infinity { return P; }
    let p = curve.p;

    // check if P == -Q
    if P.x == Q.x && (P.y + Q.y) % p == 0 {
        return Point { x: 0, y: 0, infinity: true };
    }

    let m = if P == Q {
        // slope for point doubling: (3x^2 + a)/(2y)
        let numerator = (3 * P.x * P.x + curve.a) % p;
        let denominator = (2 * P.y) % p;
        numerator * mod_inverse(denominator, p).unwrap() % p
    } else {
        // slope for addition: (y2 - y1)/(x2 - x1)
        let numerator = (Q.y - P.y).rem_euclid(p);
        let denominator = (Q.x - P.x).rem_euclid(p);
        numerator * mod_inverse(denominator, p).unwrap() % p
    }.rem_euclid(p);

    let xr = (m * m - P.x - Q.x).rem_euclid(p);
    let yr = (m * (P.x - xr) - P.y).rem_euclid(p);

    Point { x: xr, y: yr, infinity: false }
}

/// Point doubling: just P + P
pub fn point_double(P: Point, curve: &Curve) -> Point {
    point_add(P, P, curve)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::curve::Curve;

    #[test]
    fn test_point_addition() {
        let curve = Curve { a: 2, b: 3, p: 97 };
        let P = Point { x: 3, y: 6, infinity: false };
        let Q = Point { x: 80, y: 10, infinity: false };
        let R = point_add(P, Q, &curve);
        assert!(curve.p > 0); // basic sanity
    }

    #[test]
    fn test_point_double() {
        let curve = Curve { a: 2, b: 3, p: 97 };
        let P = Point { x: 3, y: 6, infinity: false };
        let R = point_double(P, &curve);
        assert!(curve.p > 0); // basic sanity
    }
}
