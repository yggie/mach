mod material_data;
mod fixed_body_ref;
mod rigid_body_ref;
mod fixed_body_data;
mod rigid_body_data;
mod dynamic_body_ref;
mod dynamic_body_type;

pub mod solvers;
pub mod integrators;

pub use self::solvers::ConstraintSolver;
pub use self::integrators::{Integratable, Integrator};
pub use self::material_data::MaterialData;
pub use self::fixed_body_ref::{FixedBodyRef, FixedBodyRefMut};
pub use self::rigid_body_ref::{RigidBodyRef, RigidBodyRefMut};
pub use self::fixed_body_data::FixedBodyData;
pub use self::rigid_body_data::RigidBodyData;
pub use self::dynamic_body_ref::{DynamicBodyRef, DynamicBodyRefMut};
pub use self::dynamic_body_type::DynamicBodyType;

use collisions::Body;

pub type DynamicBody<N, T> = Body<N, DynamicBodyType<T>>;
