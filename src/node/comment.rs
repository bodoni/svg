use std::fmt;

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
    fn append(&mut self, _: Box<dyn Node>) {}

    #[inline]
    fn assign<T, U>(&mut self, _: T, _: U)
    where
        T: Into<String>,
        U: Into<Value>,
    {
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
