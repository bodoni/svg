//! An SVG parser.

#![feature(collections)]
#![cfg_attr(test, feature(core))]

mod reader;
pub mod path;

pub struct Error {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Debug for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{} (line {}, column {})", self.message, self.line, self.column)
    }
}
