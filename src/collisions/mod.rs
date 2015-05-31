//! The `collisions` subsystem is responsible for the static behaviour of the
//! physics engine. It contains subcomponents to handle storage, retrieval and
//! queries for physical bodies.

use shapes::Shape;
use materials::Material;
use core::{ Body, State, UID };

pub use self::contact::Contact;
pub use self::simple_collisions::SimpleCollisions;
pub use self::narrowphase::proximity::Proximity;

/// A `Collisions` component is responsible for the storage, retrieval and
/// querying of physical bodies in the simulation.
pub trait Collisions {

    /// Creates an instance of a `Body` from the given properties, returns a
    /// handle which can be used to retrieve the `Body` at a later time.
    fn create_body<S: Shape, M: Material>(&mut self, S, M, State) -> UID;

    /// Searches the data structure for a matching `Body` instance with the
    /// `UID` specified and returns a reference to the `Body` if found.
    fn find_body(&self, UID) -> Option<&Body<UID>>;

    /// Searches the data structure for a matching `Body` instance with the
    /// `UID` specified and returns a mutable reference to the `Body` if found.
    fn find_body_mut(&mut self, UID) -> Option<&mut Body<UID>>;

    // TODO is there a safe way to do this?
    // /// Finds all matching `Body` objects with the `UID` specified and returns
    // /// a mutable list of these `Body` objects.
    // fn get_bodies_mut(&mut self, Vec<UID>) -> Vec<Option<&mut Body<UID>>>;

    /// Returns an iterator over unique `Body` instances managed by this object.
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=&Body<UID>> + 'a>;

    /// Returns an iterator over unique `Body` instances managed by this object.
    /// This iterator allows mutation of the `Body` objects.
    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=&mut Body<UID>> + 'a>;

    /// Computes all the contacts between bodies managed by this object.
    fn find_contacts(&self) -> Vec<Contact<UID>>;
}

mod contact;
mod simple_collisions;

mod narrowphase {
    pub mod proximity;
}
