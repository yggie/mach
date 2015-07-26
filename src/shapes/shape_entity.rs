use math::{ Vector, Quaternion };
use shapes::Shape;

/// This trait describes a physical entity which has both geometric and spatial
/// configurations.
pub trait ShapeEntity {
    /// Returns the `Shape` associated with the entity.
    fn shape(&self) -> &Shape;
    /// Returns the position of the entity.
    fn position(&self) -> Vector;
    /// Returns the rotation of the entity in the form of a quaternion.
    fn rotation(&self) -> Quaternion;
}
