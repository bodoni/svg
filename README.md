# SVG [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an SVG parser, which is currently limited to [paths][1].

## [Documentation][doc]

## Example

The example given below can be ran using the following command:

```
cargo run --example path
```

```rust
extern crate svg;

use svg::{Event, Tag};
use svg::path::{Command, Data};

fn main() {
    let file = svg::open("tests/fixtures/benton.svg").unwrap();
    for event in file.parse() {
        match event {
            Event::Tag(Tag::Path(_, attributes)) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                draw(data);
            },
            _ => {
                println!("Not sure how to react.");
            },
        }
    }
}

fn draw(data: Data) {
    for command in data.iter() {
        match *command {
            Command::MoveTo(_, ref parameters) => {
                println!("Move to {:?}.", parameters);
            },
            Command::LineTo(_, ref parameters) => {
                println!("Line to {:?}.", parameters);
            },
            Command::CurveTo(_, ref parameters) => {
                println!("Curve to {:?}.", parameters);
            },
            Command::SmoothCurveTo(_, ref parameters) => {
                println!("Smooth curve to {:?}.", parameters);
            },
            Command::ClosePath => {
                println!("Close the path.");
            },
            _ => {
                println!("Not sure what to do.");
            }
        }
    }
}
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[1]: http://www.w3.org/TR/SVG/paths.html

[doc]: https://bodoni.github.io/svg
[status-img]: https://travis-ci.org/bodoni/svg.svg?branch=master
[status-url]: https://travis-ci.org/bodoni/svg
[version-img]: https://img.shields.io/crates/v/svg.svg
[version-url]: https://crates.io/crates/svg
