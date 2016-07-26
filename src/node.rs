//! The nodes.

use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::fmt;

/// Attributes.
#[derive(Clone, Debug, Default)]
pub struct Attributes(HashMap<String, String>);

/// Children.
#[derive(Debug, Default)]
pub struct Children(Vec<Box<Node>>);

/// A node.
pub trait Node: fmt::Debug + fmt::Display {
}

impl Attributes {
    /// Get an attribute.
    #[inline]
    pub fn get<T: Into<String>>(&self, name: T) -> Option<&str> {
        let name = name.into().to_ascii_lowercase();
        self.0.get(&name).map(|name| name.as_str())
    }

    /// Set an attribute.
    #[inline]
    pub fn set<T: Into<String>>(&mut self, name: T, value: T) {
        self.0.insert(name.into().to_ascii_lowercase(), value.into());
    }
}

deref! { Attributes::0 => HashMap<String, String> }

impl fmt::Display for Attributes {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Children {
    /// Append a child.
    #[inline]
    pub fn append<T: 'static + Node>(&mut self, node: T) {
        self.0.push(Box::new(node))
    }
}

deref! { Children::0 => [Box<Node>] }

impl fmt::Display for Children {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let count = self.0.len();
        for i in 0..count {
            if i > 0 {
                try!(write!(formatter, "\n{}", self.0[i]));
            } else {
                try!(write!(formatter, "{}", self.0[i]));
            }
        }
        Ok(())
    }
}
