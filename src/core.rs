use rust_twostack::ts::TS;

#[derive(Clone)]
pub struct BUNDCore{
    pub version:        String,
    pub stack:          TS
}

impl BUNDCore {
    pub fn init() -> Self {
        Self {
            version:    env!("CARGO_PKG_VERSION").to_string(),
            stack:      TS::new(),
        }
    }
}
