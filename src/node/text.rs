use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::Hash;

use crate::node::{Node, Value};

/// A text node.
#[derive(Clone, Debug)]
pub struct Text {
    content: String,
}

impl Text {
    /// Create a node.
    #[inline]
    pub fn new<T>(content: T) -> Self
    where
        T: Into<String>,
    {
        Text {
            content: content.into(),
        }
    }
}

impl fmt::Display for Text {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.content.fmt(formatter)
    }
}

impl Node for Text {
    #[inline]
    fn append<T>(&mut self, _: T)
    where
        T: Node,
    {
    }

    #[inline]
    fn assign<T, U>(&mut self, _: T, _: U)
    where
        T: Into<String>,
        U: Into<Value>,
    {
    }
}

impl super::NodeDefaultHash for Text {
    #[inline]
    fn default_hash(&self, state: &mut DefaultHasher) {
        self.content.hash(state);
    }
}
