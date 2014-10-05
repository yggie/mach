//! Contains implementations of solvers to compute solutions to resolve forces
//! and perform time marching.

use core::Database;
use collisions::Contact;

/// Solvers which compute the forces acting on each `Body` based on the provided
/// contact information.
pub type ForceSolver = fn(&Database, &Vec<Contact>);

/// Contains implementations of the `ForceSolver` type.
pub mod force {
    pub use self::naive::naive_solver;

    mod naive;
}