use std::cell::{Ref, RefMut};

use ID;
use entities::{BodyParams, Form, RigidBody, StaticBody};
use detection::{Contact, Intersection};

/// A `Space` component is responsible for the storage, retrieval and
/// querying of physical bodies in the simulation.
pub trait Space {
    /// Creates an instance of a `RigidBody` with the properties from the
    /// `BodyParams` provided. Returns a unique identifier bound to the new
    /// instance.
    fn create_body(&mut self, &BodyParams) -> ID;

    /// Creates an instance of a `StaticBody` with the properties from the
    /// `BodyParams` provided. Returns a unique identifier bound to the new
    /// instance.
    fn create_static_body(&mut self, &BodyParams) -> ID;

    /// Searches the data structure for a matching `RigidBody` instance with the
    /// identifier specified and returns a reference to the `RigidBody` if found.
    fn find_body(&self, ID) -> Option<Ref<RigidBody>>;

    /// Returns the `StaticBody` instance associated with the identifier
    /// provided.
    fn find_static_body(&self, ID) -> Option<Ref<StaticBody>>;

    /// Searches the data structure for a matching `RigidBody` instance with the
    /// identifier specified and returns a mutable reference to the `RigidBody`
    /// if found.
    fn find_body_mut(&mut self, ID) -> Option<RefMut<RigidBody>>;

    /// Searches the data structure for a matching `StaticBody` instance with
    /// the identifier specified and returns a mutable reference to the
    /// `StaticBody` if found.
    fn find_static_body_mut(&mut self, ID) -> Option<RefMut<StaticBody>>;

    /// Returns an iterator over unique `RigidBody` instances managed by this
    /// object.
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<RigidBody>> + 'a>;

    /// Returns an iterator over unique `StaticBody` instances managed by this
    /// object.
    fn static_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<StaticBody>> + 'a>;

    /// Returns an iterator over unique `RigidBody` instances managed by this
    /// object.  This iterator allows mutation of the `RigidBody` objects.
    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<RigidBody>> + 'a>;

    /// Finds the intersection between two volumetric bodies.
    // TODO: needs refactoring: are there alternatives to not exposing this function?
    fn find_intersection(&self, &Form, &Form) -> Option<Intersection>;

    /// Finds all contacts between bodies.
    // TODO test it?
    fn find_contacts(&self) -> Option<Vec<Contact>>;
}
