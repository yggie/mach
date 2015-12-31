use std::fmt;

use {ID, Scalar};
use maths::{Matrix, Motion, Transform, Quat, Vect};
use shapes::Shape;
use entities::{BodyParams, Form, Moveable, Body};

/// Represents a physical entity in the world.
pub struct RigidBody {
    id: ID,
    mass: Scalar,
    form: Form,
    motion: Motion,
    coefficient_of_restitution: Scalar,
    _friction_coefficient: Scalar,
}

impl RigidBody {
    /// Creates a new instance of a `RigidBody` object
    pub fn new_with_id(id: ID, params: &BodyParams) -> RigidBody {
        let shape = params.shape_desc.build();
        let material = &params.material;

        RigidBody {
            id: id,
            mass: material.mass_of(&*shape),
            form: Form::new(shape, params.transform.clone()),
            motion: params.motion.clone(),
            coefficient_of_restitution: material.coefficient_of_restitution(),
            _friction_coefficient: material.friction_coefficient(),
        }
    }

    /// Returns the handle associated with the `RigidBody`.
    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    /// Returns a borrowed pointer to the Shape object held internally.
    #[inline]
    pub fn shape(&self) -> &Shape {
        self.form.shape()
    }

    /// Returns the associated `Transform` object.
    #[inline]
    pub fn transform(&self) -> &Transform {
        self.form.transform()
    }

    #[inline]
    pub fn transform_mut(&mut self) -> &mut Transform {
        self.form.transform_mut()
    }

    #[inline]
    pub fn motion(&self) -> &Motion {
        &self.motion
    }

    #[inline]
    pub fn motion_mut(&mut self) -> &mut Motion {
        &mut self.motion
    }

    /// Returns the mass of the `RigidBody`.
    #[inline]
    pub fn mass(&self) -> Scalar {
        self.mass
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

    /// Returns the inertia tensor of the `RigidBody`.
    #[inline]
    pub fn inertia(&self) -> Matrix {
        self.shape().inertia() * self.mass
    }

    /// Returns the position of the `RigidBody`.
    #[inline]
    pub fn pos(&self) -> &Vect {
        self.form.translation()
    }

    /// Returns the velocity of the `RigidBody`.
    #[inline]
    pub fn vel(&self) -> &Vect {
        &self.motion.velocity
    }

    /// Returns the rotation of the `RigidBody` expressed as a `Quat`.
    #[inline]
    pub fn rot(&self) -> &Quat {
        self.form.rotation()
    }

    /// Returns the angular velocity of the `RigidBody`.
    #[inline]
    pub fn ang_vel(&self) -> &Vect {
        &self.motion.angular_velocity
    }

    /// Returns the position of the vertex associated with the index.
    pub fn vertex(&self, index: usize) -> Vect {
        self.transform().apply_to_point(self.shape().vertex(index))
    }

    /// Returns an `Iterator` over the vertices of the `RigidBody`.
    pub fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vect> + 'a> {
        self.form.vertices_iter()
    }

    /// Sets the `RigidBody`’s position using the `Vect` provided.
    #[inline]
    pub fn set_pos(&mut self, values: &(Scalar, Scalar, Scalar)) {
        *self.form.translation_mut() = Vect::new(values.0, values.1, values.2);
    }

    /// Sets the `RigidBody`’s rotation using the `Quat` provided.
    #[inline]
    pub fn set_rot(&mut self, rot: &Quat) {
        *self.form.rotation_mut() = rot.clone();
    }

    /// Sets the `RigidBody`’s velocity using the `Vect` provided.
    #[inline]
    pub fn set_vel(&mut self, values: &(Scalar, Scalar, Scalar)) {
        self.motion.velocity = Vect::new(values.0, values.1, values.2);
    }

    /// Set the `RigidBody`’s angular velocity using the `Vect` provided.
    #[inline]
    pub fn set_ang_vel(&mut self, values: &(Scalar, Scalar, Scalar)) {
        self.motion.angular_velocity = Vect::new(values.0, values.1, values.2);
    }
}

impl Body for RigidBody {
    #[inline(always)]
    fn shape(&self) -> &Shape {
        (self as &RigidBody).shape()
    }

    #[inline(always)]
    fn transform(&self) -> &Transform {
        (self as &RigidBody).transform()
    }
}

impl Moveable for RigidBody {
    fn transform(&self) -> &Transform {
        self.form.transform()
    }

    fn transform_mut(&mut self) -> &mut Transform {
        self.form.transform_mut()
    }

    fn motion(&self) -> &Motion {
        &self.motion
    }

    fn motion_mut(&mut self) -> &mut Motion {
        &mut self.motion
    }
}

impl fmt::Display for RigidBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "RigidBody[{}]: Pos={}, Rot={}, Vel={}, AngVel={}",
            self.id(),
            self.pos(),
            self.rot(),
            self.vel(),
            self.ang_vel(),
        )
    }
}
