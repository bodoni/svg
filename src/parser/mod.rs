//! The parser.

use std::sync::Arc;

use crate::node::element::tag::{Tag, Type};
use crate::node::Attributes;

mod error;
mod reader;

pub use self::error::Error;

#[doc(hidden)]
pub use self::reader::Reader;

/// A parser.
pub struct Parser<'l> {
    #[allow(dead_code)]
    content: String,
    reader: Reader<'l>,
}

/// An event.
pub enum Event<'l> {
    /// An error.
    Error(Error),
    /// A tag.
    Tag(Arc<&'l str>, Type, Attributes),
    /// A text.
    Text(Arc<&'l str>),
    /// A comment.
    Comment,
    /// A declaration.
    Declaration,
    /// An instruction.
    Instruction,
}

/// A result.
pub type Result<T> = ::std::result::Result<T, Error>;

macro_rules! raise(
    ($parser:expr, $($argument:tt)*) => (
        return Some(Event::Error(Error::new($parser.reader.position(), format!($($argument)*))));
    );
);

impl<'l> Parser<'l> {
    /// Create a parser.
    #[inline]
    pub fn new(content: String) -> Self
    {
        let reader = Reader::new(Arc::new(&content));
        Parser { content, reader }
    }

    fn next_angle(&'l mut self) -> Option<Event<'l>> {
        let content: String = self.reader.peek_many().take(4).collect();
        if content.is_empty() {
            return None;
        } else if content.starts_with("<!--") {
            self.read_comment()
        } else if content.starts_with("<!") {
            self.read_declaration()
        } else if content.starts_with("<?") {
            self.read_instruction()
        } else if content.starts_with("<") {
            self.read_tag()
        } else {
            raise!(self, "found an unknown sequence");
        }
    }

    fn next_text(&'l mut self) -> Option<Event<'l>> {
        self.reader
            .capture(|reader| reader.consume_until_char('<'))
            .map(|content| Event::Text(Arc::new(content)))
    }

    fn read_comment(&'l mut self) -> Option<Event<'l>> {
        if !self.reader.consume_comment() {
            raise!(self, "found a malformed comment");
        }
        Some(Event::Comment)
    }

    fn read_declaration(&mut self) -> Option<Event<'l>> {
        if !self.reader.consume_declaration() {
            raise!(self, "found a malformed declaration");
        }
        Some(Event::Declaration)
    }

    fn read_instruction(&mut self) -> Option<Event<'l>> {
        if !self.reader.consume_instruction() {
            raise!(self, "found a malformed instruction");
        }
        Some(Event::Instruction)
    }

    fn read_tag(&'l mut self) -> Option<Event<'l>> {
        match self.reader.capture(|reader| reader.consume_tag()) {
            None => raise!(self, "found a malformed tag"),
            Some(content) => Some(match Tag::parse(Arc::new(&content[1..content.len() - 1])) {
                Ok(Tag(name, kind, attributes)) => Event::Tag(name, kind, attributes),
                Err(error) => Event::Error(error),
            }),
        }
    }
}

impl<'l> Iterator for Parser<'_> {
    type Item = Event<'l>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_text().or_else(|| self.next_angle())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Event, Parser};

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

        test!("<foo>".to_string(), "foo");
        test!("<foo/>".to_string(), "foo");
        test!("  <foo/>".to_string(), "foo");
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

        test!("foo <bar>".to_string(), "foo");
        test!("  foo<bar>".to_string(), "foo");
        test!("foo> <bar>".to_string(), "foo>");
    }
}
