use std::panic::{panic_any, UnwindSafe};
use std::{cell::RefCell, ops::Neg, rc::Rc};

use crate::callable::LuxCallable;
use crate::function::LuxFunction;
use crate::literal::Literal;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::{
    clock::Clock, environment::Environment, expr::Expr, runtime_error::RuntimeError,
    token_type::Types,
};
pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
}

impl UnwindSafe for Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Environment::new();
        let environment = globals.clone();
        globals
            .borrow_mut()
            .define("clock".to_string(), Literal::Clock(Clock::new()));

        Self {
            environment,
            globals,
        }
    }
    pub fn evaluate(&mut self, expr: &Expr) -> RuntimeResult<Literal> {
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
                        if let Literal::Number(num_left) = eval_left {
                            if let Literal::Number(num_right) = eval_right {
                                return Ok(Literal::Number(num_left - num_right));
                            }
                        }
                        num_err
                    }
                    Types::SLASH => {
                        if let Literal::Number(num_left) = eval_left {
                            if let Literal::Number(num_right) = eval_right {
                                return Ok(Literal::Number(num_left / num_right));
                            }
                        }
                        num_err
                    }
                    Types::STAR => {
                        if let Literal::Number(num_left) = eval_left {
                            if let Literal::Number(num_right) = eval_right {
                                return Ok(Literal::Number(num_left * num_right));
                            }
                        }
                        num_err
                    }
                    Types::PLUS => {
                        if let Literal::Number(num_left) = eval_left {
                            if let Literal::Number(num_right) = eval_right {
                                return Ok(Literal::Number(num_left + num_right));
                            }
                        }
                        if let Literal::String(str_left) = eval_left {
                            if let Literal::String(str_right) = eval_right {
                                return Ok(Literal::String(format!("{}{}", str_left, str_right)));
                            }
                        }
                        Err(RuntimeError::new(
                            operator.to_owned(),
                            "Operands must be two numbers or two strings.".to_string(),
                        ))
                    }
                    Types::GREATER => {
                        if let Literal::Number(num_left) = eval_left {
                            if let Literal::Number(num_right) = eval_right {
                                return Ok(Literal::Bool(num_left > num_right));
                            }
                        }
                        num_err
                    }
                    Types::GREATER_EQUAL => {
                        if let Literal::Number(num_left) = eval_left {
                            if let Literal::Number(num_right) = eval_right {
                                return Ok(Literal::Bool(num_left >= num_right));
                            }
                        }
                        num_err
                    }
                    Types::LESS => {
                        if let Literal::Number(num_left) = eval_left {
                            if let Literal::Number(num_right) = eval_right {
                                return Ok(Literal::Bool(num_left < num_right));
                            }
                        }
                        num_err
                    }
                    Types::LESS_EQUAL => {
                        if let Literal::Number(num_left) = eval_left {
                            if let Literal::Number(num_right) = eval_right {
                                return Ok(Literal::Bool(num_left <= num_right));
                            }
                        }
                        num_err
                    }
                    Types::BANG_EQUAL => {
                        if eval_left != eval_right {
                            return Ok(Literal::Bool(true));
                        }
                        Ok(Literal::Bool(false))
                    }
                    Types::EQUAL_EQUAL => {
                        if eval_left == eval_right {
                            return Ok(Literal::Bool(true));
                        }
                        Ok(Literal::Bool(false))
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
                        if let Literal::Number(num) = eval_right {
                            Ok(Literal::Number(num.neg()))
                        } else {
                            is_err
                        }
                    }
                    Types::BANG => {
                        if eval_right.is_truthy() {
                            Ok(Literal::Bool(true))
                        } else {
                            Ok(Literal::Bool(false))
                        }
                    }
                    _ => Err(RuntimeError::new(
                        operator.to_owned(),
                        "not implemented".to_string(),
                    )),
                }
            }
            Expr::Nil => Ok(Literal::Nil),
            Expr::Variable { name } => {
                let env = self.environment.borrow();
                let var = env.get(name)?;
                Ok(var)
            }
            Expr::Assign { name, value } => {
                let eval_val = self.evaluate(value)?;
                self.environment
                    .borrow_mut()
                    .assign(name.clone(), eval_val.clone())?;
                Ok(eval_val)
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let eval_left = self.evaluate(left)?;

                if operator.type_t == Types::OR {
                    if eval_left.is_truthy() {
                        return Ok(eval_left);
                    }
                } else if !eval_left.is_truthy() {
                    return Ok(eval_left);
                }

                self.evaluate(right)
            }
            Expr::Call {
                arguments,
                callee,
                paren,
            } => {
                let eval_callee = self.evaluate(callee.as_ref())?;
                let mut eval_arguments: Vec<Literal> = vec![];

                for argument in arguments {
                    eval_arguments.push(self.evaluate(argument)?);
                }

                match eval_callee {
                    Literal::Function(func) => {
                        if eval_arguments.len() != func.arity() {
                            return Err(RuntimeError {
                                token: paren.clone(),
                                message: format!(
                                    "Expected {} arguments but got {}.",
                                    func.arity(),
                                    eval_arguments.len()
                                ),
                            });
                        }

                        Ok(func.call(self, eval_arguments)?)
                    }
                    _ => Err(RuntimeError::new(
                        paren.clone(),
                        "Can only call functions and classes.".to_string(),
                    )),
                }
            }
        }
    }

    fn check_number_operand(operator: &Token, operand: &Literal) -> RuntimeResult<Literal> {
        if let Literal::Number(_) = operand {
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
        left: &Literal,
        right: &Literal,
    ) -> RuntimeResult<Literal> {
        let err = Err(RuntimeError::new(
            operator.to_owned(),
            "Operands must be numbers.".to_string(),
        ));
        if let Literal::Number(_) = left {
            if let Literal::Number(_) = right {
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
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let eval_cond = self.evaluate(condition)?;
                if eval_cond.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?
                }
            }
            Stmt::While { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?;
                }
            }
            Stmt::Function(stmt) => {
                let lux_function = LuxFunction::new(stmt.clone(), self.environment.clone());
                self.environment.borrow_mut().define(
                    stmt.name.lexeme.clone(),
                    Literal::Function(Box::new(lux_function)),
                )
            }
            Stmt::Return { value, .. } => {
                let value = self.evaluate(value)?;
                panic_any(value)
            }
        };
        Ok(())
    }

    pub fn execute_block(
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

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
