#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::ctx::Context;
    use bund_vm::core::BUNDCore;
    use rust_dynamic::value::{Value};

    #[test]
    fn test_association_create() {
        let mut vm = BUNDCore::new();
        vm.register_association("answer", Value::from_int(42));
        assert!(vm.is_association("answer"));
    }
    #[test]
    fn test_association_get() {
        let mut vm = BUNDCore::new();
        vm.register_association("answer", Value::from_int(42));
        assert_eq!(vm.get_association("answer").unwrap().cast_int().unwrap(), 42);
    }
}
