//! The `entities` module comprises of objects with a physical presence in the
//! simulation environment.

mod form;
mod material;
mod rigid_body;
mod body_params;
mod static_body;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod body;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod moveable;

pub use self::body::Body;
pub use self::form::Form;
pub use self::material::Material;
pub use self::moveable::Moveable;
pub use self::rigid_body::RigidBody;
pub use self::body_params::{BodyParams, ShapeDesc};
pub use self::static_body::StaticBody;
