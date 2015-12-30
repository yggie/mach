use Scalar;
use maths::Vect;
use detection::{Contact, Space};

/// The `Dynamics` component is responsible for handling the dynamic component
/// of the simulation, including stepping the simulation forward in time and
/// managing environmental effects on bodies.
pub trait Dynamics {
    /// Steps the simulation forward in time by the specified amount.
    fn update<S: Space>(&mut self, &mut S, Scalar) -> Option<Vec<Contact>>;

    /// Returns the global gravitational force acting on the `RigidBody`
    /// objects.
    fn gravity(&self) -> Vect;

    /// Adjusts the global gravitational force acting on the `RigidBody`
    /// objects.
    fn set_gravity(&mut self, Vect);
}
