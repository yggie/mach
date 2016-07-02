mod body_ref;
mod fixed_body;
mod rigid_body;
mod body_ref_mut;
mod dynamic_data;
mod material_data;
mod fixed_body_data;
mod rigid_body_data;
mod mach_physics_object_space;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod body;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod physics_object_space;

pub use self::body::Body;
pub use self::body_ref::BodyRef;
pub use self::fixed_body::FixedBody;
pub use self::rigid_body::RigidBody;
pub use self::body_ref_mut::BodyRefMut;
pub use self::dynamic_data::DynamicData;
pub use self::material_data::MaterialData;
pub use self::fixed_body_data::FixedBodyData;
pub use self::rigid_body_data::RigidBodyData;
pub use self::physics_object_space::PhysicsObjectSpace;
pub use self::mach_physics_object_space::MachPhysicsObjectSpace;

use utils::DataHandle;

pub type FixedBodyHandle<T> = DataHandle<FixedBody<T>>;
pub type RigidBodyHandle<T> = DataHandle<RigidBody<T>>;
