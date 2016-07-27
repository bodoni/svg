//! The path element.

use super::Element;

mod command;
mod data;
mod parameters;

pub use self::command::Command;
pub use self::data::Data;
pub use self::parameters::Parameters;

/// A [path][1] element.
///
/// [1]: https://www.w3.org/TR/SVG/paths.html#PathElement
pub struct Path {
    inner: Element,
}

/// A type of positioning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Position {
    /// Absolute.
    Absolute,
    /// Relative.
    Relative,
}

impl Path {
    /// Create an element.
    #[inline]
    pub fn new() -> Self {
        Path { inner: Element::new("path") }
    }
}

node! { Path::inner }
