//! The `core` module contains the core implementation of the engine’s logic.

use std::hash::Hash;

pub use self::body::Body;
pub use self::state::State;

/// A trait that must be implemented for all types acting as a unique identifier
/// for a `Body`.
pub trait Handle: Copy + Hash + Eq { }

/// A unique identifier primarily used to identify a `Body` in a `Database`.
pub type UID = usize;

impl Handle for UID { }

mod body;
mod state;
