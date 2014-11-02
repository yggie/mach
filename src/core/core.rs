//! The `core` module contains the core implementation of the engine’s logic.

pub use self::body::Body;
pub use self::state::State;
pub use self::world::World;
pub use self::database::Database;

/// A unique identifier primarily used to identify a `Body` in a `Database`.
pub type UID = uint;

mod body;
mod database;
mod state;
mod world;
