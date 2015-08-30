use std::cell::{ Ref, RefMut };

use core::{ RigidBody, UID, State, StaticBody, Transform };
use shapes::Shape;
use materials::Material;
use collisions::Contact;

/// A `CollisionSpace` component is responsible for the storage, retrieval and
/// querying of physical bodies in the simulation.
pub trait CollisionSpace {
    /// Creates an instance of a `RigidBody` from the given properties, returns
    /// an identifier which can be used to retrieve the `RigidBody` at a later
    /// time.
    fn create_body<S: Shape, M: Material>(&mut self, S, M, State) -> UID;

    /// Creates an instance of a `StaticBody` from the given properties, returns
    /// an identifier which can be used to retrieve the `StaticBody` at a later
    /// time.
    fn create_static_body<S: Shape, M: Material>(&mut self, S, M, Transform) -> UID;

    /// Searches the data structure for a matching `RigidBody` instance with the
    /// identifier specified and returns a reference to the `RigidBody` if found.
    fn find_body(&self, UID) -> Option<Ref<RigidBody>>;

    /// Returns the `StaticBody` instance associated with the identifier
    /// provided.
    fn find_static_body(&self, UID) -> Option<Ref<StaticBody>>;

    /// Searches the data structure for a matching `RigidBody` instance with the
    /// identifier specified and returns a mutable reference to the `RigidBody`
    /// if found.
    fn find_body_mut(&mut self, UID) -> Option<RefMut<RigidBody>>;

    /// Searches the data structure for a matching `StaticBody` instance with
    /// the identifier specified and returns a mutable reference to the
    /// `StaticBody` if found.
    fn find_static_body_mut(&mut self, UID) -> Option<RefMut<StaticBody>>;

    /// Returns an iterator over unique `RigidBody` instances managed by this
    /// object.
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a>;

    /// Returns an iterator over unique `StaticBody` instances managed by this
    /// object.
    fn static_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<StaticBody>> + 'a>;

    /// Returns an iterator over unique `RigidBody` instances managed by this
    /// object.  This iterator allows mutation of the `RigidBody` objects.
    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<RigidBody>> + 'a>;

    /// Finds all contacts between bodies.
    fn find_contacts(&self) -> Option<Vec<Contact>>;
}
