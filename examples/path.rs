#![feature(core, path)]

extern crate svg;

use svg::{Event, Tag};
use svg::path::{Command, Data};

fn main() {
    let file = svg::open(&Path::new("tests/fixtures/benton.svg")).unwrap();
    for event in file.parse() {
        react(event);
    }
}

fn react(event: Event) {
    match event {
        Event::Tag(Tag::Path(_, attributes)) => {
            let data = attributes.get(&("d".to_string())).unwrap();
            let data = Data::parse(data).unwrap();
            draw(data);
        },
        _ => {
            println!("Not sure what to react.");
        },
    }
}

fn draw(data: Data) {
    for command in data.iter() {
        match command {
            &Command::MoveTo(_, ref parameters) => {
                println!("Move to {:?}.", parameters);
            },
            &Command::LineTo(_, ref parameters) => {
                println!("Line to {:?}.", parameters);
            },
            &Command::CurveTo(_, ref parameters) => {
                println!("Curve to {:?}.", parameters);
            },
            &Command::SmoothCurveTo(_, ref parameters) => {
                println!("Smooth curve to {:?}.", parameters);
            },
            &Command::ClosePath => {
                println!("Close the path.");
            },
            _ => {
                println!("Not sure what to do.");
            }
        }
    }
}
