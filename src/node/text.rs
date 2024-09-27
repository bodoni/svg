use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::Hash;

use crate::node::Node;

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
        escape(&self.content).fmt(formatter)
    }
}

impl Node for Text {
    #[inline]
    fn get_name(&self) -> &str {
        "text"
    }

    #[inline]
    fn is_bare(&self) -> bool {
        true
    }
}

impl super::NodeDefaultHash for Text {
    #[inline]
    fn default_hash(&self, state: &mut DefaultHasher) {
        self.content.hash(state);
    }
}

pub(crate) fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
