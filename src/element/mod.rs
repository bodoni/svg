//! The elements.

use std::fmt;

use Node;

mod value;

pub use self::value::Value;

/// An element.
#[derive(Debug)]
pub struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Box<Node>>,
}

impl Element {
    /// Create an element.
    #[inline]
    pub fn new<T: Into<String>>(name: T) -> Self {
        Element { name: name.into(), attributes: vec![], children: vec![] }
    }

    /// Append a node.
    #[inline]
    pub fn append<T: 'static + Node>(&mut self, node: T) {
        self.children.push(Box::new(node));
    }

    /// Assign an attribute.
    #[inline]
    pub fn assign<T: Into<String>, U: Value>(&mut self, name: T, value: U) {
        self.attributes.push((name.into(), value.into()));
    }
}

impl fmt::Display for Element {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(formatter, "<{}", self.name));
        for &(ref name, ref value) in self.attributes.iter() {
            try!(write!(formatter, " {}='{}'", name, value));
        }
        if self.children.is_empty() {
            write!(formatter, "/>")
        } else {
            try!(write!(formatter, ">"));
            for child in self.children.iter() {
                try!(write!(formatter, "\n{}", child));
            }
            write!(formatter, "\n</{}>", self.name)
        }
    }
}

macro_rules! element {
    ($(#[$attribute:meta])* pub struct $struct_name:ident($name:expr)) => (
        $(#[$attribute])*
        #[derive(Debug)]
        pub struct $struct_name(::element::Element);

        impl $struct_name {
            /// Create a node.
            #[inline]
            pub fn new() -> Self {
                $struct_name(::element::Element::new($name))
            }

            /// Append a node.
            pub fn add<T: 'static + ::Node>(mut self, node: T) -> Self {
                self.0.append(node);
                self
            }

            /// Assign an attribute.
            #[inline]
            pub fn set<T: Into<String>, U: ::element::Value>(mut self, name: T, value: U) -> Self {
                self.0.assign(name, value);
                self
            }
        }

        impl ::std::fmt::Display for $struct_name {
            #[inline]
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl ::Node for $struct_name {
        }
    );
}

pub mod path;
pub mod svg;

pub use self::path::Path;
pub use self::svg::SVG;
