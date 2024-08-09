#[cfg(feature = "crossterm")]
mod crossterm;

#[cfg(feature = "crossterm")]
pub use crate::crossterm::*;
