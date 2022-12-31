use std::{collections::HashMap, fmt};

use crate::{
    desugar::{Expr, Procedure, Statement, Statements},
    error::{
        LingerError::{self, RuntimeError},
        RuntimeError::*,
    },
    parser::Program,
    tokenizer::Operator,
};

#[derive(Clone, Debug)]
pub enum Value<'a> {
    Num(i64),
    Bool(bool),
    Str(String),
    Lambda(Vec<&'a str>, Statements<'a>, Environment<'a>),
    // ! consider if Void should be an explicit value or just return an Option<Value> instead where None represents Void
    Void,
}

enum ReturnFlag {
    Return,
    Continue,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ValueStory {
    Assignment,
    Initialization,
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

type Environment<'a> = HashMap<String, (Value<'a>, ValueStory)>;

pub fn interp_program<'a>(p: Program<'a>) -> Result<Value, LingerError<'a>> {
    let mut initial_env = Environment::new();
    for Procedure { name, params, body } in p.procedures {
        initial_env.insert(
            name.to_string(),
            (
                Value::Lambda(params, body, Environment::new()),
                ValueStory::Initialization,
            ),
        );
    }

    return match interp_statements(initial_env, p.main) {
        Ok((_, value, _)) => Ok(value),
        Err(e) => Err(e),
    };
}

fn interp_statements<'a>(
    env: Environment<'a>,
    statements: Statements<'a>,
) -> Result<(Environment<'a>, Value<'a>, ReturnFlag), LingerError<'a>> {
    let mut env = env;
    let mut return_value = Value::Void;
    for statement in statements {
        let is_return_statement = match statement {
            Statement::Return(_) => true,
            _ => false,
        };
        let (new_env, value) = match interp_statement(env.clone(), statement) {
            Ok((new_env, value, return_flag)) => match return_flag {
                ReturnFlag::Return => return Ok((new_env, value, return_flag)),
                ReturnFlag::Continue => (new_env, value),
            },
            Err(e) => return Err(e),
        };

        env = new_env;
        return_value = value;
        if is_return_statement {
            return Ok((env, return_value, ReturnFlag::Return));
        }
    }
    return Ok((env, return_value, ReturnFlag::Continue));
}

fn interp_statement<'a>(
    mut env: Environment<'a>,
    statement: Statement<'a>,
) -> Result<(Environment<'a>, Value<'a>, ReturnFlag), LingerError<'a>> {
    match statement {
        Statement::Expr(expr) => match interp_expression(env.clone(), expr) {
            Ok(value) => Ok((env.clone(), value, ReturnFlag::Continue)),
            Err(e) => Err(e),
        },
        Statement::Let(id, let_expr) => match interp_expression(env.clone(), let_expr) {
            Ok(value) => {
                let mut env = env.clone();
                env.insert(id.to_string(), (value, ValueStory::Initialization));
                Ok((env, Value::Void, ReturnFlag::Continue))
            }
            Err(e) => Err(e),
        },
        Statement::Assign(id, new_expr) => match env.get(id) {
            Some(_) => {
                let mut updated_env = env.clone();
                updated_env.insert(
                    id.to_string(),
                    (interp_expression(env, new_expr)?, ValueStory::Assignment),
                );
                Ok((updated_env, Value::Void, ReturnFlag::Continue))
            }
            None => return Err(RuntimeError(UnknownVariable(id.to_string()))),
        },
        Statement::If(cond_expr, then_statements, else_statements_option) => {
            let cond_value = interp_expression(env.clone(), cond_expr)?;
            match cond_value {
                Value::Bool(b) => {
                    if b {
                        let (then_env, then_value, return_flag) =
                            interp_statements(env.clone(), then_statements)?;

                        for (var_name, (_, value_story)) in env.clone() {
                            match then_env.get(&var_name) {
                                Some((reassigned_value, ValueStory::Assignment)) => {
                                    env.insert(var_name, (reassigned_value.clone(), value_story));
                                }
                                _ => (),
                            };
                        }
                        Ok((env, then_value, return_flag))
                    } else {
                        match else_statements_option {
                            Some(else_statements) => {
                                let (else_env, else_value, return_flag) =
                                    interp_statements(env.clone(), else_statements)?;

                                for (var_name, (_, value_story)) in env.clone() {
                                    match else_env.get(&var_name) {
                                        Some((reassigned_value, ValueStory::Assignment)) => {
                                            env.insert(
                                                var_name,
                                                (reassigned_value.clone(), value_story),
                                            );
                                        }
                                        _ => (),
                                    };
                                }
                                Ok((env, else_value, return_flag))
                            }
                            None => Ok((env.clone(), Value::Void, ReturnFlag::Continue)),
                        }
                    }
                }
                v => return Err(RuntimeError(BadCondition(v))),
            }
        }
        Statement::Return(expr_option) => match expr_option {
            Some(expr) => match interp_expression(env.clone(), expr) {
                Ok(value) => Ok((env, value, ReturnFlag::Return)),
                Err(e) => return Err(e),
            },
            None => Ok((env, Value::Void, ReturnFlag::Return)),
        },
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
                body_env.insert(param.to_string(), (value, ValueStory::Initialization));
            }

            return match interp_statements(body_env.clone(), body.to_vec()) {
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
