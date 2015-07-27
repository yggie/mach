extern crate mithril;

use std::f32;

use mithril::maths::Vector;
use mithril::dynamics::Dynamics;
use mithril::collisions::Collisions;

/// A utility class which wraps around `Dynamics` components. It produces
/// parseable output for debugging and stores useful information regarding the
/// behaviour of the component.
pub struct DynamicsMonitor<D: Dynamics> {
    dynamics: D,
    previous_total_energy: f32,
    total_energy_violation_count: u32,
}

impl<D: Dynamics> DynamicsMonitor<D> {
    /// Creates a new `DynamicsMonitor` wrapped around a `Dynamics` instance
    pub fn new(dynamics: D) -> DynamicsMonitor<D> {
        DynamicsMonitor {
            dynamics: dynamics,
            previous_total_energy: f32::INFINITY,
            total_energy_violation_count: 0,
        }
    }
}

impl<D: Dynamics> DynamicsMonitor<D> {
    pub fn total_violations(&self) -> u32 {
        self.total_energy_violation_count
    }
}

impl<D: Dynamics> Dynamics for DynamicsMonitor<D> {
    type Identifier = D::Identifier;

    fn update<C: Collisions<Identifier=Self::Identifier>>(&mut self, collisions: &mut C, time_step: f32) {
        println!("[Dynamics update] START step={}", time_step);
        self.dynamics.update(collisions, time_step);

        let total_energy = collisions.bodies_iter()
            .fold(0.0, |cumulative_energy, body| {
                let kinetic_energy = 0.5 * body.mass() * body.velocity().length_sq();
                let potential_energy = body.mass() * body.position().dot(self.gravity());

                println!("[Dynamics update] {}", body);
                return cumulative_energy + kinetic_energy + potential_energy;
            });

        if total_energy > self.previous_total_energy {
            println!("[Violation] Total energy increased by {}", total_energy - self.previous_total_energy);
            self.total_energy_violation_count += 1;
        }
        self.previous_total_energy = total_energy;

        println!("[Dynamics update] END");
    }

    fn gravity(&self) -> Vector {
        self.dynamics.gravity()
    }

    fn set_gravity(&mut self, gravity: Vector) {
        self.dynamics.set_gravity(gravity)
    }
}
