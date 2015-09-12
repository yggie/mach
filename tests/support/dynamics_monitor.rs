extern crate mach;

use mach::core::{ Float, INFINITY };
use mach::maths::Vector;
use mach::utils::debug::renderevent;
use mach::dynamics::Dynamics;
use mach::collisions::CollisionSpace;

/// A utility class which wraps around `Dynamics` components. It produces
/// parseable output for debugging and stores useful information regarding the
/// behaviour of the component.
pub struct DynamicsMonitor<D: Dynamics> {
    dynamics: D,
    previous_total_energy: Float,
    total_energy_violation_count: u32,
}

impl<D: Dynamics> DynamicsMonitor<D> {
    /// Creates a new `DynamicsMonitor` wrapped around a `Dynamics` instance
    pub fn new(dynamics: D) -> DynamicsMonitor<D> {
        DynamicsMonitor {
            dynamics: dynamics,
            previous_total_energy: INFINITY,
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
    fn update<C: CollisionSpace>(&mut self, collisions: &mut C, time_step: Float) {
        renderevent::update_start(time_step);
        self.dynamics.update(collisions, time_step);

        let total_energy = collisions.bodies_iter()
            .fold(0.0, |cumulative_energy, cell| {
                let body = &*cell;
                let kinetic_energy = 0.5 * body.mass() * body.velocity().length_sq();
                let potential_energy = body.mass() * body.position().dot(self.gravity());

                renderevent::update_rigid_body(body);
                return cumulative_energy + kinetic_energy + potential_energy;
            });

        for static_body in collisions.static_bodies_iter() {
            renderevent::update_static_body(&*static_body);
        }

        if total_energy > self.previous_total_energy {
            renderevent::violation("ENERGY", &format!("Total energy increased by {}", total_energy - self.previous_total_energy));
            self.total_energy_violation_count += 1;
        }
        self.previous_total_energy = total_energy;
    }

    fn gravity(&self) -> Vector {
        self.dynamics.gravity()
    }

    fn set_gravity(&mut self, gravity: Vector) {
        self.dynamics.set_gravity(gravity)
    }
}
