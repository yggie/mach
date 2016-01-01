use std::cell::Ref;

use {ID, Scalar};
use maths::Vect;
use entities::{BodyParams, RigidBody};
use detection::Contact;

/// The `World` trait should be implemented by objects capable of behaving as a
/// physics engine.
pub trait World {
    /// Creates an instance of a `RigidBody` with the properties from the
    /// `BodyParams` provided. Returns a unique identifier bound to the new
    /// instance.
    fn create_rigid_body(&mut self, &BodyParams) -> ID;

    /// Creates an instance of a `StaticBody` with the properties from the
    /// `BodyParams` provided. Returns a unique identifier bound to the new
    /// instance.
    fn create_static_body(&mut self, &BodyParams) -> ID;

    /// Searches the world for a matching `RigidBody` instance with the
    /// identifier specified and returns a reference to the `RigidBody` if
    /// found.
    fn find_rigid_body(&self, id: ID) -> Option<Ref<RigidBody>>;

    /// Returns an iterator over unique `RigidBody` instances in the `World`.
    fn rigid_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a>;

    /// Steps the `World` forward in time by the specified amount.
    fn update(&mut self, time_step: Scalar) -> Option<Vec<Contact>>;

    /// Returns the value of the global gravity `Vect` set in the `World`.
    fn gravity(&self) -> Vect;

    /// Changes the global gravitational force acting on `RigidBody` objects.
    fn set_gravity(&mut self, gravity: Vect);
}
