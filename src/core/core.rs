//! The `core` module contains the core implementation of the engine’s logic.

pub use self::body::Body;
pub use self::database::Database;
pub use self::state::State;

/// A unique identifier primarily used to identify a `Body` in a `Database`.
pub type UID = uint;

mod body;
mod database;
mod state;
