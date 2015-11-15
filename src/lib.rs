//! Contains the implementation of the Mach Physics engine, an open-source
//! physics engine built on the Rust programming language.

#![crate_name = "mach"]
#![crate_type = "lib"]

#![warn(missing_docs)]

mod world;

pub use self::world::World;

pub mod core;
pub mod utils;
pub mod maths;
pub mod shapes;
pub mod dynamics;
pub mod entities;
pub mod collisions;

use std::fmt;

/// A unique identifier used to uniquely identify entities in the engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ID(u32);

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID({})", self.0)
    }
}
