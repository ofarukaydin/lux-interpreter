use crate::{
    error::LuxError,
    expr::Expr,
    token::{Token, TokenLiteral},
    token_type::Types,
    Lux,
};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    lux: &'a mut Lux,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, lux: &'a mut Lux) -> Self {
        Parser {
            current: 0,
            tokens,
            lux,
        }
    }

    fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(vec![Types::BANG_EQUAL, Types::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.matches(vec![
            Types::GREATER,
            Types::GREATER_EQUAL,
            Types::LESS,
            Types::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.matches(vec![Types::MINUS, Types::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(vec![Types::SLASH, Types::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(vec![Types::BANG, Types::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.matches(vec![Types::FALSE]) {
            return Expr::Literal {
                value: TokenLiteral::False,
            };
        } else if self.matches(vec![Types::TRUE]) {
            return Expr::Literal {
                value: TokenLiteral::True,
            };
        } else if self.matches(vec![Types::NIL]) {
            return Expr::Literal {
                value: TokenLiteral::None,
            };
        } else if self.matches(vec![Types::NUMBER, Types::STRING]) {
            let expr = self.expression();
            self.consume(Types::RIGHT_PAREN, "Expect ')' after expression");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }
        self.primary()
    }

    fn consume(&mut self, token_type: Types, message: &str) -> Result<&Token, LuxError> {
        if self.check(token_type) {
            return Ok(self.advance());
        } else {
            let token = self.peek();
            let mut err = self.error(token, message);
            return Err(err);
        }
    }

    fn error(&mut self, token: &Token, message: &str) -> LuxError {
        self.lux.error(token, message)
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().type_t == Types::SEMICOLON {
                return;
            }
            match self.peek().type_t {
                Types::CLASS
                | Types::FUN
                | Types::VAR
                | Types::FOR
                | Types::IF
                | Types::WHILE
                | Types::PRINT
                | Types::RETURN => return,
                _ => self.advance(),
            };
        }
    }

    fn matches(&mut self, token_types: Vec<Types>) -> bool {
        for token in token_types {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: Types) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().type_t == token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().type_t == Types::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
