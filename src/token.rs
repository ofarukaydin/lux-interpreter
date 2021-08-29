use std::fmt;

use crate::token_type::Types;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenLiteral {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub type_t: Types,
    pub lexeme: String,
    pub literal: TokenLiteral,
    pub line: usize,
}

impl fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_str())
    }
}

impl TokenLiteral {
    fn to_str(&self) -> String {
        match self {
            TokenLiteral::Number(num) => num.to_string(),
            TokenLiteral::String(str) => str.to_string(),
            TokenLiteral::Nil => "nil".to_string(),
            TokenLiteral::Bool(bool) => bool.to_string(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            TokenLiteral::Bool(bool) => *bool,
            TokenLiteral::Nil => false,
            _ => true,
        }
    }
}

impl Token {
    pub fn new(type_t: Types, lexeme: String, literal: TokenLiteral, line: usize) -> Token {
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
