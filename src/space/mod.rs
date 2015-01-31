//! The `space` subsystem is responsible for the static behaviour of the engine
//! It contains subcomponents to handle storage, retrieval and collision queries
//! for physical bodies.

#![unstable]

use shapes::Shape;
use materials::Material;
use core::{ Body, State, UID };

pub use self::contact::Contact;
pub use self::simple_space::SimpleSpace;
pub use self::narrowphase::pair::Pair;

#[macro_use]
#[cfg(test)]
#[path="../../tests/behaviours/space_behaviour.rs"]
mod behaviours;

/// The `Space` component is responsible for the storage, retrieval and querying
/// of physical bodies in the simulation.
pub trait Space {
    /// Creates an instance of a `Body` from the given properties, returns a
    /// handle which can be used to retrieve the `Body` from the `Space`.
    fn create_body<S: Shape, M: Material>(&mut self, S, M, State) -> UID;
    /// Returns a reference to the `Body` with the `UID` given.
    fn get_body(&self, UID) -> Option<&Body>;
    /// Finds all matching `Body` objects with the `UID` specified and returns
    /// them.
    fn get_bodies(&self, Vec<UID>) -> Vec<Option<&Body>>;
    /// Returns a mutable reference to the `Body` with the `UID` given.
    fn get_body_mut(&mut self, UID) -> Option<&mut Body>;
    /// Finds all matching `Body` objects with the `UID` specified and returns
    /// a mutable list of these `Body` objects.
    fn get_bodies_mut(&mut self, Vec<UID>) -> Vec<Option<&mut Body>>;
    /// Returns an iterator which iterates over unique `Body` instances in the
    /// `Space`.
    fn bodies_iter(&self) -> Box<Iterator<Item=&Body>>;
    /// Returns an iterator which iterates over unique `Body` instances in the
    /// `Space`. This iterator allows mutation of the `Body` objects.
    fn bodies_iter_mut(&mut self) -> Box<Iterator<Item=&mut Body>>;
    /// Computes the contacts between bodies within the `Space`, caches the
    /// results so that the next time it looks it up will be faster.
    fn find_contacts(&self) -> Vec<Contact>;
}

mod contact;
mod simple_space;

mod narrowphase {
    pub mod pair;
}
