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
//!     .move_to((10, 10))
//!     .line_by((0, 50))
//!     .line_by((50, 0))
//!     .line_by((0, -50))
//!     .close();
//!
//! let path = Path::new()
//!     .set("fill", "none")
//!     .set("stroke", "black")
//!     .set("stroke-width", 3)
//!     .set("d", data);
//!
//! let document = Document::new()
//!     .set("viewBox", (0, 0, 70, 70))
//!     .add(path);
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
//! let mut content = String::new();
//! for event in svg::open(path, &mut content).unwrap() {
//!     match event {
//!         Event::Tag(Path, _, attributes) => {
//!             let data = attributes.get("d").unwrap();
//!             let data = Data::parse(data).unwrap();
//!             for command in data.iter() {
//!                 match command {
//!                     &Command::Move(..) => println!("Move!"),
//!                     &Command::Line(..) => println!("Line!"),
//!                     _ => {}
//!                 }
//!             }
//!         }
//!         _ => {}
//!     }
//! }
//! # }
//! ```

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub mod node;
pub mod parser;

pub use crate::node::Node;
pub use crate::parser::Parser;

/// A document.
pub type Document = node::element::SVG;

/// Open a document.
pub fn open<'l, T>(path: T, mut content: &'l mut String) -> io::Result<Parser<'l>>
where
    T: AsRef<Path>,
{
    let mut file = File::open(path)?;
    file.read_to_string(&mut content)?;
    read(content)
}

/// Read a document.
pub fn read<'l>(content: &'l str) -> io::Result<Parser<'l>> {
    Ok(Parser::new(content))
}

/// Save a document.
pub fn save<T, U>(path: T, document: &U) -> io::Result<()>
where
    T: AsRef<Path>,
    U: Node,
{
    let mut file = File::create(path)?;
    file.write_all(&document.to_string().into_bytes())
}

/// Write a document.
pub fn write<T, U>(mut target: T, document: &U) -> io::Result<()>
where
    T: Write,
    U: Node,
{
    target.write_all(&document.to_string().into_bytes())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use crate::parser::{Event, Parser};

    const TEST_PATH: &'static str = "tests/fixtures/benton.svg";

    #[test]
    fn open() {
        let mut content = String::new();
        exercise(crate::open(self::TEST_PATH, &mut content).unwrap());
    }

    #[test]
    fn read() {
        let mut content = String::new();
        let mut file = File::open(self::TEST_PATH).unwrap();
        file.read_to_string(&mut content).unwrap();

        exercise(crate::read(&content).unwrap());
    }

    fn exercise<'l>(mut parser: Parser<'l>) {
        macro_rules! test(
            ($matcher:pat) => (match parser.next().unwrap() {
                $matcher => {}
                _ => unreachable!(),
            });
        );

        test!(Event::Instruction(_));
        test!(Event::Comment(_));
        test!(Event::Declaration(_));
        test!(Event::Tag("svg", _, _));
        test!(Event::Tag("path", _, _));
        test!(Event::Tag("path", _, _));
        test!(Event::Tag("path", _, _));
        test!(Event::Tag("path", _, _));
        test!(Event::Tag("svg", _, _));

        assert!(parser.next().is_none());
    }
}
