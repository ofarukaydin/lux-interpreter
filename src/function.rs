use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
    panic::{self, AssertUnwindSafe},
    rc::Rc,
};

use crate::{
    callable::LuxCallable,
    environment::Environment,
    interpreter::{Interpreter, RuntimeResult},
    literal::Literal,
    stmt::Stmt,
    token::Token,
};
use rand::Rng;

#[derive(PartialEq, Clone, Debug, Eq)]
pub struct Function {
    pub name: Token,
    pub param: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug, Eq)]
pub struct LuxFunction {
    decleration: Function,
    id: usize,
    closure: Rc<RefCell<Environment>>,
}

impl LuxCallable for LuxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> RuntimeResult<Literal> {
        let environment = Environment::new_with(self.closure.clone());
        for i in 0..self.decleration.param.len() {
            let name = self.decleration.param.get(i).unwrap();
            let value = arguments.get(i).unwrap();
            environment
                .borrow_mut()
                .define(name.lexeme.clone(), value.clone())
        }

        let hook = panic::take_hook();

        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            interpreter.execute_block(&self.decleration.body, environment)
        }));

        panic::set_hook(hook);

        if let Err(return_value) = result {
            let casted_val = return_value.downcast::<Literal>();
            if let Ok(val) = casted_val {
                return Ok(*val);
            }
        }

        Ok(Literal::Nil)
    }

    fn to_str(&self) -> String {
        format!("<fn {}>", self.decleration.name.lexeme)
    }

    fn arity(&self) -> usize {
        self.decleration.param.len()
    }

    fn id(&self) -> usize {
        self.id
    }
}

impl LuxFunction {
    pub fn new(decleration: Function, closure: Rc<RefCell<Environment>>) -> Self {
        let mut rng = rand::thread_rng();
        let id: usize = rng.gen();
        Self {
            decleration,
            id,
            closure,
        }
    }
}

impl PartialEq for LuxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.decleration == other.decleration
            && self.id == other.id
            && self.closure == other.closure
    }
}

impl Hash for LuxFunction {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // Delegate hashing to the MuseumNumber.
        self.id.hash(hasher);
    }
}
