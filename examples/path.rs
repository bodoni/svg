extern crate svg;

use svg::Tag;
use svg::parser::Event;
use svg::tag::path::{Command, Data};

fn main() {
    let path = "tests/fixtures/benton.svg";
    for event in svg::open(path).unwrap() {
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
