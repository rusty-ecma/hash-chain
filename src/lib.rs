mod error;
mod map;
mod set;
#[cfg(feature = "imutable")]
mod imutable;

pub use error::Error;
pub use map::ChainMap;
pub use set::ChainSet;
#[cfg(feature = "imutable")]
pub use imutable::LockedChainMap;