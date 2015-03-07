use core::{ Body, UID };
use math::{ Vector, Quaternion };
use dynamics::{ Dynamics, ForceAccumulator };
use collisions::{ Contact, Collisions };

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

    #[allow(non_snake_case)]
    fn solve_for_contact(&mut self, body_0: &Body<UID>, body_1: &Body<UID>, contact: &Contact<UID>) {
        // TODO compute dynamically
        let epsilon = 1.0;
        // body masses
        let M = [body_0.mass(), body_1.mass()];
        let Jinv = [body_0.inertia().inverse(), body_1.inertia().inverse()];
        // body velocities
        let v = [body_0.velocity(), body_1.velocity()];
        // body angular velocities
        let w = [body_0.angular_velocity(), body_1.angular_velocity()];
        // relative vector from position to contact center
        let to_contact_center = [
            contact.center - body_0.position(),
            contact.center - body_1.position(),
        ];
        // perpendicular vector (to contact normal) from position to
        // contact center
        let r = [
            to_contact_center[0].cross(contact.normal),
            to_contact_center[1].cross(contact.normal),
        ];

        let impulse = - (1.0 + epsilon) *
            (contact.normal.dot(v[0] - v[1]) + w[0].dot(r[0]) - w[1].dot(r[1])) /
            (1.0/M[0] + 1.0/M[1] + r[0].dot(Jinv[0]*r[0]) + r[1].dot(Jinv[1]*r[1]));

        let impulse_vector = contact.normal * impulse;
        self.accumulator.add_impulse(body_0,  impulse_vector, contact.center);
        self.accumulator.add_impulse(body_1, -impulse_vector, contact.center);
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
                    self.solve_for_contact(body_0, body_1, contact);
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
            let (accumulated_force, accumulated_torque) = self.accumulator.consume_forces(&body);
            let normalized_force = accumulated_force / body.mass();
            body.set_velocity_with_vector(v + normalized_force + scaled_gravity);
            let q = Quaternion::new_from_vector(body.angular_velocity()) *
                Quaternion::new_from_vector(accumulated_torque);
            body.set_angular_velocity_with_vector(Vector::new(q[1], q[2], q[3]));
            let new_velocity = body.velocity();
            body.set_position_with_vector(p + new_velocity * time_step);
            let new_q = body.rotation_quaternion() * q;
            body.set_rotation_with_quaternion(new_q);
        }
    }

    fn gravity(&self) -> Vector {
        self.gravity
    }

    fn set_gravity(&mut self, gravity: Vector) {
        self.gravity = gravity;
    }
}
