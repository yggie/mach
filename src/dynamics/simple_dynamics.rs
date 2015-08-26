use core::Body;
use maths::{ Vector, Quaternion };
use dynamics::Dynamics;
use collisions::{ Contact, Constraint, Collisions };

/// Contains the simplest implementation for a time marching scheme.
pub struct SimpleDynamics {
    gravity: Vector,
}

impl SimpleDynamics {
    /// Instantiates a new `SimpleDynamics` object.
    pub fn new() -> SimpleDynamics {
        SimpleDynamics {
            gravity: Vector::new_zero(),
        }
    }

    #[allow(non_snake_case)]
    fn solve_for_contact(&mut self, body_0: &Body, body_1: &Body, contact: &Contact) -> ((Vector, Vector), (Vector, Vector)) {
        // TODO compute dynamically
        let epsilon = 0.9;
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
        // axis of rotation for the impulse introduced by the contact. The axis
        // has been scaled by the distance to the contact.
        let k_scaled = [
            to_contact_center[0].cross(contact.normal),
            to_contact_center[1].cross(contact.normal),
        ];

        let impulse = - (1.0 + epsilon) *
            (contact.normal.dot(v[0] - v[1]) + w[0].dot(k_scaled[0]) - w[1].dot(k_scaled[1])) /
            (1.0/M[0] + 1.0/M[1] + k_scaled[0].dot(Jinv[0]*k_scaled[0]) + k_scaled[1].dot(Jinv[1]*k_scaled[1]));

        let velocity_change = contact.normal * impulse;
        let angular_velocity_change_0 = Jinv[0]*to_contact_center[0].cross( velocity_change);
        let angular_velocity_change_1 = Jinv[1]*to_contact_center[1].cross(-velocity_change);

        return ((velocity_change, angular_velocity_change_0), (-velocity_change, angular_velocity_change_1));

        // self.accumulator.add_impulse(body_0,  impulse_vector, contact.center);
        // self.accumulator.add_impulse(body_1, -impulse_vector, contact.center);
    }

    #[allow(non_snake_case)]
    fn solve_for_contact_with_static(&mut self, body_0: &Body, contact: &Contact) -> (Vector, Vector) {
        // TODO compute dynamically
        let epsilon = 0.9;
        // relative vector from position to contact center
        let to_contact_center = contact.center - body_0.position();
        // axis of rotation for the impulse introduced by the contact. The axis
        // has been scaled by the distance to the contact.
        let k_scaled = to_contact_center.cross(contact.normal);

        let v = body_0.velocity();
        let w = body_0.angular_velocity();
        let Jinv = body_0.inertia().inverse();

        let impulse = - (1.0 + epsilon) *
            (contact.normal.dot(v) + w.dot(k_scaled)) /
            (1.0/body_0.mass() + k_scaled.dot(Jinv*k_scaled));

        let velocity_change = contact.normal * impulse;
        let angular_velocity_change = Jinv*to_contact_center.cross(velocity_change);

        return (velocity_change, angular_velocity_change);
    }
}

impl Dynamics for SimpleDynamics {
    fn update<C: Collisions>(&mut self, collisions: &mut C, time_step: f32) {
        if let Some(contacts) = collisions.find_contacts() {
            println!("CONTACTS FOUND ({})", contacts.len());

            let contacts: Vec<Contact> = contacts.iter().map(|a| a.clone()).collect();
            let contact = contacts[0];

            match contact.constraint {
                Constraint::RigidRigid(id_0, id_1) => {
                    let dv_0: Vector;
                    let dw_0: Vector;
                    let dv_1: Vector;
                    let dw_1: Vector;

                    {
                        let body_0 = collisions.find_body(id_0).expect("A RigidBody went missing!");
                        let body_1 = collisions.find_body(id_1).expect("A RigidBody went missing!");

                        let tup = self.solve_for_contact(body_0, body_1, &contact);

                        dv_0 = (tup.0).0;
                        dw_0 = (tup.0).1;
                        dv_1 = (tup.1).0;
                        dw_1 = (tup.1).1;
                    }

                    {
                        let body_0 = collisions.find_body_mut(id_0).unwrap();
                        let v = body_0.velocity();
                        let w = body_0.angular_velocity();
                        body_0.set_velocity_with_vector(v + dv_0);
                        body_0.set_angular_velocity_with_vector(w + dw_0);
                    }

                    {
                        let body_1 = collisions.find_body_mut(id_1).unwrap();
                        let v = body_1.velocity();
                        let w = body_1.angular_velocity();
                        body_1.set_velocity_with_vector(v + dv_1);
                        body_1.set_angular_velocity_with_vector(w + dw_1);
                    }
                },

                Constraint::RigidStatic { rigid_id, static_id: _ } => {
                    let dv: Vector;
                    let dw: Vector;

                    {
                        let rigid_body = collisions.find_body(rigid_id).expect("A RigidBody went missing!");

                        let tup = self.solve_for_contact_with_static(rigid_body, &contact);
                        dv = tup.0;
                        dw = tup.1;
                    }

                    let rigid_body = collisions.find_body_mut(rigid_id).expect("A RigidBody went missing!");
                    let v = rigid_body.velocity();
                    let w = rigid_body.angular_velocity();
                    rigid_body.set_velocity_with_vector(v + dv);
                    rigid_body.set_angular_velocity_with_vector(w + dw);
                },
            }
        }


        let scaled_gravity = self.gravity * time_step;
        for body in collisions.bodies_iter_mut() {
            // TODO deal with temporaries
            let t = time_step;
            let p = body.position();
            let v = body.velocity();
            body.set_position_with_vector(p + v * t);
            body.set_velocity_with_vector(v + scaled_gravity);

            let w = body.angular_velocity();
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
