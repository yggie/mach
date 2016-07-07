#[macro_use]
#[cfg(test)]
#[path="../../../tests/collisions/narrowphase/narrowphase_behaviour.rs"]
mod behaviours;

mod narrowphase_ref;
mod null_narrowphase;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod narrowphase;

pub use self::narrowphase::Narrowphase;
pub use self::narrowphase_ref::{NarrowphaseRef, NarrowphaseRefMut};
pub use self::null_narrowphase::NullNarrowphase;
