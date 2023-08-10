#![deny(missing_docs)]

//! Simple WASM-4 music tracker.

mod args;
mod description;
mod tracker;

pub use args::Args;
pub use description::parse_description;
pub use tracker::{Note, NotePitch, NotePitchKey, Track};
