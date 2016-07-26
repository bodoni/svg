/// A parameter.
pub trait Parameter {
    /// Convert into a vector.
    fn into(self) -> Vec<f32>;
}

impl Parameter for Vec<f32> {
    #[inline]
    fn into(self) -> Vec<f32> {
        self
    }
}

macro_rules! implement {
    ($($primitive:ty,)*) => (
        $(impl Parameter for $primitive {
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
        impl<$($t),*> Parameter for ($($t),*) where $($t: Parameter),* {
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
