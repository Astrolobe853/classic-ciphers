use crate::cipher::Cipher;

pub struct Atbash;

impl Cipher for Atbash {
    fn name(&self) -> &str {
        "Atbash"
    }

    fn encrypt(&self, text: &str, _key: &str) -> String {
        text.to_string()
    }

    fn requires_key(&self) -> bool {
        false
    }
}
