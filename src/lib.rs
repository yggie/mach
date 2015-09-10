//! Contains the implementation of the Mach Physics engine, an open-source
//! physics engine built on the Rust programming language.

#![crate_name = "mach"]
#![crate_type = "lib"]

#![warn(missing_docs)]

pub use self::world::World;

mod world;

pub mod core;
pub mod utils;
pub mod maths;
pub mod shapes;
pub mod dynamics;
pub mod entities;
pub mod collisions;
