///! 
/// 
///  Euclid's Division Theorem: returns (quotient - q, remainder - r)
/// also a = bq + r
/// 
pub fn divmod(a: i64, b: i64) -> (i64, i64){
    assert!(b != 0, "Division by zero not allowed");

    let q = a / b;
    let r = a % b;

    (q, r)
}


///! Euclid's GCD - Greatest Common Divider
/// 500BC, Back then there was no steady concept of modular arithmetic. 
/// What Greek's would do is using geometry and ratio concept to 

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let (_, r) = divmod (a,b);
        a= b;
        b=r;
    }

    a.abs()
}

/// Extended Euclidean Algorithm
/// Returns (g, x, y) such that ax + by = g = gcd (a,b)
/// 
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64){
    if b == 0 {
        return (a,1,0);
    }

    let (g, x1, y1) = extended_gcd(b, a %b);
    let x = y1;
    let y = x1 - (a/b) * y1;
    (g, x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divmod(){
        assert_eq!(divmod(17, 5), (3,2))
    }

    #[test]
    fn test_gcd(){
        assert_eq!(gcd(48, 18), 6)
    }

    #[test]
    fn test_extended_gcd(){
        let (g, x, y) = extended_gcd(200, 46);
        assert_eq!(g, 2);
        assert_eq!(200 * x + 46 * y, g);
    }
}