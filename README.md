# SVG [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides an SVG composer and parser.

## Example: Composing

```rust
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

let data = Data::new()
    .move_to((10, 10))
    .line_by((0, 50))
    .line_by((50, 0))
    .line_by((0, -50))
    .close();

let path = Path::new()
    .set("fill", "none")
    .set("stroke", "black")
    .set("stroke-width", 3)
    .set("d", data);

let document = Document::new()
    .set("viewBox", (0, 0, 70, 70))
    .add(path);

svg::save("image.svg", &document).unwrap();
```

## Example: Parsing

```rust
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

let path = "image.svg";
let mut content = String::new();
for event in svg::open(path, &mut content).unwrap() {
    match event {
        Event::Tag(Path, _, attributes) => {
            let data = attributes.get("d").unwrap();
            let data = Data::parse(data).unwrap();
            for command in data.iter() {
                match command {
                    &Command::Move(..) => println!("Move!"),
                    &Command::Line(..) => println!("Line!"),
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[build-img]: https://github.com/bodoni/svg/workflows/build/badge.svg
[build-url]: https://github.com/bodoni/svg/actions/workflows/build.yml
[documentation-img]: https://docs.rs/svg/badge.svg
[documentation-url]: https://docs.rs/svg
[package-img]: https://img.shields.io/crates/v/svg.svg
[package-url]: https://crates.io/crates/svg
