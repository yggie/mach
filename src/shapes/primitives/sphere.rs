use shapes::Shape;

use std::f32::consts::PI;

/// A representation of a sphere in 3 dimensions.
pub struct Sphere {
    /// The radius of the sphere.
    pub radius: f32,
}

impl Sphere {

    /// Constructs a new Sphere with the radius provided.
    ///
    /// ```rust
    /// # use mithril::shapes::Sphere;
    /// let s = Sphere::new(5.0);
    ///
    /// assert!(s.radius == 5.0)
    /// ```
    pub fn new(radius: f32) -> Sphere {
        Sphere{ radius: radius }
    }
}

impl Shape for Sphere {

    /// Calculates the volume of the Sphere.
    ///
    /// ```rust
    /// # use mithril::shapes::Shape;
    /// # use mithril::shapes::Sphere;
    /// # use std::f32::consts::PI;
    /// let s = Sphere::new(0.75);
    ///
    /// # fn assert_approx_eq(a: f32, b: f32) { assert!((a - b).abs() < 1.0e-6) }
    /// assert_approx_eq(s.volume(), 0.5625*PI);
    /// ```
    fn volume(&self) -> f32 {
        4.0*PI*self.radius*self.radius*self.radius/3.0
    }
}
