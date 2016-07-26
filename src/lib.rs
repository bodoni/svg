//! An SVG composer and parser.

#[macro_use]
mod macros;

mod document;
mod reader;

pub mod element;
pub mod error;
pub mod node;
pub mod reactor;
pub mod result;
pub mod tag;

pub use document::Document;
pub use node::Node;
pub use reactor::Reactor;
pub use tag::Tag;

/// Parse a file.
#[inline]
pub fn parse<'l, T: AsRef<std::path::Path>>(path: T) -> result::Read<Reactor<'l>> {
    Document::open(path)
}
