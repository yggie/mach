#[macro_use]
#[cfg(test)]
#[path="../../../tests/collisions/narrowphase/narrowphase_behaviour.rs"]
mod behaviours;
mod null_narrowphase;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod narrowphase;

pub use self::narrowphase::{Narrowphase, NarrowphaseData};
pub use self::null_narrowphase::NullNarrowphase;
