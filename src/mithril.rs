#![crate_name = "mithril"]
#![crate_type = "lib"]

//! Contains the implementation of the Mithril Physics engine, an open-source
//! physics engine built on the Rust programming language.

#[unstable]
pub mod math;
#[unstable]
pub mod shapes;
#[experimental]
pub mod properties;
#[experimental]
pub mod bodies;
