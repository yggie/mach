#![crate_name = "mithril"]
#![crate_type = "lib"]

//! Contains the implementation of the Mithril Physics engine, an open-source
//! physics engine built on the Rust programming language.

#[unstable]
#[warn(missing_doc)]
#[path="math/math.rs"]
pub mod math;

#[unstable]
#[warn(missing_doc)]
#[path="shapes/shapes.rs"]
pub mod shapes;

#[experimental]
#[warn(missing_doc)]
#[path="properties/properties.rs"]
pub mod properties;

#[experimental]
#[warn(missing_doc)]
#[path="bodies/bodies.rs"]
pub mod bodies;
