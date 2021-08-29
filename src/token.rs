use std::{any::Any, ptr};

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

impl TokenLiteral {
    pub fn to_string(&self) -> String {
        match self {
            TokenLiteral::Number(num) => num.to_string(),
            TokenLiteral::String(str) => str.to_string(),
            TokenLiteral::Nil => "nil".to_string(),
            TokenLiteral::Bool(true) => "true".to_string(),
            TokenLiteral::Bool(false) => "false".to_string(),
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
            lexeme,
            type_t,
            line,
            literal,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
