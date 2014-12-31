use core::{ Body, UID };
use math::Vector;
use space::Space;

/// Represents a point of contact between two physical entities. Holds
/// references to the contacting bodies and a point of contact.
#[deriving(Copy)]
pub struct Contact {
    /// References to the two bodies.
    pub body_ids: [UID, ..2],
    /// The point of contact.
    pub point: Vector,
    /// The contact normal.
    pub normal: Vector,
}

impl Contact {
    /// Dereferences the contacting `Body` objects.
    pub fn deref_bodies<'a, S: Space>(&'a self, space: &'a mut S) -> (Option<&mut Body>, Option<&mut Body>) {
        let mut bodies = space.find_bodies_mut(vec!(self.body_ids[0], self.body_ids[1]));
        let body_1 = bodies.pop().unwrap();
        let body_0 = bodies.pop().unwrap();

        return (body_0, body_1);
    }
}
