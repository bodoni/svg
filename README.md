# SVG [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an SVG composer and parser.

## [Documentation][doc]

## Example: Composing

```rust
use svg::Document;
use svg::element::Path;

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
```

## Example: Parsing

```rust
use svg::Tag;
use svg::element::path::{Command, Data};
use svg::reactor::Event;

let path = "image.svg";
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
