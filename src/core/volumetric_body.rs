use core::Transform;
use maths::Vector;
use shapes::Shape;

/// This trait describes a physical entity which has both geometric and spatial
/// configurations.
pub trait VolumetricBody {
    /// Returns the `Shape` associated with the entity.
    fn shape(&self) -> &Shape;

    /// Returns the `Transform` associated with the entity.
    fn transform(&self) -> Transform;

    /// Returns the vertex at the given index for the entity.
    fn vertex(&self, index: usize) -> Vector {
        self.transform().apply_to_point(self.shape().vertex(index))
    }
}
