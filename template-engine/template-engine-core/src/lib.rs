mod err;
pub mod model;

pub use err::*;
pub type Result<T> = std::result::Result<T, crate::Error>;
