use collisions::Contact;
use bodies::Body;

use std::rc::Rc;

#[cfg(test)]
#[path="../../tests/collisions/proximitypair_test.rs"]
mod tests;

/// Represents two bodies in close proximity.
pub struct ProximityPair<'a> {
    /// References to the two bodies.
    pub bodies: [Rc<Body<'a>>, ..2],
}

impl<'a> ProximityPair<'a> {

    /// Constructs a new ProximityPair object from two bodies.
    pub fn new<'a>(a: Rc<Body>, b: Rc<Body>) -> ProximityPair<'a> {
        ProximityPair{ bodies: [a, b] }
    }

    /// Returns true if the pair is in contact.
    pub fn in_contact(&self) -> bool {
        let mut did_contact = false;
        self.if_contact(|_| did_contact = true);
        return did_contact;
    }

    /// Conditionally executes the given function if a contact is present. The
    /// execution occurs synchronously.
    pub fn if_contact(&self, callback: |Contact|) {
        let shapes = [self.bodies[0].shape(), self.bodies[1].shape()];
        let transforms = [self.bodies[0].transform(), self.bodies[1].transform()];
        let tolerance = shapes[0].surface_radius() + shapes[1].surface_radius();

        let translation_diff = transforms[1].translation_vector() - transforms[0].translation_vector();
        let dist_sq = translation_diff.length_sq();
        if dist_sq < tolerance*tolerance {
            let contact_point = translation_diff.normalize().scale(dist_sq.sqrt() / 2.0);
            callback(Contact::new(self.bodies[0].clone(), self.bodies[1].clone(), contact_point));
        }
    }
}
