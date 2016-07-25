//! An SVG composer and parser.

use std::path::Path;

#[macro_use]
mod macros;

mod reader;
mod writer;

pub mod composer;
pub mod element;
pub mod node;
pub mod parser;
pub mod tag;

pub use composer::Composer;
pub use node::Node;
pub use parser::Parser;
pub use reader::Input;
pub use tag::Tag;
pub use writer::Output;

/// A number.
pub type Number = f32;

/// Parse a file.
pub fn parse<'l, T: AsRef<Path>>(path: T) -> std::io::Result<Parser<'l>> {
    use std::fs;
    use std::io::Read;

    let mut input = String::new();
    let mut file = try!(fs::File::open(path));
    try!(file.read_to_string(&mut input));
    Ok(Parser::new(input))
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
