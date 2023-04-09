use crate::core::BUNDCore;


impl BUNDCore  {
    pub fn is_applicative(&self, name: &str) -> bool {
        self.applicatives.contains_key(&name.to_string())
    }
}
