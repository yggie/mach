#[cfg(test)]
#[path="../../tests/detection/gjk_epa_detection_test.rs"]
mod tests;

use entities::BodyHandle;
use detection::{ContactEvent, Detection};
use detection::gjkepa::ContactCache;

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
            .map(|contact_set| {
                ContactEvent::new((handle_0.clone(), handle_1.clone()), contact_set)
            })
    }
}
