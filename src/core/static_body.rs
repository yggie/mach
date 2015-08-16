use std::fmt::{ Display, Formatter, Result };

use core::{ Handle, Transform };
use maths::{ Vector, Quaternion };
use shapes::{ Shape, ShapeEntity };
use materials::Material;

/// Represents a physical entity which cannot move. Within the engine, the
/// object is simply treated as if it has infinite mass.
pub struct StaticBody<H: Handle> {
    id: H,
    shape: Box<Shape>,
    material: Box<Material>,
    transform: Transform,
}

impl<H: Handle> StaticBody<H> {
    /// Creates a new `StaticBody` instance using the components provided to
    /// construct the entity.
    pub fn new_with_id(id: H, shape: Box<Shape>, material: Box<Material>, transform: Transform) -> StaticBody<H> {
        StaticBody {
            id: id,
            shape: shape,
            material: material,
            transform: transform,
        }
    }

    /// Returns the identifier for the `StaticBody` instance.
    #[inline]
    pub fn id(&self) -> H {
        self.id
    }

    /// Returns the associated `Shape` object for the entity.
    #[inline]
    pub fn shape(&self) -> &Shape {
        &*self.shape
    }

    /// Returns the associated `Transform` object for the entity.
    #[inline]
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    /// Returns the position of the `StaticBody`.
    #[inline]
    pub fn position(&self) -> Vector {
        self.transform.translation()
    }

    /// Returns the rotation of the `StaticBody` expressed as a `Quaternion`.
    #[inline]
    pub fn rotation(&self) -> Quaternion {
        self.transform.rotation()
    }
}

impl<H: Handle> ShapeEntity for StaticBody<H> {
    #[inline(always)]
    fn shape(&self) -> &Shape {
        (self as &StaticBody<H>).shape()
    }

    fn transform(&self) -> Transform {
        *(self as &StaticBody<H>).transform()
    }
}

impl<H: Handle> Display for StaticBody<H> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,
            "StaticBody[{}]: Pos={}, Rot={}",
            self.id(),
            self.position(),
            self.rotation(),
        )
    }
}
