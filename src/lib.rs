//! An SVG composer and parser.
//!
//! ## Example: Composing
//!
//! ```
//! # extern crate svg;
//! use svg::Document;
//! use svg::element::Path;
//! use svg::element::path::Data;
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
//! use svg::{Event, Tag};
//! use svg::element::path::{Command, Data};
//!
//! # fn main() {
//! let path = "image.svg";
//! # let path = "tests/fixtures/benton.svg";
//! for event in svg::open(path).unwrap() {
//!     if let Event::Tag(Tag::Path(_, attributes)) = event {
//!         let data = attributes.get("d").unwrap();
//!         let data = Data::parse(data).unwrap();
//!         for command in data.iter() {
//!             match command {
//!                 &Command::Move(..) => println!("Move!"),
//!                 &Command::Line(..) => println!("Line!"),
//!                 _ => {},
//!             }
//!         }
//!     }
//! }
//! # }
//! ```

use std::borrow::Cow;
use std::io;
use std::path::Path;

#[macro_use]
mod macros;

mod document;
mod error;
mod reactor;
mod reader;

pub mod element;
pub mod node;
pub mod tag;

pub use document::Document;
pub use element::Element;
pub use error::Error;
pub use node::Node;
pub use reactor::{Event, Reactor};
pub use tag::Tag;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

/// Open a document.
pub fn open<'l, T>(path: T) -> io::Result<Reactor<'l>> where T: AsRef<Path> {
    use std::fs::File;
    use std::io::Read;

    let mut content = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut content));
    Ok(read(content))
}

/// Read a document.
#[inline]
pub fn read<'l, T>(content: T) -> Reactor<'l> where T: Into<Cow<'l, str>> {
    Reactor::new(content)
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
    fn parse() {
        use reactor::Event;
        use tag::Tag;

        let mut reactor = ::open("tests/fixtures/benton.svg").unwrap();

        macro_rules! test(
            ($matcher:pat) => (match reactor.next().unwrap() {
                $matcher => {},
                _ => unreachable!(),
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
