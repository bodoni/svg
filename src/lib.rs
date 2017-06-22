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
//! use std::fs::File;
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

use std::io;
use std::path::Path;

pub mod node;
pub mod parser;

pub use node::Node;
pub use parser::Parser;
use std::io::{Read, Write};

/// A document.
pub type Document = node::element::SVG;

/// Open a document from a path
pub fn open<'l, P>(path: P) -> io::Result<Parser<'l>> where P: AsRef<Path> {
    use std::fs::File;
    let mut file = File::open(path)?;
    open_internal(&mut file)
}

/// Open a document from an arbitrary source
pub fn open_from<'l, R>(source: R) -> io::Result<Parser<'l>> where R: Read {
    open_internal(source)
}

/// Save a document to a path
pub fn save<'l, P, U>(path: P, document: &U) -> io::Result<()> where P: AsRef<Path>, U: Node{
    use std::fs::File;
    let mut file = File::create(path)?;
    save_internal(&mut file, document)
}

/// Open a document from an arbitrary source
pub fn save_to<'l, W, U>(target: W, document: &U) -> io::Result<()> where W: Write, U: Node {
    save_internal(target, document)
}

#[inline(always)]
fn open_internal<'l, R>(mut source: R) -> io::Result<Parser<'l>> where R: Read {
    let mut content = String::new();
    source.read_to_string(&mut content)?;
    Ok(Parser::new(content))
}

/// Save a document to a path
#[inline(always)]
fn save_internal<W, U>(mut target: W, document: &U) -> io::Result<()> where W: Write, U: Node {
    target.write_all(&document.to_string().into_bytes())
}

#[cfg(test)]
mod tests {
    #[test]
    fn open() {
        use parser::Event;
        use std::fs::File;

        const TEST_PATH: &'static str = "tests/fixtures/benton.svg";

        let mut parser = ::open(TEST_PATH).unwrap();

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

        let _ = ::open_from(&mut File::open(TEST_PATH).unwrap());
    }
}
