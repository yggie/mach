//! The `collisions` subsystem is responsible for the static behaviour of the
//! physics engine. It contains subcomponents to handle storage, retrieval and
//! queries for physical bodies.

use shapes::Shape;
use materials::Material;
use core::{ Body, UID, State, StaticBody, Transform };

pub use self::contact::{ Contact, Constraint };
pub use self::simple_collisions::SimpleCollisions;
pub use self::narrowphase::NarrowPhase;

/// A `Collisions` component is responsible for the storage, retrieval and
/// querying of physical bodies in the simulation.
pub trait Collisions {
    /// Creates an instance of a `Body` from the given properties, returns an
    /// identifier which can be used to retrieve the `Body` at a later time.
    fn create_body<S: Shape, M: Material>(&mut self, S, M, State) -> UID;

    /// Creates an instance of a `StaticBody` from the given properties, returns
    /// an identifier which can be used to retrieve the `StaticBody` at a later
    /// time.
    fn create_static_body<S: Shape, M: Material>(&mut self, S, M, Transform) -> UID;

    /// Searches the data structure for a matching `Body` instance with the
    /// identifier specified and returns a reference to the `Body` if found.
    fn find_body(&self, UID) -> Option<&Body>;

    /// Returns the `StaticBody` instance associated with the identifier
    /// provided.
    fn find_static_body(&self, UID) -> Option<&StaticBody>;

    /// Searches the data structure for a matching `Body` instance with the
    /// identifier specified and returns a mutable reference to the `Body` if
    /// found.
    fn find_body_mut(&mut self, UID) -> Option<&mut Body>;

    /// Searches the data structure for a matching `StaticBody` instance with
    /// the identifier specified and returns a mutable reference to the
    /// `StaticBody` if found.
    fn find_static_body_mut(&mut self, UID) -> Option<&mut StaticBody>;

    /// Returns an iterator over unique `Body` instances managed by this object.
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=&Body> + 'a>;

    /// Returns an iterator over unique `StaticBody` instances managed by this object.
    fn static_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=&StaticBody> + 'a>;

    /// Returns an iterator over unique `Body` instances managed by this object.
    /// This iterator allows mutation of the `Body` objects.
    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=&mut Body> + 'a>;

    /// Computes all the contacts between bodies managed by this object.
    fn find_contacts(&self) -> Option<Vec<Contact>>;
}

mod contact;
mod simple_collisions;

pub mod narrowphase;
