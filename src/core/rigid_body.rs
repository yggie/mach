use std::fmt;

use core::{ UID, State, Transform, VolumetricBody };
use maths::{ Matrix, Vector, Quaternion };
use shapes::Shape;
use materials::Material;

/// Represents a physical entity in the world.
pub struct RigidBody {
    id: UID,
    shape: Box<Shape>,
    material: Box<Material>,
    state: State,
}

impl RigidBody {
    /// Creates a new instance of a `RigidBody` object
    pub fn new_with_id(id: UID, shape: Box<Shape>, material: Box<Material>, state: State) -> RigidBody {
        RigidBody {
            id: id,
            shape: shape,
            material: material,
            state: state,
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

    /// Returns the `Material` object associated with the `RigidBody`.
    #[inline]
    pub fn material(&self) -> &Material {
        &*self.material
    }

    /// Returns the `State` associated with the `RigidBody`.
    #[inline]
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Returns the `State` associated with the `RigidBody` as a mutable
    /// reference.
    #[inline]
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    /// Returns the mass of the `RigidBody`.
    #[inline]
    pub fn mass(&self) -> f32 {
        self.material.mass_of(&*self.shape)
    }

    /// Returns the coefficient of restitution associated with the `RigidBody`.
    #[inline]
    pub fn coefficient_of_restitution(&self) -> f32 {
        self.material.coefficient_of_restitution()
    }

    /// Returns the inertia tensor of the `RigidBody`.
    #[inline]
    pub fn inertia(&self) -> Matrix {
        self.material.inertia_for(&*self.shape)
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
        Box::new(self.shape.vertices_iter().map(move |&v| s.transform_point(v)))
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

    fn transform(&self) -> Transform {
        Transform::new(self.position(), self.rotation())
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
