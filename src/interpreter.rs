use std::{collections::HashMap, fmt};

use crate::{
    desugar::{Expr, Procedure, Statement},
    error::{
        LingerError::{self, RuntimeError},
        RuntimeError::*,
    },
    parser::Program,
    tokenizer::Operator,
};

/// A value in the Linger programming language.
#[derive(Clone, Debug)]
pub enum Value<'a> {
    /// A numerical value
    Num(i64),
    /// A boolean value
    Bool(bool),
    /// A string value
    Str(String),
    /// A function value
    Lambda(Vec<&'a str>, Vec<Statement<'a>>, Environment<'a>),
    // ! consider if Void should be an explicit value or just return an Option<Value> instead where None represents Void
    /// The void value; the value that represents the absence of a value.
    Void,
}

/// The `ControlFlow` enum is used to determine the return behavior of different statements during
/// interpretation.
enum ControlFlow {
    Return,
    Normal,
    Break,
    Continue,
}

/// Described whether a variable in the environment was most recently initialized or assigned.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ValueWas {
    Assigned,
    Initialized,
}

/// Type alias for the environment.
type Environment<'a> = HashMap<String, (Value<'a>, ValueWas)>;

/// Interprets a [Program] and returns either a [Value] or a [RuntimeError].
pub fn interp_program<'a>(p: Program<'a>) -> Result<Value, LingerError<'a>> {
    let mut initial_env = Environment::new();
    for Procedure { name, params, body } in p.procedures {
        initial_env.insert(
            name.to_string(),
            (
                Value::Lambda(params, body, Environment::new()),
                ValueWas::Initialized,
            ),
        );
    }

    return match interp_statements(initial_env, p.main, false) {
        Ok((_, value, _)) => Ok(value),
        Err(e) => Err(e),
    };
}

/// Interprets a statement
fn interp_statements<'a>(
    env: Environment<'a>,
    statements: Vec<Statement<'a>>,
    is_loop: bool,
) -> Result<(Environment<'a>, Value<'a>, ControlFlow), LingerError<'a>> {
    let mut env = env;
    let mut value = Value::Void;
    for statement in statements {
        let (updated_env, updated_value) = match interp_statement(env.clone(), statement, is_loop) {
            Ok((updated_env, updated_value, control_flow)) => match control_flow {
                ControlFlow::Return => return Ok((updated_env, updated_value, control_flow)),
                ControlFlow::Normal => (updated_env, updated_value),

                // if the statements are part of a loop, then break out of the nearest loop
                ControlFlow::Break => {
                    if is_loop {
                        return Ok((updated_env, Value::Void, ControlFlow::Break));
                    } else {
                        return Err(RuntimeError(BreakNotInLoop));
                    }
                }
                ControlFlow::Continue => {
                    if is_loop {
                        return Ok((updated_env, Value::Void, ControlFlow::Continue));
                    } else {
                        return Err(RuntimeError(ContinueNotInLoop));
                    }
                }
            },
            Err(e) => return Err(e),
        };
        env = updated_env;
        value = updated_value;
    }
    return Ok((env, value, ControlFlow::Normal));
}

fn interp_statement<'a>(
    mut env: Environment<'a>,
    statement: Statement<'a>,
    inside_loop: bool,
) -> Result<(Environment<'a>, Value<'a>, ControlFlow), LingerError<'a>> {
    match statement {
        Statement::Block(statements) => {
            let (updated_env, value, control_flow) =
                interp_statements(env.clone(), statements, inside_loop)?;
            for (var_name, (_, value_story)) in env.clone() {
                match updated_env.get(&var_name) {
                    Some((reassigned_value, ValueWas::Assigned)) => {
                        env.insert(var_name, (reassigned_value.clone(), value_story));
                    }
                    _ => (),
                };
            }
            Ok((env, value, control_flow))
        }
        Statement::Expr(expr) => match interp_expression(env.clone(), expr) {
            Ok(value) => Ok((env.clone(), value, ControlFlow::Normal)),
            Err(e) => Err(e),
        },
        Statement::Let(id, let_expr) => match interp_expression(env.clone(), let_expr) {
            Ok(value) => {
                let mut env = env.clone();
                env.insert(id.to_string(), (value, ValueWas::Initialized));
                Ok((env, Value::Void, ControlFlow::Normal))
            }
            Err(e) => Err(e),
        },
        Statement::Assign(id, new_expr) => match env.get(id) {
            Some(_) => {
                let mut updated_env = env.clone();
                updated_env.insert(
                    id.to_string(),
                    (interp_expression(env, new_expr)?, ValueWas::Assigned),
                );
                Ok((updated_env, Value::Void, ControlFlow::Normal))
            }
            None => return Err(RuntimeError(UnknownVariable(id.to_string()))),
        },
        Statement::If(cond_expr, then_block, else_statements_option) => {
            let cond_value = interp_expression(env.clone(), cond_expr)?;
            match cond_value {
                Value::Bool(b) => {
                    if b {
                        interp_statement(env.clone(), *then_block, inside_loop)
                    } else {
                        match else_statements_option {
                            Some(else_block) => {
                                interp_statement(env.clone(), *else_block, inside_loop)
                            }
                            None => Ok((env.clone(), Value::Void, ControlFlow::Normal)),
                        }
                    }
                }
                v => return Err(RuntimeError(ExpectedBool(v))),
            }
        }
        Statement::Return(expr_option) => match expr_option {
            Some(expr) => match interp_expression(env.clone(), expr) {
                Ok(value) => Ok((env, value, ControlFlow::Return)),
                Err(e) => return Err(e),
            },
            None => Ok((env, Value::Void, ControlFlow::Return)),
        },
        Statement::While(condition, while_block) => {
            let mut env = env.clone();
            return Ok(loop {
                let condition_value = interp_expression(env.clone(), condition.clone())?;
                match condition_value {
                    Value::Bool(true) => {
                        let (updated_env, body_value, body_control_flow) =
                            interp_statement(env.clone(), *while_block.clone(), true)?;
                        match body_control_flow {
                            ControlFlow::Return => {
                                break (env.clone(), body_value, body_control_flow)
                            }
                            ControlFlow::Break => break (env, Value::Void, ControlFlow::Normal),
                            ControlFlow::Normal => (),
                            ControlFlow::Continue => (),
                        };
                        env = updated_env;
                    }
                    Value::Bool(false) => break (env, Value::Void, ControlFlow::Normal),
                    _ => return Err(RuntimeError(BadArg(condition_value))),
                }
            });
        }
        Statement::Break => Ok((env, Value::Void, ControlFlow::Break)),
        Statement::Continue => Ok((env, Value::Void, ControlFlow::Continue)),
    }
}

fn interp_expression<'a>(
    env: Environment<'a>,
    expr: Expr<'a>,
) -> Result<Value<'a>, LingerError<'a>> {
    match expr {
        Expr::Num(n) => Ok(Value::Num(n)),
        Expr::Bool(b) => Ok(Value::Bool(b)),
        Expr::Str(s) => Ok(Value::Str(s)),
        Expr::Lambda(params, body) => Ok(Value::Lambda(params, body, env)),
        Expr::Var(id) => match env.get(id) {
            Some((value, _)) => Ok(value.clone()),
            None => Err(RuntimeError(UnknownVariable(id.to_string()))),
        },
        Expr::Binary(op, left, right) => match op {
            Operator::Plus => match (
                interp_expression(env.clone(), *left)?,
                interp_expression(env.clone(), *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Num(num_left + num_right))
                }
                (Value::Str(num_left), Value::Str(num_right)) => {
                    Ok(Value::Str(num_left + num_right.as_str()))
                }
                (Value::Num(_), v) => Err(RuntimeError(BadArg(v))),
                (v, _) => Err(RuntimeError(BadArg(v))),
            },
            Operator::Minus => match (
                interp_expression(env.clone(), *left)?,
                interp_expression(env.clone(), *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Num(num_left - num_right))
                }
                (Value::Num(_), v) => Err(RuntimeError(BadArg(v))),
                (v, _) => Err(RuntimeError(BadArg(v))),
            },
            Operator::Eq => match (
                interp_expression(env.clone(), *left)?,
                interp_expression(env.clone(), *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left == num_right))
                }
                (Value::Bool(bool_left), Value::Bool(bool_right)) => {
                    Ok(Value::Bool(bool_left == bool_right))
                }
                (v_left, v_right) => Err(RuntimeError(BadArgs(vec![v_left, v_right]))),
            },
            Operator::Ne => match (
                interp_expression(env.clone(), *left)?,
                interp_expression(env.clone(), *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left != num_right))
                }
                (Value::Bool(bool_left), Value::Bool(bool_right)) => {
                    Ok(Value::Bool(bool_left != bool_right))
                }
                (v_left, v_right) => Err(RuntimeError(BadArgs(vec![v_left, v_right]))),
            },
            Operator::LT => match (
                interp_expression(env.clone(), *left)?,
                interp_expression(env.clone(), *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left < num_right))
                }
                (v_left, v_right) => Err(RuntimeError(BadArgs(vec![v_left, v_right]))),
            },
            Operator::GT => match (
                interp_expression(env.clone(), *left)?,
                interp_expression(env.clone(), *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left > num_right))
                }
                (v_left, v_right) => Err(RuntimeError(BadArgs(vec![v_left, v_right]))),
            },
            Operator::LTE => match (
                interp_expression(env.clone(), *left)?,
                interp_expression(env.clone(), *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left <= num_right))
                }
                (v_left, v_right) => Err(RuntimeError(BadArgs(vec![v_left, v_right]))),
            },
            Operator::GTE => match (
                interp_expression(env.clone(), *left)?,
                interp_expression(env.clone(), *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left >= num_right))
                }
                (v_left, v_right) => Err(RuntimeError(BadArgs(vec![v_left, v_right]))),
            },
            Operator::LogicOr => match interp_expression(env.clone(), *left)? {
                Value::Bool(b) => match b {
                    true => Ok(Value::Bool(true)),
                    false => match interp_expression(env.clone(), *right)? {
                        Value::Bool(b) => Ok(Value::Bool(b)),
                        right_value => Err(RuntimeError(BadArg(right_value))),
                    },
                },
                left_value => Err(RuntimeError(BadArg(left_value))),
            },
            Operator::LogicAnd => match interp_expression(env.clone(), *left)? {
                Value::Bool(b) => match b {
                    false => Ok(Value::Bool(false)),
                    true => match interp_expression(env.clone(), *right)? {
                        Value::Bool(b) => Ok(Value::Bool(b)),
                        right_value => Err(RuntimeError(BadArg(right_value))),
                    },
                },
                left_value => Err(RuntimeError(BadArg(left_value))),
            },
            Operator::Times => match (
                interp_expression(env.clone(), *left)?,
                interp_expression(env.clone(), *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Num(num_left * num_right))
                }
                (v_left, v_right) => Err(RuntimeError(BadArgs(vec![v_left, v_right]))),
            },
            Operator::Mod => match (
                interp_expression(env.clone(), *left)?,
                interp_expression(env.clone(), *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Num(num_left % num_right))
                }
                (v_left, v_right) => Err(RuntimeError(BadArgs(vec![v_left, v_right]))),
            },
            Operator::Div => match (
                interp_expression(env.clone(), *left)?,
                interp_expression(env.clone(), *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Num(num_left / num_right))
                }
                (v_left, v_right) => Err(RuntimeError(BadArgs(vec![v_left, v_right]))),
            },
            op => Err(RuntimeError(UnaryAsBinary(op))),
        },
        Expr::Unary(op, arg) => {
            let arg_value = interp_expression(env, *arg)?;
            match op {
                Operator::Minus => match arg_value {
                    Value::Num(n) => Ok(Value::Num(-n)),
                    _ => Err(RuntimeError(BadArg(arg_value))),
                },
                Operator::LogicNot => match arg_value {
                    Value::Bool(b) => Ok(Value::Bool(!b)),
                    _ => Err(RuntimeError(BadArg(arg_value))),
                },
                op => Err(RuntimeError(BinaryAsUnary(op))),
            }
        }
        Expr::Call(f_expr, args) => {
            let f_name = match *f_expr {
                Expr::Var(f_name) => f_name,
                _ => "<lambda>",
            };
            let f_value = interp_expression(env.clone(), *f_expr)?;
            let (params, body, closure_env) = match f_value {
                Value::Lambda(params, body, env) => (params, body, env),
                _ => return Err(RuntimeError(BadArg(f_value))),
            };

            if params.len() != args.len() {
                return Err(RuntimeError(ArgMismatch(
                    f_name.to_string(),
                    args.len(),
                    params.len(),
                )));
            }

            let mut values: Vec<Value> = vec![];
            for expr in args {
                match interp_expression(env.clone(), expr) {
                    Ok(v) => values.push(v),
                    Err(e) => return Err(e),
                }
            }

            let mut body_env = env.clone();
            for (name, value) in closure_env {
                match body_env.get(&name) {
                    Some(_) => {
                        body_env.insert(name, value);
                    }
                    None => (),
                }
            }

            let bindings = params.iter().zip(values);
            for (param, value) in bindings {
                body_env.insert(param.to_string(), (value, ValueWas::Initialized));
            }

            return match interp_statements(body_env.clone(), body.to_vec(), false) {
                Ok((_, value, _)) => Ok(value),
                Err(e) => Err(e),
            };
        }
        Expr::PrimitiveCall(builtin, args) => match builtin {
            crate::parser::Builtin::Print => {
                let mut values: Vec<Value> = vec![];
                for expr in args {
                    match interp_expression(env.clone(), expr) {
                        Ok(v) => values.push(v),
                        Err(e) => return Err(e),
                    }
                }
                let values: Vec<String> = values.iter().map(|v| v.to_string()).collect();
                let values = values.join(" ");
                print!("{}", values);
                Ok(Value::Void)
            }
        },
    }
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Void => write!(f, "<void>"),
            Value::Str(s) => write!(f, "{}", s),
            Value::Lambda(..) => write!(f, "<lambda>"),
        }
    }
}
