//! src/group/3-dsa/1-params.rs
/// Domain parameters for DSA
#[derive(Debug, Clone, Copy)]
pub struct DSAParams {
    pub p: i64, // prime modulus
    pub q: i64, // prime divisor of p-1
    pub g: i64, // generator
}

impl DSAParams {
    /// Example small parameters for testing (do NOT use in production!)
    pub fn example() -> Self {
        // p = 23, q = 11, g = 2 is simple toy example
        DSAParams { p: 23, q: 11, g: 2 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_params() {
        let params = DSAParams::example();
        assert_eq!(params.p, 23);
        assert_eq!(params.q, 11);
        assert_eq!(params.g, 2);
    }
}

