pub mod error;
mod expr;
mod gen_script;
mod lux;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod token_type;
mod runtime_error;
use expr::Expr;
use gen_script::run_generator;
use lux::Lux;

use std::env;
use text_colorizer::*;
use token::{Token, TokenLiteral};
use token_type::Types;

fn main() {
    let mut lux = Lux::new();
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        println!("Usage: jlox [script]");
        eprintln!(
            "{} wrong number of arguments: expected 1, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(64);
    } else if args.len() == 1 {
        if args[0] == "gen" {
            run_generator()
        } else if args[0] == "visit" {
            let tree = Expr::Binary {
                left: Box::new(Expr::Unary {
                    operator: Token {
                        type_t: Types::MINUS,
                        literal: TokenLiteral::Nil,
                        line: 1,
                        lexeme: '-'.to_string(),
                    },
                    right: Box::new(Expr::Literal {
                        value: TokenLiteral::Number(123.0),
                    }),
                }),
                operator: Token {
                    type_t: Types::STAR,
                    lexeme: '*'.to_string(),
                    line: 1,
                    literal: TokenLiteral::Nil,
                },
                right: Box::new(Expr::Grouping {
                    expression: Box::new({
                        Expr::Literal {
                            value: TokenLiteral::Number(45.67),
                        }
                    }),
                }),
            };
            println!("{}", tree.visit());
        } else {
            lux.run_file(&args[0]);
        }
    } else {
        lux.run_prompt();
    }
}
