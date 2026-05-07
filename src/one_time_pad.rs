use crate::cipher::Cipher;

pub struct OneTimePad;

impl Cipher for OneTimePad {
    fn name(&self) -> &str {
        "One-Time Pad"
    }

    fn encrypt(&self, _text: &str, _key: &str) -> String {
        String::new()
    }

    fn requires_key(&self) -> bool {
        true
    }
}
