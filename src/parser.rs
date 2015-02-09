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

impl<'s> Iterator for Parser<'s> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        self.reader.consume_until_any("<");

        let content = String::from_str(self.reader.capture(|reader| {
            reader.consume_until_any(">");
        }));

        if content.is_empty() {
            return None
        }

        Some(match Tag::parse(&content[1..]) {
            Ok(tag) => Event::Tag(tag),
            Err(error) => Event::Error(error),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use {Event, Tag};

    #[test]
    fn next() {
        macro_rules! test(
            ($text:expr, $name:expr) => ({
                let mut parser = Parser::new($text);
                match parser.next().unwrap() {
                    Event::Tag(Tag::Unknown(name, _)) => assert_eq!(&name[], $name),
                    _ => assert!(false),
                }
            })
        );

        test!("<foo  >", "foo");

        // TODO:
        test!("  <bar/>", "bar/");
        test!("foo <!DOCTYPE>", "!DOCTYPE");
        test!("<<baz>", "<baz");
        test!("> <qux>", "qux");
    }
}
