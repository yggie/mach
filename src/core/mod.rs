//! The `core` module contains the core implementation of the engine’s logic.

mod rigid_body;
mod static_body;
mod state;
mod transform;

use std::rc::Rc;
use std::cell::RefCell;

pub use self::rigid_body::RigidBody;
pub use self::static_body::StaticBody;
pub use self::state::State;
pub use self::transform::Transform;

/// A unique identifier used to uniquely identify entities in the engine.
pub type UID = u64;

/// A shared pointer which gives access to the contained type instance.
pub type SharedCell<T> = Rc<RefCell<T>>;
