//! The [path][1] element.
//!
//! [1]: https://www.w3.org/TR/SVG/paths.html#PathElement

mod command;
mod data;

pub use self::command::{Command, Parameters, Position};
pub use self::data::Data;

node! {
    @empty
    #[doc = "An svg element."]
    pub struct Path("path") {
    }
}
