use Scalar;
use dynamics::{DynamicBody, Integrator};
use collisions::Contact;

pub trait ConstraintSolver<I, T> where I: Integrator, T: DynamicBody {
    fn solve_with_contacts(&mut self, contacts: &Vec<Contact<T>>, integrator: &I, time_step: Scalar);
}
