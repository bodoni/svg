#![feature(core)]

extern crate svg;

use svg::path::{Command, Data, Positioning};

fn main() {
    let data = Data::parse("M0,0 l0,1 1,0 0,-1 z").ok().unwrap();

    for command in data.iter() {
        match command {
            &Command::MoveTo(Positioning::Absolute, ref parameters) => {
                println!("Move to {:?}.", parameters);
            },
            &Command::LineTo(Positioning::Relative, ref parameters) => {
                println!("Draw line segments between {:?}.", parameters);
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
