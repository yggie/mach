#[cfg(test)]
#[path="../../../../tests/collisions/detection/gjkepa/gjk_epa_detection_test.rs"]
mod tests;

use {ID, INFINITY, Scalar, TOLERANCE};
use maths::{Approximations, Vec3D};
use shapes::Shape;
use algorithms::{Execute, IterativeAlgorithm, PanicOnIteration};
use collisions::{BasicCollisionData, CollisionObject, Contact, Detection, NarrowphaseData};
use collisions::detection::gjkepa::{ContactTracker, EPA, GJKSimplex, MinkowskiDifference};

pub struct GJKEPADetection { }

impl GJKEPADetection {
    // TODO return Option<&mut ContactTracker> instead
    fn find_tracker_mut(&mut self, _id_0: ID, _id_1: ID) -> Option<ContactTracker> {
        None
    }

    fn create_tracker(&mut self, object_0: (ID, &BasicCollisionData), object_1: (ID, &BasicCollisionData)) -> ContactTracker {
        ContactTracker::new(object_0.1, object_1.1)
    }
}

impl<T> Detection<T> for GJKEPADetection where T: NarrowphaseData {
    fn update(&mut self) {
        // do nothing
    }

    fn compute_contacts(&mut self, object_0: &CollisionObject<T>, object_1: &CollisionObject<T>) -> Option<Contact<T>> {
        let data_0 = object_0.data.borrow();
        let data_1 = object_1.data.borrow();

        let mut tracker = self.find_tracker_mut(object_0.id, object_1.id)
            .unwrap_or_else(|| self.create_tracker((object_0.id, data_0.basic_data()), (object_1.id, data_1.basic_data())));

        GJK::using_simplex(tracker.simplex_mut(), data_0.basic_data(), data_1.basic_data())
            .panic_on_iteration(1000, "GJK failed to complete")
            .execute()
            .map(|simplex| {
                // TODO pass the MinkowskiDifference around
                EPA::new(simplex, data_0.basic_data(), data_1.basic_data())
                    .panic_on_iteration(1000, "EPA failed to complete")
                    .execute()
                    .compute_contact_set()
            })
            .map(|contact_set| {
                Contact::new(contact_set, CollisionObject::clone(object_0), CollisionObject::clone(object_1))
            })
    }
}

pub struct GJK<'a> {
    diff: MinkowskiDifference<'a>,
    simplex: &'a mut GJKSimplex,
    last_max_distance: Scalar,
    intersection_tolerance: Scalar,
    converged_success_result: Option<bool>,
}

impl<'a> GJK<'a> {
    pub fn using_simplex(simplex: &'a mut GJKSimplex, data_0: &'a BasicCollisionData, data_1: &'a BasicCollisionData) -> GJK<'a> {
        let diff = MinkowskiDifference(data_0, data_1);
        let intersection_tolerance = diff.0.shape().surface_radius() +
            diff.1.shape().surface_radius();

        GJK {
            diff: diff,
            simplex: simplex,
            last_max_distance: INFINITY,
            intersection_tolerance: intersection_tolerance,
            converged_success_result: None,
        }
    }
}

impl<'a> IterativeAlgorithm for GJK<'a> {
    type Result = Option<&'a GJKSimplex>;

    fn result(self) -> Self::Result {
        self.converged_success_result.and_then(move |success| {
            if success {
                Some(self.simplex as &GJKSimplex)
            } else {
                None
            }
        })
    }

    fn has_converged(&self) -> bool {
        self.converged_success_result.is_some()
    }

    fn next_iteration(&mut self) {
        if let Some(_result) = self.converged_success_result {
            return;
        }

        let next_guess = self.simplex.separating_planes_with_index_of_out_of_plane_point_iter()
            .map(|(not_on_plane_index, plane)| {
                let projection = plane.normal_projection_of_origin();

                (not_on_plane_index, plane, projection)
            })
            .filter(|&(_not_on_plane_index, ref _plane, projection)| projection > self.intersection_tolerance + TOLERANCE && projection < self.last_max_distance)
            // TODO find out if this would have issues with precision
            .min_by_key(|&(_not_on_plane_index, ref _plane, projection)| (projection / TOLERANCE) as i32)
            .map(|(not_on_plane_index, plane, _projection)| (not_on_plane_index, plane));

        let (not_on_plane_index, plane) = match next_guess {
            Some(data) => data,

            None => {
                self.converged_success_result = Some(true);
                return;
            },
        };

        let new_support_point = self.diff.support_point(Vec3D::from(plane.normal()));

        if !plane.normal_projection_of(new_support_point).is_strictly_positive() {
            self.converged_success_result = Some(false);
            return;
        }

        self.simplex.vertices[not_on_plane_index] = new_support_point;
    }
}
