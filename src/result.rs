//! The results.

/// A parsing result.
pub type Parse<T> = ::std::result::Result<T, ::error::Parse>;

/// A reading result.
pub type Read<T> = ::std::result::Result<T, ::error::Read>;
