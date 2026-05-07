use crate::cipher::Cipher;

pub struct Modular;

impl Cipher for Modular {
    fn name(&self) -> &str {
        "Modular"
    }

    fn encrypt(&self, _text: &str, _key: &str) -> String {
        String::new()
    }

    fn requires_key(&self) -> bool {
        true
    }
}
