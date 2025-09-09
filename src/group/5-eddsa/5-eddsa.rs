//! src/group/5-eddsa/5-eddsa.rs
use super::params::EdDSAParams;
use super::key::{keygen};
use super::sign::sign;
use super::verify::verify;

pub fn demo() {
    let params = EdDSAParams::example();
    let d = 3;
    let keypair = keygen(&params, d);
    println!("EdDSA Keypair: {:?}", keypair);

    let m = 7;
    let (r, s) = sign(&params, &keypair, m);
    println!("Message {} signed: r={}, s={}", m, r, s);

    let valid = verify(&params, keypair.public, m, r, s);
    println!("Signature valid? {}", valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_runs() {
        super::demo();
    }
}
