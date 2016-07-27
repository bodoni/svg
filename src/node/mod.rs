//! The nodes.

use std::collections::HashMap;
use std::fmt;

mod text;
mod value;

pub mod element;

pub use self::text::Text;
pub use self::value::Value;

/// Attributes.
pub type Attributes = HashMap<String, Value>;

/// Child nodes.
pub type Children = Vec<Box<Node>>;

/// A node.
pub trait Node: 'static + fmt::Display {
    /// Append a child node.
    fn append<T>(&mut self, T) where Self: Sized, T: Node;

    /// Assign an attribute.
    fn assign<T, U>(&mut self, T, U) where Self: Sized, T: Into<String>, U: Into<Value>;
}
