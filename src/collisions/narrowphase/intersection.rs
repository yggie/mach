use Scalar;
use maths::Vector;

/// This object contains information about the intersection between two
/// entities.
pub struct Intersection(Vector, Vector, Scalar);

impl Intersection {
    /// Creates a new `Intersection` instance.
    pub fn new(center: Vector, normal: Vector, penetration_depth: Scalar) -> Intersection {
        Intersection(center, normal, penetration_depth)
    }

    /// Returns the center of the intersection.
    pub fn point(&self) -> &Vector {
        &self.0
    }

    /// Returns the normal vector associated with the intersection.
    pub fn normal(&self) -> &Vector {
        &self.1
    }

    pub fn penetration_depth(&self) -> Scalar {
        self.2
    }
}
