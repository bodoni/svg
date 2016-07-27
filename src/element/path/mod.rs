//! The [path][1] element.
//!
//! [1]: https://www.w3.org/TR/SVG/paths.html#PathElement

mod command;
mod data;
mod parameters;

pub use self::command::Command;
pub use self::data::Data;
pub use self::parameters::Parameters;

/// A positioning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Positioning {
    /// Absolute.
    Absolute,
    /// Relative.
    Relative,
}

element! {
    #[doc = "A path element."]
    struct Path("path")
}
