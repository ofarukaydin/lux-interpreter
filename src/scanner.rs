use std::collections::HashMap;

use crate::error::LuxError;
use crate::literal::Literal;
use crate::token::{Token};
use crate::token_type::Types;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, Types>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let mut scanner = Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::new(),
        };

        scanner.keywords.insert("and".to_string(), Types::AND);
        scanner.keywords.insert("class".to_string(), Types::CLASS);
        scanner.keywords.insert("else".to_string(), Types::ELSE);
        scanner.keywords.insert("false".to_string(), Types::FALSE);
        scanner.keywords.insert("for".to_string(), Types::FOR);
        scanner.keywords.insert("fun".to_string(), Types::FUN);
        scanner.keywords.insert("if".to_string(), Types::IF);
        scanner.keywords.insert("nil".to_string(), Types::NIL);
        scanner.keywords.insert("or".to_string(), Types::OR);
        scanner.keywords.insert("print".to_string(), Types::PRINT);
        scanner.keywords.insert("return".to_string(), Types::RETURN);
        scanner.keywords.insert("super".to_string(), Types::SUPER);
        scanner.keywords.insert("this".to_string(), Types::THIS);
        scanner.keywords.insert("true".to_string(), Types::TRUE);
        scanner.keywords.insert("var".to_string(), Types::VAR);
        scanner.keywords.insert("while".to_string(), Types::WHILE);

        scanner
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token().unwrap();
        }
        let token = Token::new(Types::EOF, "".to_string(), Literal::Nil, self.line);
        self.tokens.push(token);
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) -> Result<(), LuxError> {
        let c = self.advance();
        match c {
            '(' => {
                self.add_token(Types::LEFT_PAREN);
                Ok(())
            }
            ')' => {
                self.add_token(Types::RIGHT_PAREN);
                Ok(())
            }
            '{' => {
                self.add_token(Types::LEFT_BRACE);
                Ok(())
            }
            '}' => {
                self.add_token(Types::RIGHT_BRACE);
                Ok(())
            }
            ',' => {
                self.add_token(Types::COMMA);
                Ok(())
            }
            '.' => {
                self.add_token(Types::DOT);
                Ok(())
            }
            '-' => {
                self.add_token(Types::MINUS);
                Ok(())
            }
            '+' => {
                self.add_token(Types::PLUS);
                Ok(())
            }
            ';' => {
                self.add_token(Types::SEMICOLON);
                Ok(())
            }
            '*' => {
                self.add_token(Types::STAR);
                Ok(())
            }

            '!' => {
                let token_to_add = if self.matches_char('=') {
                    Types::BANG_EQUAL
                } else {
                    Types::BANG
                };
                self.add_token(token_to_add);
                Ok(())
            }

            '=' => {
                let token_to_add = if self.matches_char('=') {
                    Types::EQUAL_EQUAL
                } else {
                    Types::EQUAL
                };
                self.add_token(token_to_add);
                Ok(())
            }

            '<' => {
                let token_to_add = if self.matches_char('=') {
                    Types::LESS_EQUAL
                } else {
                    Types::LESS
                };
                self.add_token(token_to_add);
                Ok(())
            }

            '>' => {
                let token_to_add = if self.matches_char('=') {
                    Types::GREATER_EQUAL
                } else {
                    Types::GREATER
                };
                self.add_token(token_to_add);
                Ok(())
            }

            '/' => {
                if self.matches_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Types::SLASH)
                };
                Ok(())
            }

            ' ' => Ok(()),
            '\r' => Ok(()),
            '\t' => Ok(()),

            '\n' => {
                self.line += 1;
                Ok(())
            }

            '"' => {
                self.string().unwrap();
                Ok(())
            }

            ident => {
                if Scanner::is_digit(ident) {
                    self.number();
                    Ok(())
                } else if Scanner::is_alpha(ident) {
                    self.identifier();
                    Ok(())
                } else {
                    Err(LuxError {
                        line: self.line,
                        location: "".to_string(),
                        message: "Unexpected character".to_string(),
                    })
                }
            }
        }
    }

    pub fn advance(&mut self) -> char {
        let next_char = &self
            .source
            .chars()
            .nth(self.current)
            .expect("Failed to read char from advance method");
        self.current += 1;
        next_char.to_owned()
    }

    pub fn add_token(&mut self, token_type: Types) {
        self.add_token_literal(token_type, Literal::Nil)
    }

    fn add_token_literal(&mut self, token_type: Types, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line))
    }

    fn matches_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn string(&mut self) -> Result<(), LuxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LuxError {
                line: self.line,
                location: "".to_string(),
                message: "Unterminated string".to_string(),
            });
        }

        self.advance();
        let value = (&self.source[self.start + 1..self.current - 1]).to_string();
        self.add_token_literal(Types::STRING, Literal::String(value));
        Ok(())
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance();
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token_literal(
            Types::NUMBER,
            Literal::Number((&self.source[self.start..self.current]).parse().unwrap()),
        )
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let token_type = self
            .keywords
            .get(text)
            .unwrap_or(&Types::IDENTIFIER)
            .to_owned();

        self.add_token(token_type);
    }

    fn is_alpha(char: char) -> bool {
        ('a'..='z').contains(&char) || ('A'..='Z').contains(&char) || char == '_'
    }

    fn is_digit(char: char) -> bool {
        ('0'..='9').contains(&char)
    }

    fn is_alphanumeric(char: char) -> bool {
        Scanner::is_alpha(char) || Scanner::is_digit(char)
    }
}
