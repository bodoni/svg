//! The element nodes.

use std::fmt;

use node::{Attributes, Children, Node, Value};

/// An element node.
pub struct Element {
    name: String,
    attributes: Attributes,
    children: Children,
}

impl Element {
    /// Create a node.
    #[inline]
    pub fn new<T>(name: T) -> Self where T: Into<String> {
        Element { name: name.into(), attributes: Attributes::new(), children: Children::new() }
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

/// A [`svg`][1] element.
///
/// [1]: https://www.w3.org/TR/SVG/struct.html#SVGElement
pub struct SVG {
    inner: Element,
}

impl SVG {
    /// Create a node.
    #[inline]
    pub fn new() -> Self {
        let mut inner = Element::new("svg");
        inner.assign("xmlns", "http://www.w3.org/2000/svg");
        SVG { inner: inner }
    }
}

node! { SVG::inner }

macro_rules! implement {
    ($(#[$doc:meta] struct $struct_name:ident($tag_name:expr))*) => (
        $(
            #[$doc]
            pub struct $struct_name {
                inner: ::node::element::Element,
            }

            impl $struct_name {
                /// Create a node.
                #[inline]
                pub fn new() -> Self {
                    $struct_name { inner: Element::new($tag_name) }
                }
            }

            node! { $struct_name::inner }
        )*
    );
}

implement! {
    #[doc = "An [`animate`][1] element.
    [1]: https://www.w3.org/TR/SVG/animate.html#AnimateElement"]
    struct Animate("animate")

    #[doc = "An [`animateColor`][1] element.
    [1]: https://www.w3.org/TR/SVG/animate.html#AnimateColorElement"]
    struct AnimateColor("animateColor")

    #[doc = "An [`animateMotion`][1] element.
    [1]: https://www.w3.org/TR/SVG/animate.html#AnimateMotionElement"]
    struct AnimateMotion("animateMotion")

    #[doc = "An [`animateTransform`][1] element.
    [1]: https://www.w3.org/TR/SVG/animate.html#AnimateTransformElement"]
    struct AnimateTransform("animateTransform")

    #[doc = "A [`circle`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#CircleElement"]
    struct Circle("circle")

    #[doc = "A [`clipPath`][1] element.
    [1]: https://www.w3.org/TR/SVG/masking.html#ClipPathElement"]
    struct ClipPath("clipPath")

    #[doc = "A [`defs`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#DefsElement"]
    struct Definitions("defs")

    #[doc = "A [`desc`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#DescElement"]
    struct Description("desc")

    #[doc = "An [`ellipse`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#EllipseElement"]
    struct Ellipse("ellipse")

    #[doc = "A [`filter`][1] element.
    [1]: https://www.w3.org/TR/SVG/filters.html#FilterElement"]
    struct Filter("filter")

    #[doc = "A [`g`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#GElement"]
    struct Group("g")

    #[doc = "An [`image`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#ImageElement"]
    struct Image("image")

    #[doc = "A [`line`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#LineElement"]
    struct Line("line")

    #[doc = "A [`linearGradient`][1] element.
    [1]: https://www.w3.org/TR/SVG/pservers.html#LinearGradientElement"]
    struct LinearGradient("linearGradient")

    #[doc = "An [`a`][1] element.
    [1]: https://www.w3.org/TR/SVG/linking.html#AElement"]
    struct Link("a")

    #[doc = "A [`marker`][1] element.
    [1]: https://www.w3.org/TR/SVG/painting.html#MarkerElement"]
    struct Marker("marker")

    #[doc = "A [`mask`][1] element.
    [1]: https://www.w3.org/TR/SVG/masking.html#MaskElement"]
    struct Mask("mask")

    #[doc = "An [`mpath`][1] element.
    [1]: https://www.w3.org/TR/SVG/animate.html#MPathElement"]
    struct MotionPath("mpath")

    #[doc = "A [`path`][1] element.
    [1]: https://www.w3.org/TR/SVG/paths.html#PathElement"]
    struct Path("path")

    #[doc = "A [`polygon`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#PolygonElement"]
    struct Polygon("polygon")

    #[doc = "A [`polyline`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#PolylineElement"]
    struct Polyline("polyline")

    #[doc = "A [`radialGradient`][1] element.
    [1]: https://www.w3.org/TR/SVG/pservers.html#RadialGradientElement"]
    struct RadialGradient("radialGradient")

    #[doc = "A [`rect`][1] element.
    [1]: https://www.w3.org/TR/SVG/shapes.html#RectElement"]
    struct Rectangle("rect")

    #[doc = "A [`script`][1] element.
    [1]: https://www.w3.org/TR/SVG/script.html#ScriptElement"]
    struct Script("script")

    #[doc = "A [`stop`][1] element.
    [1]: https://www.w3.org/TR/SVG/pservers.html#StopElement"]
    struct Stop("stop")

    #[doc = "A [`style`][1] element.
    [1]: https://www.w3.org/TR/SVG/styling.html#StyleElement"]
    struct Style("style")

    #[doc = "A [`symbol`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#SymbolElement"]
    struct Symbol("symbol")

    #[doc = "A [`text`][1] element.
    [1]: https://www.w3.org/TR/SVG/text.html#TextElement"]
    struct Text("text")

    #[doc = "A [`textPath`][1] element.
    [1]: https://www.w3.org/TR/SVG/text.html#TextPathElement"]
    struct TextPath("textPath")

    #[doc = "A [`title`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#TitleElement"]
    struct Title("title")

    #[doc = "A [`use`][1] element.
    [1]: https://www.w3.org/TR/SVG/struct.html#UseElement"]
    struct Use("use")
}

pub mod path;

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
