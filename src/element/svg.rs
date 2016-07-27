//! The svg element.

use super::Element;

/// An [svg][1] element.
///
/// [1]: https://www.w3.org/TR/SVG/struct.html#SVGElement
pub struct SVG {
    inner: Element,
}

impl SVG {
    /// Create an element.
    #[inline]
    pub fn new() -> Self {
        SVG { inner: Element::new("svg") }
    }
}

node! { SVG::inner }
