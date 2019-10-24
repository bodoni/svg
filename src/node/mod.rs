//! The nodes.

use std::collections::HashMap;
use std::fmt;

mod comment;
mod text;
mod value;

pub use self::comment::Comment;
pub use self::text::Text;
pub use self::value::Value;

/// Attributes.
pub type Attributes = HashMap<String, Value>;

/// Child nodes.
pub type Children = Vec<Box<dyn Node>>;

/// A node.
pub trait Node: 'static + fmt::Debug + fmt::Display + NodeClone {
    /// Append a child node.
    fn append<T>(&mut self, _: T)
    where
        Self: Sized,
        T: Node;

    /// Assign an attribute.
    fn assign<T, U>(&mut self, _: T, _: U)
    where
        Self: Sized,
        T: Into<String>,
        U: Into<Value>;
}

#[doc(hidden)]
pub trait NodeClone {
    fn clone(&self) -> Box<dyn Node>;
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

macro_rules! node(
    ($struct_name:ident::$field_name:ident) => (
        impl $struct_name {
            /// Append a node.
            pub fn add<T>(&mut self, node: T) -> &mut Self
            where
                T: crate::node::Node,
            {
                crate::node::Node::append(self, node);
                self
            }

            /// Assign an attribute.
            #[inline]
            pub fn set<T, U>(&mut self, name: T, value: U) -> &mut Self
            where
                T: Into<String>,
                U: Into<crate::node::Value>,
            {
                crate::node::Node::assign(self, name, value);
                self
            }
        }

        impl crate::node::Node for $struct_name {
            #[inline]
            fn append<T>(&mut self, node: T)
            where
                T: crate::node::Node,
            {
                self.$field_name.append(node);
            }

            #[inline]
            fn assign<T, U>(&mut self, name: T, value: U)
            where
                T: Into<String>,
                U: Into<crate::node::Value>,
            {
                self.$field_name.assign(name, value);
            }
        }

        impl ::std::fmt::Display for $struct_name {
            #[inline]
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.$field_name.fmt(formatter)
            }
        }
    );
);

pub mod element;
