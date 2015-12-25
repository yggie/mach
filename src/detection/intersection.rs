use Scalar;
use maths::Vect;

/// This object contains information about the intersection between two
/// entities.
pub struct Intersection(Vect, Vect, Scalar);

impl Intersection {
    /// Creates a new `Intersection` instance.
    pub fn new(center: Vect, normal: Vect, penetration_depth: Scalar) -> Intersection {
        Intersection(center, normal, penetration_depth)
    }

    /// Returns the center of the intersection.
    pub fn point(&self) -> &Vect {
        &self.0
    }

    /// Returns the normal vector associated with the intersection.
    pub fn normal(&self) -> &Vect {
        &self.1
    }

    pub fn penetration_depth(&self) -> Scalar {
        self.2
    }
}
