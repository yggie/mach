use Scalar;
use detection::ContactEvent;

pub trait ConstraintSolver {
    fn solve_with_contacts(&mut self, time_step: Scalar, contact_events: &Vec<ContactEvent>);
}
