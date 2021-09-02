use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    interpreter::RuntimeResult, literal::Literal, runtime_error::RuntimeError, token::Token,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
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

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> RuntimeResult<Literal> {
        let key = name.lexeme.as_str();

        if let Some(val) = self.values.get(key) {
            return Ok(val.clone());
        } else if let Some(enclosing) = &self.enclosing {
            let env = enclosing.borrow();
            return env.get(name);
        }

        Err(RuntimeError::new(
            name.to_owned(),
            format!("Undefined variable '{}'.", name.lexeme.to_owned()),
        ))
    }

    pub fn assign(&mut self, token: Token, value: Literal) -> RuntimeResult<()> {
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            self.values.entry(token.lexeme.clone())
        {
            e.insert(value);
            return Ok(());
        } else if let Some(enclosing) = &mut self.enclosing {
            return enclosing.borrow_mut().assign(token, value);
        }

        Err(RuntimeError::new(
            token.clone(),
            format!("Undefined variable '{}'.", token.lexeme),
        ))
    }
}
