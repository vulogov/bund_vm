use rust_dynamic::ctx::{Context};

pub mod stdlib_print;

pub fn init_stdlib(ctx: &mut dyn Context) {
    stdlib_print::init_stdlib_print(ctx);
}
