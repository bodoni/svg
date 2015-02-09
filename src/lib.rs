//! An SVG parser.

#![feature(collections)]
#![cfg_attr(test, feature(core))]

mod reader;
pub mod path;

#[derive(Debug)]
pub struct Error {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

pub type Result<T> = std::result::Result<T, Error>;
