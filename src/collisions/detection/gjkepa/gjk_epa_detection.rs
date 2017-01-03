#[cfg(test)]
#[path="../../../../tests/collisions/detection/gjkepa/gjk_epa_detection_test.rs"]
mod tests;

use ID;
use utils::Handle;
use algorithms::{Execute, PanicOnIteration};
use collisions::{CollisionObject, Contact, Detection};
use collisions::detection::gjkepa::{ContactTracker, GJK, EPA};

pub struct GJKEPADetection { }

impl GJKEPADetection {
    pub fn new() -> GJKEPADetection {
        GJKEPADetection { }
    }

    // TODO return Option<&mut ContactTracker> instead
    fn find_tracker_mut(&mut self, _id_0: ID, _id_1: ID) -> Option<ContactTracker> {
        None
    }

    fn create_tracker<O>(&mut self, body_0: &O, body_1: &O) -> ContactTracker where O: CollisionObject {
        ContactTracker::new(body_0.collision_data(), body_1.collision_data())
    }
}

impl<O> Detection<O> for GJKEPADetection where O: CollisionObject {
    fn update(&mut self) {
        // do nothing
    }

    fn compute_contacts(&mut self, handle_0: &Handle<O>, handle_1: &Handle<O>) -> Option<Contact<O>> {
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
