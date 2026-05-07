use crate::cipher::Cipher;

pub struct Caesar;

impl Cipher for Caesar {
    fn name(&self) -> &str {
        "Caesar"
    }

    fn encrypt(&self, _text: &str, _key: &str) -> String {
        String::new()
    }

    fn requires_key(&self) -> bool {
        true
    }
}
