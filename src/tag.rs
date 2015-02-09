use std::collections::HashMap;

use reader::Reader;
use Result;

/// A tag.
pub enum Tag {
    Unknown(String, Attributes),
    Path(Attributes),
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

impl<'s> Parser<'s> {
    #[inline]
    fn new(text: &'s str) -> Parser<'s> {
        Parser {
            reader: Reader::new(text),
        }
    }

    fn process(&mut self) -> Result<Tag> {
        use std::ascii::OwnedAsciiExt;

        let name = try!(self.read_name());
        let attributes = try!(self.read_attributes());

        Ok(match &(name.clone().into_ascii_lowercase())[] {
            "path" => Tag::Path(attributes),
            _ => Tag::Unknown(name, attributes),
        })
    }

    #[inline]
    fn read_name(&mut self) -> Result<String> {
        Ok(String::from_str(self.reader.capture(|reader| {
            reader.consume_blackspace();
        })))
    }

    fn read_attributes(&mut self) -> Result<Attributes> {
        let attributes = HashMap::new();

        Ok(attributes)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn parser_read_name_success() {
        macro_rules! test(
            ($text:expr, $name:expr) => ({
                let mut parser = Parser::new($text);
                match parser.read_name() {
                    Ok(name) => assert_eq!(&name[], $name),
                    _ => assert!(false),
                }
            })
        );

        test!("foo  ", "foo");

        // TODO:
        test!("bar/", "bar/");
        test!("!-- bar", "!--");
        test!("!DOCTYPE", "!DOCTYPE");
        test!("<baz", "<baz");
    }

    #[test]
    fn parser_read_name_failure() {
        macro_rules! test(
            ($text:expr) => ({
                let mut parser = Parser::new($text);
                match parser.read_name() {
                    Ok(name) => assert!(name.is_empty()),
                    _ => assert!(false),
                }
            })
        );

        // http://www.w3.org/TR/REC-xml/#sec-starttags
        test!(" foo");
        test!("\tbar");
        test!("\nbaz");
    }
}
