use rust_twostack::ts::TS;

#[derive(Clone)]
pub struct BUNDCore{
    pub version:        String,
    pub stack:          TS
}

impl BUNDCore {
    fn init() -> Self {
        Self {
            version:    env!("CARGO_PKG_VERSION").to_string(),
            stack:      TS::new(),
        }
    }
    pub fn new() -> Self {
        let res = BUNDCore::init();
        res
    }
}
