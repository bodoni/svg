//! An SVG composer and parser.
//!
//! ## Example: Composing
//!
//! ```
//! # extern crate svg;
//! use svg::Document;
//! use svg::node::element::Path;
//! use svg::node::element::path::Data;
//!
//! # fn main() {
//! let data = Data::new()
//!                 .move_to((10, 10))
//!                 .line_by((0, 50))
//!                 .line_by((50, 0))
//!                 .line_by((0, -50))
//!                 .close();
//!
//! let path = Path::new()
//!                 .set("fill", "none")
//!                 .set("stroke", "black")
//!                 .set("stroke-width", 3)
//!                 .set("d", data);
//!
//! let document = Document::new()
//!                         .set("viewBox", (0, 0, 70, 70))
//!                         .add(path);
//!
//! svg::save("image.svg", &document).unwrap();
//! # ::std::fs::remove_file("image.svg");
//! # }
//! ```
//!
//! ## Example: Parsing
//!
//! ```
//! # extern crate svg;
//! use svg::node::element::path::{Command, Data};
//! use svg::node::element::tag::Path;
//! use svg::parser::Event;
//!
//! # fn main() {
//! let path = "image.svg";
//! # let path = "tests/fixtures/benton.svg";
//! for event in svg::open(path).unwrap() {
//!     match event {
//!         Event::Tag(Path, _, attributes) => {
//!             let data = attributes.get("d").unwrap();
//!             let data = Data::parse(data).unwrap();
//!             for command in data.iter() {
//!                 match command {
//!                     &Command::Move(..) => println!("Move!"),
//!                     &Command::Line(..) => println!("Line!"),
//!                     _ => {},
//!                 }
//!             }
//!         },
//!         _ => {},
//!     }
//! }
//! # }
//! ```

use std::borrow::Cow;
use std::io;
use std::path::Path;

pub mod node;
pub mod parser;

pub use node::Node;
pub use parser::Parser;

/// A document.
pub type Document = node::element::SVG;

/// Open a document.
pub fn open<'l, T>(path: T) -> io::Result<Parser<'l>> where T: AsRef<Path> {
    use std::fs::File;
    use std::io::Read;

    let mut content = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut content));
    Ok(parse(content))
}

/// Parse a document.
#[inline]
pub fn parse<'l, T>(content: T) -> Parser<'l> where T: Into<Cow<'l, str>> {
    Parser::new(content)
}

/// Save a document.
pub fn save<T, U>(path: T, document: &U) -> io::Result<()> where T: AsRef<Path>, U: Node {
    use std::fs::File;
    use std::io::Write;

    let mut file = try!(File::create(path));
    file.write_all(&document.to_string().into_bytes())
}

#[cfg(test)]
mod tests {
    #[test]
    fn open() {
        use parser::Event;

        let mut parser = ::open("tests/fixtures/benton.svg").unwrap();

        macro_rules! test(
            ($matcher:pat) => (match parser.next().unwrap() {
                $matcher => {},
                _ => unreachable!(),
            });
        );

        test!(Event::Instruction);
        test!(Event::Comment);
        test!(Event::Declaration);
        test!(Event::Tag("svg", _, _));
        test!(Event::Tag("path", _, _));
        test!(Event::Tag("path", _, _));
        test!(Event::Tag("path", _, _));
        test!(Event::Tag("path", _, _));
        test!(Event::Tag("svg", _, _));

        assert!(parser.next().is_none());
    }
}
