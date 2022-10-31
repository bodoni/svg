//! The element nodes.

#![allow(clippy::new_without_default)]
#![allow(clippy::should_implement_trait)]

use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::Hash;

use crate::node::{Attributes, Children, Node, Value};

pub mod path;
pub mod tag;

/// An element.
#[derive(Clone, Debug)]
pub struct Element {
    name: String,
    attributes: Attributes,
    children: Children,
}

impl Element {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Element {
            name: name.into(),
            attributes: Attributes::new(),
            children: Children::new(),
        }
    }

    #[inline]
    pub fn get_name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }

    #[inline]
    pub fn get_children(&self) -> &Children {
        &self.children
    }
}

impl fmt::Display for Element {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "<{}", self.name)?;
        let mut attributes = self.attributes.iter().collect::<Vec<_>>();
        attributes.sort_by_key(|pair| pair.0.as_str());
        for (name, value) in attributes {
            match (value.contains('\''), value.contains('"')) {
                (true, false) | (false, false) => {
                    write!(formatter, r#" {}="{}""#, name, value)?;
                }
                (false, true) => {
                    write!(formatter, r#" {}='{}'"#, name, value)?;
                }
                _ => {}
            }
        }
        if self.children.is_empty() {
            return write!(formatter, "/>");
        }
        write!(formatter, ">")?;
        for child in self.children.iter() {
            write!(formatter, "\n{}", child)?;
        }
        write!(formatter, "\n</{}>", self.name)
    }
}

impl Node for Element {
    #[inline]
    fn append<T>(&mut self, node: T)
    where
        T: Node,
    {
        self.children.push(Box::new(node));
    }

    #[inline]
    fn assign<T, U>(&mut self, name: T, value: U)
    where
        T: Into<String>,
        U: Into<Value>,
    {
        self.attributes.insert(name.into(), value.into());
    }
}

macro_rules! implement {
    ($(#[$doc:meta] struct $struct_name:ident)*) => ($(
        #[$doc]
        #[derive(Clone, Debug)]
        pub struct $struct_name {
            inner: Element,
        }

        impl $struct_name {
            /// Create a node.
            #[inline]
            pub fn new() -> Self {
                $struct_name {
                    inner: Element::new(tag::$struct_name),
                }
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl super::NodeDefaultHash for $struct_name {
            #[inline]
            fn default_hash(&self, state: &mut DefaultHasher) {
                self.inner.default_hash(state);
            }
        }

        node! { $struct_name::inner }
    )*);
}

impl super::NodeDefaultHash for Element {
    fn default_hash(&self, state: &mut DefaultHasher) {
        self.name.hash(state);
        self.attributes.iter().for_each(|(key, value)| {
            key.hash(state);
            value.hash(state)
        });
        self.children
            .iter()
            .for_each(|child| child.default_hash(state));
    }
}

implement! {
    #[doc = "An [`animate`](https://www.w3.org/TR/SVG/animate.html#AnimateElement) element."]
    struct Animate

    #[doc = "An [`animateColor`](https://www.w3.org/TR/SVG/animate.html#AnimateColorElement) element."]
    struct AnimateColor

    #[doc = "An [`animateMotion`](https://www.w3.org/TR/SVG/animate.html#AnimateMotionElement) element."]
    struct AnimateMotion

    #[doc = "An [`animateTransform`](https://www.w3.org/TR/SVG/animate.html#AnimateTransformElement) element."]
    struct AnimateTransform

    #[doc = "A [`circle`](https://www.w3.org/TR/SVG/shapes.html#CircleElement) element."]
    struct Circle

    #[doc = "A [`clipPath`](https://www.w3.org/TR/SVG/masking.html#ClipPathElement) element."]
    struct ClipPath

    #[doc = "A [`defs`](https://www.w3.org/TR/SVG/struct.html#DefsElement) element."]
    struct Definitions

    #[doc = "A [`desc`](https://www.w3.org/TR/SVG/struct.html#DescElement) element."]
    struct Description

    #[doc = "An [`ellipse`](https://www.w3.org/TR/SVG/shapes.html#EllipseElement) element."]
    struct Ellipse

    #[doc = "A [`filter`](https://www.w3.org/TR/SVG/filters.html#FilterElement) element."]
    struct Filter

    #[doc = "A [`foreignObject`](https://www.w3.org/TR/SVG/embedded.html#ForeignObjectElement) element."]
    struct ForeignObject

    #[doc = "A [`g`](https://www.w3.org/TR/SVG/struct.html#GElement) element."]
    struct Group

    #[doc = "An [`image`](https://www.w3.org/TR/SVG/struct.html#ImageElement) element."]
    struct Image

    #[doc = "A [`line`](https://www.w3.org/TR/SVG/shapes.html#LineElement) element."]
    struct Line

    #[doc = "A [`linearGradient`](https://www.w3.org/TR/SVG/pservers.html#LinearGradientElement) element."]
    struct LinearGradient

    #[doc = "An [`a`](https://www.w3.org/TR/SVG/linking.html#AElement) element."]
    struct Link

    #[doc = "A [`marker`](https://www.w3.org/TR/SVG/painting.html#MarkerElement) element."]
    struct Marker

    #[doc = "A [`mask`](https://www.w3.org/TR/SVG/masking.html#MaskElement) element."]
    struct Mask

    #[doc = "An [`mpath`](https://www.w3.org/TR/SVG/animate.html#MPathElement) element."]
    struct MotionPath

    #[doc = "A [`path`](https://www.w3.org/TR/SVG/paths.html#PathElement) element."]
    struct Path

    #[doc = "A [`pattern`](https://www.w3.org/TR/SVG/pservers.html#PatternElement) element."]
    struct Pattern

    #[doc = "A [`polygon`](https://www.w3.org/TR/SVG/shapes.html#PolygonElement) element."]
    struct Polygon

    #[doc = "A [`polyline`](https://www.w3.org/TR/SVG/shapes.html#PolylineElement) element."]
    struct Polyline

    #[doc = "A [`radialGradient`](https://www.w3.org/TR/SVG/pservers.html#RadialGradientElement) element."]
    struct RadialGradient

    #[doc = "A [`rect`](https://www.w3.org/TR/SVG/shapes.html#RectElement) element."]
    struct Rectangle

    #[doc = "A [`stop`](https://www.w3.org/TR/SVG/pservers.html#StopElement) element."]
    struct Stop

    #[doc = "A [`symbol`](https://www.w3.org/TR/SVG/struct.html#SymbolElement) element."]
    struct Symbol

    #[doc = "A [`text`](https://www.w3.org/TR/SVG/text.html#TextElement) element."]
    struct Text

    #[doc = "A [`textPath`](https://www.w3.org/TR/SVG/text.html#TextPathElement) element."]
    struct TextPath

    #[doc = "A [`title`](https://www.w3.org/TR/SVG/struct.html#TitleElement) element."]
    struct Title

    #[doc = "A [`tspan`](https://www.w3.org/TR/SVG/text.html#TextElement) element."]
    struct TSpan

    #[doc = "A [`use`](https://www.w3.org/TR/SVG/struct.html#UseElement) element."]
    struct Use
}

macro_rules! implement {
    (@itemize $i:item) => ($i);
    ($(
        #[$doc:meta]
        struct $struct_name:ident
        [$($pn:ident: $($pt:tt)*),*] [$inner:ident $(,$an:ident: $at:ty)*] $body:block
    )*) => ($(
        #[$doc]
        #[derive(Clone, Debug)]
        pub struct $struct_name {
            inner: Element,
        }

        implement! { @itemize
            impl $struct_name {
                /// Create a node.
                #[inline]
                pub fn new<$($pn: $($pt)*),*>($($an: $at),*) -> Self {
                    #[inline(always)]
                    fn initialize<$($pn: $($pt)*),*>($inner: &mut Element $(, $an: $at)*) $body
                    let mut inner = Element::new(tag::$struct_name);
                    initialize(&mut inner $(, $an)*);
                    $struct_name {
                        inner,
                    }
                }
            }
        }

        impl super::NodeDefaultHash for $struct_name {
            fn default_hash(&self, state: &mut DefaultHasher) {
                self.inner.default_hash(state);
            }
        }

        node! { $struct_name::inner }
    )*);
}

implement! {
    #[doc = "An [`svg`](https://www.w3.org/TR/SVG/struct.html#SVGElement) element."]
    struct SVG [] [inner] {
        inner.assign("xmlns", "http://www.w3.org/2000/svg");
    }

    #[doc = "A [`script`](https://www.w3.org/TR/SVG/script.html#ScriptElement) element."]
    struct Script [T: Into<String>] [inner, content: T] {
        inner.append(crate::node::Text::new(content));
    }

    #[doc = "A [`style`](https://www.w3.org/TR/SVG/styling.html#StyleElement) element."]
    struct Style [T: Into<String>] [inner, content: T] {
        inner.append(crate::node::Text::new(content));
    }
}

#[cfg(test)]
mod tests {
    use super::{Element, Style};
    use crate::node::Node;

    #[test]
    fn element_display() {
        let mut element = Element::new("foo");
        element.assign("x", -10);
        element.assign("y", "10px");
        element.assign("s", (12.5, 13.0));
        element.assign("c", "green");
        element.append(Element::new("bar"));

        assert_eq!(
            element.to_string(),
            "<foo c=\"green\" s=\"12.5 13\" x=\"-10\" y=\"10px\">\n\
             <bar/>\n\
             </foo>\
             "
        );
    }

    #[test]
    fn element_display_quotes() {
        let mut element = Element::new("foo");
        element.assign("s", "'single'");
        element.assign("d", r#""double""#);
        element.assign("m", r#""mixed'"#);

        assert_eq!(element.to_string(), r#"<foo d='"double"' s="'single'"/>"#);
    }

    #[test]
    fn style_display() {
        let element = Style::new("* { font-family: foo; }");

        assert_eq!(
            element.to_string(),
            "<style>\n\
             * { font-family: foo; }\n\
             </style>\
             "
        );
    }
}
