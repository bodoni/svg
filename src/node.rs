//! The nodes.

use std::ascii::AsciiExt;
use std::collections::HashMap;

use composer::{Composer, Result};

/// Attributes.
#[derive(Clone, Debug)]
pub struct Attributes {
    mapping: HashMap<String, String>,
}

/// A node.
#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    attributes: Attributes,
    children: Vec<Node>,
}

impl Attributes {
    /// Create attributes.
    #[inline]
    pub fn new() -> Self {
        Attributes { mapping: HashMap::new() }
    }

    /// Get an attribute.
    #[inline]
    pub fn get<T: Into<String>>(&self, name: T) -> Option<&str> {
        let name = name.into().to_ascii_lowercase();
        self.mapping.get(&name).map(|name| name.as_str())
    }

    /// Set an attribute.
    #[inline]
    pub fn set<T: Into<String>>(&mut self, name: T, value: T) {
        self.mapping.insert(name.into().to_ascii_lowercase(), value.into());
    }
}

impl Node {
    /// Create a node.
    pub fn new<T: Into<String>>(name: T) -> Self {
        Node { name: name.into(), attributes: Attributes::new(), children: vec![] }
    }

    /// Append a node.
    pub fn append<T: Into<Node>>(&mut self, node: T) {
        self.children.push(node.into());
    }

    /// Write the node.
    pub fn compose<T>(&mut self, _: &mut Composer<T>) -> Result<()> {
        Ok(())
    }
}

deref! { Node::attributes => Attributes }
