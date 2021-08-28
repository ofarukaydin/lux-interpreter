use crate::{error::LuxError, parser::Parser, scanner::Scanner, token::Token, token_type::Types};
use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
};

pub struct Lux {
    had_error: bool,
}

impl Lux {
    pub fn new() -> Lux {
        Lux { had_error: false }
    }

    fn report(&mut self, line: usize, location: &str, message: &str) -> LuxError {
        let err = LuxError {
            line,
            location: location.to_string(),
            message: message.to_string(),
        };
        println!(
            "{}",
            LuxError {
                line,
                location: location.to_string(),
                message: message.to_string()
            }
        );
        self.had_error = true;
        err
    }

    pub fn error(&mut self, token: &Token, message: &str) -> LuxError {
        if token.type_t == Types::EOF {
            self.report(token.line, " at end", message)
        } else {
            self.report(token.line, &format!(" at '{}'", &token.lexeme), message)
        }
    }

    fn run(&mut self, source: &str) {
        let mut sc = Scanner::new(source.to_string());

        let tokens = sc.scan_tokens().to_owned();

        let parser = Parser::new(tokens, self);

        if self.had_error {
            std::process::exit(65)
        };
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
