use crate::cipher::Cipher;

pub struct DigraphBlock;

impl Cipher for DigraphBlock {
    fn name(&self) -> &str {
        "Digraph / Block"
    }

    fn encrypt(&self, _text: &str, _key: &str) -> String {
        String::new()
    }

    fn requires_key(&self) -> bool {
        true
    }
}
