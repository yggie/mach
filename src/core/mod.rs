//! The `core` module contains the core implementation of the engine’s logic.

use std::fmt;
use std::hash::Hash;

pub use self::body::Body;
pub use self::state::State;

/// A trait that must be implemented for all types acting as a unique identifier
/// for a `Body`.
pub trait Handle: Copy + Hash + Eq + fmt::Display { }

impl Handle for usize { }

mod body;
mod state;
