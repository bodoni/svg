//! The parser.

use std::borrow::Cow;

use node::Attributes;
use node::element::tag::{Tag, Type};

mod error;
mod reader;

pub use self::error::Error;

#[doc(hidden)]
pub use self::reader::Reader;

/// A parser.
pub struct Parser<'l> {
    #[allow(dead_code)]
    content: Cow<'l, str>,
    reader: Reader<'l>,
}

/// An event.
pub enum Event<'l> {
    /// An error.
    Error(Error),
    /// A tag.
    Tag(&'l str, Type, Attributes),
    /// A text.
    Text(&'l str),
    /// A comment.
    Comment,
    /// A declaration.
    Declaration,
    /// An instruction.
    Instruction,
}

/// A result.
pub type Result<T> = ::std::result::Result<T, Error>;

impl<'l> Parser<'l> {
    /// Create a parser.
    #[inline]
    pub fn new<T>(content: T) -> Self where T: Into<Cow<'l, str>> {
        let content = content.into();
        let reader = unsafe { ::std::mem::transmute(Reader::new(&*content)) };
        Parser { content: content, reader: reader }
    }
}

macro_rules! raise(
    ($parser:expr, $($argument:tt)*) => (
        return Some(Event::Error(Error::new($parser.reader.position(), format!($($argument)*))));
    );
);

impl<'l> Iterator for Parser<'l> {
    type Item = Event<'l>;

    fn next(&mut self) -> Option<Self::Item> {
        let content = self.reader.capture(|reader| {
            reader.consume_until_char('<');
        });
        if let Some(content) = content {
            return Some(Event::Text(content));
        }
        if !self.reader.consume_char('<') {
            return None;
        }
        let content = self.reader.capture(|reader| {
            reader.consume_until_char('>');
        });
        if content.is_none() {
            return raise!(self, "found an empty tag");
        }
        if !self.reader.consume_char('>') {
            raise!(self, "missing a closing angle bracket");
        }
        let content = content.unwrap();
        Some(if content.starts_with("!--") {
            Event::Comment
        } else if content.starts_with("!") {
            Event::Declaration
        } else if content.starts_with("?") {
            Event::Instruction
        } else {
            match Tag::parse(&content) {
                Ok(Tag(name, kind, attributes)) => Event::Tag(name, kind, attributes),
                Err(error) => Event::Error(error),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use parser::{Event, Parser};

    #[test]
    fn next_tag() {
        macro_rules! test(
            ($content:expr, $value:expr) => ({
                let mut parser = Parser::new($content);
                match parser.next().unwrap() {
                    Event::Tag(value, _, _) => assert_eq!(value, $value),
                    _ => unreachable!(),
                }
            })
        );

        test!("<foo>", "foo");
        test!("<foo/>", "foo");
        test!("  <foo/>", "foo");
    }

    #[test]
    fn next_text() {
        macro_rules! test(
            ($content:expr, $value:expr) => ({
                let mut parser = Parser::new($content);
                match parser.next().unwrap() {
                    Event::Text(value) => assert_eq!(value, $value),
                    _ => unreachable!(),
                }
            })
        );
        test!("foo <bar>", "foo");
        test!("  foo<bar>", "foo");
        test!("foo> <bar>", "foo>");
    }
}
