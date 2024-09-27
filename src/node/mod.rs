//! The nodes.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt;

mod blob;
mod comment;
mod text;
mod value;

pub use self::blob::Blob;
pub use self::comment::Comment;
pub use self::text::Text;
pub use self::value::Value;

/// Attributes.
pub type Attributes = HashMap<String, Value>;

/// Child nodes.
pub type Children = Vec<Box<dyn Node>>;

/// A node.
pub trait Node:
    'static + fmt::Debug + fmt::Display + NodeClone + NodeDefaultHash + Send + Sync
{
    /// Append a child node.
    #[inline]
    fn append<T>(&mut self, _: T)
    where
        Self: Sized,
        T: Into<Box<dyn Node>>,
    {
    }

    /// Assign an attribute.
    #[inline]
    fn assign<T, U>(&mut self, _: T, _: U)
    where
        Self: Sized,
        T: Into<String>,
        U: Into<Value>,
    {
    }

    /// Return the name.
    fn get_name(&self) -> &str;

    /// Return the attributes.
    #[inline]
    fn get_attributes(&self) -> Option<&Attributes> {
        None
    }

    /// Return the attributes as mutable.
    fn get_attributes_mut(&mut self) -> Option<&mut Attributes> {
        None
    }

    /// Return the children.
    fn get_children(&self) -> Option<&Children> {
        None
    }

    /// Return the children as mutable.
    fn get_children_mut(&mut self) -> Option<&mut Children> {
        None
    }

    #[doc(hidden)]
    fn is_bare(&self) -> bool {
        false
    }

    #[doc(hidden)]
    fn is_bareable(&self) -> bool {
        false
    }
}

#[doc(hidden)]
pub trait NodeClone {
    fn clone(&self) -> Box<dyn Node>;
}

#[doc(hidden)]
pub trait NodeDefaultHash {
    fn default_hash(&self, state: &mut DefaultHasher);
}

impl<T> NodeClone for T
where
    T: Node + Clone,
{
    #[inline]
    fn clone(&self) -> Box<dyn Node> {
        Box::new(Clone::clone(self))
    }
}

impl Clone for Box<dyn Node> {
    #[inline]
    fn clone(&self) -> Self {
        NodeClone::clone(&**self)
    }
}

impl<T> From<T> for Box<dyn Node>
where
    T: Node,
{
    #[inline]
    fn from(node: T) -> Box<dyn Node> {
        Box::new(node)
    }
}

impl NodeDefaultHash for Box<dyn Node> {
    #[inline]
    fn default_hash(&self, state: &mut DefaultHasher) {
        NodeDefaultHash::default_hash(&**self, state)
    }
}

pub mod element;
