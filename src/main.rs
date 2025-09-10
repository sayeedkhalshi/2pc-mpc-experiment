mod group;
mod class_group;
mod standard;
use group::secp256k1::params::Secp256k1;
use group::secp256k1::point::{AffinePoint, JacobianPoint};
fn main() {

    //! implement the standard folder here




    //! Test practice part (class_group, group folder ) below
    // --- GCD ---
    let g = group::rsa::gcd::gcd(48, 18);
    println!("GCD(48, 18) = {}", g);

    // --- CRT ---
    let a = [2, 3];
    let n = [3, 5];
    let (res, _) = group::rsa::crt::crt(&a, &n).unwrap();
    println!("CRT result: {}", res);

    // --- Fermat’s Little Theorem ---
    println!("Fermat(3,7): {}", group::rsa::fermat::fermat_theorem(3,7));

    // --- Euler’s Totient Function ---
    let n = 36;
    let phi_n = group::rsa::euler::phi(n);
    println!("Euler phi({}) = {}", n, phi_n);

    let a = 5;
    let n = 36;
    let euler_result = group::rsa::euler::euler_theorem(a, n);
    println!("Euler theorem: {}^φ({}) ≡ {} (mod {})", a, n, euler_result, n);

    // --- Lagrange’s Little Theorem ---
    let lag = group::rsa::lagrange::lagrange_theorem(2, 7);
    println!("Lagrange(2,7): {}", lag);

    // --- RSA Demo ---
    let p = 61;
    let q = 53;
    let e = 17;
    if let Some(kp) = group::rsa::rsa::keygen_from_primes(p, q, e) {
        let message: i64 = 42;
        let cipher = group::rsa::rsa::encrypt(message, kp.e, kp.n);
        let decrypted = group::rsa::rsa::decrypt(cipher, kp.d, kp.n);
        println!("RSA demo:");
        println!("  message   = {}", message);
        println!("  encrypted = {}", cipher);
        println!("  decrypted = {}", decrypted);

        if let Some(crt_msg) = group::rsa::rsa::crt_decrypt(cipher, kp.p, kp.q, kp.d) {
            println!("  CRT decrypted = {}", crt_msg);
        }
    } else {
        println!("RSA keygen failed!");
    }

    // --- ECC Demo ---
    use group::ecc::field::mod_pow;
    use group::ecc::curve::{Curve, is_on_curve};
    use group::ecc::point::{Point, point_add, point_double};
    use group::ecc::scalar::scalar_mul;
    use group::ecc::ecc::keygen;

    let curve = Curve { a: 2, b: 3, p: 97 };
    let G = Point { x: 3, y: 6, infinity: false };
    println!("G is on curve? {}", is_on_curve(G.x, G.y, &curve));

    let R = point_add(G, G, &curve);
    println!("2G = {:?}", R);

    let S = scalar_mul(20, G, &curve);
    println!("20G = {:?}", S);

    let d = 15;
    let keypair = keygen(d, G, &curve);
    println!("ECC keypair: private={}, public={:?}", keypair.private, keypair.public);

    // --- DSA Demo ---
    use group::dsa::dsa::demo as dsa_demo;
    dsa_demo();

        let secp = Secp256k1::new();
    let g_aff = AffinePoint { x: secp.gx.clone(), y: secp.gy.clone(), infinity: false };
    let g_jac = JacobianPoint::from_affine(&g_aff);
    let g_back = g_jac.to_affine(&secp.p);
    println!("Back to affine: {:?}", g_back);

    // // --- ECDSA Demo ---
    // use group::ecdsa::ecdsa::demo as ecdsa_demo;
    // ecdsa_demo();

    // // --- EdDSA Demo ---
    // use group::eddsa::eddsa::demo as eddsa_demo;
    // eddsa_demo();

    // 1. Generate a keypair (2048 bits is a good default)
    let kp = group::secp256k1::pailliar::Keypair::generate(2048);
    let pk = kp.pk;
    let sk = kp.sk;

    // 2. Encrypt a plaintext message
    let m =num_bigint::BigInt::from(123456789u64);
    let c = pk.encrypt(m.clone()).unwrap();
    println!("Ciphertext: {}", &c.0);

    // 3. Decrypt back
    let d = sk.decrypt(&pk, &c).unwrap();
    println!("Decrypted: {}", d);
    assert_eq!(m, d);

    // 4. Homomorphic addition
    let m1 = num_bigint::BigInt::from(42u32);
    let m2 = num_bigint::BigInt::from(99u32);

    let c1 = pk.encrypt(m1.clone()).unwrap();
    let c2 = pk.encrypt(m2.clone()).unwrap();
    let c_sum = pk.add(&c1, &c2);

    let d_sum = sk.decrypt(&pk, &c_sum).unwrap();
    println!("Homomorphic sum decrypts to: {}", d_sum);
    assert_eq!(d_sum, m1.clone() + m2.clone());

    // 5. Homomorphic scalar multiplication
    let k = num_bigint::BigInt::from(5);
    let c_scaled = pk.mul_scalar(&c1, k.clone());
    let d_scaled = sk.decrypt(&pk, &c_scaled).unwrap();
    println!("Homomorphic scalar multiply result: {}", d_scaled);
    assert_eq!(d_scaled, m1.clone() * k);

    // 6. Rerandomization (ciphertext changes, plaintext same)
    let c_rr = pk.rerandomize(&c).unwrap();
    let d_rr = sk.decrypt(&pk, &c_rr).unwrap();
    println!("Rerandomized decrypts to: {}", d_rr);
    assert_eq!(d_rr, m);

}
