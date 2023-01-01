use crate::{
    parser::{Builtin, SugaredExpr, SugaredStatement, SugaredStatements},
    tokenizer::Operator,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Procedure<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
    pub body: Statements<'a>,
}

pub type Statements<'a> = Vec<Statement<'a>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement<'a> {
    Expr(Expr<'a>),
    Let(&'a str, Expr<'a>),
    Assign(&'a str, Expr<'a>),
    If(Expr<'a>, Statements<'a>, Option<Statements<'a>>),
    While(Expr<'a>, Statements<'a>),
    Return(Option<Expr<'a>>),
    Break,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr<'a> {
    Num(i64),
    Bool(bool),
    Str(String),
    Var(&'a str),
    Binary(Operator, Box<Expr<'a>>, Box<Expr<'a>>),
    Unary(Operator, Box<Expr<'a>>),
    PrimitiveCall(Builtin, Vec<Expr<'a>>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    Lambda(Vec<&'a str>, Statements<'a>),
}

pub fn desugar_statements(sugared_statements: SugaredStatements) -> Statements {
    sugared_statements
        .iter()
        .map(|s| desugar_statement(s.clone()))
        .collect()
}

pub fn desugar_statement(sugared_statement: SugaredStatement) -> Statement {
    match sugared_statement {
        SugaredStatement::Expr(sugared_expr) => Statement::Expr(desugar_expression(sugared_expr)),
        SugaredStatement::Let(name, sugared_expr) => {
            Statement::Let(name, desugar_expression(sugared_expr))
        }
        SugaredStatement::Assign(name, sugared_expr) => {
            Statement::Assign(name, desugar_expression(sugared_expr))
        }
        SugaredStatement::If(if_cond, then_statements, else_ifs, else_option) => {
            let desugared_else_option = match else_option {
                Some(else_statements) => Some(desugar_statements(else_statements)),
                None => None,
            };

            let nested_if_statement = else_ifs.iter().rfold(
                desugared_else_option,
                |acc, (cur_sugared_cond_expr, cur_sugared_statements)| {
                    return Some(vec![Statement::If(
                        desugar_expression(cur_sugared_cond_expr.clone()),
                        desugar_statements(cur_sugared_statements.clone()),
                        acc,
                    )]);
                },
            );

            return Statement::If(
                desugar_expression(if_cond),
                desugar_statements(then_statements),
                nested_if_statement,
            );
        }

        SugaredStatement::Return(sugared_expr_option) => {
            Statement::Return(match sugared_expr_option {
                Some(sugared_expr) => Some(desugar_expression(sugared_expr)),
                None => None,
            })
        }
        SugaredStatement::While(sugared_while_cond, sugared_while_body) => Statement::While(
            desugar_expression(sugared_while_cond),
            desugar_statements(sugared_while_body),
        ),
        SugaredStatement::Break => Statement::Break,
    }
}

pub fn desugar_expression(sugared_expr: SugaredExpr) -> Expr {
    match sugared_expr {
        SugaredExpr::Num(n) => Expr::Num(n),
        SugaredExpr::Bool(b) => Expr::Bool(b),
        SugaredExpr::Str(s) => Expr::Str(s),
        SugaredExpr::Var(id) => Expr::Var(id),
        SugaredExpr::Binary(op, left_sugared_expr, right_sugared_expr) => Expr::Binary(
            op,
            Box::new(desugar_expression(*left_sugared_expr)),
            Box::new(desugar_expression(*right_sugared_expr)),
        ),
        SugaredExpr::Unary(op, expr) => Expr::Unary(op, Box::new(desugar_expression(*expr))),
        SugaredExpr::PrimitiveCall(name, sugared_args) => Expr::PrimitiveCall(
            name,
            sugared_args
                .iter()
                .map(|sugared_arg_expr| desugar_expression(sugared_arg_expr.clone()))
                .collect(),
        ),
        SugaredExpr::Call(sugared_proc_expr, sugared_args) => Expr::Call(
            Box::new(desugar_expression(*sugared_proc_expr)),
            sugared_args
                .iter()
                .map(|sugared_arg_expr| desugar_expression(sugared_arg_expr.clone()))
                .collect(),
        ),
        SugaredExpr::Lambda(params, sugared_body) => {
            Expr::Lambda(params, desugar_statements(sugared_body))
        }
    }
}
