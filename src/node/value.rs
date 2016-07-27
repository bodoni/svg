use std::fmt;
use std::ops::Deref;

/// A value of an attribute.
#[derive(Clone)]
pub struct Value(String);

impl Deref for Value {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Value {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<Value> for String {
    #[inline]
    fn from(Value(inner): Value) -> Self {
        inner
    }
}

macro_rules! implement {
    ($($primitive:ty,)*) => (
        $(impl From<$primitive> for Value {
            #[inline]
            fn from(inner: $primitive) -> Self {
                Value(inner.to_string())
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

impl<'l> From<&'l str> for Value {
    #[inline]
    fn from(inner: &'l str) -> Value {
        Value(inner.to_string())
    }
}

impl<T> From<Vec<T>> for Value where T: Into<Value> {
    fn from(mut inner: Vec<T>) -> Self {
        let mut result = String::new();
        for (i, inner) in inner.drain(..).enumerate() {
            let Value(inner) = inner.into();
            if i > 0 {
                result.push(' ');
            }
            result.push_str(&inner);
        }
        Value(result)
    }
}

macro_rules! implement {
    (@express $e:expr) => ($e);
    ($pattern:expr, $(($t:ident, $n:tt)),*) => (
        impl<$($t),*> From<($($t),*)> for Value where $($t: Into<Value>),* {
            fn from(inner: ($($t),*)) -> Self {
                Value(format!($pattern, $(implement!(@express inner.$n).into()),*))
            }
        }
    );
}

implement! { "{} {}", (T0, 0), (T1, 1) }
implement! { "{} {} {} {}", (T0, 0), (T1, 1), (T2, 2), (T3, 3) }
