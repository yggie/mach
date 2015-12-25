use entities::Body;
use detection::Intersection;

use detection::contact_detector::ContactDetector;

use super::polytope::Polytope;
use super::simplex_cache::SimplexCache;
use super::minkowski_difference::MinkowskiDifference;

pub struct ContactCache(SimplexCache);

impl ContactCache {
    pub fn new(body_0: &Body, body_1: &Body) -> ContactCache {
        let diff = MinkowskiDifference::new(body_0, body_1);

        return ContactCache(SimplexCache::new(&diff));
    }
}

impl ContactDetector for ContactCache {
    fn compute_contacts(&mut self, body_0: &Body, body_1: &Body) -> Option<Intersection> {
        let diff = MinkowskiDifference::new(body_0, body_1);

        return self.0.update_to_contain_origin(diff)
            .map(|simplex| {
                Polytope::new(simplex).compute_contact_points()
            });
    }
}
