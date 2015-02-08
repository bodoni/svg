#![feature(core)]

extern crate svg;

use svg::path::{Command, Data, Positioning};

fn main() {
    let data = Data::parse("M0,0 l0,1 1,0 0,-1 z").ok().unwrap();

    for command in data.iter() {
        match command {
            &Command::MoveTo(Positioning::Absolute, ref coordinates) => {
                println!("Move to {:?}.", coordinates);
            },
            &Command::LineTo(Positioning::Relative, ref coordinates) => {
                println!("Draw line segments between {:?}.", coordinates);
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
