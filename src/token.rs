use crate::token_type::Types;

#[derive(Debug, Clone)]
pub enum TokenLiteral {
    Number(f64),
    String(String),
    None,
    True,
    False,
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
            TokenLiteral::None => "nil".to_string(),
            TokenLiteral::True => "true".to_string(),
            TokenLiteral::False => "false".to_string(),
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
