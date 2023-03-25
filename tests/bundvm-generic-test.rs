#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_vm::core::BUNDCore;

    #[test]
    fn test_vm_create() {
        let vm = BUNDCore::new();
        assert_eq!(vm.version, env!("CARGO_PKG_VERSION").to_string());
    }
}
