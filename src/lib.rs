//! Contains the implementation of the Mithril Physics engine, an open-source
//! physics engine built on the Rust programming language.

#![crate_name = "mithril"]
#![crate_type = "lib"]

#![feature(core)]

#![unstable]
#![warn(missing_docs)]

pub use self::world::World;

mod world;

pub mod math;
pub mod shapes;
pub mod materials;
pub mod core;
pub mod collisions;
pub mod dynamics;
pub mod utils;
