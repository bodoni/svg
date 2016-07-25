use std::fmt;

use node::{Attributes, Children, Node};

/// An [svg][1] element.
///
/// [1]: https://www.w3.org/TR/SVG/struct.html#SVGElement
#[derive(Default)]
pub struct SVG {
    attributes: Attributes,
    children: Children,
}

impl SVG {
    /// Create an element.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

deref! { SVG::children => Children }

impl fmt::Display for SVG {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let attributes = self.attributes.to_string();
        if attributes.is_empty() {
            write!(formatter, "<svg>\n{}\n</svg>", self.children)
        } else {
            write!(formatter, "<svg {}>\n{}\n</svg>", attributes, self.children)
        }
    }
}

impl Node for SVG { }
