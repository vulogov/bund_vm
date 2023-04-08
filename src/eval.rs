use crate::core::BUNDCore;
use rust_dynamic::value::{Value};


impl BUNDCore {
    pub fn eval_value(&mut self, _v: Value) -> Option<Value> {
        None
    }
}
