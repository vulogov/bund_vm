use rust_dynamic::value::{Value};
use rust_dynamic::ctx::{Context, CtxApplicative, NOEXTRA};

fn stdlib_print(_ctx: &dyn Context, _name: &str, _value: Value) -> Option<Value> {
    None
}

pub fn init_stdlib_print(ctx: &mut dyn Context) {
    ctx.register("print", CtxApplicative::new("print", NOEXTRA, stdlib_print));
}
