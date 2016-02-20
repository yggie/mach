use std::fmt;

use {ID, Scalar};
use maths::{Matrix, Motion, Transform, Quat, Vect};
use shapes::Shape;
use entities::{Body, BodyParams, BodyType, BodyTypeMut, Form, Moveable};

/// Represents a physical entity in the world.
pub struct RigidBody {
    id: ID,
    mass: Scalar,
    form: Form,
    motion: Motion,
    coefficient_of_restitution: Scalar,
    friction_coefficient: Scalar,
}

impl RigidBody {
    form_field_accessors!(field_name: form);
    motion_field_accessors!(field_name: motion);

    /// Creates a new instance of a `RigidBody` object
    pub fn with_id(id: ID, params: &BodyParams) -> RigidBody {
        let shape = params.shape_desc.build();
        let material = &params.material;

        RigidBody {
            id: id,
            mass: material.mass_of(&*shape),
            form: Form::new(shape, params.transform.clone()),
            motion: params.motion.clone(),
            coefficient_of_restitution: material.coefficient_of_restitution(),
            friction_coefficient: material.friction_coefficient(),
        }
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
    pub fn coefficient_of_restitution(&self) -> Scalar {
        self.coefficient_of_restitution
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

impl Moveable for RigidBody {
    fn transform(&self) -> &Transform {
        RigidBody::transform(self)
    }

    fn transform_mut(&mut self) -> &mut Transform {
        RigidBody::transform_mut(self)
    }

    fn motion(&self) -> &Motion {
        RigidBody::motion(self)
    }

    fn motion_mut(&mut self) -> &mut Motion {
        RigidBody::motion_mut(self)
    }
}

impl fmt::Display for RigidBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "RigidBody[{}]: Pos={}, Rot={}, Vel={}, AngVel={}",
            self.id(),
            self.translation(),
            self.rotation(),
            self.velocity(),
            self.angular_velocity(),
        )
    }
}
