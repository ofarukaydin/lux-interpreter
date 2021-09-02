mod callable;
mod clock;
mod environment;
pub mod error;
mod expr;
mod function;
mod interpreter;
mod literal;
mod lux;
pub mod parser;
mod runtime_error;
pub mod scanner;
mod stmt;
pub mod token;
pub mod token_type;
use lux::Lux;
use std::env;
use text_colorizer::*;

fn main() {
    let mut lux = Lux::new();
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => lux.run_prompt().unwrap(),
        1 => lux.run_file(&args[0]).unwrap(),
        _ => {
            println!("Usage: rslux [script]");
            eprintln!(
                "{} wrong number of arguments: expected 1, got {}.",
                "Error:".red().bold(),
                args.len()
            );
            std::process::exit(64);
        }
    }
}

// if args[0] == "visit" {
//     let tree = Expr::Binary {
//         left: Box::new(Expr::Unary {
//             operator: Token {
//                 type_t: Types::MINUS,
//                 literal: Literal::Nil,
//                 line: 1,
//                 lexeme: '-'.to_string(),
//             },
//             right: Box::new(Expr::Literal {
//                 value: Literal::Number(123.0),
//             }),
//         }),
//         operator: Token {
//             type_t: Types::STAR,
//             lexeme: '*'.to_string(),
//             line: 1,
//             literal: Literal::Nil,
//         },
//         right: Box::new(Expr::Grouping {
//             expression: Box::new({
//                 Expr::Literal {
//                     value: Literal::Number(45.67),
//                 }
//             }),
//         }),
//     };
//     println!("{}", tree.visit());
