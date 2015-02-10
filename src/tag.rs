//! A tag.

use std::collections::HashMap;

use {Error, Result};
use reader::Reader;

/// A tag.
pub enum Tag {
    Path(Type, Attributes),
    Unknown(String, Type, Attributes),
}

/// The type of a tag.
///
/// http://www.w3.org/TR/REC-xml/#sec-starttags
pub enum Type {
    Start,
    End,
    EmptyElement,
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
        if self.reader.consume_char('/') {
            self.read_end_tag()
        } else {
            self.read_start_or_empty_element_tag()
        }
    }

    fn read_attribute(&mut self) -> Result<Option<(String, String)>> {
        let attribute = self.reader.capture(|reader| {
            reader.consume_attribute();
        }).and_then(|attribute| Some(String::from_str(attribute)));

        match attribute {
            Some(attribute) => {
                let k = (&attribute).find('=').unwrap();
                let name = (&attribute[0..k]).trim_right();
                let value = (&attribute[(k+1)..]).trim_left();
                let value = &value[1..(value.len()-1)];
                Ok(Some((String::from_str(name), String::from_str(value))))
            },
            _ => Ok(None),
        }
    }

    fn read_attributes(&mut self) -> Result<Attributes> {
        let mut attributes = HashMap::new();

        loop {
            self.reader.consume_whitespace();

            match try!(self.read_attribute()) {
                Some((name, value)) => {
                    attributes.insert(name, value);
                },
                _ => break,
            }
        }

        Ok(attributes)
    }

    fn read_end_tag(&mut self) -> Result<Tag> {
        use std::ascii::OwnedAsciiExt;

        let name = try!(self.read_name());

        self.reader.consume_whitespace();

        if !self.reader.is_done() {
            raise!(self, "found an end tag with excessive data");
        }

        Ok(match &(name.clone().into_ascii_lowercase())[] {
            "path" => Tag::Path(Type::End, HashMap::new()),
            _ => Tag::Unknown(name, Type::End, HashMap::new()),
        })
    }

    fn read_name(&mut self) -> Result<String> {
        let name = self.reader.capture(|reader| {
            reader.consume_name();
        }).and_then(|name| Some(String::from_str(name)));

        match name {
            Some(name) => Ok(name),
            None => raise!(self, "expected a name"),
        }
    }

    fn read_start_or_empty_element_tag(&mut self) -> Result<Tag> {
        use std::ascii::OwnedAsciiExt;

        let name = try!(self.read_name());
        let attributes = try!(self.read_attributes());

        self.reader.consume_whitespace();

        let tail = self.reader.capture(|reader| {
            reader.consume_all();
        }).and_then(|tail| Some(String::from_str(tail)));

        let typo = match tail {
            Some(tail) => match &tail[] {
                "/" => Type::EmptyElement,
                _ => raise!(self, "found an unexpected ending of a tag"),
            },
            _ => Type::Start,
        };

        Ok(match &(name.clone().into_ascii_lowercase())[] {
            "path" => Tag::Path(typo, attributes),
            _ => Tag::Unknown(name, typo, attributes),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Parser, Tag, Type};

    #[test]
    fn parser_process() {
        macro_rules! test(
            ($text:expr, $typo:ident) => ({
                let mut parser = Parser::new($text);
                match parser.process().unwrap() {
                    Tag::Unknown(_, Type::$typo, _) => {},
                    _ => assert!(false),
                }
            });
        );

        test!("foo", Start);
        test!("foo ", Start);
        test!("/foo", End);
        test!("/foo ", End);
        test!("foo/", EmptyElement);
        test!("foo /", EmptyElement);
    }

    #[test]
    fn parser_read_attribute() {
        macro_rules! test(
            ($text:expr, $name:expr, $value:expr) => ({
                let mut parser = Parser::new($text);
                let (name, value) = parser.read_attribute().unwrap().unwrap();
                assert_eq!(&name[], $name);
                assert_eq!(&value[], $value);
            });
        );

        test!("foo='bar'", "foo", "bar");
        test!("foo =\"bar\"", "foo", "bar");
        test!("foo= \"bar\"", "foo", "bar");
        test!("foo\t=\n'bar'  ", "foo", "bar");
    }
}
