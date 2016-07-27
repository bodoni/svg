//! Elements.

use std::fmt;

use node::{Attributes, Children, Node, Value};

/// An element.
pub struct Element {
    name: String,
    attributes: Attributes,
    children: Children,
}

impl Element {
    /// Create an element.
    #[inline]
    pub fn new<T>(name: T) -> Self where T: Into<String> {
        Element {
            name: name.into(),
            attributes: Attributes::new(),
            children: Children::new(),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(formatter, "<{}", self.name));
        let mut attributes = self.attributes.iter().collect::<Vec<_>>();
        attributes.sort_by_key(|pair| pair.0.as_str());
        for (name, value) in attributes {
            try!(write!(formatter, " {}='{}'", name, value));
        }
        if self.children.is_empty() {
            return write!(formatter, "/>");
        }
        try!(write!(formatter, ">"));
        for child in self.children.iter() {
            try!(write!(formatter, "\n{}", child));
        }
        write!(formatter, "\n</{}>", self.name)
    }
}

impl Node for Element {
    #[inline]
    fn append<T>(&mut self, node: T) where T: Node {
        self.children.push(Box::new(node));
    }

    #[inline]
    fn assign<T, U>(&mut self, name: T, value: U) where T: Into<String>, U: Into<Value> {
        self.attributes.insert(name.into(), value.into());
    }
}

macro_rules! element {
    ($(#[$attribute:meta])* struct $struct_name:ident($name:expr)) => (
        $(#[$attribute])*
        pub struct $struct_name {
            inner: ::element::Element,
        }

        impl $struct_name {
            /// Create an element.
            #[inline]
            pub fn new() -> Self {
                $struct_name { inner: ::element::Element::new($name) }
            }
        }

        node! { $struct_name::inner }
    );
}

pub mod path;

pub use self::path::Path;

element! {
    #[doc = "
    An [svg][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#SVGElement"]
    struct SVG("svg")
}

#[cfg(test)]
mod tests {
    use node::Node;
    use super::Element;

    #[test]
    fn display() {
        let mut element = Element::new("foo");
        element.assign("x", -15);
        element.assign("y", "10px");
        element.assign("size", (42.5, 69.0));
        element.assign("color", "green");
        element.append(Element::new("bar"));
        assert_eq!(element.to_string(), "\
            <foo color='green' size='42.5 69' x='-15' y='10px'>\n\
            <bar/>\n\
            </foo>\
        ");
    }
}
