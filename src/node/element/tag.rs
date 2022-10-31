//! The tags.

#![allow(non_upper_case_globals)]

use crate::node::Attributes;
use crate::parser::{Error, Reader, Result};

/// A tag.
#[derive(Clone, Debug)]
pub struct Tag<'l>(pub &'l str, pub Type, pub Attributes);

/// A [type][1] of a tag.
///
/// [1]: http://www.w3.org/TR/REC-xml/#sec-starttags
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    /// A start tag.
    Start,
    /// An end tag.
    End,
    /// An empty tag.
    Empty,
}

struct Parser<'l> {
    reader: Reader<'l>,
}

impl<'l> Tag<'l> {
    /// Parse a tag.
    #[inline]
    pub fn parse(content: &'l str) -> Result<Tag<'l>> {
        Parser::new(content).process()
    }
}

macro_rules! raise(
    ($parser:expr, $($argument:tt)*) => (
        return Err(Error::new($parser.reader.position(), format!($($argument)*)))
    );
);

impl<'l> Parser<'l> {
    #[inline]
    fn new(content: &'l str) -> Self {
        Parser {
            reader: Reader::new(content),
        }
    }

    fn process(&mut self) -> Result<Tag<'l>> {
        if self.reader.consume_char('/') {
            self.read_end_tag()
        } else {
            self.read_start_or_empty_tag()
        }
    }

    fn read_attribute(&mut self) -> Result<Option<(String, String)>> {
        let attribute = self
            .reader
            .capture(|reader| reader.consume_attribute())
            .and_then(|attribute| Some(String::from(attribute)));
        match attribute {
            Some(attribute) => {
                let k = (&attribute).find('=').unwrap();
                let name = (&attribute[0..k]).trim_end();
                let value = (&attribute[(k + 1)..]).trim_start();
                let value = &value[1..(value.len() - 1)];
                Ok(Some((String::from(name), String::from(value))))
            }
            _ => Ok(None),
        }
    }

    fn read_attributes(&mut self) -> Result<Attributes> {
        let mut attributes = Attributes::new();
        loop {
            self.reader.consume_whitespace();
            match self.read_attribute()? {
                Some((name, value)) => {
                    attributes.insert(name, value.into());
                }
                _ => break,
            }
        }
        Ok(attributes)
    }

    fn read_end_tag(&mut self) -> Result<Tag<'l>> {
        let name = self.read_name()?;
        self.reader.consume_whitespace();
        if !self.reader.is_done() {
            raise!(self, "found an end tag with excessive data");
        }
        Ok(Tag(name, Type::End, Attributes::default()))
    }

    fn read_name(&mut self) -> Result<&'l str> {
        let name = self.reader.capture(|reader| reader.consume_name());
        match name {
            Some(name) => Ok(name),
            _ => raise!(self, "expected a name"),
        }
    }

    fn read_start_or_empty_tag(&mut self) -> Result<Tag<'l>> {
        let name = self.read_name()?;
        let attributes = self.read_attributes()?;
        self.reader.consume_whitespace();
        let tail = self.reader.capture(|reader| reader.consume_all());
        match tail {
            Some("/") => Ok(Tag(name, Type::Empty, attributes)),
            Some(_) => raise!(self, "found an unexpected ending of a tag"),
            _ => Ok(Tag(name, Type::Start, attributes)),
        }
    }
}

macro_rules! implement {
    ($($const_name:ident: $tag_name:expr,)*) => ($(
        #[doc = $tag_name]
        pub const $const_name: &'static str = $tag_name;
    )*);
}

implement! {
    Animate: "animate",
    AnimateColor: "animateColor",
    AnimateMotion: "animateMotion",
    AnimateTransform: "animateTransform",
    Circle: "circle",
    ClipPath: "clipPath",
    Definitions: "defs",
    Description: "desc",
    Ellipse: "ellipse",
    Filter: "filter",
    ForeignObject: "foreignObject",
    Group: "g",
    Image: "image",
    Line: "line",
    LinearGradient: "linearGradient",
    Link: "a",
    Marker: "marker",
    Mask: "mask",
    MotionPath: "mpath",
    Path: "path",
    Pattern: "pattern",
    Polygon: "polygon",
    Polyline: "polyline",
    RadialGradient: "radialGradient",
    Rectangle: "rect",
    Script: "script",
    Stop: "stop",
    Style: "style",
    SVG: "svg",
    Symbol: "symbol",
    Text: "text",
    TextPath: "textPath",
    Title: "title",
    TSpan: "tspan",
    Use: "use",
}

#[cfg(test)]
mod tests {
    use super::{Parser, Tag, Type};

    #[test]
    fn parser_process() {
        macro_rules! test(
            ($content:expr, $kind:ident) => ({
                let mut parser = Parser::new($content);
                match parser.process().unwrap() {
                    Tag("foo", Type::$kind, _) => {}
                    _ => unreachable!(),
                }
            });
        );

        test!("foo", Start);
        test!("foo ", Start);
        test!("/foo", End);
        test!("/foo ", End);
        test!("foo/", Empty);
        test!("foo /", Empty);
    }

    #[test]
    fn parser_read_attribute() {
        macro_rules! test(
            ($content:expr, $name:expr, $value:expr) => ({
                let mut parser = Parser::new($content);
                let (name, value) = parser.read_attribute().unwrap().unwrap();
                assert_eq!(&*name, $name);
                assert_eq!(&*value, $value);
            });
        );

        test!("foo=''", "foo", "");
        test!("foo='bar'", "foo", "bar");
        test!("foo =\"bar\"", "foo", "bar");
        test!("foo= \"bar\"", "foo", "bar");
        test!("foo\t=\n'bar'  ", "foo", "bar");
        test!("標籤='數值'", "標籤", "數值");
    }
}
