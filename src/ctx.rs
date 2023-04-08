use crate::core::BUNDCore;
use rust_dynamic::ctx::{Context, CtxApplicative};
use rust_dynamic::value::{Value};


impl Context for BUNDCore {
    fn new() -> BUNDCore {
        let res = BUNDCore::init();
        res
    }
    fn resolve(&self, _name: &str) -> Option<CtxApplicative> {
        fn none_fn(_ctx: &dyn Context, _name: &str, _value: Value) -> Option<Value> {
            None
        }
        Some(CtxApplicative::new("none_fn", none_fn))
    }
    fn get_association(&self, _name: &str) -> Option<Value> {
        None
    }
    fn register(&self, _name: &str, _f: CtxApplicative) -> bool {
        true
    }
    fn eval(&self, _value: Value)  -> Option<Value> {
        None
    }

}
