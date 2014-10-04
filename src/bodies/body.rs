use math::{ Vector, Transform };
use shapes::Shape;
use properties::Property;

/// Represents a physical entity in the world.
pub struct Body<'a> {
    shape: Box<Shape + 'a>,
    property: Box<Property + 'a>,
    transform: Transform,
    velocity: Vector,
    impulse: Vector,
}

impl<'a> Body<'a> {
    /// Creates a new instance of a Body object
    pub fn new<'a>(shape: Box<'a Shape>, property: Box<'a Property>,
                   transform: Transform, derivative_transform: Transform) -> Body<'a> {
        Body {
            shape: shape,
            property: property,
            transform: transform,
            velocity: derivative_transform.translation_vector(),
            impulse: Vector::new_zero(),
        }
    }

    /// Returns a borrowed pointer to the Shape object held internally.
    #[inline]
    pub fn shape(&self) -> &Shape+'a {
        &*self.shape
    }

    /// Returns the property object associated with the Body.
    #[inline]
    pub fn property(&self) -> &Property+'a {
        &*self.property
    }

    /// Returns the transformation matrix associated with the Body.
    #[inline]
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    /// Returns the velocity associated with the Body.
    #[inline]
    pub fn velocity(&self) -> Vector {
        self.velocity
    }

    /// Returns the mass of the Body.
    #[inline]
    pub fn mass(&self) -> f32 {
        self.property.mass_of(&*self.shape)
    }

    /// Returns the position of the Body.
    #[inline]
    pub fn position(&self) -> Vector {
        self.transform.translation_vector()
    }

    /// Applies an impulse on the body.
    pub fn apply_impulse(&mut self, impulse: Vector) {
        self.impulse = self.impulse + impulse;
    }
}
