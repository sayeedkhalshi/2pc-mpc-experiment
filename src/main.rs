mod group;
mod ring;
mod class_group;

fn main() {
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

    // --- Lagrange’s Little Theorem (prime modulus check) ---
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

        // Optional: CRT-accelerated decryption
        if let Some(crt_msg) = group::rsa::rsa::crt_decrypt(cipher, kp.p, kp.q, kp.d) {
            println!("  CRT decrypted = {}", crt_msg);
        }
    } else {
        println!("RSA keygen failed!");
    }
}
