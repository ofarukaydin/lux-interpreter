use crate::{
    error::LuxError,
    expr::Expr,
    stmt::Stmt,
    token::{Token, TokenLiteral},
    token_type::Types,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

type ParserResult<T> = Result<T, LuxError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> ParserResult<Vec<Stmt>> {
        let mut statements = Vec::<Stmt>::new();
        while !self.is_at_end() {
            let stmt = self.decleration()?;
            statements.push(stmt);
        }
        Ok(statements)
    }

    fn decleration(&mut self) -> ParserResult<Stmt> {
        let decleration = if self.matches(vec![Types::VAR]) {
            self.var_decleration()
        } else {
            self.statement()
        };
        if decleration.is_err() {
            self.synchronize();
        }
        decleration
    }

    fn var_decleration(&mut self) -> ParserResult<Stmt> {
        let name = self
            .consume(Types::IDENTIFIER, "Expect a variable name.")?
            .clone();
        let initializer = if self.matches(vec![Types::EQUAL]) {
            self.expression()?
        } else {
            Expr::Nil
        };

        self.consume(Types::SEMICOLON, "Expect ';' after variable declaration.")?;

        Ok(Stmt::Var {
            name,
            initializer: Box::new(initializer),
        })
    }

    fn statement(&mut self) -> ParserResult<Stmt> {
        if self.matches(vec![Types::PRINT]) {
            self.print_statement()
        } else if self.matches(vec![Types::LEFT_BRACE]) {
            Ok(Stmt::Block {
                statements: self.block()?,
            })
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> ParserResult<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(Types::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.decleration()?)
        }

        self.consume(Types::RIGHT_BRACE, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> ParserResult<Stmt> {
        let expr = self.expression()?;
        self.consume(Types::SEMICOLON, "Expect ';' after value")?;
        Ok(Stmt::Print {
            expression: Box::new(expr),
        })
    }

    fn expression_statement(&mut self) -> ParserResult<Stmt> {
        let expr = self.expression()?;
        self.consume(Types::SEMICOLON, "Expect ';' after value")?;
        Ok(Stmt::Expression {
            expression: Box::new(expr),
        })
    }

    fn expression(&mut self) -> ParserResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParserResult<Expr> {
        let expr = self.equality()?;

        if self.matches(vec![Types::EQUAL]) {
            match expr {
                Expr::Variable { name } => {
                    let value = self.assignment()?;
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    });
                }
                _ => {
                    let equals = self.previous();
                    return Err(self.error(equals, "Invalid assignment target."));
                }
            }
        }
        Ok(expr)
    }

    fn equality(&mut self) -> ParserResult<Expr> {
        let mut expr = self.comparison()?;

        while self.matches(vec![Types::BANG_EQUAL, Types::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ParserResult<Expr> {
        let mut expr = self.term()?;

        while self.matches(vec![
            Types::GREATER,
            Types::GREATER_EQUAL,
            Types::LESS,
            Types::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> ParserResult<Expr> {
        let mut expr = self.factor()?;

        while self.matches(vec![Types::MINUS, Types::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ParserResult<Expr> {
        let mut expr = self.unary()?;

        while self.matches(vec![Types::SLASH, Types::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParserResult<Expr> {
        if self.matches(vec![Types::BANG, Types::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> ParserResult<Expr> {
        if self.matches(vec![Types::FALSE]) {
            Ok(Expr::Literal {
                value: TokenLiteral::Bool(false),
            })
        } else if self.matches(vec![Types::TRUE]) {
            Ok(Expr::Literal {
                value: TokenLiteral::Bool(true),
            })
        } else if self.matches(vec![Types::NIL]) {
            Ok(Expr::Literal {
                value: TokenLiteral::Nil,
            })
        } else if self.matches(vec![Types::NUMBER, Types::STRING]) {
            Ok(Expr::Literal {
                value: self.previous().literal.clone(),
            })
        } else if self.matches(vec![Types::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(Types::RIGHT_PAREN, "Expect ')' after expression")?;
            Ok(Expr::Grouping {
                expression: Box::new(expr),
            })
        } else if self.matches(vec![Types::IDENTIFIER]) {
            let token = self.previous().clone();
            Ok(Expr::Variable { name: token })
        } else {
            let err = LuxError::new(self.peek(), "Expect expression.");
            Err(err)
        }
    }

    fn consume(&mut self, token_type: Types, message: &str) -> ParserResult<&Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        } else {
            let token = self.peek();
            let err = self.error(token, message);
            Err(err)
        }
    }

    fn error(&self, token: &Token, message: &str) -> LuxError {
        LuxError::new(token, message)
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
