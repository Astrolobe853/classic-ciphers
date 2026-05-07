use crate::cipher::Cipher;

pub struct Playfair;

impl Cipher for Playfair {
    fn name(&self) -> &str {
        "Playfair"
    }

    fn encrypt(&self, _text: &str, _key: &str) -> String {
        String::new()
    }

    fn requires_key(&self) -> bool {
        true
    }
}
