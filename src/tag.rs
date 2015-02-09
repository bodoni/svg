use std::collections::HashMap;

use reader::Reader;
use Result;

/// A tag.
pub enum Tag {
    Unknown(String, Attributes),
    Path(Attributes),
}

/// The attibutes of a tag.
pub type Attributes = HashMap<String, String>;

struct Parser<'s> {
    reader: Reader<'s>,
}

impl Tag {
    pub fn parse(text: &str) -> Result<Tag> {
        Parser::new(text).process()
    }
}

impl<'s> Parser<'s> {
    #[inline]
    fn new(text: &'s str) -> Parser<'s> {
        Parser {
            reader: Reader::new(text),
        }
    }

    fn process(&mut self) -> Result<Tag> {
        use std::ascii::OwnedAsciiExt;

        self.reader.consume_whitespace();

        let name = String::from_str(self.reader.capture(|reader| {
            reader.consume_blackspace();
        })).into_ascii_lowercase();

        let attributes = HashMap::new();

        Ok(match &name[] {
            "path" => Tag::Path(attributes),
            _ => Tag::Unknown(name, attributes),
        })
    }
}
