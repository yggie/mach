use Scalar;
use maths::Vector;
use collisions::CollisionSpace;

/// The `Dynamics` component is responsible for handling the dynamic component
/// of the simulation, including stepping the simulation forward in time and
/// managing environmental effects on bodies.
pub trait Dynamics {
    /// Steps the simulation forward in time by the specified amount.
    fn update<C: CollisionSpace>(&mut self, &mut C, Scalar);

    /// Returns the global gravitational force acting on the `RigidBody`
    /// objects.
    fn gravity(&self) -> Vector;

    /// Adjusts the global gravitational force acting on the `RigidBody`
    /// objects.
    fn set_gravity(&mut self, Vector);
}
