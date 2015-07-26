use core::{ Handle, Transform };
use math::{ Vector, Quaternion };
use shapes::{ Shape, ShapeEntity };
use materials::Material;

/// Represents a physical entity which cannot move. Within the engine, the
/// object is simply treated as if it has infinite mass.
pub struct StaticBody<H: Handle> {
    id: H,
    shape: Box<Shape>,
    material: Box<Material>,
    position: Vector,
    rotation: Quaternion,
}

impl<H: Handle> StaticBody<H> {
    pub fn new_with_id(id: H, shape: Box<Shape>, material: Box<Material>, position: Vector, rotation: Quaternion) -> StaticBody<H> {
        StaticBody {
            id: id,
            shape: shape,
            material: material,
            position: position,
            rotation: rotation,
        }
    }

    /// Returns a borrowed pointer to the Shape object held internally.
    #[inline]
    pub fn shape(&self) -> &Shape {
        &*self.shape
    }

    /// Returns the position of the `StaticBody`.
    #[inline]
    pub fn position(&self) -> Vector {
        self.position
    }

    /// Returns the rotation of the `StaticBody` expressed as a `Quaternion`.
    #[inline]
    pub fn rotation(&self) -> Quaternion {
        self.rotation
    }
}

impl<H: Handle> ShapeEntity for StaticBody<H> {
    #[inline(always)]
    fn shape(&self) -> &Shape {
        (self as &StaticBody<H>).shape()
    }

    fn transform(&self) -> Transform {
        Transform::new(self.position(), self.rotation())
    }
}
