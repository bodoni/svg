use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::Hash;

use crate::node::{Node, Value};

/// A comment node.
#[derive(Clone, Debug)]
pub struct Comment {
    content: String,
}

impl Comment {
    /// Create a node.
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

impl super::NodeDefaultHash for Comment {
    #[inline]
    fn default_hash(&self, state: &mut DefaultHasher) {
        self.content.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::Comment;

    #[test]
    fn comment_display() {
        let comment = Comment::new("valid");
        assert_eq!(comment.to_string(), "<!-- valid -->");

        let comment = Comment::new("invalid -->");
        assert_eq!(comment.to_string(), "<!-- invalid --> -->");
    }
}
