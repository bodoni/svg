//! The tags.

#![allow(non_upper_case_globals)]

macro_rules! implement {
    ($($const_name:ident: $tag_name:expr,)*) => (
        $(pub const $const_name: &'static str = $tag_name;)*
    );
}

implement! {
    Animate: "animate",
    AnimateColor: "animateColor",
    AnimateMotion: "animateMotion",
    AnimateTransform: "animateTransform",
    Circle: "circle",
    ClipPath: "clipPath",
    Definitions: "defs",
    Description: "desc",
    Ellipse: "ellipse",
    Filter: "filter",
    Group: "g",
    Image: "image",
    Line: "line",
    LinearGradient: "linearGradient",
    Link: "a",
    Marker: "marker",
    Mask: "mask",
    MotionPath: "mpath",
    Path: "path",
    Polygon: "polygon",
    Polyline: "polyline",
    RadialGradient: "radialGradient",
    Rectangle: "rect",
    Script: "script",
    Stop: "stop",
    Style: "style",
    SVG: "svg",
    Symbol: "symbol",
    Text: "text",
    TextPath: "textPath",
    Title: "title",
    Use: "use",
}
