extern crate svg;

use svg::{Document, Tag};
use svg::element::path::{Command, Data};
use svg::reactor::Event;

fn main() {
    let path = "tests/fixtures/benton.svg";
    for event in Document::open(path).unwrap() {
        if let Event::Tag(Tag::Path(_, attributes)) = event {
            let data = attributes.get("d").unwrap();
            let data = Data::parse(data).unwrap();
            for command in data.iter() {
                match command {
                    &Command::MoveTo(..) => println!("Move!"),
                    &Command::LineTo(..) => println!("Line!"),
                    &Command::CurveTo(..) => println!("Curve!"),
                    _ => {},
                }
            }
        }
    }
}
