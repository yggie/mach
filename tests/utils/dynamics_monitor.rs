extern crate mithril;

use mithril::math::Vector;
use mithril::dynamics::Dynamics;
use mithril::collisions::Collisions;

/// A utility class which wraps around `Dynamics` components. It produces
/// parseable output for debugging and stores useful information regarding the
/// behaviour of the component.
pub struct DynamicsMonitor<D: Dynamics>(D);

impl<D: Dynamics> DynamicsMonitor<D> {
    /// Creates a new `DynamicsMonitor` wrapped around a `Dynamics` instance
    pub fn new(dynamics: D) -> DynamicsMonitor<D> {
        DynamicsMonitor(dynamics)
    }
}

impl<D: Dynamics> Dynamics for DynamicsMonitor<D> {
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
