//! Contains the implementation of the Mithril Physics engine, an open-source
//! physics engine built on the Rust programming language.

#![crate_name = "mithril"]
#![crate_type = "lib"]

#![feature(core)]
#![unstable]
#![warn(missing_docs)]

pub use self::core::{ World, Body, State };
pub use self::math::{ Vector, Quaternion, Matrix };
pub use self::space::{ Space, SimpleSpace };
pub use self::materials::{ Material, Rigid };
pub use self::dynamics::{ Dynamics, SimpleDynamics };

pub mod math;
pub mod shapes;
pub mod materials;
pub mod core;
pub mod space;
pub mod dynamics;
pub mod utils;
