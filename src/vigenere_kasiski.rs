use crate::cipher::Cipher;

pub struct VigenereKasiski;

impl Cipher for VigenereKasiski {
    fn name(&self) -> &str {
        "Vigenère + Kasiski"
    }

    fn encrypt(&self, _text: &str, _key: &str) -> String {
        String::new()
    }

    fn requires_key(&self) -> bool {
        true
    }
}
