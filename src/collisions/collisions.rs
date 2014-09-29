//! Contains an implementation of strategies for optimizing spatial queries.

pub use self::space::Space;
pub use self::contact::Contact;
pub use self::proximitypair::ProximityPair;
pub use self::resolution::contactgraph::ContactGraph;
pub use self::detection::broadphase::bruteforce::BruteForce;

#[cfg(test)]
#[path="../../tests/collisions/collisions_test.rs"]
pub mod tests;

mod space;
mod contact;
mod proximitypair;

mod detection {
    pub mod broadphase {
        pub mod bruteforce;
    }
}

mod resolution {
    pub mod contactgraph;
}
