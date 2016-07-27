use element::SVG;

/// A document.
pub struct Document {
    root: SVG,
}

impl Document {
    /// Create a document.
    pub fn new() -> Document {
        let root = SVG::new().set("xmlns", "http://www.w3.org/2000/svg");
        Document { root: root }
    }
}

node! { Document::root }
