use std::fmt::{ Display, Formatter, Result };

use core::{ Float, UID };
use maths::{ Transform, Quaternion, Vector };
use shapes::Shape;
use entities::{ Material, VolumetricBody };

/// Represents a physical entity which cannot move. Within the engine, the
/// object is simply treated as if it has infinite mass.
pub struct StaticBody {
    id: UID,
    shape: Box<Shape>,
    transform: Transform,
    coefficient_of_restitution: Float,
}

impl StaticBody {
    /// Creates a new `StaticBody` instance using the components provided to
    /// construct the entity.
    pub fn new_with_id(id: UID, shape: Box<Shape>, material: &Material, transform: Transform) -> StaticBody {
        StaticBody {
            id: id,
            shape: shape,
            transform: transform,
            coefficient_of_restitution: material.coefficient_of_restitution(),
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
    pub fn coefficient_of_restitution(&self) -> Float {
        self.coefficient_of_restitution
    }

    /// Returns the associated `Transform` object for the entity.
    #[inline]
    pub fn transform(&self) -> Transform {
        self.transform
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
        (self as &StaticBody).transform()
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
