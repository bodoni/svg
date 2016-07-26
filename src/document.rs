use std::borrow::Cow;
use std::path::Path;

use element::SVG;
use reactor::Reactor;
use result::Read as Result;

/// A document.
pub struct Document;

impl Document {
    /// Create a document.
    #[inline]
    pub fn new() -> SVG {
        Default::default()
    }

    /// Open a document.
    pub fn open<'l, T: AsRef<Path>>(path: T) -> Result<Reactor<'l>> {
        use std::fs::File;
        use std::io::Read;

        let mut content = String::new();
        let mut file = try!(File::open(path));
        try!(file.read_to_string(&mut content));
        Ok(Document::read(content))
    }

    /// Read a document.
    #[inline]
    pub fn read<'l, T: Into<Cow<'l, str>>>(content: T) -> Reactor<'l> {
        Reactor::new(content)
    }
}

#[cfg(test)]
mod tests {
    use document::Document;

    #[test]
    fn parse() {
        use reactor::Event;
        use tag::Tag;

        let mut reactor = Document::open("tests/fixtures/benton.svg").unwrap();

        macro_rules! test(
            ($matcher:pat) => (match reactor.next().unwrap() {
                $matcher => {},
                _ => assert!(false),
            });
        );

        test!(Event::Instruction);
        test!(Event::Comment);
        test!(Event::Declaration);
        test!(Event::Tag(Tag::Unknown(..)));
        test!(Event::Tag(Tag::Path(..)));
        test!(Event::Tag(Tag::Path(..)));
        test!(Event::Tag(Tag::Path(..)));
        test!(Event::Tag(Tag::Path(..)));
        test!(Event::Tag(Tag::Unknown(..)));

        assert!(reactor.next().is_none());
    }
}
