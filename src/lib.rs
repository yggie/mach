//! Contains the implementation of the Mithril Physics engine, an open-source
//! physics engine built on the Rust programming language.

#![crate_name = "mithril"]
#![crate_type = "lib"]

#![warn(missing_docs)]

extern crate rand;

pub use self::world::World;

mod world;

pub mod maths;
pub mod shapes;
pub mod materials;
pub mod core;
pub mod collisions;
pub mod dynamics;
pub mod utils;
