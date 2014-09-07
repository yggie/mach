#[cfg(test)]
#[path="../../tests/math/vector_test.rs"]
mod tests;

/// A 3-dimensional vector
pub struct Vector {
    /// The x-coordinate (or first element) of the vector
    pub x: f32,
    /// The y-coordinate (or second element) of the vector
    pub y: f32,
    /// The z-coordinate (or third element) of the vector
    pub z: f32
}

impl Vector {
    /// A simple constructor which builds a vector given the three elements
    ///
    /// ```rust
    /// use mithril::math::Vector;
    ///
    /// let vect = Vector::new(1.0, 2.0, 3.0);
    ///
    /// assert!(vect.x == 1.0)
    /// assert!(vect.y == 2.0)
    /// assert!(vect.z == 3.0)
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }
}
