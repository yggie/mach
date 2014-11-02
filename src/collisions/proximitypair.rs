use collisions::Contact;
use core::{ Body, UID };

#[cfg(test)]
#[path="../../tests/unit/collisions/proximitypair_test.rs"]
mod tests;

/// Represents two bodies in close proximity.
pub struct ProximityPair {
    /// References to the two bodies.
    pub body_ids: [UID, ..2],
}

impl ProximityPair {

    /// Constructs a new ProximityPair object from two bodies.
    pub fn new(body_0: &Body, body_1: &Body) -> ProximityPair {
        ProximityPair{ body_ids: [body_0.id(), body_1.id()] }
    }

    /// Computes the contact point and optionally returns the value if present.
    pub fn compute_contact<'a>(&mut self, body_0: &Body, body_1: &Body) -> Option<Contact<'a>> {
        let shapes = [body_0.shape(), body_1.shape()];
        let states = [body_0.state(), body_1.state()];
        let tolerance = shapes[0].surface_radius() + shapes[1].surface_radius();

        let rel_pos = states[1].position() - states[0].position();
        let dist_sq = rel_pos.length_sq();

        if dist_sq < tolerance*tolerance {
            let contact_normal = rel_pos.normalize();
            let contact_point = contact_normal * (dist_sq.sqrt() / 2.0);

            return Some(Contact {
                body_ids: self.body_ids,
                point: contact_point,
                normal: contact_normal,
            });
        } else {
            return None;
        }
    }
}
