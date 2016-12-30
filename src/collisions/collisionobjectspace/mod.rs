#[macro_use]
#[cfg(test)]
#[path="../../../tests/collisions/collisionobjectspace/collision_object_space_behaviour.rs"]
mod behaviours;

mod collision_object_space;
mod mach_collision_object_space;

pub use self::collision_object_space::CollisionObjectSpace;
pub use self::mach_collision_object_space::MachCollisionObjectSpace;
