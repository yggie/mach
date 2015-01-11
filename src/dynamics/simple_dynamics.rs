use space::Space;
use dynamics::Dynamics;

#[cfg(test)]
#[path="../../tests/dynamics/simple_dynamics_test.rs"]
mod tests;

/// Contains the simplest implementation for a time marching scheme.
#[derive(Clone, Copy)]
pub struct SimpleDynamics;

impl SimpleDynamics {
    /// Instantiates a new `SimpleDynamics` object.
    pub fn new() -> SimpleDynamics {
        SimpleDynamics
    }
}

impl Dynamics for SimpleDynamics {
    fn update<S: Space>(&mut self, space: &mut S, time_step: f32) {
        let contacts = space.find_contacts();

        for contact in contacts.iter() {
            match contact.deref_bodies(space) {
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

        for body in space.bodies_mut() {
            // TODO rotation component
            // TODO deal with temporaries
            let v = body.velocity();
            let p = body.position();
            let i = body.accumulated_force();
            body.set_velocity_with_vector(v + i * time_step);
            let v2 = body.velocity();
            body.set_position_with_vector(p + v2 * time_step);
            body.reset_accumulators();
        }
    }
}
