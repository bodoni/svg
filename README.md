# SVG [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an SVG composer and parser.

## [Documentation][doc]

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
use svg::parser::Event;

let path = "image.svg";
for event in svg::open(path).unwrap() {
    match event {
        Event::Tag("path", _, attributes) => {
            let data = attributes.get("d").unwrap();
            let data = Data::parse(data).unwrap();
            for command in data.iter() {
                match command {
                    &Command::Move(..) => println!("Move!"),
                    &Command::Line(..) => println!("Line!"),
                    _ => {},
                }
            }
        },
        _ => {},
    }
}
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[doc]: https://bodoni.github.io/svg
[status-img]: https://travis-ci.org/bodoni/svg.svg?branch=master
[status-url]: https://travis-ci.org/bodoni/svg
[version-img]: https://img.shields.io/crates/v/svg.svg
[version-url]: https://crates.io/crates/svg
