use element::SVG;

/// A document.
pub struct Document {
    root: SVG,
}

impl Document {
    /// Create a document.
    pub fn new() -> Document {
        Document { root: SVG::new().set("xmlns", "http://www.w3.org/2000/svg") }
    }
}

node! { Document::root }
