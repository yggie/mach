use entities::Form;
use detection::ContactSet;

use super::polytope::Polytope;
use super::simplex_cache::SimplexCache;
use super::minkowski_difference::MinkowskiDifference;

pub struct ContactCache(SimplexCache);

impl ContactCache {
    pub fn new(form_0: &Form, form_1: &Form) -> ContactCache {
        let diff = MinkowskiDifference(form_0, form_1);

        return ContactCache(SimplexCache::new(&diff));
    }

    pub fn compute_contacts(&mut self, form_0: &Form, form_1: &Form) -> Option<ContactSet> {
        let diff = MinkowskiDifference(form_0, form_1);

        return self.0.update_to_contain_origin(diff)
            .map(|simplex| {
                Polytope::new(simplex).compute_contact_points()
            });
    }
}
