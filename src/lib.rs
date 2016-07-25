//! An SVG composer and parser.

use std::fmt;
use std::path::Path;

pub use file::File;
pub use parser::{Event, Parser};
pub use tag::Tag;

mod file;
mod parser;
mod reader;

pub mod path;
pub mod tag;

/// An error.
pub struct Error {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.line > 0 && self.column > 0 {
            write!(formatter, "{} (line {}, column {})", self.message, self.line, self.column)
        } else if self.line > 0 {
            write!(formatter, "{} (line {})", self.message, self.line)
        } else {
            fmt::Debug::fmt(&self.message, formatter)
        }
    }
}

/// Open a file.
#[inline]
pub fn open<T: AsRef<Path>>(path: T) -> Result<File> {
    File::open(path)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    pub fn find_fixture(name: &str) -> PathBuf {
        let mut path = PathBuf::from("tests").join("fixtures").join(name);
        path.set_extension("svg");
        assert!(fs::metadata(&path).is_ok());
        path
    }
}
