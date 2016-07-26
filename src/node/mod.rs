//! The nodes.

use std::fmt;

mod value;

pub use self::value::Value;

/// A node.
#[derive(Debug)]
pub struct Node {
    name: String,
    is_empty: bool,
    attributes: Vec<(String, String)>,
    children: Vec<Node>,
}

impl Node {
    /// Create a node.
    #[inline]
    pub fn new<T: Into<String>>(name: T, is_empty: bool) -> Self {
        Node {
            name: name.into(),
            is_empty: is_empty,
            attributes: Default::default(),
            children: Default::default(),
        }
    }

    /// Append a child.
    #[inline]
    pub fn append<T: Into<Node>>(&mut self, node: T) {
        self.children.push(node.into())
    }

    /// Assign an attribute.
    #[inline]
    pub fn assign<T: Into<String>, U: Value>(&mut self, name: T, value: U) {
        self.attributes.push((name.into(), value.into()));
    }
}

impl fmt::Display for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(formatter, "<{}", self.name));
        for &(ref name, ref value) in self.attributes.iter() {
            try!(write!(formatter, " {}='{}'", name, value));
        }
        if self.is_empty {
            return write!(formatter, "/>");
        }
        try!(write!(formatter, ">"));
        for child in self.children.iter() {
            try!(write!(formatter, "\n{}", child));
        }
        write!(formatter, "\n</{}>", self.name)
    }
}
