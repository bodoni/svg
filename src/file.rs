use {Error, Parser, Result};

/// A file.
pub struct File {
    text: String,
}

macro_rules! io(
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
        use std::old_io::File as IoFile;

        Ok(File {
            text: io!(io!(IoFile::open(path)).read_to_string()),
        })
    }

    /// Return an iterator over the content of the file.
    pub fn parse<'s>(&'s self) -> Parser<'s> {
        Parser::new(&self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::File;
    use tests::find_fixture;
    use {Event, Tag};

    #[test]
    fn parse() {
        let file = File::open(&find_fixture("benton")).unwrap();
        let mut parser = file.parse();

        macro_rules! test(
            ($parser:expr, $tag:ident) => (
                match $parser.next().unwrap() {
                    Event::Tag(Tag::$tag(..)) => {},
                    _ => assert!(false),
                }
            );
        );

        for _ in range(0, 4) {
            test!(parser, Unknown);
        }
        for _ in range(0, 4) {
            test!(parser, Path);
        }
        test!(parser, Unknown);
        assert!(parser.next().is_none());
    }
}
