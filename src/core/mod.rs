//! The `core` module contains the core implementation of the engine’s logic.

use std::f32;

use std::rc::Rc;
use std::cell::RefCell;

/// A unique identifier used to uniquely identify entities in the engine.
pub type UID = u64;

/// A shared pointer which gives access to the contained type instance.
pub type SharedCell<T> = Rc<RefCell<T>>;

/// A floating point type.
pub type Float = f32;

/// The PI constant.
pub static PI: f32 = f32::consts::PI;
/// Infinity.
pub static INFINITY: f32 = f32::INFINITY;
/// Negative Infinity.
pub static NEG_INFINITY: f32 = f32::NEG_INFINITY;
/// The tolerance used to resolve floating point differences.
pub static TOLERANCE: f32 = 1e-4;
