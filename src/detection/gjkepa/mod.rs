mod simplex;
mod polytope;
mod contact_cache;
mod simplex_cache;
mod minkowski_difference;

pub use self::contact_cache::ContactCache;

#[cfg(test)]
#[path="../../../tests/private/detection/gjkepa/polytope_test.rs"]
mod polytope_test;

#[cfg(test)]
#[path="../../../tests/private/detection/gjkepa/simplex_cache_test.rs"]
mod simplex_cache_test;

#[cfg(test)]
#[path="../../../tests/private/detection/gjkepa/minkowski_difference_test.rs"]
mod minkowski_difference_test;
