use core::{ Handle, State };
use math::Vector;
use shapes::Shape;
use materials::Material;

/// Represents a physical entity in the world.
pub struct Body<H: Handle> {
    handle: H,
    shape: Box<Shape>,
    material: Box<Material>,
    state: State,
}

impl<H: Handle> Body<H> {
    /// Creates a new instance of a Body object
    pub fn new_with_handle(handle: H, shape: Box<Shape>, material: Box<Material>, state: State) -> Body<H> {
        Body {
            handle: handle,
            shape: shape,
            material: material,
            state: state,
        }
    }

    /// Returns the handle associated with the `Body`.
    #[inline]
    pub fn handle(&self) -> H {
        self.handle
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
}
