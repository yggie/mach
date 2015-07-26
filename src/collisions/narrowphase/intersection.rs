use math::Vector;

/// This object contains information about the intersection between two
/// entities.
pub struct Intersection(Vector, Vector);

impl Intersection {
    /// Creates a new `Intersection` instance.
    pub fn new(center: Vector, normal: Vector) -> Intersection {
        Intersection(center, normal)
    }

    /// Returns the center of the intersection.
    pub fn point(&self) -> Vector {
        self.0
    }

    /// Returns the normal vector associated with the intersection.
    pub fn normal(&self) -> Vector {
        self.1
    }
}
