use core::Body;
use math::Vector;
use space::Space;
use dynamics::Dynamics;

#[cfg(test)]
#[path="../../tests/dynamics/simple_dynamics_test.rs"]
mod tests;

/// Contains the simplest implementation for a time marching scheme.
#[derive(Copy)]
pub struct SimpleDynamics {
    gravity: Vector,
}

impl SimpleDynamics {
    /// Instantiates a new `SimpleDynamics` object.
    pub fn new() -> SimpleDynamics {
        SimpleDynamics{
            gravity: Vector::new_zero(),
        }
    }
}

impl Dynamics for SimpleDynamics {
    fn update<S: Space>(&mut self, space: &mut S, time_step: f32) {
        let contacts = space.find_contacts();

        for contact in contacts.iter() {
            let mut bodies: Vec<Option<&mut Body>> = space.get_bodies_mut(vec!(contact.body_ids[0], contact.body_ids[1]));
            let option_1 = bodies.pop().unwrap();
            let option_0 = bodies.pop().unwrap();

            match (option_0, option_1) {
                (Some(body_0), Some(body_1)) => {
                    let masses = [body_0.mass(), body_1.mass()];
                    let relative_velocity = [
                        body_0.velocity().dot(contact.normal),
                        body_1.velocity().dot(contact.normal),
                    ];

                    let impulse = relative_velocity[1]*masses[1] - relative_velocity[0]*masses[0];

                    // TODO compiler satisfying haxxx
                    let pos = [body_0.position(), body_1.position()];
                    body_0.apply_impulse(contact.normal * (-impulse / masses[0]), pos[0]);
                    body_1.apply_impulse(contact.normal * (impulse / masses[1]), pos[1]);
                }

                _ => {
                    panic!("One or more bodies went missing!! [0: {}, 1: {}]", contact.body_ids[0], contact.body_ids[1]);
                }
            }
        }

        let scaled_gravity = self.gravity * time_step;
        for body in space.bodies_mut() {
            // TODO rotation component
            // TODO deal with temporaries
            let v = body.velocity();
            let p = body.position();
            let accumulated_force = body.accumulated_force();
            body.set_velocity_with_vector(v + accumulated_force + scaled_gravity);
            let new_velocity = body.velocity();
            body.set_position_with_vector(p + new_velocity * time_step);
            body.reset_accumulators();
        }
    }

    fn gravity(&self) -> Vector {
        self.gravity
    }

    fn set_gravity(&mut self, gravity: Vector) {
        self.gravity = gravity;
    }
}
