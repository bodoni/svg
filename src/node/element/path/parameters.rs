use std::ops::Deref;

use super::Number;

/// Parameters of a command.
#[derive(Clone, Debug)]
pub struct Parameters(Vec<Number>);

impl Deref for Parameters {
    type Target = [Number];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Parameters> for String {
    fn from(Parameters(inner): Parameters) -> Self {
        inner
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl From<Vec<Number>> for Parameters {
    #[inline]
    fn from(inner: Vec<Number>) -> Self {
        Parameters(inner)
    }
}

impl From<Parameters> for Vec<Number> {
    #[inline]
    fn from(Parameters(inner): Parameters) -> Self {
        inner
    }
}

impl<'l> From<&'l mut Parameters> for &'l mut Vec<Number> {
    #[inline]
    fn from(Parameters(inner): &'l mut Parameters) -> Self {
        inner
    }
}

macro_rules! implement {
    ($($primitive:ty,)*) => (
        $(impl From<$primitive> for Parameters {
            #[inline]
            fn from(inner: $primitive) -> Self {
                Parameters(vec![inner as Number])
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
        impl<$($t),*> From<($($t),*)> for Parameters
        where
            $($t: Into<Parameters>),*
        {
            fn from(inner: ($($t),*)) -> Self {
                let mut result = vec![];
                $(result.append(&mut implement!(@express inner.$n).into().into());)*
                Parameters(result)
            }
        }
    );
}

implement! { (T0, 0), (T1, 1) }
implement! { (T0, 0), (T1, 1), (T2, 2) }
implement! { (T0, 0), (T1, 1), (T2, 2), (T3, 3) }
implement! { (T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4) }
implement! { (T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4), (T5, 5) }
implement! { (T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4), (T5, 5), (T6, 6) }
implement! { (T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4), (T5, 5), (T6, 6), (T7, 7) }
