//! src/group/4-ecdsa/5-ecdsa.rs
use super::params::ECDSAParams;
use super::key::keygen;
use super::sign::sign;
use super::verify::verify;

pub fn demo() {
    let params = ECDSAParams::example();
    let d = 3;
    let keypair = keygen(&params, d);
    println!("ECDSA Keypair: {:?}", keypair);

    let m = 7; // message
    let k = 2; // random nonce
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
        super::demo(); // ensure no panic
    }
}

       
