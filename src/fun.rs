use crate::core::BUNDCore;
use rust_dynamic::value::{Value};

impl BUNDCore {
    pub fn register_fun(&mut self, name: String, v: Value) -> Result<&mut BUNDCore, Box<dyn std::error::Error>> {
        self.fun.insert(name.to_string(), v);
        Result::Ok(self)
    }
    pub fn resolve_fun(&self, name: String) -> Result<Value, Box<dyn std::error::Error>> {
        if self.fun.contains_key(&name.to_string()) {
            match self.fun.get(&name.to_string()) {
                Some(val) => return Result::Ok(val.clone()),
                None => {},
            }
        }
        Err(format!("Function {} not exists", name).into())
    }
}
