use std::fmt;

use crate::node::{Node, Value};

/// A comment node.
#[derive(Clone, Debug)]
pub struct Comment {
    content: String,
}

impl Comment {
    /// Create a comment.
    #[inline]
    pub fn new<T>(content: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            content: content.into(),
        }
    }
}

impl fmt::Display for Comment {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "<!-- {} -->", self.content)
    }
}

impl Node for Comment {
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
