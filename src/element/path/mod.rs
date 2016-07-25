//! The path element.

use std::fmt;

use node::{Attributes, Node};

mod data;

pub use self::data::{Data, Command, Position};

/// A [path][1] element.
///
/// [1]: https://www.w3.org/TR/SVG/paths.html#PathElement
#[derive(Default)]
pub struct Path {
    attributes: Attributes,
}

impl Path {
    /// Create an element.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let attributes = self.attributes.to_string();
        if attributes.is_empty() {
            write!(formatter, "<path/>")
        } else {
            write!(formatter, "<path {}/>", self.attributes)
        }
    }
}

impl Node for Path { }
