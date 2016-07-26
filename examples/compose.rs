extern crate svg;

use svg::Document;
use svg::element::Path;

fn main() {
    let path = Path::new()
                    .stroke("black")
                    .stroke_width(3)
                    .move_to((10, 10))
                    .line_by((0, 50))
                    .line_by((50, 0))
                    .line_by((0, -50))
                    .close();

    let document = Document::new()
                            .view_box((0, 0, 70, 70))
                            .append(path);

    println!("{}", document);
}
