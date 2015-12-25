//! The `entities` module comprises of objects with a physical presence in the
//! simulation environment.

mod material;
mod rigid_body;
mod static_body;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod body;

pub use self::body::Body;
pub use self::material::Material;
pub use self::rigid_body::RigidBody;
pub use self::static_body::StaticBody;
