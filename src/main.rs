use std::str;

#[derive(Debug)]
pub struct Caesar {
    pub shift: u8,
    pub text: String,
}


impl Caesar {

    const A: u8 = 'A' as u8;
    const A_TO_Z: u8 = 'Z' as u8 - Caesar::A + 1;

    pub fn new(text: String, shift: u8) -> Caesar {
        Caesar { text, shift }
    }

    pub fn encrypt(&self) -> String {
        let encrypt_shift = self.shift % Caesar::A_TO_Z;
        Caesar::caesar(encrypt_shift, &self.text)
    }

    pub fn decrypt(&self) -> String {
        let decrypt_shift = Caesar::A_TO_Z - self.shift % Caesar::A_TO_Z;
        Caesar::caesar(decrypt_shift, &self.text)
    }

    fn caesar(shift: u8, text: &str) -> String {
        text.chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map( |c| (((c.to_ascii_uppercase() as u8 - Caesar::A + shift))
                       % Caesar::A_TO_Z + Caesar::A) as char)
            .collect()
    }
}

fn main() {
    let plaintext = "rJ rYbArCzYk";
    let shift = 3;
    let encrypt1 = Caesar::new(plaintext.to_string(), shift).encrypt();
    let encrypt2 = Caesar::new(encrypt1.to_string(), shift).decrypt();

    println!("plaintext: {}", plaintext);
    println!("encrypt: {}", encrypt1);
    println!("decrypt: {}", encrypt2);
}
