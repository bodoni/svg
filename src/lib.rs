//! An SVG composer and parser.

use std::path::Path;

mod reader;

pub mod element;
pub mod parser;
pub mod tag;

pub use parser::Parser;
pub use reader::Content;
pub use tag::Tag;

/// A number.
pub type Number = f32;

/// Open a file.
pub fn open<'l, T: AsRef<Path>>(path: T) -> std::io::Result<Parser<'l>> {
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
    fn open() {
        use parser::Event;
        use tag::Tag;

        let mut parser = ::open(fixture("benton")).unwrap();

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
