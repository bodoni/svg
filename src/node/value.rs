use std::fmt;
use std::ops::Deref;

/// A value of an attribute.
#[derive(Clone, Debug)]
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

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(mut inner: Vec<T>) -> Self {
        Value(
            inner
                .drain(..)
                .map(|value| value.into().0)
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

macro_rules! implement {
    (@express $e:expr) => ($e);
    ($pattern:expr, $(($t:ident, $n:tt)),*) => (
        impl<$($t),*> From<($($t),*)> for Value
        where
            $($t: Into<Value>),*
        {
            fn from(inner: ($($t),*)) -> Self {
                Value(format!($pattern, $(implement!(@express inner.$n).into()),*))
            }
        }
    );
}

implement! { "{} {}", (T0, 0), (T1, 1) }
implement! { "{} {} {} {}", (T0, 0), (T1, 1), (T2, 2), (T3, 3) }

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn value_from_vector() {
        assert_eq!(String::from(Value::from(vec![42, 69])), "42 69");
    }
}
