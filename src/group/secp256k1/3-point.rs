// src/group/secp256k1/3-point.rs
use num_bigint::BigUint;
use num_traits::{Zero, One};
use crate::group::secp256k1::params::Secp256k1;
use crate::group::secp256k1::field::FieldElement;

/// Affine point
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffinePoint {
    pub x: BigUint,
    pub y: BigUint,
    pub infinity: bool,
}

/// Jacobian projective point (X:Y:Z) corresponds to (X/Z^2, Y/Z^3) in affine
#[derive(Debug, Clone)]
pub struct JacobianPoint {
    pub X: BigUint,
    pub Y: BigUint,
    pub Z: BigUint,
    pub infinity: bool,
}

impl JacobianPoint {
    /// Convert affine to jacobian
    pub fn from_affine(P: &AffinePoint) -> Self {
        if P.infinity {
            JacobianPoint {
                X: BigUint::zero(),
                Y: BigUint::one(),
                Z: BigUint::zero(),
                infinity: true,
            }
        } else {
            JacobianPoint {
                X: P.x.clone(),
                Y: P.y.clone(),
                Z: BigUint::one(),
                infinity: false,
            }
        }
    }

    /// Convert jacobian -> affine (requires modular inverse of Z)
    pub fn to_affine(&self, p: &BigUint) -> AffinePoint {
        if self.infinity || self.Z.is_zero() {
            return AffinePoint { x: BigUint::zero(), y: BigUint::zero(), infinity: true };
        }
        // z_inv = Z^{-1}
        let z_fe = FieldElement::new(self.Z.clone(), p);
        let zinv_fe = z_fe.inv().expect("Z inverse");
        let zinv = zinv_fe.n;
        let zinv2 = (&zinv * &zinv) % p;
        let zinv3 = (&zinv2 * &zinv) % p;
        let x = (&self.X * &zinv2) % p;
        let y = (&self.Y * &zinv3) % p;
        AffinePoint { x, y, infinity: false }
    }
}

/// Point doubling in Jacobian coords (algorithm from "Explicit formulas for EC")
/// This implementation is written for clarity, not the utmost micro-optimization.
pub fn jacobian_double(P: &JacobianPoint, curve: &Secp256k1) -> JacobianPoint {
    if P.infinity || P.Y.is_zero() {
        return JacobianPoint { X: BigUint::zero(), Y: BigUint::one(), Z: BigUint::zero(), infinity: true };
    }
    let p = &curve.p;
    // convert to FieldElement for arithmetic convenience
    let X1 = FieldElement::new(P.X.clone(), p);
    let Y1 = FieldElement::new(P.Y.clone(), p);
    let Z1 = FieldElement::new(P.Z.clone(), p);

    // s = 4 * X1 * Y1^2
    let y1sq = Y1.pow(&BigUint::from(2u32));
    let s = FieldElement::new((BigUint::from(4u32) * &X1.n * &y1sq.n) % p, p);
    // m = 3 * X1^2 + a * Z1^4  (a=0 for secp256k1, so skip)
    let x1sq = X1.pow(&BigUint::from(2u32));
    let mut m = FieldElement::new((BigUint::from(3u32) * &x1sq.n) % p, p);
    if !curve.a.is_zero() {
        let z1pow4 = Z1.pow(&BigUint::from(4u32));
        let term = FieldElement::new((curve.a.clone() * z1pow4.n) % p, p);
        m = m.add(&term);
    }
    // X3 = m^2 - 2*s
    let m2 = m.pow(&BigUint::from(2u32));
    let x3_fe = m2.sub(&s.mul(&FieldElement::new(BigUint::from(2u32), p)));
    // Y3 = m * (s - X3) - 8 * Y1^4
    let y1pow4 = y1sq.pow(&BigUint::from(2u32));
    let y3_fe = m.mul(&s.sub(&x3_fe)).sub(&FieldElement::new((BigUint::from(8u32) * y1pow4.n) % p, p));
    // Z3 = 2 * Y1 * Z1
    let z3_fe = FieldElement::new((BigUint::from(2u32) * &Y1.n * &Z1.n) % p, p);

    JacobianPoint {
        X: x3_fe.n,
        Y: y3_fe.n,
        Z: z3_fe.n,
        infinity: false,
    }
}

/// Jacobian addition P + Q (P != Q, general formula)
pub fn jacobian_add(P: &JacobianPoint, Q: &JacobianPoint, curve: &Secp256k1) -> JacobianPoint {
    // Handle special cases
    if P.infinity {
        return Q.clone();
    }
    if Q.infinity {
        return P.clone();
    }
    let p_mod = &curve.p;

    // Convert to field elements
    let X1 = FieldElement::new(P.X.clone(), p_mod);
    let Y1 = FieldElement::new(P.Y.clone(), p_mod);
    let Z1 = FieldElement::new(P.Z.clone(), p_mod);
    let X2 = FieldElement::new(Q.X.clone(), p_mod);
    let Y2 = FieldElement::new(Q.Y.clone(), p_mod);
    let Z2 = FieldElement::new(Q.Z.clone(), p_mod);

    // U1 = X1 * Z2^2
    let z2sq = Z2.pow(&BigUint::from(2u32));
    let u1 = X1.mul(&z2sq);
    // U2 = X2 * Z1^2
    let z1sq = Z1.pow(&BigUint::from(2u32));
    let u2 = X2.mul(&z1sq);
    // S1 = Y1 * Z2^3
    let z2cub = Z2.pow(&BigUint::from(3u32));
    let s1 = Y1.mul(&z2cub);
    // S2 = Y2 * Z1^3
    let z1cub = Z1.pow(&BigUint::from(3u32));
    let s2 = Y2.mul(&z1cub);

    if u1 == u2 {
        // If S1 != S2 => P + Q = infinity, else doubling
        if s1 != s2 {
            return JacobianPoint { X: BigUint::zero(), Y: BigUint::one(), Z: BigUint::zero(), infinity: true };
        } else {
            return jacobian_double(P, curve);
        }
    }

    let h = u2.sub(&u1); // h = U2 - U1
    let i = FieldElement::new((BigUint::from(2u32) * &h.n).modpow(&BigUint::from(2u32), p_mod), p_mod); // i = (2*h)^2
    let j = h.mul(&i); // j = h * i
    let r = s2.sub(&s1).mul(&FieldElement::new(BigUint::from(2u32), p_mod)); // r = 2*(S2-S1)
    let v = u1.mul(&i); // v = U1 * i

    // X3 = r^2 - j - 2*v
    let r2 = r.pow(&BigUint::from(2u32));
    let x3 = r2.sub(&j).sub(&v.mul(&FieldElement::new(BigUint::from(2u32), p_mod)));
    // Y3 = r*(v - X3) - 2*S1*j
    let y3 = r.mul(&v.sub(&x3)).sub(&s1.mul(&j).mul(&FieldElement::new(BigUint::from(2u32), p_mod)));
    // Z3 = ((Z1+Z2)^2 - Z1^2 - Z2^2) * h
    let z1z2 = Z1.add(&Z2);
    let z1z2sq = z1z2.pow(&BigUint::from(2u32));
    let z3 = z1z2sq.sub(&z1sq).sub(&z2sq).mul(&h);

    JacobianPoint {
        X: x3.n,
        Y: y3.n,
        Z: z3.n,
        infinity: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_double_add_roundtrip() {
        let secp = Secp256k1::new();
        let G_aff = AffinePoint { x: secp.gx.clone(), y: secp.gy.clone(), infinity: false };
        let P = JacobianPoint::from_affine(&G_aff);
        let Q = jacobian_double(&P, &secp);
        let q_aff = Q.to_affine(&secp.p);
        assert!(!q_aff.infinity);
    }
}
