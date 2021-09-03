use crate::{expr::Expr, function::Function, token::Token};

#[derive(PartialEq, Clone, Debug, Eq)]
pub enum Stmt {
    Expression {
        expression: Box<Expr>,
    },
    Print {
        expression: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Box<Expr>,
    },
    Block {
        statements: Vec<Stmt>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Function(Function),
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
    Return {
        keyword: Token,
        value: Box<Expr>,
    },
}
