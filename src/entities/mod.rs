//! The `entities` module comprises of objects with a physical presence in the
//! simulation environment.

mod rigid_body;
mod static_body;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod volumetric_body;

pub use self::rigid_body::RigidBody;
pub use self::static_body::StaticBody;
pub use self::volumetric_body::VolumetricBody;
