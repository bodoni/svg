extern crate svg;

use svg::element::{Path, SVG};

fn main() {
    let mut root = SVG::new();
    let child = Path::new();
    root.append(child);
    println!("{}", root);
}
