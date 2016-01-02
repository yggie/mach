use std::fmt;

use {ID, Scalar};
use maths::{Transform, Quat, Vect};
use shapes::Shape;
use entities::{Body, BodyParams, BodyType, Form};

/// Represents a physical entity which cannot move. Within the engine, the
/// object is simply treated as if it has infinite mass.
pub struct StaticBody {
    id: ID,
    form: Form,
    coefficient_of_restitution: Scalar,
    friction_coefficient: Scalar,
}

impl StaticBody {
    form_field_accessors!(field_name: form);

    /// Creates a new `StaticBody` instance using the components provided to
    /// construct the entity.
    pub fn with_id(id: ID, params: &BodyParams) -> StaticBody {
        let shape = params.shape_desc.build();
        let material = &params.material;

        StaticBody {
            id: id,
            form: Form::new(shape, params.transform.clone()),
            coefficient_of_restitution: material.coefficient_of_restitution(),
            friction_coefficient: material.friction_coefficient(),
        }
    }

    /// Returns the identifier for the `StaticBody` instance.
    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    /// Returns the coefficient of restitution associated with the `RigidBody`.
    #[inline]
    pub fn coefficient_of_restitution(&self) -> Scalar {
        self.coefficient_of_restitution
    }

    /// Returns the friction coefficient associated with the `RigidBody`.
    #[inline]
    pub fn friction_coefficient(&self) -> Scalar {
        self.friction_coefficient
    }
}

impl Body for StaticBody {
    fn downcast<'a>(&'a self) -> BodyType<'a> {
        BodyType::Static(self as &StaticBody)
    }

    fn form(&self) -> &Form {
        StaticBody::form(self)
    }
}

impl fmt::Display for StaticBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "StaticBody[{}]: Pos={}, Rot={}",
            self.id(),
            self.translation(),
            self.rotation(),
        )
    }
}
