use std::fmt::Write;

/// An output
pub trait Output: Write {}

pub struct Writer<T> {
    output: T,
}

impl<T> Output for T where T: Write {}

impl<T> Writer<T> {
    #[inline]
    pub fn new(output: T) -> Self {
        Writer { output: output }
    }
}

deref! { Writer<T>::output => T }
