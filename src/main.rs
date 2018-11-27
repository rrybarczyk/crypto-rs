pub mod historical;
pub use historical::caesar::Caesar;


fn main() {
    let plaintext = "sAn FrAnCiScO";
    let shift = 3;
    let encrypt1 = Caesar::new(plaintext.to_string(), shift).encrypt();
    let encrypt2 = Caesar::new(encrypt1.to_string(), shift).decrypt();

    println!("plaintext: {}", plaintext);
    println!("encrypt: {}", encrypt1);
    println!("decrypt: {}", encrypt2);
}
