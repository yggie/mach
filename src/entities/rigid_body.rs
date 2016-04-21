use {ID, Scalar};
use maths::{IntegratableMut, Matrix, Motion, Transform, UnitQuat, Vect};
use shapes::Shape;
use entities::{Body, BodyType, BodyTypeMut, Form};

/// Represents a physical entity in the world.
#[derive(Clone)]
pub struct RigidBody {
    id: ID,
    mass: Scalar,
    form: Form,
    motion: Motion,
    restitution_coefficient: Scalar,
    friction_coefficient: Scalar,
}

impl RigidBody {
    include_form_helpers! {
        struct_name: RigidBody,
    }
    include_motion_helpers! {
        struct_name: RigidBody,
    }

    /// Returns the handle associated with the `RigidBody`.
    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    /// Returns the mass of the `RigidBody`.
    #[inline]
    pub fn mass(&self) -> Scalar {
        self.mass
    }

    #[inline]
    pub fn mass_inverse(&self) -> Scalar {
        1.0 / self.mass
    }

    /// Returns the coefficient of restitution associated with the `RigidBody`.
    #[inline]
    pub fn restitution_coefficient(&self) -> Scalar {
        self.restitution_coefficient
    }

    /// Returns the friction coefficient associated with the `RigidBody`.
    #[inline]
    pub fn friction_coefficient(&self) -> Scalar {
        self.friction_coefficient
    }

    /// Returns the inertia tensor of the `RigidBody`.
    #[inline]
    pub fn inertia(&self) -> Matrix {
        self.shape().inertia() * self.mass
    }

    #[inline]
    pub fn inertia_inverse(&self) -> Matrix {
        self.inertia().inverse()
    }

    pub fn as_integratable_mut<'a>(&'a mut self) -> IntegratableMut<'a> {
        IntegratableMut::new(self.form.transform_mut(), &mut self.motion)
    }

    #[inline]
    pub fn with_id_(self, id: ID) -> RigidBody {
        RigidBody {
            id: id,
            .. self
        }
    }

    #[inline]
    pub fn with_mass(self, mass: Scalar) -> RigidBody {
        RigidBody {
            mass: mass,
            .. self
        }
    }

    #[inline]
    pub fn with_restitution_coefficient(self, coefficient: Scalar) -> RigidBody {
        RigidBody {
            restitution_coefficient: coefficient,
            .. self
        }
    }
}

impl Default for RigidBody {
    fn default() -> RigidBody {
        RigidBody {
            id: ID(0),
            mass: 1.0,
            form: Form::default(),
            motion: Motion::stationary(),
            restitution_coefficient: 0.8,
            friction_coefficient: 0.6,
        }
    }
}

impl Body for RigidBody {
    fn downcast<'a>(&'a self) -> BodyType<'a> {
        BodyType::Rigid(self as &RigidBody)
    }

    fn downcast_mut<'a>(&'a mut self) -> BodyTypeMut<'a> {
        BodyTypeMut::Rigid(self as &mut RigidBody)
    }

    fn id(&self) -> ID {
        RigidBody::id(self)
    }

    fn form(&self) -> &Form {
        RigidBody::form(self)
    }

    fn form_mut(&mut self) -> &mut Form {
        RigidBody::form_mut(self)
    }
}
