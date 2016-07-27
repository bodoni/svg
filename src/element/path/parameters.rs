use std::ops::Deref;

/// Parameters of a command.
#[derive(Clone)]
pub struct Parameters(Vec<f32>);

impl Deref for Parameters {
    type Target = [f32];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<f32>> for Parameters {
    #[inline]
    fn from(inner: Vec<f32>) -> Self {
        Parameters(inner)
    }
}

impl From<Parameters> for Vec<f32> {
    #[inline]
    fn from(Parameters(inner): Parameters) -> Self {
        inner
    }
}

macro_rules! implement {
    ($($primitive:ty,)*) => (
        $(impl From<$primitive> for Parameters {
            #[inline]
            fn from(inner: $primitive) -> Self {
                Parameters(vec![inner as f32])
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
        impl<$($t),*> From<($($t),*)> for Parameters where $($t: Into<Parameters>),* {
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
