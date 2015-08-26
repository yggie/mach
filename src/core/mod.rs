//! The `core` module contains the core implementation of the engine’s logic.

mod rigid_body;
mod static_body;
mod state;
mod transform;

pub use self::rigid_body::RigidBody;
pub use self::static_body::StaticBody;
pub use self::state::State;
pub use self::transform::Transform;

/// A unique identifier used to uniquely identify entities in the engine.
pub type UID = u64;
