#[path = "1-rsa/mod.rs"]
pub mod rsa;

#[path = "2-ecc/mod.rs"]
pub mod ecc;

#[path = "3-dsa/mod.rs"]
pub mod dsa;

#[path = "4-ecdsa/mod.rs"]
pub mod ecdsa;

#[path = "5-eddsa/mod.rs"]
pub mod eddsa;

// optional shared utilities
pub mod utils;

#[path ="secp256k1/mod.rs"]
pub mod secp256k1;
