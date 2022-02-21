use euclid::{Point2D, UnknownUnit};

use super::{Number, Parameters};

/// If you use custom units in euclid, implement SVGUnit for CustomUnit to
/// enable using Point2D<N, CustomUnit> as svg Parameters.
pub trait SVGUnit {}

impl SVGUnit for UnknownUnit {}

macro_rules! implement {
    ($($primitive:ty,)*) => (
        $(impl<U> From<Point2D<$primitive, U>> for Parameters
            where U: SVGUnit, {
            #[inline]
            fn from(inner: Point2D<$primitive, U>) -> Self {
                vec![inner.x as Number, inner.y as Number].into()
            }
        })*
    );
}

implement! {
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize,
    f32, f64,
}
