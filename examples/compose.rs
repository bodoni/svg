extern crate svg;

use svg::Composer;
use svg::element::{Path, SVG};

fn main() {
    let mut root = SVG::new();
    let element = Path::new();
    root.append(element);

    let mut output = String::new();
    root.compose(&mut Composer::new(&mut output)).unwrap();
    println!("{}", output);
}
