# SVG [![Build Status][travis-img]][travis-url]

Currently the library is limited to [paths][1].

## [Documentation][docs]

## Example

The example given below can be ran using the following command:

```
cargo run --example path
```

```rust
#![feature(core, path)]

extern crate svg;

use svg::{Event, Tag};
use svg::path::{Command, Data};

fn main() {
    let file = svg::open(&Path::new("tests/fixtures/benton.svg")).unwrap();
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
        match command {
            &Command::MoveTo(_, ref parameters) => {
                println!("Move to {:?}.", parameters);
            },
            &Command::LineTo(_, ref parameters) => {
                println!("Line to {:?}.", parameters);
            },
            &Command::CurveTo(_, ref parameters) => {
                println!("Curve to {:?}.", parameters);
            },
            &Command::SmoothCurveTo(_, ref parameters) => {
                println!("Smooth curve to {:?}.", parameters);
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
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Create a pull request.

[1]: http://www.w3.org/TR/SVG/paths.html

[travis-img]: https://travis-ci.org/stainless-steel/svg.svg?branch=master
[travis-url]: https://travis-ci.org/stainless-steel/svg
[docs]: https://stainless-steel.github.io/svg
