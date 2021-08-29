use std::{cell::RefCell, ops::Neg, rc::Rc};

use crate::{
    environment::Environment,
    expr::Expr,
    runtime_error::RuntimeError,
    stmt::Stmt,
    token::{Token, TokenLiteral},
    token_type::Types,
};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = Environment::new();
        Self { environment }
    }
    pub fn evaluate(&mut self, expr: &Expr) -> RuntimeResult<TokenLiteral> {
        match expr {
            Expr::Binary {
                left,
                right,
                operator,
            } => {
                let eval_left = self.evaluate(left)?;
                let eval_right = self.evaluate(right)?;
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
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => Ok(value.to_owned()),
            Expr::Unary { operator, right } => {
                let eval_right = self.evaluate(right)?;

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
            Expr::Nil => Ok(TokenLiteral::Nil),
            Expr::Variable { name } => {
                let env = self.environment.borrow();
                let var = env.get(name)?;
                Ok(var)
            }
            Expr::Assign { name, value } => {
                let eval_val = self.evaluate(value)?;
                self.environment
                    .borrow_mut()
                    .assign(name, eval_val.clone())?;
                Ok(eval_val)
            }
        }
    }

    fn check_number_operand(
        operator: &Token,
        operand: &TokenLiteral,
    ) -> RuntimeResult<TokenLiteral> {
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
    ) -> RuntimeResult<TokenLiteral> {
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

    pub fn interpret(&mut self, statements: &[Stmt]) -> RuntimeResult<()> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    pub fn execute(&mut self, statement: &Stmt) -> RuntimeResult<()> {
        match statement {
            Stmt::Expression { expression } => {
                self.evaluate(expression)?;
            }
            Stmt::Print { expression } => {
                let eval_exp = self.evaluate(expression).unwrap().to_string();
                println!("{}", eval_exp);
            }
            Stmt::Var { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value)
            }
            Stmt::Block { statements } => {
                let environment = self.environment.clone();
                self.execute_block(statements, Environment::new_with(environment))?;
            }
        };
        Ok(())
    }

    fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Rc<RefCell<Environment>>,
    ) -> RuntimeResult<()> {
        let previous = self.environment.clone();
        self.environment = environment;
        for statement in statements {
            if self.execute(statement).is_err() {
                self.environment = previous;
                return Ok(());
            }
        }
        self.environment = previous;
        Ok(())
    }
}
