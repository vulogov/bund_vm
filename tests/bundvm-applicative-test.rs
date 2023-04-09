#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::ctx::Context;
    use bund_vm::core::BUNDCore;
    use rust_dynamic::value::{Value};

    #[test]
    fn test_is_applicative() {
        let vm = BUNDCore::new();
        assert!(vm.is_applicative("print"));
    }
    #[test]
    fn test_get_applicative() {
        let vm = BUNDCore::new();
        let app = vm.resolve("print").unwrap();
        assert_eq!(app.name, "print");
    }
}
