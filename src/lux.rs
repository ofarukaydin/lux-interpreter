use crate::{interpreter::Interpreter, parser::Parser, scanner::Scanner, stmt::Stmt};
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

    fn run(&mut self, source: &str) -> Vec<Stmt> {
        let mut sc = Scanner::new(source.to_string());
        let tokens = sc.scan_tokens().to_owned();
        let mut parser = Parser::new(tokens);
        let statements = match parser.parse() {
            Ok(val) => val,
            Err(err) => {
                self.had_error = true;
                println!("{}", err.to_string());
                std::process::exit(65)
            }
        };
        statements
    }

    pub fn run_file<P>(&mut self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut file = File::open(&path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let mut interpreter = Interpreter::new();
        let statements = self.run(&buffer);
        match interpreter.interpret(&statements) {
            Ok(_) => println!("Executed successfully"),
            Err(err) => {
                self.had_runtime_error = true;
                println!("{}", err.to_string());
                std::process::exit(70)
            }
        }
        self.had_error = false;
        Ok(())
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut interpreter = Interpreter::new();
        for line_result in stdin.lock().lines() {
            let line = line_result?;
            if line.is_empty() {
                break;
            }
            let statements = self.run(&line);
            match interpreter.interpret(&statements) {
                Ok(_) => println!("statement executed"),
                Err(err) => {
                    self.had_runtime_error = true;
                    println!("{}", err.to_string());
                    std::process::exit(70)
                }
            }
            self.had_error = false;
        }
        Ok(())
    }
}
