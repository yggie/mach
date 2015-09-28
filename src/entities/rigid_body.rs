use std::fmt;

use core::{ Float, UID };
use maths::{ Matrix, State, Transform, Quaternion, Vector };
use shapes::Shape;
use entities::{ Material, VolumetricBody };

/// Represents a physical entity in the world.
pub struct RigidBody {
    id: UID,
    mass: Float,
    shape: Box<Shape>,
    state: State,
    coefficient_of_restitution: Float,
    _friction_coefficient: Float,
}

impl RigidBody {
    /// Creates a new instance of a `RigidBody` object
    pub fn new_with_id(id: UID, shape: Box<Shape>, material: &Material, state: State) -> RigidBody {
        RigidBody {
            id: id,
            mass: material.mass_of(&*shape),
            shape: shape,
            state: state,
            coefficient_of_restitution: material.coefficient_of_restitution(),
            _friction_coefficient: material.friction_coefficient(),
        }
    }

    /// Returns the handle associated with the `RigidBody`.
    #[inline]
    pub fn id(&self) -> UID {
        self.id
    }

    /// Returns a borrowed pointer to the Shape object held internally.
    #[inline]
    pub fn shape(&self) -> &Shape {
        &*self.shape
    }

    /// Returns the `State` associated with the `RigidBody`.
    #[inline]
    pub fn state(&self) -> State {
        self.state
    }

    /// Returns the `State` associated with the `RigidBody` as a mutable
    /// reference.
    #[inline]
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    /// Returns the associated `Transform` object.
    #[inline]
    pub fn transform(&self) -> Transform {
        self.state.transform()
    }

    /// Returns the mass of the `RigidBody`.
    #[inline]
    pub fn mass(&self) -> Float {
        self.mass
    }

    /// Returns the coefficient of restitution associated with the `RigidBody`.
    #[inline]
    pub fn coefficient_of_restitution(&self) -> Float {
        self.coefficient_of_restitution
    }

    /// Returns the friction coefficient associated with the `RigidBody`.
    #[inline]
    pub fn friction_coefficient(&self) -> Float {
        self._friction_coefficient
    }

    /// Returns the inertia tensor of the `RigidBody`.
    #[inline]
    pub fn inertia(&self) -> Matrix {
        self.shape.inertia() * self.mass
    }

    /// Returns the position of the `RigidBody`.
    #[inline]
    pub fn position(&self) -> Vector {
        self.state.position()
    }

    /// Returns the velocity of the `RigidBody`.
    #[inline]
    pub fn velocity(&self) -> Vector {
        self.state.velocity()
    }

    /// Returns the rotation of the `RigidBody` expressed as a `Quaternion`.
    #[inline]
    pub fn rotation(&self) -> Quaternion {
        self.state.rotation()
    }

    /// Returns the angular velocity of the `RigidBody`.
    #[inline]
    pub fn angular_velocity(&self) -> Vector {
        self.state.angular_velocity()
    }

    /// Returns the position of the vertex associated with the index.
    pub fn vertex(&self, index: usize) -> Vector {
        self.state.transform_point(self.shape.vertex(index))
    }

    /// Returns an `Iterator` over the vertices of the `RigidBody`.
    pub fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vector> + 'a> {
        let s = self.state.clone();
        Box::new(self.shape.vertices_iter().map(move |v| s.transform_point(v)))
    }

    /// Sets the `RigidBody`’s position using the `Vector` provided.
    #[inline]
    pub fn set_position_with_vector(&mut self, position: Vector) {
        self.state.set_position_with_vector(position);
    }

    /// Sets the `RigidBody`’s rotation using the `Quaternion` provided.
    #[inline]
    pub fn set_rotation(&mut self, rotation: Quaternion) {
        self.state.set_rotation(rotation);
    }

    /// Sets the `RigidBody`’s velocity using the `Vector` provided.
    #[inline]
    pub fn set_velocity_with_vector(&mut self, velocity: Vector) {
        self.state.set_velocity_with_vector(velocity);
    }

    /// Set the `RigidBody`’s angular velocity using the `Vector` provided.
    #[inline]
    pub fn set_angular_velocity_with_vector(&mut self, angular_velocity: Vector) {
        self.state.set_angular_velocity(angular_velocity[0], angular_velocity[1], angular_velocity[2]);
    }
}

impl VolumetricBody for RigidBody {
    #[inline(always)]
    fn shape(&self) -> &Shape {
        (self as &RigidBody).shape()
    }

    #[inline(always)]
    fn transform(&self) -> Transform {
        (self as &RigidBody).transform()
    }
}

impl fmt::Display for RigidBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "RigidBody[{}]: Pos={}, Rot={}, Vel={}, AngVel={}",
            self.id(),
            self.position(),
            self.rotation(),
            self.velocity(),
            self.angular_velocity(),
        )
    }
}
