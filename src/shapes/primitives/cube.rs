use shapes::Shape;

/// A representation of a cube in 3 dimensions.
pub struct Cube {
    /// The width of the cube
    pub width: f32,
    /// The height of the cube
    pub height: f32,
    /// The depth of the cube
    pub depth: f32,
}

impl Cube {

    /// Constructs a new Cube given the width, height and depth dimensions.
    ///
    /// ```rust
    /// # use mithril::shapes::Cube;
    /// let c = Cube::new(5.0, 3.0, 7.5);
    ///
    /// assert!((c.width, c.height, c.depth) == (5.0, 3.0, 7.5))
    /// ```
    pub fn new(width: f32, height: f32, depth: f32) -> Cube {
        Cube{ width: width, height: height, depth: depth }
    }
}

impl Shape for Cube {

    /// Calculates the volume of the Cube.
    ///
    /// ```rust
    /// # use mithril::shapes::Shape;
    /// # use mithril::shapes::Cube;
    /// let c = Cube::new(2.0, 3.0, 4.0);
    ///
    /// assert!(c.volume() == 24.0)
    /// ```
    fn volume(&self) -> f32 {
        self.width * self.height * self.depth
    }
}
