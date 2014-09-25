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
    ///
    /// # Example
    ///
    /// ```rust
    /// # use mithril::properties::Rigid;
    /// # use mithril::math::Transform;
    /// # use mithril::shapes::Sphere;
    /// # use mithril::bodies::Body;
    /// let p = Rigid::new(1.3);
    /// let s = Sphere::new(2.5);
    /// let t = Transform::identity();
    ///
    /// let b = Body::new(box s.clone(), box p.clone(), t);
    /// ```
    pub fn new(shape: Box<Shape>, property: Box<Property>, transform: Transform) -> Body {
        Body{ shape: shape, property: property, transform: transform }
    }
}
