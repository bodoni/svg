//! The [path][1] element.
//!
//! [1]: https://www.w3.org/TR/SVG/paths.html#PathElement

mod command;
mod data;
mod parameters;

pub use self::command::{Command, Position};
pub use self::data::Data;
pub use self::parameters::Parameters;

element! {
    #[doc = "A path element."]
    pub struct Path("path")
}
