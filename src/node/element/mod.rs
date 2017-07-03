//! The element nodes.

use std::fmt;

use node::{Attributes, Children, Node, Value};

pub mod path;
pub mod tag;

#[doc(hidden)]
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
}

impl fmt::Display for Element {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(formatter, "<{}", self.name));
        let mut attributes = self.attributes.iter().collect::<Vec<_>>();
        attributes.sort_by_key(|pair| pair.0.as_str());
        for (name, value) in attributes {
            match (value.contains("'"), value.contains('"')) {
                (true, false) | (false, false) => {
                    try!(write!(formatter, r#" {}="{}""#, name, value));
                }
                (false, true) => {
                    try!(write!(formatter, r#" {}='{}'"#, name, value));
                }
                _ => {}
            }
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

        node! { $struct_name::inner }
    )*);
}

implement! {
    #[doc = "An [`animate`][1] element.
    [1]: https://www.w3.org/TR/SVG/animate.html#AnimateElement"]
    struct Animate

    #[doc = "An [`animateColor`][1] element.
    [1]: https://www.w3.org/TR/SVG/animate.html#AnimateColorElement"]
    struct AnimateColor

    #[doc = "An [`animateMotion`][1] element.
    [1]: https://www.w3.org/TR/SVG/animate.html#AnimateMotionElement"]
    struct AnimateMotion

    #[doc = "An [`animateTransform`][1] element.
    [1]: https://www.w3.org/TR/SVG/animate.html#AnimateTransformElement"]
    struct AnimateTransform

    #[doc = "A [`circle`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#CircleElement"]
    struct Circle

    #[doc = "A [`clipPath`][1] element.
    [1]: https://www.w3.org/TR/SVG/masking.html#ClipPathElement"]
    struct ClipPath

    #[doc = "A [`defs`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#DefsElement"]
    struct Definitions

    #[doc = "A [`desc`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#DescElement"]
    struct Description

    #[doc = "An [`ellipse`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#EllipseElement"]
    struct Ellipse

    #[doc = "A [`filter`][1] element.
    [1]: https://www.w3.org/TR/SVG/filters.html#FilterElement"]
    struct Filter

    #[doc = "A [`g`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#GElement"]
    struct Group

    #[doc = "An [`image`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#ImageElement"]
    struct Image

    #[doc = "A [`line`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#LineElement"]
    struct Line

    #[doc = "A [`linearGradient`][1] element.
    [1]: https://www.w3.org/TR/SVG/pservers.html#LinearGradientElement"]
    struct LinearGradient

    #[doc = "An [`a`][1] element.
    [1]: https://www.w3.org/TR/SVG/linking.html#AElement"]
    struct Link

    #[doc = "A [`marker`][1] element.
    [1]: https://www.w3.org/TR/SVG/painting.html#MarkerElement"]
    struct Marker

    #[doc = "A [`mask`][1] element.
    [1]: https://www.w3.org/TR/SVG/masking.html#MaskElement"]
    struct Mask

    #[doc = "An [`mpath`][1] element.
    [1]: https://www.w3.org/TR/SVG/animate.html#MPathElement"]
    struct MotionPath

    #[doc = "A [`path`][1] element.
    [1]: https://www.w3.org/TR/SVG/paths.html#PathElement"]
    struct Path

    #[doc = "A [`polygon`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#PolygonElement"]
    struct Polygon

    #[doc = "A [`polyline`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#PolylineElement"]
    struct Polyline

    #[doc = "A [`radialGradient`][1] element.
    [1]: https://www.w3.org/TR/SVG/pservers.html#RadialGradientElement"]
    struct RadialGradient

    #[doc = "A [`rect`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#RectElement"]
    struct Rectangle

    #[doc = "A [`stop`][1] element.
    [1]: https://www.w3.org/TR/SVG/pservers.html#StopElement"]
    struct Stop

    #[doc = "A [`symbol`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#SymbolElement"]
    struct Symbol

    #[doc = "A [`text`][1] element.
    [1]: https://www.w3.org/TR/SVG/text.html#TextElement"]
    struct Text

    #[doc = "A [`textPath`][1] element.
    [1]: https://www.w3.org/TR/SVG/text.html#TextPathElement"]
    struct TextPath

    #[doc = "A [`title`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#TitleElement"]
    struct Title

    #[doc = "A [`use`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#UseElement"]
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
                        inner: inner,
                    }
                }
            }
        }

        node! { $struct_name::inner }
    )*);
}

implement! {
    #[doc = "An [`svg`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#SVGElement"]
    struct SVG [] [inner] {
        inner.assign("xmlns", "http://www.w3.org/2000/svg");
    }

    #[doc = "A [`script`][1] element.
    [1]: https://www.w3.org/TR/SVG/script.html#ScriptElement"]
    struct Script [T: Into<String>] [inner, content: T] {
        inner.append(::node::Text::new(content));
    }

    #[doc = "A [`style`][1] element.
    [1]: https://www.w3.org/TR/SVG/styling.html#StyleElement"]
    struct Style [T: Into<String>] [inner, content: T] {
        inner.append(::node::Text::new(content));
    }
}

#[cfg(test)]
mod tests {
    use node::Node;
    use super::{Element, Style};

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
