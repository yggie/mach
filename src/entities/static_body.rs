use std::fmt;

use {ID, Scalar};
use maths::{Transform, Quat, Vect};
use shapes::Shape;
use entities::{BodyParams, Form, Body};

/// Represents a physical entity which cannot move. Within the engine, the
/// object is simply treated as if it has infinite mass.
pub struct StaticBody {
    id: ID,
    form: Form,
    coefficient_of_restitution: Scalar,
    _friction_coefficient: Scalar,
}

impl StaticBody {
    /// Creates a new `StaticBody` instance using the components provided to
    /// construct the entity.
    pub fn new_with_id(id: ID, params: &BodyParams) -> StaticBody {
        let shape = params.shape_desc.build();
        let material = &params.material;

        StaticBody {
            id: id,
            form: Form::new(shape, params.transform.clone()),
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
        self.form.shape()
    }

    #[inline]
    pub fn form(&self) -> &Form {
        &self.form
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
        &self.form.transform()
    }

    /// Returns the position of the `StaticBody`.
    #[inline]
    pub fn position(&self) -> &Vect {
        self.form.translation()
    }

    /// Returns the rotation of the `StaticBody` expressed as a `Quat`.
    #[inline]
    pub fn rotation(&self) -> &Quat {
        self.form.rotation()
    }
}

impl Body for StaticBody {
    fn form(&self) -> &Form {
        StaticBody::form(self)
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
