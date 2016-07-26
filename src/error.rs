//! The errors.

use std::{error, fmt, io};

/// A parsing error.
#[derive(Debug)]
pub struct Parse {
    line: usize,
    column: usize,
    message: String,
}

/// A reading error.
pub type Read = io::Error;

impl Parse {
    /// Create an error.
    #[inline]
    pub fn new<T: Into<String>>((line, column): (usize, usize), message: T) -> Self {
        Parse { line: line, column: column, message: message.into() }
    }
}

impl error::Error for Parse {
    #[inline]
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Parse {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.line > 0 && self.column > 0 {
            write!(formatter, "{} (line {}, column {})", self.message, self.line, self.column)
        } else if self.line > 0 {
            write!(formatter, "{} (line {})", self.message, self.line)
        } else {
            self.message.fmt(formatter)
        }
    }
}
