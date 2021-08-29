use std::fmt;

use crate::{token::Token, token_type::Types};

#[derive(Debug, Clone)]
pub struct LuxError {
    pub message: String,
    pub line: usize,
    pub location: String,
}

// Errors should be printable.
impl fmt::Display for LuxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "line {} Error {}: {}",
            self.line, self.location, self.message
        )
    }
}

impl std::error::Error for LuxError {}

impl LuxError {
    fn report(line: usize, location: &str, message: &str) -> LuxError {
        LuxError {
            line,
            location: location.to_string(),
            message: message.to_string(),
        }
    }

    pub fn new(token: &Token, message: &str) -> LuxError {
        if token.type_t == Types::EOF {
            Self::report(token.line, "at end", message)
        } else {
            Self::report(token.line, &format!("at '{}'", &token.lexeme), message)
        }
    }
}
