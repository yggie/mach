#[macro_use]
#[cfg(test)]
#[path="../../../tests/collisions/narrowphase/narrowphase_behaviour.rs"]
mod behaviours;

mod narrowphase;
mod narrowphase_ref;
mod null_narrowphase;

pub use self::narrowphase::Narrowphase;
pub use self::narrowphase_ref::{NarrowphaseRef, NarrowphaseRefMut};
pub use self::null_narrowphase::NullNarrowphase;
