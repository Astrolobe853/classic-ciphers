pub trait Cipher {
    fn name(&self) -> &str;
    fn encrypt(&self, text: &str, key: &str) -> String;
    fn requires_key(&self) -> bool {
        true
    }
}
