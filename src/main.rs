pub mod historical;
pub use historical::caesar::Caesar;
pub use historical::vigenere::Vigenere;


fn main() {
    let plaintext = "PLAINTEXT";
    let key = "KEY";
    let shift = 3;
    print_caesar(&plaintext, shift);
    print_vigenere(&plaintext, &key);
}

fn print_vigenere(plaintext: &str, key: &str) {
    let encrypt1 = Vigenere::new(plaintext.to_string(), key.to_string()).encrypt();
    println!("Vigenere: encrypt: {}", encrypt1);
}

fn print_caesar(plaintext: &str, shift: u8) {
    let encrypt1 = Caesar::new(plaintext.to_string(), shift).encrypt();
    let encrypt2 = Caesar::new(encrypt1.to_string(), shift).decrypt();

    println!("Caesar: plaintext: {}", plaintext);
    println!("Caesar: encrypt: {}", encrypt1);
    println!("Caesar: decrypt: {}", encrypt2);
}
