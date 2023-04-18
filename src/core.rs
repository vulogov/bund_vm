use rust_twostack::ts::TS;
use rust_dynamic::ctx::{CtxApplicative};
use rust_dynamic::value::{Value};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct BUNDCore{
    pub version:        String,
    pub associations:   HashMap<String, VecDeque<Value>>,
    pub applicatives:   HashMap<String, VecDeque<CtxApplicative>>,
    pub fun:            HashMap<String, Value>,
    pub stack:          TS
}

impl BUNDCore {
    pub fn init() -> Self {
        Self {
            version:        env!("CARGO_PKG_VERSION").to_string(),
            stack:          TS::new(),
            associations:   HashMap::new(),
            applicatives:   HashMap::new(),
            fun:            HashMap::new(),
        }
    }
}
