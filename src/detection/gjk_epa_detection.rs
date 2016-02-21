#[cfg(test)]
#[path="../../tests/detection/gjk_epa_detection_test.rs"]
mod tests;

use shapes::Plane;
use entities::BodyHandle;
use detection::{ContactCache, ContactDetector, ContactEvent, ContactSet, Detection};

pub struct GjkEpaDetection;

impl GjkEpaDetection {
    pub fn new() -> GjkEpaDetection {
        GjkEpaDetection
    }
}

impl Detection for GjkEpaDetection {
    fn update(&mut self) {
        // do nothing
    }

    fn compute_contacts(&mut self, handle_0: &BodyHandle, handle_1: &BodyHandle) -> Option<ContactEvent> {
        let body_0 = handle_0.borrow();
        let body_1 = handle_1.borrow();
        let form_0 = body_0.form();
        let form_1 = body_1.form();

        ContactCache::new(form_0, form_1)
            .compute_contacts(form_0, form_1)
            .map(|intersection| {
                let point_on_plane = intersection.point() - intersection.normal() * intersection.penetration_depth();
                let contact_plane = Plane::from_point(&point_on_plane, intersection.normal());
                ContactEvent {
                    bodies: (body_0.id(), body_1.id()),
                    contact_set: ContactSet::new(contact_plane, vec!(intersection.point().clone())),
                }
            })
    }
}
