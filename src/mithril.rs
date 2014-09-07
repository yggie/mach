#![crate_name = "mithril"]
#![crate_type = "lib"]

//! Contains the implementation of the Mithril Physics engine, an open-source
//! physics engine built on the Rust programming language.

/// The math module contains all the logic associated with primitive mathematical
/// operations
pub mod math {
    pub use self::vector::Vector;

    mod vector;
}
