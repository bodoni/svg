//! The errors.

use std::{error, fmt};

/// An error.
#[derive(Debug)]
pub struct Error {
    line: usize,
    column: usize,
    message: String,
}

impl Error {
    /// Create an error.
    #[inline]
    pub fn new<T: Into<String>>((line, column): (usize, usize), message: T) -> Self {
        Error {
            line,
            column,
            message: message.into(),
        }
    }
}

impl error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.line > 0 && self.column > 0 {
            write!(
                formatter,
                "{} (line {}, column {})",
                self.message, self.line, self.column,
            )
        } else if self.line > 0 {
            write!(formatter, "{} (line {})", self.message, self.line)
        } else {
            self.message.fmt(formatter)
        }
    }
}
