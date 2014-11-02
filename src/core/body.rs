use math::Vector;
use shapes::Shape;
use properties::Property;
use core::{ UID, State };

/// Represents a physical entity in the world.
pub struct Body {
    id: UID,
    shape: Box<Shape>,
    property: Box<Property>,
    state: State,
    pub impulse: Vector,
}

impl Body {
    /// Creates a new instance of a Body object
    pub fn new(shape: Box<Shape>, property: Box<Property>, state: State) -> Body {
        Body::new_with_id(0u, shape, property, state)
    }

    /// Creates a new instance of a `Body` with the specified id.
    pub fn new_with_id(id: UID, shape: Box<Shape>, property: Box<Property>, state: State) -> Body {
        Body {
            id: id,
            shape: shape,
            property: property,
            state: state,
            impulse: Vector::new_zero(),
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

    /// Returns the property object associated with the Body.
    #[inline]
    pub fn property(&self) -> &Property {
        &*self.property
    }

    /// Returns the `State` associated with the Body.
    #[inline]
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Returns the mass of the `Body`.
    #[inline]
    pub fn mass(&self) -> f32 {
        self.property.mass_of(&*self.shape)
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

    /// Returns the impulse currently acting on the `Body`.
    #[inline]
    pub fn impulse(&self) -> Vector {
        self.impulse
    }

    /// Applies an impulse on the `Body`.
    pub fn apply_impulse(&mut self, impulse: Vector) {
        self.impulse = self.impulse + impulse;
    }
}
