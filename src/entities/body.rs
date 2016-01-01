use maths::{Transform, Vect};
use shapes::Shape;
use entities::Form;

/// This trait describes a physical entity which has both geometric and spatial
/// configurations.
pub trait Body {
    fn form(&self) -> &Form;

    /// Returns the `Shape` associated with the entity.
    fn shape(&self) -> &Shape {
        self.form().shape()
    }

    /// Returns the `Transform` associated with the entity.
    fn transform(&self) -> &Transform {
        self.form().transform()
    }

    fn translation(&self) -> &Vect {
        &self.transform().translation
    }

    /// Returns the vertex at the given index for the entity.
    fn vertex(&self, index: usize) -> Vect {
        self.transform().apply_to_point(self.shape().vertex(index))
    }
}
