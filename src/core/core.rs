//! The bodies module defines objects representing physical entities.

pub use self::body::Body;
pub use self::database::Database;

/// A unique identifier primarily used to identify a `Body` in a `Database`.
pub type UID = uint;

mod body;
mod database;
