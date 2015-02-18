use math::Vector;
use shapes::Shape;
use materials::Material;
use core::{ UID, State };

/// Represents a physical entity in the world.
pub struct Body {
    id: UID,
    shape: Box<Shape>,
    material: Box<Material>,
    state: State,
    force_impulse_accumulated: Vector,
    torque_impulse_accumulated: Vector,
}

impl Body {
    /// Creates a new instance of a Body object
    pub fn new(shape: Box<Shape>, material: Box<Material>, state: State) -> Body {
        Body::new_with_id(0, shape, material, state)
    }

    /// Creates a new instance of a `Body` with the specified id.
    pub fn new_with_id(id: UID, shape: Box<Shape>, material: Box<Material>, state: State) -> Body {
        Body {
            id: id,
            shape: shape,
            material: material,
            state: state,
            force_impulse_accumulated: Vector::new_zero(),
            torque_impulse_accumulated: Vector::new_zero(),
        }
    }

    /// Returns the `Body`’s UID.
    #[inline]
    pub fn id(&self) -> UID {
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

    /// Returns the position of the `Body`.
    #[inline]
    pub fn position(&self) -> Vector {
        self.state.position()
    }

    /// Returns the velocity associated with the Body.
    #[inline]
    pub fn velocity(&self) -> Vector {
        self.state.velocity()
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

    /// Sets the `Body`’s velocity using the `Vector` provided.
    #[inline]
    pub fn set_velocity_with_vector(&mut self, velocity: Vector) {
        self.state.set_velocity_with_vector(velocity);
    }

    /// Returns the total accumulated force acting on the `Body`.
    #[inline(always)]
    pub fn accumulated_force(&self) -> Vector {
        self.force_impulse_accumulated
    }

    /// Returns the total accumulated torque acting on the `Body`.
    #[inline(always)]
    pub fn accumulated_torque(&self) -> Vector {
        self.torque_impulse_accumulated
    }

    /// Applies a force on the `Body` acting on a specific point. This adds to
    /// the total accumulated force and torque.
    pub fn apply_impulse(&mut self, impulse: Vector, point: Vector) {
        self.force_impulse_accumulated = self.force_impulse_accumulated + impulse;
        self.torque_impulse_accumulated = self.torque_impulse_accumulated + impulse.cross(point - self.position());
    }

    /// Clears the accumulated force and torque acting on the `Body`.
    #[inline]
    pub fn reset_accumulators(&mut self) {
        self.force_impulse_accumulated = Vector::new_zero();
        self.torque_impulse_accumulated = Vector::new_zero();
    }
}
