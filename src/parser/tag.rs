//! Tags.

use node::Attributes;
use parser::{Error, Reader, Result};

/// A tag.
pub enum Tag {
    /// A path tag.
    Path(Type, Attributes),
    /// An unknown tag.
    Unknown(Type, String, Attributes),
}

/// A [type][1] of a tag.
///
/// [1]: http://www.w3.org/TR/REC-xml/#sec-starttags
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    /// A start tag.
    Start,
    /// An end tag.
    End,
    /// An empty tag.
    Empty,
}

struct Parser<'l> {
    reader: Reader<'l>,
}

impl Tag {
    /// Parse a tag.
    #[inline]
    pub fn parse(content: &str) -> Result<Self> {
        Parser::new(content).process()
    }
}

macro_rules! raise(
    ($parser:expr, $($argument:tt)*) => (
        return Err(Error::new($parser.reader.position(), format!($($argument)*)));
    );
);

impl<'l> Parser<'l> {
    #[inline]
    fn new(content: &'l str) -> Self {
        Parser { reader: Reader::new(content) }
    }

    fn process(&mut self) -> Result<Tag> {
        if self.reader.consume_char('/') {
            self.read_end_tag()
        } else {
            self.read_start_or_empty_tag()
        }
    }

    fn read_attribute(&mut self) -> Result<Option<(String, String)>> {
        let attribute = self.reader.capture(|reader| {
            reader.consume_attribute();
        }).and_then(|attribute| Some(String::from(attribute)));
        match attribute {
            Some(attribute) => {
                let k = (&attribute).find('=').unwrap();
                let name = (&attribute[0..k]).trim_right();
                let value = (&attribute[(k+1)..]).trim_left();
                let value = &value[1..(value.len()-1)];
                Ok(Some((String::from(name), String::from(value))))
            },
            _ => Ok(None),
        }
    }

    fn read_attributes(&mut self) -> Result<Attributes> {
        let mut attributes = Attributes::new();
        loop {
            self.reader.consume_whitespace();
            match try!(self.read_attribute()) {
                Some((name, value)) => {
                    attributes.insert(name, value.into());
                },
                _ => break,
            }
        }
        Ok(attributes)
    }

    fn read_end_tag(&mut self) -> Result<Tag> {
        let name = try!(self.read_name());
        self.reader.consume_whitespace();
        if !self.reader.is_done() {
            raise!(self, "found an end tag with excessive data");
        }
        Ok(match name.as_str() {
            "path" => Tag::Path(Type::End, Default::default()),
            _ => Tag::Unknown(Type::End, name, Default::default()),
        })
    }

    fn read_name(&mut self) -> Result<String> {
        let name = self.reader.capture(|reader| {
            reader.consume_name();
        }).and_then(|name| Some(String::from(name)));
        match name {
            Some(name) => Ok(name),
            _ => raise!(self, "expected a name"),
        }
    }

    fn read_start_or_empty_tag(&mut self) -> Result<Tag> {
        let name = try!(self.read_name());
        let attributes = try!(self.read_attributes());
        self.reader.consume_whitespace();
        let tail = self.reader.capture(|reader| {
            reader.consume_all();
        }).and_then(|tail| Some(String::from(tail)));
        let kind = match tail {
            Some(tail) => match &*tail {
                "/" => Type::Empty,
                _ => raise!(self, "found an unexpected ending of a tag"),
            },
            _ => Type::Start,
        };
        Ok(match name.as_str() {
            "path" => Tag::Path(kind, attributes),
            _ => Tag::Unknown(kind, name, attributes),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Parser, Tag, Type};

    #[test]
    fn parser_process() {
        macro_rules! test(
            ($content:expr, $kind:ident) => ({
                let mut parser = Parser::new($content);
                match parser.process().unwrap() {
                    Tag::Unknown(Type::$kind, _, _) => {},
                    _ => unreachable!(),
                }
            });
        );

        test!("foo", Start);
        test!("foo ", Start);
        test!("/foo", End);
        test!("/foo ", End);
        test!("foo/", Empty);
        test!("foo /", Empty);
    }

    #[test]
    fn parser_read_attribute() {
        macro_rules! test(
            ($content:expr, $name:expr, $value:expr) => ({
                let mut parser = Parser::new($content);
                let (name, value) = parser.read_attribute().unwrap().unwrap();
                assert_eq!(&*name, $name);
                assert_eq!(&*value, $value);
            });
        );

        test!("foo='bar'", "foo", "bar");
        test!("foo =\"bar\"", "foo", "bar");
        test!("foo= \"bar\"", "foo", "bar");
        test!("foo\t=\n'bar'  ", "foo", "bar");
    }
}
