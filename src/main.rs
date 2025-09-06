mod group;
mod ring;
mod class_group;

fn main() {
    let g = group::rsa::gcd::gcd(48, 18);
    println!("GCD(48, 18) = {}", g);

    let a = [2, 3];
    let n = [3, 5];
    let (res, _) = group::rsa::crt::crt(&a, &n).unwrap();
    println!("CRT result: {}", res);
}
