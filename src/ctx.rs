use crate::core::BUNDCore;
use rust_dynamic::ctx::{Context, CtxApplicative};
use rust_dynamic::value::{Value};


impl Context for BUNDCore {
    fn new() -> BUNDCore {
        let res = BUNDCore::init();
        res
    }
    fn resolve(&self, name: &str) -> Option<CtxApplicative> {
        if self.applicatives.contains_key(&name.to_string()) {
            match self.applicatives.get(&name.to_string()) {
                Some(app) => return Some(app.clone()),
                None => return None,
            }
        }
        None
    }
    fn get_association(&self, name: &str) -> Option<Value> {
        if self.associations.contains_key(&name.to_string()) {
            match self.associations.get(&name.to_string()) {
                Some(val) => return Some(val.clone()),
                None => return None,
            }
        }
        None
    }
    fn register(&mut self, name: &str, f: CtxApplicative) -> bool {
        self.applicatives.insert(name.to_string(), f);
        true
    }
    fn register_association(&mut self, name: &str, v: Value) -> bool {
        self.associations.insert(name.to_string(), v);
        true
    }
    fn eval(&mut self, _value: Value)  -> Option<Value> {
        None
    }

}
