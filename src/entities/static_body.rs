use std::fmt::{ Display, Formatter, Result };

use core::{ UID, Transform };
use maths::{ Vector, Quaternion };
use shapes::Shape;
use entities::VolumetricBody;
use materials::Material;

/// Represents a physical entity which cannot move. Within the engine, the
/// object is simply treated as if it has infinite mass.
pub struct StaticBody {
    id: UID,
    shape: Box<Shape>,
    material: Box<Material>,
    transform: Transform,
}

impl StaticBody {
    /// Creates a new `StaticBody` instance using the components provided to
    /// construct the entity.
    pub fn new_with_id(id: UID, shape: Box<Shape>, material: Box<Material>, transform: Transform) -> StaticBody {
        StaticBody {
            id: id,
            shape: shape,
            material: material,
            transform: transform,
        }
    }

    /// Returns the identifier for the `StaticBody` instance.
    #[inline]
    pub fn id(&self) -> UID {
        self.id
    }

    /// Returns the associated `Shape` object for the entity.
    #[inline]
    pub fn shape(&self) -> &Shape {
        &*self.shape
    }

    /// Returns the coefficient of restitution associated with the `RigidBody`.
    #[inline]
    pub fn coefficient_of_restitution(&self) -> f32 {
        self.material.coefficient_of_restitution()
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

impl VolumetricBody for StaticBody {
    #[inline(always)]
    fn shape(&self) -> &Shape {
        (self as &StaticBody).shape()
    }

    fn transform(&self) -> Transform {
        *(self as &StaticBody).transform()
    }
}

impl Display for StaticBody {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,
            "StaticBody[{}]: Pos={}, Rot={}",
            self.id(),
            self.position(),
            self.rotation(),
        )
    }
}
