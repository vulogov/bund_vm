use crate::core::BUNDCore;
use rust_dynamic::ctx::{Context, CtxApplicative, NOEXTRA, JUSTONE, JUSTTWO, TAKEALL};
use rust_dynamic::value::{Value};
use rust_dynamic::types::*;
use rust_twostack::ts::{StackOp};


impl BUNDCore {
    pub fn eval_value(&mut self, _v: Value) -> Option<Value> {
        None
    }
    fn apply_and_eval(&mut self, value: Value) -> Result<&mut BUNDCore, Box<dyn std::error::Error>> {
        match self.eval(value) {
            Some(ret) => {
                self.stack.push(ret);
            }
            None => {},
        }
        Result::Ok(self)
    }
    fn apply_applicative_to_value(&mut self, app: CtxApplicative, v: Value, op: StackOp) -> Result<&mut BUNDCore, Box<dyn std::error::Error>> {
        match op {
            StackOp::None => {
                match app.extra {
                    NOEXTRA => return self.apply_and_eval(v),
                    JUSTONE => {
                        if v.attr_len() < 1 {
                            match self.stack.value(v, StackOp::TakeOne) {
                                Ok(val) => return self.apply_and_eval(val),
                                Err(err) => return Err(err),
                            }
                        }
                    }
                    JUSTTWO => {
                        if v.attr_len() == 0 {
                            match self.stack.value(v, StackOp::TakeTwo) {
                                Ok(val) => return self.apply_and_eval(val),
                                Err(err) => return Err(err),
                            }
                        } else if v.attr_len() == 1 {
                            match self.stack.value(v, StackOp::TakeOne) {
                                Ok(val) => return self.apply_and_eval(val),
                                Err(err) => return Err(err),
                            }
                        }
                    }
                    TAKEALL => {
                        match self.stack.value(v, StackOp::TakeAll) {
                            Ok(val) => return self.apply_and_eval(val),
                            Err(err) => return Err(err),
                        }
                    }
                    _ => {},
                }
            }
            StackOp::TakeOne => {
                match self.stack.value(v, StackOp::TakeOne) {
                    Ok(val) => return self.apply_and_eval(val),
                    Err(err) => return Err(err),
                }
            }
            StackOp::TakeTwo => {
                match self.stack.value(v, StackOp::TakeTwo) {
                    Ok(val) => return self.apply_and_eval(val),
                    Err(err) => return Err(err),
                }
            }
            StackOp::TakeAll => {
                match self.stack.value(v, StackOp::TakeAll) {
                    Ok(val) => return self.apply_and_eval(val),
                    Err(err) => return Err(err),
                }
            }
        }
        Result::Ok(self)
    }
    pub fn apply(&mut self, v: Value, op: StackOp) -> Result<&mut BUNDCore, Box<dyn std::error::Error>> {
        match v.dt {
            CALL => {
                match v.cast_string() {
                    Ok(name) => {
                        match self.resolve(&name) {
                            Some(app) => {
                                return self.apply_applicative_to_value(app, v, op);
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
