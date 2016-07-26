//! The [svg][1] element.
//!
//! [1]: https://www.w3.org/TR/SVG/struct.html#SVGElement

node! {
    #[doc = "An svg element."]
    pub struct SVG("svg") {
        view_box ["viewBox"] [<natural natural natural natural>],
    }
}
