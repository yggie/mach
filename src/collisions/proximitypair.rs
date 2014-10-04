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

    /// Computes the contact point and optionally returns the value if present.
    pub fn compute_contact(&self) -> Option<Contact<'a>> {
        let shapes = [self.bodies[0].shape(), self.bodies[1].shape()];
        let transforms = [self.bodies[0].transform(), self.bodies[1].transform()];
        let tolerance = shapes[0].surface_radius() + shapes[1].surface_radius();

        let translation_diff = transforms[1].translation_vector() - transforms[0].translation_vector();
        let dist_sq = translation_diff.length_sq();

        if dist_sq < tolerance*tolerance {
            let contact_normal = translation_diff.normalize();
            let contact_point = contact_normal.scale(dist_sq.sqrt() / 2.0);

            return Some(Contact {
                bodies: [self.bodies[0].clone(), self.bodies[1].clone()],
                point: contact_point,
                normal: contact_normal,
            });
        } else {
            return None;
        }
    }
}
