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
pub struct Database {
    bodies: Vec<Body>,
}

impl Database {
    /// Creates a new empty `Database` instance.
    pub fn new() -> Database {
        Database{ bodies: Vec::new() }
    }

    /// Creates a new `Body` and store it in the database. The `Body` is
    /// instantiated with the `Shape`, `Property`, and `Transform`s provided.
    pub fn create_body<T: Shape, U: Property>(&mut self,
                                              shape: T,
                                              property: U,
                                              transform: Transform,
                                              derivative_transform: Transform) -> UID {
        let uid = self.bodies.len();
        self.bodies.push(Body::new_with_id(uid, box shape, box property, transform, derivative_transform));

        return uid;
    }

    /// Creates a new `Body` and store it in the database. The `Body` is
    /// instantiated with the `Shape`, and `Property` provided. The `Body` wil
    /// be instantiated as a stationary object.
    pub fn create_body_stationary<T: Shape, U: Property>(&mut self,
                                                         shape: T,
                                                         property: U) -> UID {
        let identity = Transform::new_identity();
        self.create_body(shape, property, identity, identity)
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

    /// Returns the matching `Body` for the provided `UID`.
    pub fn find_mut(&mut self, id: UID) -> Option<&mut Body> {
        for body in self.bodies.iter_mut() {
            if body.id() == id {
                return Some(body);
            }
        }

        return None;
    }

    /// Returns the matching pair of `Body` instances for the two given `UID`.
    pub fn find_pair_mut(&mut self, id_0: UID, id_1: UID) -> (Option<&mut Body>, Option<&mut Body>) {
        // (self.find_mut(id_0), self.find_mut(id_1))
        let mut option_0: Option<&mut Body> = None;
        let mut option_1: Option<&mut Body> = None;
        for body in self.bodies.iter_mut() {
            if body.id() == id_0 {
                option_0 = Some(body);
            } else if body.id() == id_1 {
                option_1 = Some(body);
            }
        }

        return (option_0, option_1);
    }

    pub fn each_body_mut(&mut self, callback: |&mut Body|) {
        for body in self.bodies.iter_mut() {
            callback(body);
        }
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
