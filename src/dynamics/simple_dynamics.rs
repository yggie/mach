use core::{ Body, StaticBody };
use math::{ Vector, Quaternion };
use dynamics::{ Dynamics, ForceAccumulator };
use collisions::{ Contact, ContactPair, Collisions };

/// Contains the simplest implementation for a time marching scheme.
pub struct SimpleDynamics {
    gravity: Vector,
    accumulator: ForceAccumulator<usize>,
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
    fn solve_for_contact(&mut self, body_0: &Body<usize>, body_1: &Body<usize>, contact: &Contact<usize>) {
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

    #[allow(non_snake_case)]
    fn solve_for_contact_with_static(&mut self, body_0: &Body<usize>, body_1: &StaticBody<usize>, contact: &Contact<usize>) {
        // TODO compute dynamically
        let epsilon = 1.0;
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
            (contact.normal.dot(body_0.velocity()) + body_0.angular_velocity().dot(r[0])) /
            (1.0/body_0.mass() + r[0].dot(body_0.inertia().inverse()*r[0]));

        let impulse_vector = contact.normal * impulse;
        self.accumulator.add_impulse(body_0,  impulse_vector, contact.center);
    }
}

impl Dynamics for SimpleDynamics {
    type Identifier = usize;

    fn update<C: Collisions<Identifier=Self::Identifier>>(&mut self, collisions: &mut C, time_step: f32) {
        let contacts = collisions.find_contacts();

        for contact in contacts.iter() {
            match contact.ids {
                ContactPair::RigidRigid(id_0, id_1) => {
                    let body_0 = collisions.find_body(id_0).expect("A RigidBody went missing!");
                    let body_1 = collisions.find_body(id_1).expect("A RigidBody went missing!");

                    self.solve_for_contact(body_0, body_1, &contact);
                },

                ContactPair::RigidStatic { rigid_id, static_id } => {
                    let rigid_body = collisions.find_body(rigid_id).expect("A RigidBody went missing!");
                    let static_body = collisions.find_static_body(static_id).expect("A StaticBody went missing!");

                    self.solve_for_contact_with_static(rigid_body, static_body, &contact);
                },
            }
        }

        let scaled_gravity = self.gravity * time_step;
        for body in collisions.bodies_iter_mut() {
            // TODO deal with temporaries
            let t = time_step;
            let v = body.velocity();
            let p = body.position();
            let (accumulated_force, accumulated_torque) = self.accumulator.consume_forces(&body);
            let impulse = accumulated_force / body.mass();
            body.set_velocity_with_vector(v + impulse + scaled_gravity);
            let new_velocity = body.velocity();
            body.set_position_with_vector(p + (new_velocity + impulse) * t);

            let angular_impulse = body.inertia().inverse() * accumulated_torque;
            let w_old = body.angular_velocity();
            body.set_angular_velocity_with_vector(w_old + angular_impulse);

            let w = body.angular_velocity() + angular_impulse;
            let w_as_quat = Quaternion::new(0.0, w[0] * t, w[1] * t, w[2] * t);
            let q = body.rotation_quaternion();
            let new_rotation = q + w_as_quat * q * 0.5;

            body.set_rotation_with_quaternion(new_rotation.normalize());
        }
    }

    fn gravity(&self) -> Vector {
        self.gravity
    }

    fn set_gravity(&mut self, gravity: Vector) {
        self.gravity = gravity;
    }
}
