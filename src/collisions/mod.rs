mod collision_data;
mod collision_space;
mod collision_object;
mod basic_collision_data;

pub mod shapes;
pub mod geometry;
pub mod detection;
pub mod broadphase;
pub mod narrowphase;
pub mod collisionobjectspace;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod collision_object_lifecycle_event_listener;

pub use self::geometry::SupportMap;
pub use self::detection::{Contact, Detection};
pub use self::broadphase::{Broadphase, BruteForceBroadphase};
pub use self::narrowphase::{Narrowphase, NarrowphaseData};
pub use self::collision_data::CollisionData;
pub use self::collision_space::CollisionSpace;
pub use self::collision_object::CollisionObject;
pub use self::basic_collision_data::BasicCollisionData;
pub use self::collisionobjectspace::{CollisionObjectSpace, MachCollisionObjectSpace};
pub use self::collision_object_lifecycle_event_listener::CollisionObjectLifecycleEventListener;

use utils::DataHandle;

pub type CollisionDataHandle<T> = DataHandle<CollisionData<T>>;
