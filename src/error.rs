use std::fmt;

#[derive(Debug, Clone)]
pub struct LuxError {
    pub message: String,
    pub line: usize,
    pub location: String,
}

// Errors should be printable.
impl fmt::Display for LuxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "[line {} Error {}: {}",
            self.line, self.location, self.message
        )
    }
}

impl std::error::Error for LuxError { }