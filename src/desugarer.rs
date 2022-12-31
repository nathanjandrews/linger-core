use crate::{
    parser::{Builtin, Expr, Statement, Statements},
    tokenizer::Operator,
};

pub type SugaredStatements<'a> = Vec<SugaredStatement<'a>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SugaredStatement<'a> {
    Expr(SugaredExpr<'a>),
    Let(&'a str, SugaredExpr<'a>),
    Assign(&'a str, SugaredExpr<'a>),
    If(
        SugaredExpr<'a>,
        SugaredStatements<'a>,
        Vec<(SugaredExpr<'a>, SugaredStatements<'a>)>,
        Option<SugaredStatements<'a>>,
    ),
    Return(Option<SugaredExpr<'a>>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SugaredExpr<'a> {
    Num(i64),
    Bool(bool),
    Str(String),
    Var(&'a str),
    Binary(Operator, Box<SugaredExpr<'a>>, Box<SugaredExpr<'a>>),
    Unary(Operator, Box<SugaredExpr<'a>>),
    PrimitiveCall(Builtin, Vec<SugaredExpr<'a>>),
    Call(Box<SugaredExpr<'a>>, Vec<SugaredExpr<'a>>),
    Lambda(Vec<&'a str>, SugaredStatements<'a>),
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
