use std::path::Path;

use {Error, Parser, Result};

/// A file.
pub struct File {
    text: String,
}

macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(result) => result,
            Err(error) => return Err(Error {
                line: 0,
                column: 0,
                message: format!("{:?}", error),
            }),
        }
    );
);

impl File {
    /// Open a file.
    #[inline]
    pub fn open(path: &Path) -> Result<File> {
        use std::fs;
        use std::io::Read;

        let mut text = String::new();
        let mut file = ok!(fs::File::open(path));
        ok!(file.read_to_string(&mut text));

        Ok(File { text: text })
    }

    /// Return an iterator over the content of the file.
    pub fn parse<'s>(&'s self) -> Parser<'s> {
        Parser::new(&self.text)
    }
}

#[cfg(test)]
mod tests {
    use Event;
    use super::File;
    use tag::Tag;
    use tests::find_fixture;

    #[test]
    fn parse() {
        let file = File::open(&find_fixture("benton")).unwrap();
        let mut parser = file.parse();

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
