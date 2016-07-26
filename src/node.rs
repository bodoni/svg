//! The nodes.

use std::collections::HashMap;
use std::fmt;

/// Attributes.
#[derive(Clone, Debug, Default)]
pub struct Attributes(HashMap<String, String>);

/// Children.
#[derive(Debug, Default)]
pub struct Children(Vec<Box<Node>>);

/// A node.
pub trait Node: fmt::Debug + fmt::Display {
}

/// A value.
pub trait Value {
    /// Convert into a string.
    fn into(self) -> String;
}

impl Attributes {
    /// Get an attribute.
    #[inline]
    pub fn get<T: AsRef<str>>(&self, name: T) -> Option<&str> {
        self.0.get(name.as_ref()).map(|name| name.as_str())
    }

    /// Set an attribute.
    #[inline]
    pub fn set<T: Into<String>, U: Value>(&mut self, name: T, value: U) {
        self.0.insert(name.into(), value.into());
    }
}

deref! { Attributes::0 => HashMap<String, String> }

impl fmt::Display for Attributes {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Children {
    /// Append a child.
    #[inline]
    pub fn append<T: 'static + Node>(&mut self, node: T) {
        self.0.push(Box::new(node))
    }
}

deref! { Children::0 => [Box<Node>] }

impl fmt::Display for Children {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let count = self.0.len();
        for i in 0..count {
            if i > 0 {
                try!(write!(formatter, "\n{}", self.0[i]));
            } else {
                try!(write!(formatter, "{}", self.0[i]));
            }
        }
        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use node::Attributes;

    #[test]
    fn set() {
        let mut attributes = Attributes::default();
        attributes.set("foo", 42);
    }
}
