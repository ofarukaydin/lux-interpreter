use std::collections::{HashMap, VecDeque};

use crate::{
    error::LuxError, expr::Expr, function::Function, interpreter::Interpreter, stmt::Stmt,
    token::Token,
};

pub struct Resolver<'a> {
    scopes: VecDeque<HashMap<String, bool>>,
    interpreter: &'a mut Interpreter,
    current_function: FunctionType,
}

type ResolverResult<T> = Result<T, LuxError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionType {
    None,
    Function,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        let scopes: VecDeque<HashMap<String, bool>> = VecDeque::new();
        let current_function = FunctionType::None;
        Self {
            scopes,
            interpreter,
            current_function,
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) -> ResolverResult<()> {
        for statement in statements {
            self.resolve_one(statement)?
        }

        Ok(())
    }

    fn resolve_one(&mut self, statement: &Stmt) -> ResolverResult<()> {
        match statement {
            Stmt::Expression { expression } => self.resolve_expr(expression),
            Stmt::Print { expression } => self.resolve_expr(expression.as_ref()),
            Stmt::Var { name, initializer } => {
                self.declare(name)?;
                if **initializer != Expr::Nil {
                    self.resolve_expr(initializer)?
                }
                self.define(name);

                Ok(())
            }
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();

                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_one(then_branch)?;

                if let Some(branch) = else_branch {
                    self.resolve_one(branch)?;
                }

                Ok(())
            }
            Stmt::Function(func) => {
                self.declare(&func.name)?;
                self.define(&func.name);
                self.resolve_func(func, FunctionType::Function)
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_one(body)
            }
            Stmt::Return { value, keyword } => {
                if self.current_function == FunctionType::None {
                    return Err(LuxError::new(keyword, "Can't return from top-level code."));
                }

                self.resolve_expr(value.as_ref())
            }
        }
    }

    fn resolve_func(
        &mut self,
        function: &Function,
        function_type: FunctionType,
    ) -> ResolverResult<()> {
        let enclosing_function = self.current_function;
        self.current_function = function_type;

        self.begin_scope();

        for param in &function.param {
            self.declare(param)?;
            self.define(param);
        }

        self.resolve(&function.body)?;
        self.end_scope();

        self.current_function = enclosing_function;

        Ok(())
    }

    fn declare(&mut self, name: &Token) -> ResolverResult<()> {
        if self.scopes.is_empty() {
            return Ok(());
        }

        let scope = self.scopes.back_mut();

        if scope.unwrap().contains_key(&name.lexeme) {
            return Err(LuxError::new(
                name,
                "Already a variable with this name in this scope.",
            ));
        }

        if let Some(map) = self.scopes.back_mut() {
            map.insert(name.lexeme.clone(), false);
        }

        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.back_mut();

        if let Some(map) = scope {
            map.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&mut self, expr: &Expr, name: Token) {
        let mut i = (self.scopes.len() as i64) - 1;

        while i >= 0 {
            let scope = self.scopes.get(i as usize).unwrap();
            if scope.contains_key(&name.lexeme) {
                self.interpreter
                    .resolve(expr, self.scopes.len() - 1 - i as usize)
            }
            i -= 1;
        }
    }

    fn resolve_expr(&mut self, expression: &Expr) -> ResolverResult<()> {
        match expression {
            Expr::Binary { left, right, .. } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            }
            Expr::Grouping { expression } => self.resolve_expr(expression),
            Expr::Literal { .. } => Ok(()),
            Expr::Unary { right, .. } => self.resolve_expr(right),
            Expr::Variable { name } => {
                if !self.scopes.is_empty() {
                    let value = self.scopes.back().unwrap().get(&name.lexeme);
                    if let Some(val) = value {
                        if !(*val) {
                            return Err(LuxError::new(
                                name,
                                "Can't read local variable in its own initializer.",
                            ));
                        }
                    }
                }
                self.resolve_local(&expression, name.clone());
                Ok(())
            }
            Expr::Assign { name, value } => {
                self.resolve_expr(value)?;
                self.resolve_local(&expression, name.clone());
                Ok(())
            }
            Expr::Logical { left, right, .. } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            }
            Expr::Call {
                callee, arguments, ..
            } => {
                self.resolve_expr(callee)?;
                for argument in arguments {
                    self.resolve_expr(argument)?;
                }
                Ok(())
            }
            Expr::Nil => Ok(()),
        }
    }

    fn begin_scope(&mut self) {
        let scope: HashMap<String, bool> = HashMap::new();
        self.scopes.push_back(scope)
    }

    fn end_scope(&mut self) {
        self.scopes.pop_back();
    }
}
