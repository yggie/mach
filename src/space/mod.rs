//! The `space` subsystem is responsible for the static behaviour of the engine
//! It contains subcomponents to handle storage, retrieval and collision queries
//! for physical bodies.

#![unstable]

use std::slice::{ Iter, IterMut };

use shapes::Shape;
use materials::Material;
use core::{ Body, State, UID };

pub use self::contact::Contact;
pub use self::simple_space::SimpleSpace;

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
    /// Returns a mutable reference to the `Body` with the `UID` given.
    fn get_body_mut(&mut self, UID) -> Option<&mut Body>;
    /// Finds all matching `Body` objects with the `UID` specified and returns
    /// a mutable list of these `Body` objects.
    fn get_bodies_mut(&mut self, Vec<UID>) -> Vec<Option<&mut Body>>;
    /// Returns an iterator which iterates over unique `Body` instances in the
    /// `Space`.
    #[experimental]
    fn bodies(&self) -> Iter<Body>;
    /// Returns an iterator which iterates over unique `Body` instances in the
    /// `Space`. This iterator allows mutation of the `Body` objects.
    #[experimental]
    fn bodies_mut(&mut self) -> IterMut<Body>;
    /// Computes the contacts between bodies within the `Space`, caches the
    /// results so that the next time it looks it up will be faster.
    #[experimental]
    fn find_contacts(&self) -> Vec<Contact>;
}

#[experimental]
mod contact;
#[experimental]
mod simple_space;
