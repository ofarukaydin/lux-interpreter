use std::fmt;

use crate::{literal::Literal, token_type::Types};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Token {
    pub type_t: Types,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(type_t: Types, lexeme: String, literal: Literal, line: usize) -> Token {
        Token {
            type_t,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Line: {}, Lexeme: {}, Literal: {}, Type: {:?}",
            self.line, self.lexeme, self.literal, self.type_t
        )
    }
}
