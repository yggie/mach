use core::UID;
use math::{ Vector, Transform };
use shapes::Shape;
use properties::Property;

/// Represents a physical entity in the world.
pub struct Body {
    id: UID,
    shape: Box<Shape>,
    property: Box<Property>,
    transform: Transform,
    velocity: Vector,
    impulse: Vector,
}

impl Body {
    /// Creates a new instance of a Body object
    pub fn new(shape: Box<Shape>, property: Box<Property>,
                   transform: Transform, derivative_transform: Transform) -> Body {
        Body::new_with_id(0u, shape, property, transform, derivative_transform)
    }

    /// Creates a new instance of a `Body` with the specified id.
    pub fn new_with_id(id: UID, shape: Box<Shape>, property: Box<Property>,
                             transform: Transform, derivative_transform: Transform) -> Body {
        Body {
            id: id,
            shape: shape,
            property: property,
            transform: transform,
            velocity: derivative_transform.translation_vector(),
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

    /// Returns the mass of the `Body`.
    #[inline]
    pub fn mass(&self) -> f32 {
        self.property.mass_of(&*self.shape)
    }

    /// Returns the position of the `Body`.
    #[inline]
    pub fn position(&self) -> Vector {
        self.transform.translation_vector()
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
