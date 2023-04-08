use rust_twostack::ts::TS;
use rust_dynamic::ctx::{CtxApplicative};
use rust_dynamic::value::{Value};
use std::collections::HashMap;

#[derive(Clone)]
pub struct BUNDCore{
    pub version:        String,
    pub associations:   HashMap<String, Value>,
    pub applicatives:   HashMap<String, CtxApplicative>,
    pub stack:          TS
}

impl BUNDCore {
    pub fn init() -> Self {
        Self {
            version:        env!("CARGO_PKG_VERSION").to_string(),
            stack:          TS::new(),
            associations:   HashMap::new(),
            applicatives:   HashMap::new(),
        }
    }
}
