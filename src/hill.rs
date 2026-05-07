use crate::cipher::Cipher;

pub struct Hill;

impl Cipher for Hill {
    fn name(&self) -> &str {
        "Hill"
    }

    fn encrypt(&self, _text: &str, _key: &str) -> String {
        String::new()
    }

    fn requires_key(&self) -> bool {
        true
    }
}
