use std::fmt;

use {ID, Scalar};
use maths::{Transform, Quat, Vect};
use shapes::Shape;
use entities::{Material, Body};

/// Represents a physical entity which cannot move. Within the engine, the
/// object is simply treated as if it has infinite mass.
pub struct StaticBody {
    id: ID,
    shape: Box<Shape>,
    transform: Transform,
    coefficient_of_restitution: Scalar,
    _friction_coefficient: Scalar,
}

impl StaticBody {
    /// Creates a new `StaticBody` instance using the components provided to
    /// construct the entity.
    pub fn new_with_id(id: ID, shape: Box<Shape>, material: &Material, transform: Transform) -> StaticBody {
        StaticBody {
            id: id,
            shape: shape,
            transform: transform,
            coefficient_of_restitution: material.coefficient_of_restitution(),
            _friction_coefficient: material.friction_coefficient(),
        }
    }

    /// Returns the identifier for the `StaticBody` instance.
    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    /// Returns the associated `Shape` object for the entity.
    #[inline]
    pub fn shape(&self) -> &Shape {
        &*self.shape
    }

    /// Returns the coefficient of restitution associated with the `RigidBody`.
    #[inline]
    pub fn coefficient_of_restitution(&self) -> Scalar {
        self.coefficient_of_restitution
    }

    /// Returns the friction coefficient associated with the `RigidBody`.
    #[inline]
    pub fn friction_coefficient(&self) -> Scalar {
        self._friction_coefficient
    }

    /// Returns the associated `Transform` object for the entity.
    #[inline]
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    /// Returns the position of the `StaticBody`.
    #[inline]
    pub fn position(&self) -> Vect {
        self.transform.translation()
    }

    /// Returns the rotation of the `StaticBody` expressed as a `Quat`.
    #[inline]
    pub fn rotation(&self) -> Quat {
        self.transform.rotation()
    }
}

impl Body for StaticBody {
    #[inline(always)]
    fn shape(&self) -> &Shape {
        (self as &StaticBody).shape()
    }

    fn transform(&self) -> &Transform {
        (self as &StaticBody).transform()
    }
}

impl fmt::Display for StaticBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "StaticBody[{}]: Pos={}, Rot={}",
            self.id(),
            self.position(),
            self.rotation(),
        )
    }
}
