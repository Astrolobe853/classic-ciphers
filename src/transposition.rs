use crate::cipher::Cipher;

pub struct Transposition;

impl Cipher for Transposition {
    fn name(&self) -> &str {
        "Transposition"
    }

    fn encrypt(&self, _text: &str, _key: &str) -> String {
        String::new()
    }

    fn requires_key(&self) -> bool {
        true
    }
}
