//! The `core` module contains the core implementation of the engine’s logic.

pub use self::body::Body;
pub use self::state::State;
pub use self::world::World;

/// A unique identifier primarily used to identify a `Body` in a `Database`.
pub type UID = usize;

mod body;
mod state;
mod world;
