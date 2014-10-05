use core::{ Body, UID };
use math::Transform;
use shapes::Shape;
use properties::Property;

#[cfg(test)]
#[path="../../tests/unit/core/database_test.rs"]
mod tests;

/// The `Database` component is responsible for storing and retrieving bodies in
/// the engine. It’s main purpose is to generate a unique id for each body, and
/// allow access to the body via the generated id.
pub struct Database<'a> {
    bodies: Vec<Body<'a>>,
}

impl<'a> Database<'a> {
    /// Creates a new empty `Database` instance.
    pub fn new() -> Database<'a> {
        Database{ bodies: Vec::new() }
    }

    /// Creates a new `Body` and store it in the database. The `Body` is
    /// instantiated with the `Shape` and `Property` provided.
    pub fn create_body<T: Shape, U: Property>(&mut self, shape: T, property: U) -> UID {
        let ident = Transform::new_identity();
        let uid = self.bodies.len();
        self.bodies.push(Body::new_with_id(uid, box shape, box property, ident, ident));

        return uid;
    }

    /// Returns the number of `Body` instances stored in the `Database`.
    pub fn size(&self) -> uint {
        self.bodies.len()
    }

    /// Returns the matching `Body` for the provided `UID`.
    pub fn find(&self, id: UID) -> Option<&Body> {
        for body in self.bodies.iter() {
            if body.id() == id {
                return Some(body);
            }
        }

        return None;
    }

    /// Returns the matching `Body` instances for the provided `UID` pair.
    pub fn find_pair(&self, ids: [UID, ..2]) -> (Option<&Body>, Option<&Body>) {
        (self.find(ids[0]), self.find(ids[1]))
    }

    /// Loops over all unique `Body` instance pairs and runs the callback
    /// function once for each pair.
    pub fn each_body_pair(&self, callback: |&Body, &Body|) {
        let total = self.bodies.len();
        for i in range(0u, total) {
            for j in range(i + 1, total) {
                callback(&self.bodies[i], &self.bodies[j]);
            }
        }
    }
}
