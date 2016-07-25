# SVG [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an SVG composer and parser.

## [Documentation][doc]

## Example

```rust
extern crate svg;

use svg::Tag;
use svg::element::path::{Command, Data};
use svg::parser::Event;

fn main() {
    let path = "benton.svg";
    for event in svg::parse(path).unwrap() {
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
