use crate::cipher::Cipher;

pub struct RailFence;

impl Cipher for RailFence {
    fn name(&self) -> &str {
        "Rail Fence"
    }

    fn encrypt(&self, _text: &str, _key: &str) -> String {
        String::new()
    }

    fn requires_key(&self) -> bool {
        true
    }
}
