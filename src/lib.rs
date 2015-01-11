//! Contains the implementation of the Mithril Physics engine, an open-source
//! physics engine built on the Rust programming language.

#![crate_name = "mithril"]
#![crate_type = "lib"]

#![allow(unstable)]
#![unstable]
#![warn(missing_docs)]

pub mod math;
pub mod shapes;
pub mod materials;
pub mod core;
pub mod space;
pub mod dynamics;
