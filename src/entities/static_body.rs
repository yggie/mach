use std::fmt;

use {ID, Scalar};
use maths::{Transform, Quat, Vect};
use shapes::Shape;
use entities::{Body, BodyType, BodyTypeMut, Form};

/// Represents a physical entity which cannot move. Within the engine, the
/// object is simply treated as if it has infinite mass.
#[derive(Clone)]
pub struct StaticBody {
    id: ID,
    form: Form,
    restitution_coefficient: Scalar,
    friction_coefficient: Scalar,
}

impl StaticBody {
    include_form_helpers! {
        struct_name: StaticBody,
    }

    /// Returns the identifier for the `StaticBody` instance.
    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    /// Returns the coefficient of restitution associated with the `StaticBody`.
    #[inline]
    pub fn restitution_coefficient(&self) -> Scalar {
        self.restitution_coefficient
    }

    /// Returns the friction coefficient associated with the `StaticBody`.
    #[inline]
    pub fn friction_coefficient(&self) -> Scalar {
        self.friction_coefficient
    }

    #[inline]
    pub fn with_id_(self, id: ID) -> StaticBody {
        StaticBody {
            id: id,
            .. self
        }
    }

    #[inline]
    pub fn with_restitution_coefficient(self, coefficient: Scalar) -> StaticBody {
        StaticBody {
            restitution_coefficient: coefficient,
            .. self
        }
    }
}

impl Default for StaticBody {
    fn default() -> StaticBody {
        StaticBody {
            id: ID(0),
            form: Form::default(),
            restitution_coefficient: 0.8,
            friction_coefficient: 0.6,
        }
    }
}

impl Body for StaticBody {
    fn downcast<'a>(&'a self) -> BodyType<'a> {
        BodyType::Static(self as &StaticBody)
    }

    fn downcast_mut<'a>(&'a mut self) -> BodyTypeMut<'a> {
        BodyTypeMut::Static(self as &mut StaticBody)
    }

    fn id(&self) -> ID {
        StaticBody::id(self)
    }

    fn form(&self) -> &Form {
        StaticBody::form(self)
    }

    fn form_mut(&mut self) -> &mut Form {
        StaticBody::form_mut(self)
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
