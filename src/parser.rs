//! The parser.

use std::{error, fmt};

use reader::{Input, Reader};
use tag::Tag;

/// A parser.
pub struct Parser<'l> {
    reader: Reader<'l>,
}

/// An event.
#[derive(Clone, Debug)]
pub enum Event {
    /// An error.
    Error(Error),
    /// A comment.
    Comment,
    /// A declaration.
    Declaration,
    /// An instruction.
    Instruction,
    /// A tag.
    Tag(Tag),
}

/// An error.
#[derive(Clone, Debug)]
pub struct Error {
    /// The line number.
    pub line: usize,
    /// The column number.
    pub column: usize,
    /// The message.
    pub message: String,
}

/// A result.
pub type Result<T> = ::std::result::Result<T, Error>;

impl<'l> Parser<'l> {
    /// Create a parser.
    #[inline]
    pub fn new<T: Input<'l>>(input: T) -> Self {
        Parser { reader: Reader::new(input) }
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
        let input = self.reader.capture(|reader| {
            reader.consume_until_char('>');
        }).and_then(|input| Some(String::from(input)));
        if input.is_none() {
            return raise!(self, "found an empty tag");
        }
        if !self.reader.consume_char('>') {
            raise!(self, "missing a closing angle bracket");
        }
        let input = input.unwrap();
        Some(if input.starts_with("!--") {
            Event::Comment
        } else if input.starts_with("!") {
            Event::Declaration
        } else if input.starts_with("?") {
            Event::Instruction
        } else {
            match Tag::parse(input) {
                Ok(tag) => Event::Tag(tag),
                Err(error) => Event::Error(error),
            }
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.line > 0 && self.column > 0 {
            write!(formatter, "{} (line {}, column {})", self.message, self.line, self.column)
        } else if self.line > 0 {
            write!(formatter, "{} (line {})", self.message, self.line)
        } else {
            self.message.fmt(formatter)
        }
    }
}

impl error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, Parser};
    use tag::Tag;

    #[test]
    fn next() {
        macro_rules! test(
            ($input:expr, $name:expr) => ({
                let mut parser = Parser::new($input);
                match parser.next().unwrap() {
                    Event::Tag(Tag::Unknown(name, _, _)) => assert_eq!(&*name, $name),
                    _ => assert!(false),
                }
            })
        );

        test!("<foo>", "foo");
        test!("<foo/>", "foo");
        test!("  <foo/>", "foo");
        test!("foo <bar>", "bar");
        test!("foo> <bar>", "bar");
    }
}
