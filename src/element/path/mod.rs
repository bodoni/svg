//! The [path][1] element.
//!
//! [1]: https://www.w3.org/TR/SVG/paths.html#PathElement

mod data;

pub use self::data::{Data, Command, Position};

node! {
    @empty
    #[doc = "An svg element."]
    pub struct Path("path") {
    }
}

impl Path {
    pub fn move_to<T>(self, _: T) -> Self {
        self
    }

    pub fn line_by<T>(self, _: T) -> Self {
        self
    }

    pub fn close(self) -> Self {
        self
    }
}
