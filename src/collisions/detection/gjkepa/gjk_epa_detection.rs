#[cfg(test)]
#[path="../../../../tests/collisions/detection/gjkepa/gjk_epa_detection_test.rs"]
mod tests;

use {ID, Scalar, TOLERANCE};
use maths::{Approximations, Vec3D};
use utils::Handle;
use algorithms::{Execute, IterativeAlgorithm, PanicOnIteration};
use collisions::{CollisionBody, CollisionData, Contact, Detection};
use collisions::geometry::shapes::Shape;
use collisions::detection::gjkepa::{ContactTracker, EPA, GJKSimplex, MinkowskiDifference};

pub struct GJKEPADetection { }

impl GJKEPADetection {
    pub fn new() -> GJKEPADetection {
        GJKEPADetection { }
    }

    // TODO return Option<&mut ContactTracker> instead
    fn find_tracker_mut(&mut self, _id_0: ID, _id_1: ID) -> Option<ContactTracker> {
        None
    }

    fn create_tracker<B>(&mut self, body_0: &B, body_1: &B) -> ContactTracker where B: CollisionBody {
        ContactTracker::new(body_0.collision_data(), body_1.collision_data())
    }
}

impl<B> Detection<B> for GJKEPADetection where B: CollisionBody {
    fn update(&mut self) {
        // do nothing
    }

    fn compute_contacts(&mut self, handle_0: &Handle<B>, handle_1: &Handle<B>) -> Option<Contact<B>> {
        let body_0 = handle_0.borrow();
        let body_1 = handle_1.borrow();

        let mut tracker = self.find_tracker_mut(body_0.id(), body_1.id())
            .unwrap_or_else(|| self.create_tracker(&*body_0, &*body_1));

        GJK::using_simplex(tracker.simplex_mut(), body_0.collision_data(), body_1.collision_data())
            .panic_on_iteration(1000, "GJK failed to complete")
            .execute()
            .map(|simplex| {
                // TODO pass the MinkowskiDifference around
                EPA::new(simplex, body_0.collision_data(), body_1.collision_data())
                    .panic_on_iteration(1000, "EPA failed to complete")
                    .execute()
                    .compute_contact_set()
            })
            .map(|contact_set| {
                Contact::new(contact_set, Handle::clone(handle_0), Handle::clone(handle_1))
            })
    }
}

pub struct GJK<'a> {
    diff: MinkowskiDifference<'a>,
    simplex: &'a mut GJKSimplex,
    intersection_tolerance: Scalar,
    converged_success_result: Option<bool>,
}

impl<'a> GJK<'a> {
    pub fn using_simplex(simplex: &'a mut GJKSimplex, data_0: &'a CollisionData, data_1: &'a CollisionData) -> GJK<'a> {
        let diff = MinkowskiDifference(data_0, data_1);
        let intersection_tolerance = diff.0.shape().surface_radius() +
            diff.1.shape().surface_radius();

        GJK {
            diff: diff,
            simplex: simplex,
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
            .filter(|&(_not_on_plane_index, ref _plane, projection)| {
                projection > self.intersection_tolerance + TOLERANCE
            })
            // TODO find out if this would have issues with precision
            .max_by_key(|&(_not_on_plane_index, ref _plane, projection)| (projection / TOLERANCE) as i32);

        let (not_on_plane_index, plane) = match next_guess {
            Some((not_on_plane_index, plane, _projection)) => {
                (not_on_plane_index, plane)
            },

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

        *self.simplex.vertex_mut(not_on_plane_index) = new_support_point;
    }
}
