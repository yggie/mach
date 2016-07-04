use Scalar;
use dynamics::{DynamicBodyType, Integrator};
use collisions::{Contact, Narrowphase};

pub trait ConstraintSolver<I, N, T> where I: Integrator, N: Narrowphase {
    fn solve_with_contacts(&mut self, contacts: &Vec<Contact<N, DynamicBodyType<T>>>, integrator: &I, time_step: Scalar);
}
