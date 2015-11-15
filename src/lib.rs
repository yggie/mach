//! Contains the implementation of the Mach Physics engine, an open-source
//! physics engine built on the Rust programming language.

#![crate_name = "mach"]
#![crate_type = "lib"]

#![warn(missing_docs)]

mod world;

pub use self::world::World;

pub mod utils;
pub mod maths;
pub mod shapes;
pub mod dynamics;
pub mod entities;
pub mod collisions;

use std::f32;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

/// A shared pointer which gives access to the contained type instance.
pub type SharedCell<T> = Rc<RefCell<T>>;

/// A unique identifier used to uniquely identify entities in the engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ID(u32);

/// A floating point type.
pub type Float = f32;

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID({})", self.0)
    }
}

/// The PI constant.
pub static PI: f32 = f32::consts::PI;
/// Infinity.
pub static INFINITY: f32 = f32::INFINITY;
/// Negative Infinity.
pub static NEG_INFINITY: f32 = f32::NEG_INFINITY;
/// The tolerance used to resolve floating point differences.
pub static TOLERANCE: f32 = 1e-4;
