mod brute_force;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod broadphase;

pub use self::broadphase::Broadphase;
pub use self::brute_force::BruteForce;
