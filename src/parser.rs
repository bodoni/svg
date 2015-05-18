use Error;
use reader::Reader;
use tag::Tag;

/// A parser.
pub struct Parser<'s> {
    reader: Reader<'s>,
}

/// An event of a parser.
pub enum Event {
    Error(Error),
    Comment,
    Declaration,
    Instruction,
    Tag(Tag),
}

impl<'s> Parser<'s> {
    /// Create a new parser.
    pub fn new(text: &'s str) -> Parser<'s> {
        Parser {
            reader: Reader::new(text),
        }
    }
}

macro_rules! raise(
    ($parser:expr, $($arg:tt)*) => ({
        let (line, column) = $parser.reader.position();
        return Some(Event::Error(Error {
            line: line,
            column: column,
            message: format!($($arg)*),
        }))
    });
);

impl<'s> Iterator for Parser<'s> {
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
