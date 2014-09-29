use math::Transform;
use shapes::Shape;
use properties::Property;

/// Represents a physical entity in the world.
pub struct Body<'a> {
    shape: Box<Shape + 'a>,
    property: Box<Property + 'a>,
    transform: Transform,
}

impl<'a> Body<'a> {
    /// Creates a new instance of a Body object
    pub fn new<'a>(shape: Box<'a Shape>, property: Box<'a Property>, transform: Transform) -> Body<'a> {
        Body{ shape: shape, property: property, transform: transform }
    }

    /// Returns a borrowed pointer to the Shape object held internally.
    #[inline]
    pub fn shape(&'a self) -> &Shape+'a {
        &*self.shape
    }

    /// Returns the transformation matrix associated with the Body.
    #[inline]
    pub fn transform(&self) -> &Transform {
        &self.transform
    }
}
