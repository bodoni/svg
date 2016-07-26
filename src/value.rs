/// A value.
pub trait Value {
    /// Convert into a string.
    fn into(self) -> String;
}

macro_rules! value {
    ($($primitive:ty,)*) => (
        $(impl Value for $primitive {
            #[inline]
            fn into(self) -> String {
                self.to_string()
            }
        })*
    );
}

value! {
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

impl<T, U> Value for (T, U) where T: Value, U: Value {
    #[inline]
    fn into(self) -> String {
        format!("{} {}", self.0.into(), self.1.into())
    }
}

impl<T, U, V, W> Value for (T, U, V, W) where T: Value, U: Value, V: Value, W: Value {
    #[inline]
    fn into(self) -> String {
        format!("{} {} {} {}", self.0.into(), self.1.into(), self.2.into(), self.3.into())
    }
}
