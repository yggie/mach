use std::fmt;

use core::{ Handle, State, Transform };
use maths::{ Matrix, Vector, Quaternion };
use shapes::{ Shape, ShapeEntity };
use materials::Material;

/// Represents a physical entity in the world.
pub struct Body<H: Handle> {
    id: H,
    shape: Box<Shape>,
    material: Box<Material>,
    state: State,
}

impl<H: Handle> Body<H> {
    /// Creates a new instance of a Body object
    pub fn new_with_id(id: H, shape: Box<Shape>, material: Box<Material>, state: State) -> Body<H> {
        Body {
            id: id,
            shape: shape,
            material: material,
            state: state,
        }
    }

    /// Returns the handle associated with the `Body`.
    #[inline]
    pub fn id(&self) -> H {
        self.id
    }

    /// Returns a borrowed pointer to the Shape object held internally.
    #[inline]
    pub fn shape(&self) -> &Shape {
        &*self.shape
    }

    /// Returns the `Material` object associated with the Body.
    #[inline]
    pub fn material(&self) -> &Material {
        &*self.material
    }

    /// Returns the `State` associated with the Body.
    #[inline]
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Returns the mass of the `Body`.
    #[inline]
    pub fn mass(&self) -> f32 {
        self.material.mass_of(&*self.shape)
    }

    /// Returns the inertia tensor of the `Body`.
    #[inline]
    pub fn inertia(&self) -> Matrix {
        self.material.inertia_for(&*self.shape)
    }

    /// Returns the position of the `Body`.
    #[inline]
    pub fn position(&self) -> Vector {
        self.state.position()
    }

    /// Returns the velocity of the Body.
    #[inline]
    pub fn velocity(&self) -> Vector {
        self.state.velocity()
    }

    /// Returns the rotation of the `Body` expressed as a `Quaternion`.
    #[inline]
    pub fn rotation_quaternion(&self) -> Quaternion {
        self.state.rotation()
    }

    /// Returns the angular velocity of the Body.
    #[inline]
    pub fn angular_velocity(&self) -> Vector {
        self.state.angular_velocity()
    }

    /// Returns the position of the vertex associated with the index.
    pub fn vertex(&self, index: usize) -> Vector {
        self.state.transform_point(self.shape.vertex(index))
    }

    /// Returns an `Iterator` over the vertices of the `Body`.
    pub fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vector> + 'a> {
        let s = self.state.clone();
        Box::new(self.shape.vertices_iter().map(move |&v| s.transform_point(v)))
    }

    /// Sets the `Body`’s position using the `Vector` provided.
    #[inline]
    pub fn set_position_with_vector(&mut self, position: Vector) {
        self.state.set_position_with_vector(position);
    }

    /// Sets the `Body`’s rotation using the `Quaternion` provided.
    #[inline]
    pub fn set_rotation_with_quaternion(&mut self, rotation: Quaternion) {
        self.state.set_rotation_with_quaternion(rotation);
    }

    /// Sets the `Body`’s velocity using the `Vector` provided.
    #[inline]
    pub fn set_velocity_with_vector(&mut self, velocity: Vector) {
        self.state.set_velocity_with_vector(velocity);
    }

    /// Set the `Body`’s angular velocity using the `Vector` provided.
    #[inline]
    pub fn set_angular_velocity_with_vector(&mut self, angular_velocity: Vector) {
        self.state.set_angular_velocity(angular_velocity[0], angular_velocity[1], angular_velocity[2]);
    }
}

impl<H: Handle> ShapeEntity for Body<H> {
    #[inline(always)]
    fn shape(&self) -> &Shape {
        (self as &Body<H>).shape()
    }

    fn transform(&self) -> Transform {
        Transform::new(self.position(), self.rotation_quaternion())
    }
}

impl<H: Handle> fmt::Display for Body<H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "Body[{}]: Pos={}, Rot={}, Vel={}, AngVel={}",
            self.id(),
            self.position(),
            self.rotation_quaternion(),
            self.velocity(),
            self.angular_velocity(),
        )
    }
}
