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
    /// Create an element.
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

    /// Return the name.
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Return the attributes.
    #[inline]
    pub fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Return the attributes as mutable.
    #[inline]
    pub fn get_attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    /// Return the children.
    #[inline]
    pub fn get_children(&self) -> &Children {
        &self.children
    }

    /// Return the children as mutable.
    #[inline]
    pub fn get_children_mut(&mut self) -> &mut Children {
        &mut self.children
    }
}

impl fmt::Display for Element {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "<{}", self.name)?;
        let mut attributes = self.attributes.iter().collect::<Vec<_>>();
        attributes.sort_by_key(|pair| pair.0.as_str());
        for (name, value) in attributes {
            write!(formatter, r#" {}="{}""#, name, escape(value))?;
        }
        if self.children.is_empty() {
            return write!(formatter, "/>");
        }
        write!(formatter, ">")?;
        let mut bare = false;
        for child in self.children.iter() {
            bare = child.is_bare() && !formatter.alternate();
            if !bare {
                writeln!(formatter)?;
            }
            write!(formatter, "{}", child)?;
        }
        if !bare {
            writeln!(formatter)?;
        }
        write!(formatter, "</{}>", self.name)
    }
}

impl Node for Element {
    #[inline]
    fn append<T>(&mut self, node: T)
    where
        T: Into<Box<dyn Node>>,
    {
        self.children.push(node.into());
    }

    #[inline]
    fn assign<T, U>(&mut self, name: T, value: U)
    where
        T: Into<String>,
        U: Into<Value>,
    {
        self.attributes.insert(name.into(), value.into());
    }

    #[inline]
    fn get_name(&self) -> &str {
        Self::get_name(self)
    }

    #[inline]
    fn get_attributes(&self) -> Option<&Attributes> {
        Self::get_attributes(self).into()
    }

    #[inline]
    fn get_attributes_mut(&mut self) -> Option<&mut Attributes> {
        Self::get_attributes_mut(self).into()
    }

    #[inline]
    fn get_children(&self) -> Option<&Children> {
        Self::get_children(self).into()
    }

    #[inline]
    fn get_children_mut(&mut self) -> Option<&mut Children> {
        Self::get_children_mut(self).into()
    }
}

macro_rules! implement_nested(
    ($struct_name:ident::$field_name:ident) => (
        implement_nested!($struct_name::$field_name []);
    );
    ($struct_name:ident::$field_name:ident [$($indicator_name:ident),*]) => (
        impl $struct_name {
            /// Append a node.
            pub fn add<T>(mut self, node: T) -> Self
            where
                T: Into<Box<dyn Node>>,
            {
                Node::append(&mut self, node);
                self
            }

            /// Assign an attribute.
            #[inline]
            pub fn set<T, U>(mut self, name: T, value: U) -> Self
            where
                T: Into<String>,
                U: Into<Value>,
            {
                Node::assign(&mut self, name, value);
                self
            }
        }

        impl Node for $struct_name {
            #[inline]
            fn append<T>(&mut self, node: T)
            where
                T: Into<Box<dyn Node>>,
            {
                self.$field_name.append(node);
            }

            #[inline]
            fn assign<T, U>(&mut self, name: T, value: U)
            where
                T: Into<String>,
                U: Into<Value>,
            {
                self.$field_name.assign(name, value);
            }

            #[inline]
            fn get_name(&self) -> &str {
                self.$field_name.get_name()
            }

            #[inline]
            fn get_attributes(&self) -> Option<&Attributes> {
                self.$field_name.get_attributes().into()
            }

            #[inline]
            fn get_attributes_mut(&mut self) -> Option<&mut Attributes> {
                self.$field_name.get_attributes_mut().into()
            }

            #[inline]
            fn get_children(&self) -> Option<&Children> {
                self.$field_name.get_children().into()
            }

            #[inline]
            fn get_children_mut(&mut self) -> Option<&mut Children> {
                self.$field_name.get_children_mut().into()
            }

            $(
                #[inline]
                fn $indicator_name(&self) -> bool {
                    true
                }
            )*
        }

        impl std::ops::Deref for $struct_name {
            type Target = Element;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field_name
            }
        }

        impl std::ops::DerefMut for $struct_name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field_name
            }
        }

        impl std::fmt::Display for $struct_name {
            #[inline]
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
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

        implement_nested! { $struct_name::inner }
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
    #[doc = "An [`a`](https://www.w3.org/TR/SVG/linking.html#AElement) element."]
    struct Anchor

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

    #[doc = "A [`feBlend`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feBlend) element."]
    struct FilterEffectBlend

    #[doc = "A [`feColorMatrix`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feColorMatrix) element."]
    struct FilterEffectColorMatrix

    #[doc = "A [`feComponentTransfer`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feComponentTransfer) element."]
    struct FilterEffectComponentTransfer

    #[doc = "A [`feComposite`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feComposite) element."]
    struct FilterEffectComposite

    #[doc = "A [`feConvolveMatrix`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feConvolveMatrix) element."]
    struct FilterEffectConvolveMatrix

    #[doc = "A [`feDiffuseLighting`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDiffuseLighting) element."]
    struct FilterEffectDiffuseLighting

    #[doc = "A [`feDisplacementMap`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDisplacementMap) element."]
    struct FilterEffectDisplacementMap

    #[doc = "A [`feDistantLight`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDistantLight) element."]
    struct FilterEffectDistantLight

    #[doc = "A [`feDropShadow`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDropShadow) element."]
    struct FilterEffectDropShadow

    #[doc = "A [`feFlood`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFlood) element."]
    struct FilterEffectFlood

    #[doc = "A [`feFuncA`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncA) element."]
    struct FilterEffectFunctionA

    #[doc = "A [`feFuncB`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncB) element."]
    struct FilterEffectFunctionB

    #[doc = "A [`feFuncG`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncG) element."]
    struct FilterEffectFunctionG

    #[doc = "A [`feFuncR`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncR) element."]
    struct FilterEffectFunctionR

    #[doc = "A [`feGaussianBlur`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feGaussianBlur) element."]
    struct FilterEffectGaussianBlur

    #[doc = "A [`feImage`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feImage) element."]
    struct FilterEffectImage

    #[doc = "A [`feMerge`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMerge) element."]
    struct FilterEffectMerge

    #[doc = "A [`feMergeNode`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMergeNode) element."]
    struct FilterEffectMergeNode

    #[doc = "A [`feMorphology`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMorphology) element."]
    struct FilterEffectMorphology

    #[doc = "A [`feOffset`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feOffset) element."]
    struct FilterEffectOffset

    #[doc = "A [`fePointLight`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/fePointLight) element."]
    struct FilterEffectPointLight

    #[doc = "A [`feSpecularLighting`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feSpecularLighting) element."]
    struct FilterEffectSpecularLighting

    #[doc = "A [`feSpotLight`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feSpotLight) element."]
    struct FilterEffectSpotLight

    #[doc = "A [`feTile`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feTile) element."]
    struct FilterEffectTile

    #[doc = "A [`feTurbulence`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feTurbulence) element."]
    struct FilterEffectTurbulence

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

    #[doc = "A [`link`](https://www.w3.org/TR/SVG/styling.html#LinkElement) element."]
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

    #[doc = "A [`use`](https://www.w3.org/TR/SVG/struct.html#UseElement) element."]
    struct Use
}

macro_rules! implement {
    (@itemize $i:item) => ($i);
    ($(
        #[$doc:meta]
        struct $struct_name:ident
        [$($indicator_name:ident),*]
        [$($trait_name:ident: $($trait_type:tt)*),*]
        [$inner:ident $(,$argument_name:ident: $argument_type:ty)*] $body:block
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
                pub fn new<$($trait_name: $($trait_type)*),*>($($argument_name: $argument_type),*) -> Self {
                    #[inline(always)]
                    fn initialize<$($trait_name: $($trait_type)*),*>(
                        $inner: &mut Element $(, $argument_name: $argument_type)*
                    ) $body
                    let mut inner = Element::new(tag::$struct_name);
                    initialize(&mut inner $(, $argument_name)*);
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

        implement_nested! { $struct_name::inner [$($indicator_name),*] }
    )*);
}

implement! {
    #[doc = "An [`svg`](https://www.w3.org/TR/SVG/struct.html#SVGElement) element."]
    struct SVG [is_bareable] [] [inner] {
        inner.assign("xmlns", "http://www.w3.org/2000/svg");
    }

    #[doc = "A [`script`](https://www.w3.org/TR/SVG/script.html#ScriptElement) element."]
    struct Script [is_bareable] [T: Into<String>] [inner, content: T] {
        inner.append(crate::node::Text::new(content));
    }

    #[doc = "A [`style`](https://www.w3.org/TR/SVG/styling.html#StyleElement) element."]
    struct Style [is_bareable] [T: Into<String>] [inner, content: T] {
        inner.append(crate::node::Text::new(content));
    }

    #[doc = "A [`text`](https://www.w3.org/TR/SVG/text.html#TextElement) element."]
    struct Text [is_bareable] [T: Into<String>] [inner, content: T] {
        inner.append(crate::node::Text::new(content));
    }

    #[doc = "A [`textPath`](https://www.w3.org/TR/SVG/text.html#TextPathElement) element."]
    struct TextPath [] [T: Into<String>] [inner, content: T] {
        inner.append(crate::node::Text::new(content));
    }

    #[doc = "A [`title`](https://www.w3.org/TR/SVG/struct.html#TitleElement) element."]
    struct Title [] [T: Into<String>] [inner, content: T] {
        inner.append(crate::node::Text::new(content));
    }

    #[doc = "A [`tspan`](https://www.w3.org/TR/SVG/text.html#TextElement) element."]
    struct TSpan [] [T: Into<String>] [inner, content: T] {
        inner.append(crate::node::Text::new(content));
    }
}

fn escape(value: &str) -> String {
    crate::node::text::escape(value)
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::{Element, Rectangle, Style, Title};
    use crate::node::element;

    #[test]
    fn element_children() {
        let mut one = element::Group::new()
            .add(element::Text::new("foo"))
            .add(element::Text::new("bar"))
            .add(element::Text::new("buz"));
        let two = element::Group::new()
            .add(one.get_children()[0].clone())
            .add(one.get_children_mut().pop().unwrap());

        assert_eq!(
            one.to_string(),
            "<g>\n<text>\nfoo\n</text>\n<text>\nbar\n</text>\n</g>",
        );
        assert_eq!(
            two.to_string(),
            "<g>\n<text>\nfoo\n</text>\n<text>\nbuz\n</text>\n</g>",
        );
    }

    #[test]
    fn element_display() {
        use crate::node::Node;

        let mut element = Element::new("foo");
        element.assign("x", -10);
        element.assign("y", "10px");
        element.assign("s", (12.5, 13.0));
        element.assign("c", "green");
        element.append(Element::new("bar"));

        assert_eq!(
            element.to_string().lines().collect::<Vec<_>>(),
            &[
                r#"<foo c="green" s="12.5 13" x="-10" y="10px">"#,
                "<bar/>",
                "</foo>",
            ],
        );
    }

    #[test]
    fn element_display_angles() {
        let element = Rectangle::new()
            .set("fill", "#FF780088")
            .set("height", 10)
            .set("width", 0.3088995)
            .set("x", 328.0725)
            .set("y", 120)
            .add(Title::new("widgets >=3.0.9, <3.1.dev0"));

        assert_eq!(
            element.to_string().lines().collect::<Vec<_>>(),
            &[
                r###"<rect fill="#FF780088" height="10" width="0.3088995" x="328.0725" y="120">"###,
                "<title>widgets &gt;=3.0.9, &lt;3.1.dev0</title>",
                "</rect>",
            ],
        );
    }

    #[test]
    fn element_display_quotes() {
        use crate::node::Node;

        let mut element = Element::new("foo");
        element.assign("s", "'single'");
        element.assign("d", r#""double""#);
        element.assign("m", r#""mixed'"#);

        assert_eq!(
            element.to_string(),
            r#"<foo d="&quot;double&quot;" m="&quot;mixed&apos;" s="&apos;single&apos;"/>"#,
        );
    }

    #[test]
    fn style_display() {
        let element = Style::new("* { font-family: foo; }");

        assert_eq!(
            element.to_string().lines().collect::<Vec<_>>(),
            &["<style>", "* { font-family: foo; }", "</style>"],
        );
    }
}
