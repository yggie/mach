use entities::Form;
use detection::Intersection;

pub trait ContactDetector {
    fn compute_contacts(&mut self, &Form, &Form) -> Option<Intersection>;
}
