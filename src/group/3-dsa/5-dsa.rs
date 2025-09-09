//! src/group/3-dsa/5-dsa.rs
use super::params::DSAParams;
use super::key::keygen;
use super::sign::sign;
use super::verify::verify;

pub fn demo() {
    let params = DSAParams::example();
    let private = 3; // private key x
    let keypair = keygen(&params, private);
    println!("DSA Keypair: {:?}", keypair);

    let m = 7; // message
    let k = 2; // random nonce (must be coprime to q)
    let signature = sign(&params, &keypair, m, k).unwrap();
    println!("Message {} signed: {:?}", m, signature);

    let valid = verify(&params, keypair.public, m, signature.0, signature.1);
    println!("Signature valid? {}", valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_runs() {
        super::demo(); // just ensure it runs without panic
    }
}

