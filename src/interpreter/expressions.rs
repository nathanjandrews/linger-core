use crate::{
    desugar::Expr,
    environment::{AssignmentType, Binding, Entry, Environment, Mutability},
    error::RuntimeError::{self, *},
    tokenizer::Operator,
};

use super::{
    statements::interp_statement,
    utils::{ensure_list, ensure_single_arg},
    Value,
};

pub fn interp_expression<'a>(env: &mut Environment, expr: Expr) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Nil => Ok(Value::Nil),
        Expr::Num(n) => Ok(Value::Num(n)),
        Expr::Bool(b) => Ok(Value::Bool(b)),
        Expr::Str(s) => Ok(Value::Str(s)),
        Expr::Lambda(params, body) => Ok(Value::Proc(params, *body, env.clone())),
        Expr::Var(id) => match env.get(id.to_string())? {
            v => Ok(v),
        },
        Expr::Binary(op, left, right) => match op {
            Operator::Plus => {
                match (
                    interp_expression(env, *left)?,
                    interp_expression(env, *right)?,
                ) {
                    (Value::Num(num_left), Value::Num(num_right)) => {
                        Ok(Value::Num(num_left + num_right))
                    }
                    (Value::Str(num_left), Value::Str(num_right)) => {
                        Ok(Value::Str(num_left + num_right.as_str()))
                    }
                    (Value::List(mut list_left), Value::List(mut list_right)) => {
                        list_left.append(&mut list_right);
                        Ok(Value::List(list_left))
                    }
                    (Value::Num(_), v) => Err(BadArg(v)),
                    (v, _) => Err(BadArg(v)),
                }
            }
            Operator::Minus => match (
                interp_expression(env, *left)?,
                interp_expression(env, *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Num(num_left - num_right))
                }
                (Value::Num(_), v) => Err(BadArg(v)),
                (v, _) => Err(BadArg(v)),
            },
            Operator::Eq => match (
                interp_expression(env, *left)?,
                interp_expression(env, *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left == num_right))
                }
                (Value::Bool(bool_left), Value::Bool(bool_right)) => {
                    Ok(Value::Bool(bool_left == bool_right))
                }
                (v_left, v_right) => Err(BadArgs(vec![v_left, v_right])),
            },
            Operator::Ne => match (
                interp_expression(env, *left)?,
                interp_expression(env, *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left != num_right))
                }
                (Value::Bool(bool_left), Value::Bool(bool_right)) => {
                    Ok(Value::Bool(bool_left != bool_right))
                }
                (v_left, v_right) => Err(BadArgs(vec![v_left, v_right])),
            },
            Operator::LT => match (
                interp_expression(env, *left)?,
                interp_expression(env, *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left < num_right))
                }
                (v_left, v_right) => Err(BadArgs(vec![v_left, v_right])),
            },
            Operator::GT => match (
                interp_expression(env, *left)?,
                interp_expression(env, *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left > num_right))
                }
                (v_left, v_right) => Err(BadArgs(vec![v_left, v_right])),
            },
            Operator::LTE => match (
                interp_expression(env, *left)?,
                interp_expression(env, *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left <= num_right))
                }
                (v_left, v_right) => Err(BadArgs(vec![v_left, v_right])),
            },
            Operator::GTE => match (
                interp_expression(env, *left)?,
                interp_expression(env, *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Bool(num_left >= num_right))
                }
                (v_left, v_right) => Err(BadArgs(vec![v_left, v_right])),
            },
            Operator::LogicOr => match interp_expression(env, *left)? {
                Value::Bool(b) => match b {
                    true => Ok(Value::Bool(true)),
                    false => match interp_expression(env, *right)? {
                        Value::Bool(b) => Ok(Value::Bool(b)),
                        right_value => Err(BadArg(right_value)),
                    },
                },
                left_value => Err(BadArg(left_value)),
            },
            Operator::LogicAnd => match interp_expression(env, *left)? {
                Value::Bool(b) => match b {
                    false => Ok(Value::Bool(false)),
                    true => match interp_expression(env, *right)? {
                        Value::Bool(b) => Ok(Value::Bool(b)),
                        right_value => Err(BadArg(right_value)),
                    },
                },
                left_value => Err(BadArg(left_value)),
            },
            Operator::Times => match (
                interp_expression(env, *left)?,
                interp_expression(env, *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Num(num_left * num_right))
                }
                (v_left, v_right) => Err(BadArgs(vec![v_left, v_right])),
            },
            Operator::Mod => match (
                interp_expression(env, *left)?,
                interp_expression(env, *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Num(num_left % num_right))
                }
                (v_left, v_right) => Err(BadArgs(vec![v_left, v_right])),
            },
            Operator::Div => match (
                interp_expression(env, *left)?,
                interp_expression(env, *right)?,
            ) {
                (Value::Num(num_left), Value::Num(num_right)) => {
                    Ok(Value::Num(num_left / num_right))
                }
                (v_left, v_right) => Err(BadArgs(vec![v_left, v_right])),
            },
            op => Err(UnaryAsBinary(op)),
        },
        Expr::Unary(op, operand) => match op {
            Operator::PreIncrement => {
                let var_name = match *operand {
                    Expr::Var(ref id) => id.to_string(),
                    _ => return Err(InvalidAssignmentTarget),
                };

                let num_value = match interp_expression(env, *operand)? {
                    Value::Num(n) => n,
                    v => return Err(BadArg(v)),
                };

                env.reassign(var_name, Value::Num(num_value + 1.0))?;

                return Ok(Value::Num(num_value + 1.0));
            }
            Operator::PostIncrement => {
                let var_name = match *operand {
                    Expr::Var(ref id) => id.to_string(),
                    _ => return Err(InvalidAssignmentTarget),
                };

                let original_num_value = match interp_expression(env, *operand)? {
                    Value::Num(n) => n,
                    v => return Err(BadArg(v)),
                };

                env.reassign(var_name, Value::Num(original_num_value + 1.0))?;

                return Ok(Value::Num(original_num_value));
            }
            Operator::PreDecrement => {
                let var_name = match *operand {
                    Expr::Var(ref id) => id.to_string(),
                    _ => return Err(InvalidAssignmentTarget),
                };

                let num_value = match interp_expression(env, *operand)? {
                    Value::Num(n) => n,
                    v => return Err(BadArg(v)),
                };

                env.reassign(var_name, Value::Num(num_value - 1.0))?;

                return Ok(Value::Num(num_value - 1.0));
            }
            Operator::PostDecrement => {
                let var_name = match *operand {
                    Expr::Var(ref id) => id.to_string(),
                    _ => return Err(InvalidAssignmentTarget),
                };

                let original_num_value = match interp_expression(env, *operand)? {
                    Value::Num(n) => n,
                    v => return Err(BadArg(v)),
                };

                env.reassign(var_name, Value::Num(original_num_value - 1.0))?;

                return Ok(Value::Num(original_num_value));
            }
            Operator::Minus => match interp_expression(env, *operand)? {
                Value::Num(n) => Ok(Value::Num(-n)),
                v => Err(BadArg(v)),
            },
            Operator::LogicNot => match interp_expression(env, *operand)? {
                Value::Bool(b) => Ok(Value::Bool(!b)),
                v => Err(BadArg(v)),
            },
            op => Err(BinaryAsUnary(op)),
        },
        Expr::Call(f_expr, args) => {
            let f_name = match *f_expr {
                Expr::Var(ref f_name) => f_name.to_string(),
                _ => "<lambda>".to_string(),
            };

            let (f_params, f_body, f_env) = match interp_expression(env, *f_expr)? {
                Value::Proc(params, body, env) => (params, body, env),
                v => return Err(BadArg(v)),
            };

            if args.len() != f_params.len() {
                return Err(ArgMismatch(
                    f_name.to_string(),
                    f_params.len(), // expected
                    args.len(),     // actual
                ));
            }

            let arg_values_result: Result<Vec<Value>, RuntimeError> = args
                .into_iter()
                .map(|arg| interp_expression(env, arg))
                .collect();
            let arg_values = match arg_values_result {
                Ok(values) => values,
                Err(e) => return Err(e),
            };

            let entries: Vec<Entry> = arg_values
                .into_iter()
                .map(|v| (v, AssignmentType::Initialized, Mutability::Constant))
                .collect();

            let param_bindings: Vec<Binding> = f_params
                .iter()
                .map(|param| param.to_string())
                .zip(entries)
                .collect();

            return match interp_statement(&mut f_env.extend(param_bindings), f_body, false)? {
                (value, _) => Ok(value),
            };
        }
        Expr::PrimitiveCall(builtin, args) => match builtin {
            crate::parser::Builtin::Print => {
                let mut values: Vec<Value> = vec![];
                for expr in args {
                    values.push(interp_expression(env, expr)?);
                }
                let values: Vec<String> = values.iter().map(|v| v.to_string()).collect();
                let values = values.join(" ");
                print!("{}", values);
                Ok(Value::Nil)
            }
            crate::parser::Builtin::List => {
                let mut values = vec![];
                for expr in args {
                    values.push(interp_expression(env, expr)?);
                }
                Ok(Value::List(values))
            }
            crate::parser::Builtin::IsEmpty => {
                let arg = ensure_single_arg(args)?;
                let list = ensure_list(interp_expression(env, arg)?)?;
                Ok(Value::Bool(list.is_empty()))
            }
            crate::parser::Builtin::IsNil => {
                let arg = ensure_single_arg(args)?;
                match interp_expression(env, arg)? {
                    Value::Nil => Ok(Value::Bool(true)),
                    _ => Ok(Value::Bool(false)),
                }
            }
            crate::parser::Builtin::Head => {
                let arg = ensure_single_arg(args)?;
                let list = ensure_list(interp_expression(env, arg)?)?;

                match list.as_slice() {
                    [hd, ..] => Ok(hd.clone()),
                    [] => Ok(Value::Nil),
                }
            }
            crate::parser::Builtin::Rest => {
                let arg = ensure_single_arg(args)?;
                let list = ensure_list(interp_expression(env, arg)?)?;

                match list.as_slice() {
                    [_, tail @ ..] => Ok(Value::List(tail.to_vec())),
                    [] => Ok(Value::Nil),
                }
            }
        },
        Expr::Index(indexable_expr, index_expr) => match interp_expression(env, *indexable_expr)? {
            Value::List(list) => match interp_expression(env, *index_expr)? {
                Value::Num(num) => {
                    if num.fract() != 0.0 {
                        return Err(ExpectedInteger(num.to_string()));
                    }

                    let index = num as i64;
                    if index < 0 {
                        return Err(IndexOutOfBounds(index));
                    }

                    let value = match list.into_iter().nth(index as usize) {
                        Some(v) => v,
                        None => return Err(IndexOutOfBounds(index)),
                    };

                    return Ok(value);
                }
                bad_value => return Err(ExpectedInteger(bad_value.to_string())),
            },
            Value::Str(str) => match interp_expression(env, *index_expr)? {
                Value::Num(num) => {
                    if num.fract() != 0.0 {
                        return Err(ExpectedInteger(num.to_string()));
                    }

                    let index = num as i64;
                    if index < 0 {
                        return Err(IndexOutOfBounds(index));
                    }

                    let character = match str.chars().nth(index as usize) {
                        Some(char) => char.to_string(),
                        None => return Err(IndexOutOfBounds(index)),
                    };

                    return Ok(Value::Str(character));
                }
                bad_value => return Err(ExpectedInteger(bad_value.to_string())),
            },
            value => return Err(NotIndexable(value.to_string())),
        },
    }
}
