#[cfg(test)]
#[path="../../tests/detection/gjk_epa_detection_test.rs"]
mod tests;

use std::marker::PhantomData;

use ID;
use shapes::Plane;
use entities::EntityStore;
use detection::{ContactCache, ContactDetector, ContactEvent, ContactSet, Detection};

pub struct GjkEpaDetection<ES: EntityStore> {
    _phantom: PhantomData<ES>,
}

impl<ES> GjkEpaDetection<ES> where ES: EntityStore {
    pub fn new() -> GjkEpaDetection<ES> {
        GjkEpaDetection {
            _phantom: PhantomData,
        }
    }
}

impl<ES> Detection for GjkEpaDetection<ES> where ES: EntityStore {
    type EntityStore = ES;

    fn update(&mut self) {
        // do nothing
    }

    fn compute_contacts(&mut self, entity_store: &Self::EntityStore, id_0: ID, id_1: ID) -> Option<ContactEvent> {
        let body_0 = entity_store.find_body(id_0).unwrap();
        let body_1 = entity_store.find_body(id_1).unwrap();
        let form_0 = body_0.form();
        let form_1 = body_1.form();

        ContactCache::new(form_0, form_1)
            .compute_contacts(form_0, form_1)
            .map(|intersection| {
                let point_on_plane = intersection.point() - intersection.normal() * intersection.penetration_depth();
                let contact_plane = Plane::from_point(&point_on_plane, intersection.normal());
                ContactEvent {
                    bodies: (id_0, id_1),
                    contact_set: ContactSet::new(contact_plane, vec!(intersection.point().clone())),
                }
            })
    }
}
