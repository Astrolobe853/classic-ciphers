use crate::cipher::Cipher;

pub struct Vigenere;

impl Cipher for Vigenere {
    fn name(&self) -> &str {
        "Vigenère"
    }

    fn encrypt(&self, text: &str, key: &str) -> String {
        let key = key.to_uppercase();
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        if key_len == 0 {
            return text.to_string();
        }

        let mut key_idx = 0;
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let first = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                    let shift =
                        (key_bytes[key_idx % key_len] as i16 - b'A' as i16).rem_euclid(26) as u8;

                    key_idx += 1;

                    let res = (c as u8 - first + shift).rem_euclid(26) + first;
                    res as char
                } else {
                    c
                }
            })
            .collect()
    }
}
