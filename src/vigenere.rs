use crate::cipher::Cipher;

pub struct Vigenere;

impl Cipher for Vigenere {
    fn name(&self) -> &str {
        "Vigenère"
    }

    fn encrypt(&self, text: &str, key: &str) -> String {
        let key_bytes: Vec<u8> = key
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_uppercase() as u8)
            .collect();
        let key_len = key_bytes.len();

        let normalized_text: String = text
            .chars()
            .filter(|c| c.is_ascii_alphabetic() || *c == ' ')
            .map(|c| if c.is_ascii_alphabetic() { c.to_ascii_lowercase() } else { c })
            .collect();

        if key_len == 0 {
            return normalized_text;
        }

        let mut key_idx = 0;
        normalized_text
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let shift =
                        (key_bytes[key_idx % key_len] as i16 - b'A' as i16).rem_euclid(26) as u8;

                    key_idx += 1;

                    let res = (c as u8 - b'a' + shift).rem_euclid(26) + b'a';
                    res as char
                } else {
                    c
                }
            })
            .collect()
    }
}
