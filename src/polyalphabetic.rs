use crate::cipher::Cipher;

pub struct Polyalphabetic;

impl Cipher for Polyalphabetic {
    fn name(&self) -> &str {
        "Polyalphabetic Cryptanalysis"
    }

    fn encrypt(&self, _text: &str, _key: &str) -> String {
        String::new()
    }

    fn requires_key(&self) -> bool {
        true
    }
}
