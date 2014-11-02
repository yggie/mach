#![crate_name = "mithril"]
#![crate_type = "lib"]

#![feature(macro_rules)]

//! Contains the implementation of the Mithril Physics engine, an open-source
//! physics engine built on the Rust programming language.

#[unstable]
#[warn(missing_docs)]
#[path="math/math.rs"]
pub mod math;

#[unstable]
#[warn(missing_docs)]
#[path="shapes/shapes.rs"]
pub mod shapes;

#[experimental]
#[warn(missing_docs)]
#[path="properties/properties.rs"]
pub mod properties;

#[experimental]
#[warn(missing_docs)]
#[path="core/core.rs"]
pub mod core;

#[experimental]
#[warn(missing_docs)]
#[path="collisions/collisions.rs"]
pub mod collisions;

#[experimental]
#[warn(missing_docs)]
#[path="solvers/solvers.rs"]
pub mod solvers;

#[experimental]
#[warn(missing_docs)]
#[path="integrators/integrators.rs"]
pub mod integrators;
