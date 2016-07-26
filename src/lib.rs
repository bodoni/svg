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
//! let data = Data::new().move_to((10, 10))
//!                       .line_by((0, 50))
//!                       .line_by((50, 0))
//!                       .line_by((0, -50))
//!                       .close();
//!
//! let path = Path::new()
//!                 .set("stroke", "black")
//!                 .set("stroke-width", 3)
//!                 .set("d", data);
//!
//! let document = Document::new()
//!                         .set("viewBox", (0, 0, 70, 70))
//!                         .append(path);
//!
//! println!("{}", document);
//! # }
//! ```
//!
//! ## Example: Parsing
//!
//! ```
//! # extern crate svg;
//! use svg::Tag;
//! use svg::element::path::{Command, Data};
//! use svg::reactor::Event;
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
use std::path::Path;

#[macro_use]
mod macros;

mod reader;

pub mod element;
pub mod error;
pub mod node;
pub mod reactor;
pub mod result;
pub mod tag;

pub use node::Node;
pub use reactor::Reactor;
pub use tag::Tag;

/// A document.
pub type Document = element::SVG;

/// Open a document.
pub fn open<'l, T: AsRef<Path>>(path: T) -> result::Read<Reactor<'l>> {
    use std::fs::File;
    use std::io::Read;

    let mut content = String::new();
    let mut file = try!(File::open(path));
    try!(file.read_to_string(&mut content));
    Ok(read(content))
}

/// Read a document.
#[inline]
pub fn read<'l, T: Into<Cow<'l, str>>>(content: T) -> Reactor<'l> {
    Reactor::new(content)
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
