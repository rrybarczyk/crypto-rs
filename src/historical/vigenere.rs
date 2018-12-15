pub use historical::caesar::Caesar;

#[derive(Debug)]
pub struct Vigenere {
    key: String,
    text: String,
}

impl Vigenere {
    
    // A ASCII char
    const A: u8 = 'A' as u8;

    // New Vigenere Cipher
    pub fn new(text: String, key: String) -> Vigenere {
        Vigenere { text, key }
    }

    pub fn encrypt(&self) -> String {
   
        // Repeat key to have same length as text and store
        let mut key_map = Vec::new();
        for i in 0..self.text.len() {
            key_map.push(self.key.chars().nth(i%self.key.len()).unwrap());
        }

        // Use Caesar cipher to encrypt each index of plaintext with corresponding key index
        let mut encrypted = Vec::new();
        for (idx, c) in self.text.chars().enumerate() {
            encrypted
                .push(Caesar::new(c.to_string(), (key_map[idx] as u8) + Vigenere::A)
                .encrypt());
        }
        // Collect encrypted char vec into a String and return
        encrypted.into_iter().collect()
    }
}
