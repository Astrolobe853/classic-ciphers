use crate::cipher::Cipher;

pub struct Affine;

impl Cipher for Affine {
    fn name(&self) -> &str {
        "Affine"
    }

    fn encrypt(&self, _text: &str, _key: &str) -> String {
        String::new()
    }

    fn requires_key(&self) -> bool {
        true
    }
}
