/// The trait which all shapes must implement.
pub trait Shape {
    /// Computes the volume for the shape.
    fn volume(&self) -> f32;
}
