use crate::core::BUNDCore;
use rust_dynamic::ctx::{Context, NOEXTRA, JUSTONE, TAKEALL};
use rust_dynamic::value::{Value};
use rust_dynamic::types::*;
use rust_twostack::ts::{StackOp};


impl BUNDCore {
    pub fn eval_value(&mut self, _v: Value) -> Option<Value> {
        None
    }
    pub fn apply(&mut self, v: Value, op: StackOp) -> Result<&mut BUNDCore, Box<dyn std::error::Error>> {
        match v.dt {
            CALL => {
                match v.cast_string() {
                    Ok(name) => {
                        match self.resolve(&name) {
                            Some(app) => {
                                match op {
                                    StackOp::None => {
                                        match app.extra {
                                            NOEXTRA => {

                                            }
                                            JUSTONE => {

                                            }
                                            TAKEALL => {

                                            }
                                            _ => {},
                                        }
                                    }
                                    StackOp::TakeOne => {

                                    }
                                    StackOp::TakeAll => {

                                    }
                                }
                            }
                            None => return Err(format!("Applicative not found: {}", &name).into()),
                        }
                    }
                    Err(err) => return Err(err),
                }
            }
            _ => {
                match self.stack.apply(v, op) {
                    Ok(_) => {},
                    Err(err) => return Err(err),
                }
            }
        }
        return Result::Ok(self);
    }
}
