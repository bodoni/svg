//! An SVG parser.

#![feature(collections, io, std_misc)]
#![cfg_attr(test, feature(core, path))]

use std::fmt;

pub use file::File;
pub use parser::{Event, Parser};

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
pub fn open(path: &Path) -> Result<File> {
    File::open(path)
}

#[cfg(test)]
mod tests {
    pub fn find_fixture(name: &str) -> Path {
        use std::old_io::fs::PathExtensions;

        let mut path = Path::new("tests").join_many(&["fixtures", name]);
        path.set_extension("svg");
        assert!(path.exists());

        path
    }
}
