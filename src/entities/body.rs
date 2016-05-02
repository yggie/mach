use ID;
use maths::{Transform, UnitQuat, Vec3D};
use shapes::Shape;
use entities::{BodyRef, BodyRefMut, Form};

/// This trait describes a physical entity which has both geometric and spatial
/// configurations.
pub trait Body {
    fn downcast<'a>(&'a self) -> BodyRef<'a>;
    fn downcast_mut<'a>(&'a mut self) -> BodyRefMut<'a>;

    fn id(&self) -> ID;
    fn form(&self) -> &Form;
    fn form_mut(&mut self) -> &mut Form;

    /// Returns the `Shape` associated with the entity.
    fn shape(&self) -> &Shape {
        self.form().shape()
    }

    /// Returns the `Transform` associated with the entity.
    fn transform(&self) -> &Transform {
        self.form().transform()
    }

    fn translation(&self) -> &Vec3D {
        self.form().translation()
    }

    fn translation_mut(&mut self) -> &mut Vec3D {
        self.form_mut().translation_mut()
    }

    fn rotation(&self) -> UnitQuat {
        self.form().rotation()
    }

    fn rotation_mut(&mut self) -> &mut UnitQuat {
        self.form_mut().rotation_mut()
    }

    /// Returns the vertex at the given index for the entity.
    fn vertex(&self, index: usize) -> Vec3D {
        self.form().vertex(index)
    }
}
