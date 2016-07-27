//! The path element.

mod command;
mod data;
mod parameters;

pub use self::command::Command;
pub use self::data::Data;
pub use self::parameters::Parameters;

/// A type of positioning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Position {
    /// Absolute.
    Absolute,
    /// Relative.
    Relative,
}
