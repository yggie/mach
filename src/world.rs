use std::cell::Ref;

use {EntityDesc, ID, Scalar};
use maths::{State, Vector};
use shapes::Shape;
use entities::{Material, RigidBody};

/// The `World` trait should be implemented by objects capable of behaving as a
/// physics engine.
pub trait World {
    /// Creates an instance of a `RigidBody` from the given components, returns
    /// a handle which can later be used to retrieve the `RigidBody`.
    fn create_body<S: Shape>(&mut self, shape: S, material: &Material, state: State) -> ID;

    /// Creates an instance of a `StaticBody` with the properties from the
    /// `EntityDesc` provided. Returns a unique identifier bound to the new
    /// instance.
    fn create_static_body(&mut self, entity_desc: &EntityDesc) -> ID;

    /// Searches the world for a matching `RigidBody` instance with the
    /// identifier specified and returns a reference to the `RigidBody` if
    /// found.
    fn find_body(&self, id: ID) -> Option<Ref<RigidBody>>;

    /// Returns an iterator over unique `RigidBody` instances in the `World`.
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a>;

    /// Steps the `World` forward in time by the specified amount.
    fn update(&mut self, time_step: Scalar);

    /// Returns the value of the global gravity `Vector` set in the `World`.
    fn gravity(&self) -> Vector;

    /// Changes the global gravitational force acting on `RigidBody` objects.
    fn set_gravity(&mut self, gravity: Vector);
}
