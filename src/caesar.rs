use crate::cipher::Cipher;

pub struct Caesar;

impl Caesar {
    fn parse_key(key: &str) -> u8 {
        let stripped: String = key.chars().filter(|c| !c.is_whitespace()).collect();
        if let Ok(n) = stripped.parse::<i32>() {
            return ((n % 26) as u8).wrapping_rem(26);
        }
        stripped
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .next()
            .map(|c| c.to_ascii_uppercase() as u8 - b'A')
            .unwrap_or(0)
    }
}

impl Cipher for Caesar {
    fn name(&self) -> &str {
        "Caesar"
    }

    fn encrypt(&self, text: &str, key: &str) -> String {
        let shift = Self::parse_key(key);

        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                    let res = (c as u8 - base + shift).rem_euclid(26) + base;
                    res as char
                } else {
                    c
                }
            })
            .collect()
    }
}
