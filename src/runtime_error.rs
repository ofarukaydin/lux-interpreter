use std::fmt;

use crate::token::Token;

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: String) -> RuntimeError {
        RuntimeError { token, message }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} {} \n [line {}]",
            self.token.to_string(),
            self.message,
            self.token.line
        )
    }
}

impl std::error::Error for RuntimeError {}
