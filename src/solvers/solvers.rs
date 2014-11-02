//! Contains implementations of algorithms used to compute the forces acting on
//! `Body` instances.

use core::Database;
use collisions::Contact;

pub use self::naivesolver::naive_solver;

/// Solvers which compute the forces acting on each `Body` based on the provided
/// contact information.
pub type ForceSolver = fn(&mut Database, &Vec<Contact>);

mod naivesolver;
