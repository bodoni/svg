//! An SVG composer and parser.

use std::borrow::Cow;
use std::path::Path;

#[macro_use]
mod macros;

mod node;
mod reader;

pub mod element;
pub mod parser;
pub mod tag;

pub use node::Node;
pub use parser::Parser;
pub use tag::Tag;

/// A content.
pub trait Content<'l>: Into<Cow<'l, str>> { }

/// A document.
pub type Document = element::SVG;

impl<'l, T> Content<'l> for T where T: Into<Cow<'l, str>> { }

/// Parse a file.
pub fn parse<'l, T: AsRef<Path>>(path: T) -> std::io::Result<Parser<'l>> {
    use std::fs;
    use std::io::Read;

    let mut content = String::new();
    let mut file = try!(fs::File::open(path));
    try!(file.read_to_string(&mut content));
    Ok(Parser::new(content))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    pub fn fixture(name: &str) -> PathBuf {
        let mut path = PathBuf::from("tests").join("fixtures").join(name);
        path.set_extension("svg");
        assert!(path.exists());
        path
    }

    #[test]
    fn parse() {
        use parser::Event;
        use tag::Tag;

        let mut parser = ::parse(fixture("benton")).unwrap();

        macro_rules! test(
            ($parser:expr, $matcher:pat) => (
                match $parser.next().unwrap() {
                    $matcher => {},
                    _ => assert!(false),
                }
            );
        );

        test!(parser, Event::Instruction);
        test!(parser, Event::Comment);
        test!(parser, Event::Declaration);

        test!(parser, Event::Tag(Tag::Unknown(..)));
        test!(parser, Event::Tag(Tag::Path(..)));
        test!(parser, Event::Tag(Tag::Path(..)));
        test!(parser, Event::Tag(Tag::Path(..)));
        test!(parser, Event::Tag(Tag::Path(..)));
        test!(parser, Event::Tag(Tag::Unknown(..)));

        assert!(parser.next().is_none());
    }
}
