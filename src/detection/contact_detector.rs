use entities::Body;
use detection::Intersection;

pub trait ContactDetector {
    fn compute_contacts(&mut self, &Body, &Body) -> Option<Intersection>;
}
