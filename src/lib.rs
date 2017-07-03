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

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub mod node;
pub mod parser;

pub use node::Node;
pub use parser::Parser;

/// A document.
pub type Document = node::element::SVG;

/// Open a document.
pub fn open<'l, T>(path: T) -> io::Result<Parser<'l>> where T: AsRef<Path> {
    let mut file = File::open(path)?;
    read_internal(&mut file)
}

/// Read a document.
pub fn read<'l, T>(source: T) -> io::Result<Parser<'l>> where T: Read {
    read_internal(source)
}

/// Save a document.
pub fn save<T, U>(path: T, document: &U) -> io::Result<()> where T: AsRef<Path>, U: Node {
    let mut file = File::create(path)?;
    write_internal(&mut file, document)
}

/// Write a document.
pub fn write<T, U>(target: T, document: &U) -> io::Result<()> where T: Write, U: Node {
    write_internal(target, document)
}

#[inline(always)]
fn read_internal<'l, R>(mut source: R) -> io::Result<Parser<'l>> where R: Read {
    let mut content = String::new();
    source.read_to_string(&mut content)?;
    Ok(Parser::new(content))
}

#[inline(always)]
fn write_internal<T, U>(mut target: T, document: &U) -> io::Result<()> where T: Write, U: Node {
    target.write_all(&document.to_string().into_bytes())
}

#[cfg(test)]
mod tests {

    const TEST_PATH: &'static str = "tests/fixtures/benton.svg";

    #[test]
    fn open() {
        use parser::Event;

        let mut parser = ::open(self::TEST_PATH).unwrap();

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

    #[test]
    fn read() {
        use std::fs::File;
        let _ = ::read(&mut File::open(self::TEST_PATH).unwrap());
    }
}
