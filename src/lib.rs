//! An SVG composer and parser.

use std::path::Path;

pub use file::File;
pub use parser::Parser;
pub use tag::Tag;

mod file;
mod reader;

pub mod parser;
pub mod tag;

/// Open a file.
#[inline]
pub fn open<T: AsRef<Path>>(path: T) -> parser::Result<File> {
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
