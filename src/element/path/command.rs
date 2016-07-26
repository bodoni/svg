/// A command.
#[derive(Clone, Debug)]
pub enum Command {
    /// [Establish][1] a new current point.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataMovetoCommands
    Move(Position, Vec<f32>),

    /// [Draw][1] straight lines.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    Line(Position, Vec<f32>),

    /// [Draw][1] horizontal lines.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    HorizontalLine(Position, Vec<f32>),

    /// [Draw][1] vertical lines.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    VerticalLine(Position, Vec<f32>),

    /// [Draw][1] a quadratic Bézier curve.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    QuadraticCurve(Position, Vec<f32>),

    /// [Draw][1] a quadratic Bézier curve assuming the control point to be the
    /// reflection of the control point on the previous command relative to the
    /// current point.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    SmoothQuadraticCurve(Position, Vec<f32>),

    /// [Draw][1] a cubic Bézier curve.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    CubicCurve(Position, Vec<f32>),

    /// [Draw][1] a cubic Bézier curve assuming the first control point to be
    /// the reflection of the second control point on the previous command
    /// relative to the current point.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    SmoothCubicCurve(Position, Vec<f32>),

    /// [Draw][1] an elliptical arc.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataEllipticalArcCommands
    EllipticalArc(Position, Vec<f32>),

    /// [End][1] the current subpath.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataClosePathCommand
    Close,
}

/// Parameters.
pub trait Parameters {
    /// Convert into a vector.
    fn into(self) -> Vec<f32>;
}

/// A type of positioning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Position {
    /// Absolute.
    Absolute,
    /// Relative.
    Relative,
}

impl Parameters for Vec<f32> {
    #[inline]
    fn into(self) -> Vec<f32> {
        self
    }
}

macro_rules! implement {
    ($($primitive:ty,)*) => (
        $(impl Parameters for $primitive {
            #[inline]
            fn into(self) -> Vec<f32> {
                vec![self as f32]
            }
        })*
    );
}

implement! {
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize,
    f32, f64,
}

macro_rules! implement {
    (@express $e:expr) => ($e);
    ($(($t:ident, $n:tt)),*) => (
        impl<$($t),*> Parameters for ($($t),*) where $($t: Parameters),* {
            #[inline]
            fn into(self) -> Vec<f32> {
                let mut result = vec![];
                $(result.append(&mut implement!(@express self.$n).into());)*
                result
            }
        }
    );
}

implement! { (T0, 0), (T1, 1) }
implement! { (T0, 0), (T1, 1), (T2, 2), (T3, 3) }
implement! { (T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4), (T5, 5) }
implement! { (T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4), (T5, 5), (T6, 6) }
