use std::num::Float;

use core::{ Body, UID };
use space::{ Contact, Space };

/// A `Pair` object manages the relationship between two `Body` objects in close
/// proximity.
#[derive(Clone, Copy, Show)]
pub struct Pair {
    body_ids: [UID; 2],
}

impl Pair {
    /// Creates a new `Pair` object from the two input `Body` objects.
    pub fn new(body_0: &Body, body_1: &Body) -> Pair {
        Pair{ body_ids: [body_0.id(), body_1.id()] }
    }

    /// Computes the `Contact` between the `Body` and returns the result if any.
    pub fn compute_contact<S: Space>(&self, space: &S) -> Option<Contact> {
        let bodies: Vec<&Body> = space.get_bodies(vec!(self.body_ids[0], self.body_ids[1])).iter().map(|b| b.unwrap()).collect();
        let shapes = [bodies[0].shape(), bodies[1].shape()];
        let states = [bodies[0].state(), bodies[1].state()];
        let tolerance = shapes[0].surface_radius() + shapes[1].surface_radius();

        let rel_pos = states[1].position() - states[0].position();
        let dist_sq = rel_pos.length_sq();

        if dist_sq > tolerance*tolerance {
            return None;
        }

        let contact_normal = rel_pos.normalize();
        let contact_point = contact_normal * (dist_sq.sqrt() / 2.0);
        return Some(Contact {
            body_ids: [bodies[0].id(), bodies[1].id()],
            point: contact_point,
            normal: contact_normal,
        });
    }
}
