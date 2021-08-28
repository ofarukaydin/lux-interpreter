use crate::token::{Token, TokenLiteral};
pub enum Expr {
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: TokenLiteral,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn visit(&self) -> String {
        match self {
            Expr::Binary {
                left,
                right,
                operator,
            } => {
                format!("({} {} {})", &operator.lexeme, left.visit(), right.visit())
            }
            Expr::Grouping { expression } => {
                format!("(group {})", expression.visit())
            }
            Expr::Literal { value } => value.to_string(),
            Expr::Unary { operator, right } => {
                format!("({} {})", &operator.lexeme, right.visit())
            }
        }
    }
}
