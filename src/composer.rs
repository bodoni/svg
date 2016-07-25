//! The composer.

use std::fmt;

use writer::{Output, Writer};

/// A composer.
pub struct Composer<T> {
    writer: Writer<T>,
}

/// An error.
pub type Error = fmt::Error;

/// A result.
pub type Result<T> = ::std::result::Result<T, Error>;

impl<T> Composer<T> where T: Output {
    /// Create a composer.
    #[inline]
    pub fn new(output: T) -> Self {
        Composer { writer: Writer::new(output) }
    }
}

deref! { Composer<T>::writer => T }
