/// A value of an attribute.
pub trait Value {
    /// Convert into a string.
    fn into(self) -> String;
}

macro_rules! implement {
    ($($primitive:ty,)*) => (
        $(impl Value for $primitive {
            #[inline]
            fn into(self) -> String {
                self.to_string()
            }
        })*
    );
}

implement! {
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize,
    f32, f64,
    String,
    bool,
}

impl<'l> Value for &'l str {
    #[inline]
    fn into(self) -> String {
        self.to_string()
    }
}

impl<T> Value for Vec<T> where T: Value {
    fn into(mut self) -> String {
        let mut result = String::new();
        for (i, value) in self.drain(..).enumerate() {
            if i > 0 {
                result.push(' ');
            }
            result.push_str(&value.into());
        }
        result
    }
}

macro_rules! implement {
    (@express $e:expr) => ($e);
    ($pattern:expr, $(($t:ident, $n:tt)),*) => (
        impl<$($t),*> Value for ($($t),*) where $($t: Value),* {
            #[inline]
            fn into(self) -> String {
                format!($pattern, $(implement!(@express self.$n).into()),*)
            }
        }
    );
}

implement! { "{} {}", (T0, 0), (T1, 1) }
implement! { "{} {} {} {}", (T0, 0), (T1, 1), (T2, 2), (T3, 3) }
