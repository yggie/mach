#[macro_use]
#[cfg(test)]
#[path="../../../tests/collisions/collisionobjectspace/collision_object_space_behaviour.rs"]
mod behaviours;

mod mach_collision_object_space;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod collision_object_space;

pub use self::collision_object_space::CollisionObjectSpace;
pub use self::mach_collision_object_space::MachCollisionObjectSpace;
