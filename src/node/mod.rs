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

pub type ChildrenIter<'a> = std::slice::Iter<'a, Box<dyn Node>>;
pub type ChildrenIterMut<'a> = std::slice::IterMut<'a, Box<dyn Node>>;

const FAKE_CHILDREN: &[Box<dyn Node>] = &[];
static mut FAKE_CHILDREN_MUT: &mut [Box<dyn Node>] = &mut [];

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

    #[allow(unused_variables)]
    /// Get a reference to the [`Value`] associated with attribute name `k`,
    /// if it is present.
    fn get_attribute(&self, k: &str) -> Option<&Value>
    {
        None
    }

    #[allow(unused)]
    /// Assign an attribute with a value explicitly.
    ///
    /// Ergonomically, [`Node::assign`] is easier to use
    /// but is not dispatch-safe, while this method is.
    fn set_attribute(&mut self, name: String, value: Value) {}

    /// Get the name of the node type.
    fn get_name(&self) -> &'static str;

    /// Iterate over references to the children of this [`Node`].
    fn iter_children(&self) -> ChildrenIter {
        FAKE_CHILDREN.iter()
    }

    /// Iterate over mutable references to the children of this [`Node`]
    fn iter_children_mut(&mut self) -> ChildrenIterMut {
        // The default implementation uses a global shared mutable slice
        // which requires using unsafe. Since this slice is private and is never
        // filled with anything, this is reasonable to do.
        unsafe { FAKE_CHILDREN_MUT.iter_mut() }
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

macro_rules! node(
    ($struct_name:ident::$field_name:ident) => (
        node!($struct_name::$field_name []);
    );
    // ($struct_name:ident::$field_name:ident [$($indicator_name:ident),*])
    ($struct_name:ident::$field_name:ident [$($indicator_name:ident),*]) => (
        impl $struct_name {
            /// Append a node.
            pub fn add<T>(mut self, node: T) -> Self
            where
                T: Into<Box<dyn crate::node::Node>>,
            {
                crate::node::Node::append(&mut self, node);
                self
            }

            /// Assign an attribute.
            #[inline]
            pub fn set<T, U>(mut self, name: T, value: U) -> Self
            where
                T: Into<String>,
                U: Into<crate::node::Value>,
            {
                crate::node::Node::assign(&mut self, name, value);
                self
            }
        }

        impl crate::node::Node for $struct_name {
            #[inline]
            fn append<T>(&mut self, node: T)
            where
                T: Into<Box<dyn crate::node::Node>>,
            {
                self.$field_name.append(node);
            }

            fn get_attribute(&self, k: &str) -> Option<&Value> {
                self.$field_name.get_attribute(k)
            }

            fn get_name(&self) -> &str {
                self.$field_name.get_name()
            }

            fn iter_children(&self) -> crate::node::ChildrenIter {
                self.$field_name.iter_children()
            }

            fn iter_children_mut(&mut self) -> crate::node::ChildrenIterMut {
                self.$field_name.iter_children_mut()
            }

            #[inline]
            fn assign<T, U>(&mut self, name: T, value: U)
            where
                T: Into<String>,
                U: Into<crate::node::Value>,
            {
                self.$field_name.assign(name, value);
            }

            #[inline]
            fn set_attribute(&mut self, name: String, value: Value)
            {
                self.$field_name.set_attribute(name, value);
            }

            $(
                #[inline]
                fn $indicator_name(&self) -> bool {
                    true
                }
            )*
        }

        impl ::std::ops::Deref for $struct_name {
            type Target = Element;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field_name
            }
        }

        impl ::std::ops::DerefMut for $struct_name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field_name
            }
        }

        impl ::std::fmt::Display for $struct_name {
            #[inline]
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                if self.is_bareable() {
                    write!(formatter, "{:#}", self.$field_name)
                } else {
                    self.$field_name.fmt(formatter)
                }
            }
        }

        impl From<$struct_name> for Element {
            #[inline]
            fn from(value: $struct_name) -> Self {
                value.$field_name
            }
        }
    );
);

pub mod element;
