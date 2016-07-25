//! The parser.

use std::fmt;

use reader::Reader;
use tag::Tag;

/// A parser.
pub struct Parser<'l> {
    reader: Reader<'l>,
}

/// A parsing event.
pub enum Event {
    Error(Error),
    Comment,
    Declaration,
    Instruction,
    Tag(Tag),
}

/// A parsing error.
pub struct Error {
    /// The line number.
    pub line: usize,
    /// The column number.
    pub column: usize,
    /// The description.
    pub message: String,
}

/// A parsing result.
pub type Result<T> = ::std::result::Result<T, Error>;

impl<'l> Parser<'l> {
    /// Create a parser.
    #[inline]
    pub fn new(text: &'l str) -> Parser<'l> {
        Parser { reader: Reader::new(text) }
    }
}

macro_rules! raise(
    ($parser:expr, $($arg:tt)*) => ({
        let (line, column) = $parser.reader.position();
        return Some(Event::Error(Error { line: line, column: column, message: format!($($arg)*) }))
    });
);

impl<'l> Iterator for Parser<'l> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        self.reader.consume_until_char('<');

        if !self.reader.consume_char('<') {
            return None;
        }

        let content = self.reader.capture(|reader| {
            reader.consume_until_char('>');
        }).and_then(|content| Some(String::from(content)));

        if content.is_none() {
            return raise!(self, "found an empty tag");
        }

        if !self.reader.consume_char('>') {
            raise!(self, "missing a closing angle bracket");
        }

        let content = &(content.unwrap());

        Some(if content.starts_with("!--") {
            Event::Comment
        } else if content.starts_with("!") {
            Event::Declaration
        } else if content.starts_with("?") {
            Event::Instruction
        } else {
            match Tag::parse(content) {
                Ok(tag) => Event::Tag(tag),
                Err(error) => Event::Error(error),
            }
        })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.line > 0 && self.column > 0 {
            write!(formatter, "{} (line {}, column {})", self.message, self.line, self.column)
        } else if self.line > 0 {
            write!(formatter, "{} (line {})", self.message, self.line)
        } else {
            fmt::Debug::fmt(&self.message, formatter)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, Parser};
    use tag::Tag;

    #[test]
    fn next() {
        macro_rules! test(
            ($text:expr, $name:expr) => ({
                let mut parser = Parser::new($text);
                match parser.next().unwrap() {
                    Event::Tag(Tag::Unknown(name, _, _)) => assert_eq!(&*name, $name),
                    _ => assert!(false),
                }
            })
        );

        test!("<foo>", "foo");
        test!("<foo/>", "foo");
        test!("  <foo/>", "foo");

        // TODO:
        test!("foo <bar>", "bar");
        test!("foo> <bar>", "bar");
    }
}
