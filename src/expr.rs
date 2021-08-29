use std::{any::Any, ops::Neg, ptr};

use crate::{
    runtime_error::RuntimeError,
    token::{Token, TokenLiteral},
    token_type::Types,
};

type RuntimeResult = Result<TokenLiteral, RuntimeError>;

#[derive(Debug)]
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

    pub fn evaluate(&self) -> RuntimeResult {
        match self {
            Expr::Binary {
                left,
                right,
                operator,
            } => {
                let eval_left = left.evaluate()?;
                let eval_right = right.evaluate()?;
                let num_err = Self::check_number_operands(operator, &eval_left, &eval_right);

                match operator.type_t {
                    Types::MINUS => {
                        if let TokenLiteral::Number(num_left) = eval_left {
                            if let TokenLiteral::Number(num_right) = eval_right {
                                return Ok(TokenLiteral::Number(num_left - num_right));
                            }
                        }
                        num_err
                    }
                    Types::SLASH => {
                        if let TokenLiteral::Number(num_left) = eval_left {
                            if let TokenLiteral::Number(num_right) = eval_right {
                                return Ok(TokenLiteral::Number(num_left / num_right));
                            }
                        }
                        num_err
                    }
                    Types::STAR => {
                        if let TokenLiteral::Number(num_left) = eval_left {
                            if let TokenLiteral::Number(num_right) = eval_right {
                                return Ok(TokenLiteral::Number(num_left * num_right));
                            }
                        }
                        num_err
                    }
                    Types::PLUS => {
                        if let TokenLiteral::Number(num_left) = eval_left {
                            if let TokenLiteral::Number(num_right) = eval_right {
                                return Ok(TokenLiteral::Number(num_left + num_right));
                            }
                        }
                        if let TokenLiteral::String(str_left) = eval_left {
                            if let TokenLiteral::String(str_right) = eval_right {
                                return Ok(TokenLiteral::String(format!(
                                    "{}{}",
                                    str_left, str_right
                                )));
                            }
                        }
                        Err(RuntimeError::new(
                            operator.to_owned(),
                            "Operands must be two numbers or two strings.".to_string(),
                        ))
                    }
                    Types::GREATER => {
                        if let TokenLiteral::Number(num_left) = eval_left {
                            if let TokenLiteral::Number(num_right) = eval_right {
                                return Ok(TokenLiteral::Bool(num_left > num_right));
                            }
                        }
                        num_err
                    }
                    Types::GREATER_EQUAL => {
                        if let TokenLiteral::Number(num_left) = eval_left {
                            if let TokenLiteral::Number(num_right) = eval_right {
                                return Ok(TokenLiteral::Bool(num_left >= num_right));
                            }
                        }
                        num_err
                    }
                    Types::LESS => {
                        if let TokenLiteral::Number(num_left) = eval_left {
                            if let TokenLiteral::Number(num_right) = eval_right {
                                return Ok(TokenLiteral::Bool(num_left < num_right));
                            }
                        }
                        num_err
                    }
                    Types::LESS_EQUAL => {
                        if let TokenLiteral::Number(num_left) = eval_left {
                            if let TokenLiteral::Number(num_right) = eval_right {
                                return Ok(TokenLiteral::Bool(num_left <= num_right));
                            }
                        }
                        num_err
                    }
                    Types::BANG_EQUAL => {
                        if eval_left != eval_right {
                            return Ok(TokenLiteral::Bool(true));
                        }
                        Ok(TokenLiteral::Bool(false))
                    }
                    Types::EQUAL => {
                        if eval_left == eval_right {
                            return Ok(TokenLiteral::Bool(true));
                        }
                        Ok(TokenLiteral::Bool(false))
                    }

                    _ => Err(RuntimeError::new(
                        operator.to_owned(),
                        "Operator type not implemented!".to_string(),
                    )),
                }
            }
            Expr::Grouping { expression } => expression.evaluate(),
            Expr::Literal { value } => Ok(value.to_owned()),
            Expr::Unary { operator, right } => {
                let eval_right = right.evaluate()?;

                match operator.type_t {
                    Types::MINUS => {
                        let is_err = Self::check_number_operand(operator, &eval_right);
                        if let TokenLiteral::Number(num) = eval_right {
                            Ok(TokenLiteral::Number(num.to_owned().neg()))
                        } else {
                            is_err
                        }
                    }
                    Types::BANG => {
                        if eval_right.is_truthy() {
                            Ok(TokenLiteral::Bool(true))
                        } else {
                            Ok(TokenLiteral::Bool(false))
                        }
                    }
                    _ => Err(RuntimeError::new(
                        operator.to_owned(),
                        "not implemented".to_string(),
                    )),
                }
            }
        }
    }

    fn check_number_operand(operator: &Token, operand: &TokenLiteral) -> RuntimeResult {
        if let TokenLiteral::Number(_) = operand {
            Ok(operand.to_owned())
        } else {
            Err(RuntimeError::new(
                operator.to_owned(),
                "Operand must be a number.".to_string(),
            ))
        }
    }

    fn check_number_operands(
        operator: &Token,
        left: &TokenLiteral,
        right: &TokenLiteral,
    ) -> RuntimeResult {
        let err = Err(RuntimeError::new(
            operator.to_owned(),
            "Operands must be numbers.".to_string(),
        ));
        if let TokenLiteral::Number(_) = left {
            if let TokenLiteral::Number(_) = right {
                return Ok(left.to_owned());
            }
            err
        } else {
            err
        }
    }

    pub fn interpret(&self) -> Result<String, RuntimeError> {
        match self.evaluate()? {
            TokenLiteral::Number(num) => Ok(num.to_string()),
            TokenLiteral::String(str) => Ok(str),
            TokenLiteral::Bool(bool) => Ok(bool.to_string()),
            TokenLiteral::Nil => Ok("nil".to_string()),
        }
    }
}
