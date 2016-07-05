#[macro_use]
#[cfg(test)]
#[path="../../../tests/collisions/broadphase/broadphase_behaviour.rs"]
mod behaviours;

mod close_proximity_pair;
mod brute_force_broadphase;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod broadphase;

pub use self::broadphase::Broadphase;
pub use self::close_proximity_pair::CloseProximityPair;
pub use self::brute_force_broadphase::BruteForceBroadphase;
