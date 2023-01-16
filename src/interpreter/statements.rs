use crate::{
    desugar::Statement,
    environment::Environment,
    error::RuntimeError::{self, *},
};

use super::{expressions::interp_expression, Value};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ControlFlow {
    Return,
    Normal,
    Break,
    Continue,
}

pub fn interp_statement(
    env: &mut Environment,
    statement: Statement,
    in_loop: bool,
) -> Result<(Value, ControlFlow), RuntimeError> {
    match statement {
        Statement::Expr(expr) => match interp_expression(env, expr)? {
            value => Ok((value, ControlFlow::Normal)),
        },
        Statement::Let(id, new_expr) => {
            let new_value = interp_expression(env, new_expr)?;
            env.insert_new_mutable_value(id, new_value);
            Ok((Value::Nil, ControlFlow::Normal))
        }
        Statement::Const(id, new_expr) => {
            let new_value = interp_expression(env, new_expr)?;
            env.insert_new_constant_value(id, new_value);
            Ok((Value::Nil, ControlFlow::Normal))
        }
        Statement::Assign(id, expr) => {
            let value = interp_expression(env, expr)?;
            env.reassign(id, value)?;
            Ok((Value::Nil, ControlFlow::Normal))
        }
        Statement::If(cond_expr, then_statement, else_statement_option) => {
            let cond_bool = match interp_expression(env, cond_expr)? {
                Value::Bool(b) => b,
                v => return Err(BadArg(v)),
            };
            if cond_bool {
                interp_statement(env, *then_statement, in_loop)
            } else {
                match else_statement_option {
                    Some(else_statement) => interp_statement(env, *else_statement, in_loop),
                    None => Ok((Value::Nil, ControlFlow::Normal)),
                }
            }
        }
        Statement::While(cond_expr, while_block) => Ok(loop {
            let cond_bool = match interp_expression(env, cond_expr.clone())? {
                Value::Bool(b) => b,
                v => return Err(BadArg(v)),
            };
            if cond_bool {
                match interp_statement(env, *while_block.clone(), true)? {
                    (value, ControlFlow::Return) => break (value, ControlFlow::Return),
                    (_, ControlFlow::Break) => break (Value::Nil, ControlFlow::Normal),
                    (_, ControlFlow::Normal) => (),
                    (_, ControlFlow::Continue) => (),
                };
            } else {
                break (Value::Nil, ControlFlow::Normal);
            }
        }),
        Statement::Return(expr_option) => match expr_option {
            Some(expr) => Ok((interp_expression(env, expr)?, ControlFlow::Return)),
            None => Ok((Value::Nil, ControlFlow::Return)),
        },
        Statement::Break => Ok((Value::Nil, ControlFlow::Break)),
        Statement::Continue => Ok((Value::Nil, ControlFlow::Continue)),
        Statement::Block(statements) => {
            let mut block_value = Value::Nil;
            let mut block_env = env.clone();
            for statement in statements {
                let statement_value = match interp_statement(&mut block_env, statement, in_loop)? {
                    (value, ControlFlow::Normal) => value,
                    (value, ControlFlow::Return) => {
                        env.update_reassigned_entries(&block_env)?;
                        return Ok((value, ControlFlow::Return));
                    }
                    (value, ControlFlow::Break) => {
                        if in_loop {
                            env.update_reassigned_entries(&block_env)?;
                            return Ok((value, ControlFlow::Break));
                        } else {
                            return Err(BreakNotInLoop);
                        }
                    }
                    (value, ControlFlow::Continue) => {
                        if in_loop {
                            env.update_reassigned_entries(&block_env)?;
                            return Ok((value, ControlFlow::Continue));
                        } else {
                            return Err(ContinueNotInLoop);
                        }
                    }
                };
                block_value = statement_value;
            }
            env.update_reassigned_entries(&block_env)?;
            return Ok((block_value, ControlFlow::Normal));
        }
    }
}
