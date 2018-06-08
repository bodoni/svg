use super::Parameters;
use super::Position;

/// A command of a data attribute.
#[derive(Clone, Debug)]
pub enum Command {
    /// [Establish][1] a new current point.
    ///
    /// [1]: https://www.w3.org/TR/SVG/paths.html#PathDataMovetoCommands
    Move(Position, Parameters),

    /// [Draw][1] straight lines.
    ///
    /// [1]: https://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    Line(Position, Parameters),

    /// [Draw][1] horizontal lines.
    ///
    /// [1]: https://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    HorizontalLine(Position, Parameters),

    /// [Draw][1] vertical lines.
    ///
    /// [1]: https://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    VerticalLine(Position, Parameters),

    /// [Draw][1] a quadratic Bézier curve.
    ///
    /// [1]: https://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    QuadraticCurve(Position, Parameters),

    /// [Draw][1] a quadratic Bézier curve assuming the control point to be the
    /// reflection of the control point on the previous command relative to the
    /// current point.
    ///
    /// [1]: https://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    SmoothQuadraticCurve(Position, Parameters),

    /// [Draw][1] a cubic Bézier curve.
    ///
    /// [1]: https://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    CubicCurve(Position, Parameters),

    /// [Draw][1] a cubic Bézier curve assuming the first control point to be
    /// the reflection of the second control point on the previous command
    /// relative to the current point.
    ///
    /// [1]: https://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    SmoothCubicCurve(Position, Parameters),

    /// [Draw][1] an elliptical arc.
    ///
    /// [1]: https://www.w3.org/TR/SVG/paths.html#PathDataEllipticalArcCommands
    EllipticalArc(Position, Parameters),

    /// [End][1] the current subpath.
    ///
    /// [1]: https://www.w3.org/TR/SVG/paths.html#PathDataClosePathCommand
    Close,
}

macro_rules! implement {
    ($($command:ident($position:ident) => $letter:expr,)*) => (
        impl From<Command> for String {
            fn from(command: Command) -> Self {
                use self::Command::*;
                use super::Position::*;
                match command {
                    $($command($position, parameters) => {
                        format!(concat!($letter, "{}"), String::from(parameters))
                    })*
                    Close => String::from("z"),
                }
            }
        }
    );
}

implement! {
    Move(Absolute) => "M",
    Move(Relative) => "m",
    Line(Absolute) => "L",
    Line(Relative) => "l",
    HorizontalLine(Absolute) => "H",
    HorizontalLine(Relative) => "h",
    VerticalLine(Absolute) => "V",
    VerticalLine(Relative) => "v",
    QuadraticCurve(Absolute) => "Q",
    QuadraticCurve(Relative) => "q",
    SmoothQuadraticCurve(Absolute) => "T",
    SmoothQuadraticCurve(Relative) => "t",
    CubicCurve(Absolute) => "C",
    CubicCurve(Relative) => "c",
    SmoothCubicCurve(Absolute) => "S",
    SmoothCubicCurve(Relative) => "s",
    EllipticalArc(Absolute) => "A",
    EllipticalArc(Relative) => "a",
}
