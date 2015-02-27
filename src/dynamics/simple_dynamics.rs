use core::UID;
use math::Vector;
use dynamics::{ Dynamics, ForceAccumulator };
use collisions::Collisions;

#[cfg(test)]
#[path="../../tests/dynamics/simple_dynamics_test.rs"]
mod tests;

/// Contains the simplest implementation for a time marching scheme.
pub struct SimpleDynamics {
    gravity: Vector,
    accumulator: ForceAccumulator<UID>,
}

impl SimpleDynamics {
    /// Instantiates a new `SimpleDynamics` object.
    pub fn new() -> SimpleDynamics {
        SimpleDynamics {
            gravity: Vector::new_zero(),
            accumulator: ForceAccumulator::new(),
        }
    }
}

impl Dynamics for SimpleDynamics {
    fn update<C: Collisions>(&mut self, collisions: &mut C, time_step: f32) {
        let contacts = collisions.find_contacts();

        for contact in contacts.iter() {
            let option_0 = collisions.find_body(contact.body_ids[0]);
            let option_1 = collisions.find_body(contact.body_ids[1]);

            match (option_0, option_1) {
                (Some(body_0), Some(body_1)) => {
                    let masses = [body_0.mass(), body_1.mass()];
                    let relative_velocity = [
                        body_0.velocity().dot(contact.normal),
                        body_1.velocity().dot(contact.normal),
                    ];

                    let impulse = relative_velocity[1]*masses[1] - relative_velocity[0]*masses[0];

                    self.accumulator.add_impulse(body_0, -contact.normal * impulse / masses[0], contact.center);
                    self.accumulator.add_impulse(body_1, contact.normal * impulse / masses[1], contact.center);
                }

                _ => {
                    panic!("One or more bodies went missing!! [0: {}, 1: {}]", contact.body_ids[0], contact.body_ids[1]);
                }
            }
        }

        let scaled_gravity = self.gravity * time_step;
        for body in collisions.bodies_iter_mut() {
            // TODO rotation component
            // TODO deal with temporaries
            let v = body.velocity();
            let p = body.position();
            let (accumulated_force, _) = self.accumulator.consume_forces(&body);
            body.set_velocity_with_vector(v + accumulated_force + scaled_gravity);
            let new_velocity = body.velocity();
            body.set_position_with_vector(p + new_velocity * time_step);
        }
    }

    fn gravity(&self) -> Vector {
        self.gravity
    }

    fn set_gravity(&mut self, gravity: Vector) {
        self.gravity = gravity;
    }
}
