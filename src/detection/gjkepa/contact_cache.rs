use entities::Form;
use detection::ContactSet;
use algorithms::{IterativeAlgorithmExecutor, PanicOnIteration};

use detection::gjkepa::{EPA, GJK};
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
        let algorithm = PanicOnIteration::new(
            GJK::new(&mut self.0, diff),
            1000,
            "GJK failed to complete within 1000 iterations",
        );

        return IterativeAlgorithmExecutor::execute(algorithm)
            .map(|simplex| {
                let algorithm = PanicOnIteration::new(
                    EPA::new(simplex),
                    1000,
                    "EPA failed to complete within 1000 iterations",
                );

                IterativeAlgorithmExecutor::execute(algorithm)
                    .compute_contact_points()
            });
    }
}
