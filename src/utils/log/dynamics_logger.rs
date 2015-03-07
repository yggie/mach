use math::Vector;
use dynamics::Dynamics;
use collisions::Collisions;

/// A utility class which wraps around `Dynamics` components and produces
/// parseable output for debugging.
pub struct DynamicsLogger<D: Dynamics>(D);

impl<D: Dynamics> DynamicsLogger<D> {
    /// Creates a new `DynamicsLogger` wrapped around a `Dynamics` instance
    pub fn new(dynamics: D) -> DynamicsLogger<D> {
        DynamicsLogger(dynamics)
    }
}

impl<D: Dynamics> Dynamics for DynamicsLogger<D> {
    fn update<C: Collisions>(&mut self, collisions: &mut C, time_step: f32) {
        println!("[Dynamics update] START step={}", time_step);
        self.0.update(collisions, time_step);

        for body in collisions.bodies_iter() {
            println!("[Dynamics update] {}", body);
        }

        println!("[Dynamics update] END");
    }

    fn gravity(&self) -> Vector {
        self.0.gravity()
    }

    fn set_gravity(&mut self, gravity: Vector) {
        self.0.set_gravity(gravity)
    }
}
