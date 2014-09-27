use math::Transform;
use shapes::Shape;
use properties::Property;

/// Represents a physical entity in the world.
pub struct Body {
    shape: Box<Shape>,
    property: Box<Property>,
    transform: Transform,
}

impl Body {
    /// Creates a new instance of a Body object
    pub fn new(shape: Box<Shape>, property: Box<Property>, transform: Transform) -> Body {
        Body{ shape: shape, property: property, transform: transform }
    }
}
