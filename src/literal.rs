use std::fmt;

use crate::{callable::LuxCallable, clock::Clock, function::LuxFunction};

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Clock(Clock),
    Function(Box<LuxFunction>),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_str())
    }
}

unsafe impl Send for Literal {}

impl Literal {
    fn to_str(&self) -> String {
        match self {
            Literal::Number(num) => num.to_string(),
            Literal::String(str) => str.to_string(),
            Literal::Nil => r#"nil"#.to_string(),
            Literal::Bool(bool) => bool.to_string(),
            Literal::Clock(clock) => clock.to_string(),
            Literal::Function(func) => func.to_str(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Bool(bool) => *bool,
            Literal::Nil => false,
            _ => true,
        }
    }
}
