mod mach_constraint_solver;

pub use self::mach_constraint_solver::MachConstraintSolver;

use Scalar;
use collisions::{Contact, NarrowphaseData};

pub trait ConstraintSolver<T> where T: NarrowphaseData {
    fn solve_with_contacts(&mut self, contacts: &Vec<Contact<T>>, time_step: Scalar);
}
