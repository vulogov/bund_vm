use crate::core::BUNDCore;


impl BUNDCore  {
    pub fn is_association(&mut self, name: &str) -> bool {
        self.associations.contains_key(&name.to_string())
    }
}
