use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    interpreter::RuntimeResult,
    runtime_error::RuntimeError,
    token::{Token, TokenLiteral},
};

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, TokenLiteral>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing: None,
        }))
    }
    pub fn new_with(enclosing: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }))
    }

    pub fn define(&mut self, name: String, value: TokenLiteral) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> RuntimeResult<TokenLiteral> {
        let key = name.lexeme.as_str();

        if let Some(val) = self.values.get(key) {
            return Ok(val.clone());
        } else if let Some(enclosing) = &self.enclosing {
            let env = enclosing.borrow();
            return env.get(name);
        }

        Err(RuntimeError::new(
            name.to_owned(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn assign(&mut self, token: &Token, value: TokenLiteral) -> RuntimeResult<()> {
        if self.values.contains_key(&token.lexeme) {
            self.values.insert(token.lexeme.to_owned(), value);
            return Ok(());
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(token, value)?;
        }

        Err(RuntimeError::new(
            token.to_owned(),
            format!("Undefined variable '{}'.", token.lexeme),
        ))
    }
}
