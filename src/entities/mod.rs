//! The `entities` module comprises of objects with a physical presence in the
//! simulation environment.

#[macro_use] mod form;

mod body_type;
mod mach_store;
mod rigid_body;
mod body_handle;
mod static_body;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod body;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod entity_store;

pub use self::body::Body;
pub use self::form::Form;
pub use self::body_type::{BodyType, BodyTypeMut};
pub use self::mach_store::MachStore;
pub use self::rigid_body::RigidBody;
pub use self::body_handle::{BodyHandle, Rc, Ref, RefMut};
pub use self::static_body::StaticBody;
pub use self::entity_store::EntityStore;
