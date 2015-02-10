use std::collections::HashMap;

use {Error, Result};
use reader::Reader;

/// A tag.
pub enum Tag {
    Empty,
    Path(Attributes),
    Unknown(String, Attributes),
}

/// The attributes of a tag.
pub type Attributes = HashMap<String, String>;

struct Parser<'s> {
    reader: Reader<'s>,
}

impl Tag {
    /// Parse the content between a pair of angle brackets.
    pub fn parse(text: &str) -> Result<Tag> {
        Parser::new(text).process()
    }
}

macro_rules! raise(
    ($parser:expr, $($arg:tt)*) => ({
        let (line, column) = $parser.reader.position();
        return Err(Error {
            line: line,
            column: column,
            message: format!($($arg)*),
        })
    });
);

impl<'s> Parser<'s> {
    #[inline]
    fn new(text: &'s str) -> Parser<'s> {
        Parser {
            reader: Reader::new(text),
        }
    }

    fn process(&mut self) -> Result<Tag> {
        use std::ascii::OwnedAsciiExt;

        self.reader.consume_char('/');

        let name = try!(self.read_name());
        let attributes = try!(self.read_attributes());

        Ok(match &(name.clone().into_ascii_lowercase())[] {
            "path" => Tag::Path(attributes),
            _ => Tag::Unknown(name, attributes),
        })
    }

    #[inline]
    fn read_name(&mut self) -> Result<String> {
        let name = self.reader.capture(|reader| {
            reader.consume_name();
        }).and_then(|name| Some(String::from_str(name)));

        match name {
            Some(name) => Ok(name),
            None => raise!(self, "expected a name"),
        }
    }

    fn read_attributes(&mut self) -> Result<Attributes> {
        let mut attributes = HashMap::new();

        loop {
            match try!(self.read_attribute()) {
                Some((name, value)) => {
                    attributes.insert(name, value);
                },
                _ => break,
            }
        }

        Ok(attributes)
    }

    fn read_attribute(&mut self) -> Result<Option<(String, String)>> {
        self.reader.consume_whitespace();

        Ok(None)
    }
}
