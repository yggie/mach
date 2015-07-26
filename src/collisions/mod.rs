//! The `collisions` subsystem is responsible for the static behaviour of the
//! physics engine. It contains subcomponents to handle storage, retrieval and
//! queries for physical bodies.

use shapes::Shape;
use materials::Material;
use math::{ Vector, Quaternion };
use core::{ Body, Handle, State };

pub use self::contact::{ Contact, ContactPair };
pub use self::simple_collisions::SimpleCollisions;
pub use self::narrowphase::NarrowPhase;

/// A `Collisions` component is responsible for the storage, retrieval and
/// querying of physical bodies in the simulation.
pub trait Collisions {
    /// The identifier used to dereference `Body` instances.
    type Identifier: Handle;

    /// Creates an instance of a `Body` from the given properties, returns an
    /// identifier which can be used to retrieve the `Body` at a later time.
    fn create_body<S: Shape, M: Material>(&mut self, S, M, State) -> Self::Identifier;

    /// Creates an instance of a `StaticBody` from the given properties, returns
    /// an identifier which can be used to retrieve the `StaticBody` at a later
    /// time.
    fn create_static_body<S: Shape, M: Material>(&mut self, S, M, Vector, Quaternion) -> Self::Identifier;

    /// Searches the data structure for a matching `Body` instance with the
    /// identifier specified and returns a reference to the `Body` if found.
    fn find_body(&self, Self::Identifier) -> Option<&Body<Self::Identifier>>;

    /// Searches the data structure for a matching `Body` instance with the
    /// identifier specified and returns a mutable reference to the `Body` if
    /// found.
    fn find_body_mut(&mut self, Self::Identifier) -> Option<&mut Body<Self::Identifier>>;

    /// Returns an iterator over unique `Body` instances managed by this object.
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=&Body<Self::Identifier>> + 'a>;

    /// Returns an iterator over unique `Body` instances managed by this object.
    /// This iterator allows mutation of the `Body` objects.
    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=&mut Body<Self::Identifier>> + 'a>;

    /// Computes all the contacts between bodies managed by this object.
    fn find_contacts(&self) -> Vec<Contact<Self::Identifier>>;
}

mod contact;
mod simple_collisions;

pub mod narrowphase;
