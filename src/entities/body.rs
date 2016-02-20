use maths::{Transform, Vect};
use shapes::Shape;
use entities::{BodyType, Form};

/// This trait describes a physical entity which has both geometric and spatial
/// configurations.
pub trait Body {
    fn downcast<'a>(&'a self) -> BodyType<'a>;
    // fn downcast_mut<'a>(&'a mut self) -> BodyTypeMut<'a>;

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
        self.form().vertex(index)
    }
}
