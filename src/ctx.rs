use std::collections::VecDeque;
use crate::core::BUNDCore;
use crate::stdlib;
use rust_dynamic::ctx::{Context, CtxApplicative};
use rust_dynamic::value::{Value};


impl Context for BUNDCore {
    fn new() -> BUNDCore {
        let mut res = BUNDCore::init();
        stdlib::init_stdlib(&mut res);
        res
    }
    fn resolve(&self, name: &str) -> Option<CtxApplicative> {
        if self.applicatives.contains_key(&name.to_string()) {
            match self.applicatives.get(&name.to_string()) {
                Some(app) => return Some(app.back().unwrap().clone()),
                None => return None,
            }
        }
        None
    }
    fn get_association(&self, name: &str) -> Option<Value> {
        if self.associations.contains_key(&name.to_string()) {
            match self.associations.get(&name.to_string()) {
                Some(val) => return Some(val.back().unwrap().clone()),
                None => return None,
            }
        }
        None
    }
    fn register(&mut self, name: &str, f: CtxApplicative) -> bool {
        if ! self.applicatives.contains_key(&name.to_string()) {
            let mut q: VecDeque<CtxApplicative> = VecDeque::new();
            q.push_back(f);
            self.applicatives.insert(name.to_string(), q);
        } else {
            let q = self.applicatives.get_mut(&name.to_string());
            q.expect("Applicative queue expected").push_back(f);
        }
        true
    }
    fn register_association(&mut self, name: &str, v: Value) -> bool {
        if ! self.associations.contains_key(&name.to_string()) {
            let mut q: VecDeque<Value> = VecDeque::new();
            q.push_back(v);
            self.associations.insert(name.to_string(), q);
        } else {
            let q = self.associations.get_mut(&name.to_string());
            q.expect("Associations queue expected").push_back(v);
        }
        true
    }
    fn eval(&mut self, value: Value)  -> Option<Value> {
        self.eval_value(value)
    }

}
