mod body;
mod body_def;
mod body_data;
mod collision_data;
mod collision_group;
// mod collision_space;

pub mod shapes;
pub mod geometry;
pub mod detection;
pub mod broadphase;
pub mod narrowphase;
pub mod collisionobjectspace;

pub use self::body::Body;
pub use self::body_def::BodyDef;
pub use self::body_data::BodyData;
pub use self::geometry::SupportMap;
pub use self::detection::{Contact, Detection};
pub use self::broadphase::{Broadphase, CloseProximityPair};
pub use self::narrowphase::Narrowphase;
pub use self::collision_data::CollisionData;
pub use self::collision_group::CollisionGroup;
// pub use self::collision_space::CollisionSpace;
pub use self::collisionobjectspace::{CollisionObjectSpace, MachCollisionObjectSpace};

use utils::DataHandle;

pub type BodyHandle<D, N> = DataHandle<Body<D, N>>;
