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

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

/// A shared pointer which gives access to the contained type instance.
pub type SharedCell<T> = Rc<RefCell<T>>;

macro_rules! set_precision {
    ($precision:ident, $tolerance:expr) => {
        use std::$precision;

        /// A floating point type used throughout the engine, depends on the precision
        /// that the engine was compiled with.
        pub type Float = $precision;

        /// The PI constant. This is simply an alias for the constants in the standard
        /// library, depending on the precision the library was compiled with.
        pub static PI: $precision = $precision::consts::PI;
        /// Infinity. This is simply an alias for the constants in the standard library,
        /// depending on the precision the library was compiled with.
        pub static INFINITY: $precision = $precision::INFINITY;
        /// Negative infinity. This is simply an alias for the constants in the standard
        /// library, depending on the precision the library was compiled with.
        pub static NEG_INFINITY: $precision = $precision::NEG_INFINITY;
        /// The tolerance used to resolve floating point differences.
        pub static TOLERANCE: $precision = $tolerance;
    };
}

#[cfg(feature = "high_precision")]
set_precision!(f64, 1e-6);

#[cfg(not(feature = "high_precision"))]
set_precision!(f32, 1e-4);

/// A unique identifier used to uniquely identify entities in the engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ID(u32);

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID({})", self.0)
    }
}
