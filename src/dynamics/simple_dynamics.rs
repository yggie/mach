use core::UID;
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
    fn solve_for_contact<C: Collisions>(&mut self, collisions: &C, uid_0: UID, uid_1: UID, contact: &Contact) -> ((Vector, Vector), (Vector, Vector)) {
        let body_0 = collisions.find_body(uid_0).expect("A RigidBody went missing!");
        let body_1 = collisions.find_body(uid_1).expect("A RigidBody went missing!");

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
    fn solve_for_contact_with_static<C: Collisions>(&mut self, collisions: &C, rigid_body_uid: UID, contact: &Contact) -> (Vector, Vector) {
        let rigid_body = collisions.find_body(rigid_body_uid).expect("A RigidBody went missing!");

        // TODO compute dynamically
        let epsilon = 1.0;
        // relative vector from position to contact center
        let to_contact_center = contact.center - rigid_body.position();
        // axis of rotation for the impulse introduced by the contact. The axis
        // has been scaled by the distance to the contact.
        let k_scaled = to_contact_center.cross(contact.normal);

        let v = rigid_body.velocity();
        let w = rigid_body.angular_velocity();
        let Jinv = rigid_body.inertia().inverse();

        let impulse = - (1.0 + epsilon) *
            (contact.normal.dot(v) + w.dot(k_scaled)) /
            (1.0/rigid_body.mass() + k_scaled.dot(Jinv*k_scaled));

        let velocity_change = contact.normal * impulse;
        let angular_velocity_change = Jinv*to_contact_center.cross(velocity_change);

        return (velocity_change, angular_velocity_change);
    }

    fn update_rigid_body<C: Collisions>(&self, collisions: &mut C, uid: UID, change: (Vector, Vector)) {
        let mut rigid_body = collisions.find_body_mut(uid).unwrap();
        let v = rigid_body.velocity();
        let w = rigid_body.angular_velocity();
        rigid_body.set_velocity_with_vector(v + change.0);
        rigid_body.set_angular_velocity_with_vector(w + change.1);
    }
}

impl Dynamics for SimpleDynamics {
    fn update<C: Collisions>(&mut self, collisions: &mut C, time_step: f32) {
        if let Some(contacts) = collisions.find_contacts() {
            println!("CONTACTS FOUND ({})", contacts.len());

            let contacts: Vec<Contact> = contacts.iter().map(|a| a.clone()).collect();
            let contact = contacts[0];

            match contact.constraint {
                Constraint::RigidRigid(uid_0, uid_1) => {
                    let (change_0, change_1) = self.solve_for_contact(collisions, uid_0, uid_1, &contact);

                    self.update_rigid_body(collisions, uid_0, change_0);
                    self.update_rigid_body(collisions, uid_1, change_1);
                },

                Constraint::RigidStatic { rigid_id, static_id: _ } => {
                    let change = self.solve_for_contact_with_static(collisions, rigid_id, &contact);

                    self.update_rigid_body(collisions, rigid_id, change);
                },
            }
        }


        let scaled_gravity = self.gravity * time_step;
        for mut body in collisions.bodies_iter_mut() {
            // TODO deal with temporaries
            let t = time_step;
            let p = body.position();
            let v = body.velocity();
            let new_velocity = v + scaled_gravity;
            body.set_velocity_with_vector(new_velocity);
            body.set_position_with_vector(p + new_velocity * t);

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
