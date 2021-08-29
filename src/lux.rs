use crate::{error::LuxError, parser::Parser, scanner::Scanner, token::Token, token_type::Types};
use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
};

pub struct Lux {
    pub had_error: bool,
    pub had_runtime_error: bool,
}

impl Lux {
    pub fn new() -> Lux {
        Lux {
            had_error: false,
            had_runtime_error: false,
        }
    }

    fn run(&mut self, source: &str) {
        let mut sc = Scanner::new(source.to_string());

        let tokens = sc.scan_tokens().to_owned();

        let mut parser = Parser::new(tokens);

        let expr = match parser.parse() {
            Ok(val) => {
                // println!("{}", val.visit());
                val
            }
            Err(err) => {
                self.had_error = true;
                println!("{}", err.to_string());
                std::process::exit(65)
            }
        };

        match expr.interpret() {
            Ok(result) => println!("{}", result),
            Err(err) => {
                self.had_runtime_error = true;
                println!("{}", err.to_string());
                std::process::exit(70)
            }
        }
    }

    pub fn run_file<P>(&mut self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut file = File::open(&path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        self.run(&buffer);
        Ok(())
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        for line_result in stdin.lock().lines() {
            let line = line_result?;
            if line.is_empty() {
                break;
            }
            self.run(&line);
            self.had_error = false;
        }
        Ok(())
    }
    // to be implemented had_error = true;
}
