use core::State;
use maths::Vector;
use dynamics::{ Dynamics, SemiImplicitEuler };
use entities::{ RigidBody, StaticBody };
use collisions::{ ContactPair, CollisionSpace };
use collisions::narrowphase::Intersection;

/// Contains the simplest implementation for a time marching scheme.
pub struct SimpleDynamics {
    gravity: Vector,
    integrator: SemiImplicitEuler,
}

impl SimpleDynamics {
    /// Instantiates a new `SimpleDynamics` object.
    pub fn new() -> SimpleDynamics {
        SimpleDynamics {
            gravity: Vector::new_zero(),
            integrator: SemiImplicitEuler,
        }
    }

    #[allow(non_snake_case)]
    fn solve_for_contact(&mut self, rigid_body_0: &RigidBody, rigid_body_1: &RigidBody, contact_center: Vector, contact_normal: Vector) -> ((Vector, Vector), (Vector, Vector)) {
        let epsilon = rigid_body_0.coefficient_of_restitution() * rigid_body_1.coefficient_of_restitution();
        // body masses
        let M = [rigid_body_0.mass(), rigid_body_1.mass()];
        let Jinv = [rigid_body_0.inertia().inverse(), rigid_body_1.inertia().inverse()];
        // body velocities
        let v = [rigid_body_0.velocity(), rigid_body_1.velocity()];
        // body angular velocities
        let w = [rigid_body_0.angular_velocity(), rigid_body_1.angular_velocity()];
        // relative vector from position to contact center
        let to_contact_center = [
            contact_center - rigid_body_0.position(),
            contact_center - rigid_body_1.position(),
        ];
        // axis of rotation for the impulse introduced by the contact. The axis
        // has been scaled by the distance to the contact.
        let k_scaled = [
            to_contact_center[0].cross(contact_normal),
            to_contact_center[1].cross(contact_normal),
        ];

        let impulse = - (1.0 + epsilon) *
            (contact_normal.dot(v[0] - v[1]) + w[0].dot(k_scaled[0]) - w[1].dot(k_scaled[1])) /
            (1.0/M[0] + 1.0/M[1] + k_scaled[0].dot(Jinv[0]*k_scaled[0]) + k_scaled[1].dot(Jinv[1]*k_scaled[1]));

        let velocity_change = contact_normal * impulse;
        let angular_velocity_change_0 = Jinv[0]*to_contact_center[0].cross( velocity_change);
        let angular_velocity_change_1 = Jinv[1]*to_contact_center[1].cross(-velocity_change);

        return ((velocity_change / M[0], angular_velocity_change_0), (-velocity_change / M[1], angular_velocity_change_1));
    }

    #[allow(non_snake_case)]
    fn solve_for_contact_with_static(&mut self, rigid_body: &RigidBody, static_body: &StaticBody, contact_center: Vector, contact_normal: Vector) -> (Vector, Vector) {
        let epsilon = rigid_body.coefficient_of_restitution() * static_body.coefficient_of_restitution();
        // relative vector from position to contact center
        let to_contact_center = contact_center - rigid_body.position();
        // axis of rotation for the impulse introduced by the contact. The axis
        // has been scaled by the distance to the contact.
        let k_scaled = to_contact_center.cross(contact_normal);

        let m = rigid_body.mass();
        let v = rigid_body.velocity();
        let w = rigid_body.angular_velocity();
        let Jinv = rigid_body.inertia().inverse();

        let impulse = - (1.0 + epsilon) *
            (contact_normal.dot(v) + w.dot(k_scaled)) /
            (1.0/m + k_scaled.dot(Jinv*k_scaled));

        let velocity_change = contact_normal * impulse;
        let angular_velocity_change = Jinv*to_contact_center.cross(velocity_change);

        return (velocity_change / m, angular_velocity_change);
    }

    fn revert_to_time_of_contact<C: CollisionSpace>(&self, collision_space: &mut C, current_intersection: Intersection, rigid_body_0: &mut RigidBody, rigid_body_1: &mut RigidBody, time_window: f32) -> (Intersection, f32) {
        let mut last_intersection: (Intersection, f32, State, State) = (current_intersection, 0.0, rigid_body_0.state().clone(), rigid_body_1.state().clone());
        let mut did_intersect_last_step = true;
        let mut current_time = time_window;

        for i in (0..5) {
            let multiplier = if did_intersect_last_step {
                -1.0
            } else {
                1.0
            };

            let step = multiplier * time_window / ((2usize << i) as f32);
            current_time = current_time + step;

            self.integrator.integrate_in_place(rigid_body_0.state_mut(), step, self.gravity);
            self.integrator.integrate_in_place(rigid_body_1.state_mut(), step, self.gravity);

            if let Some(intersection) = collision_space.find_intersection(rigid_body_0, rigid_body_1) {
                did_intersect_last_step = true;
                last_intersection = (intersection, current_time, rigid_body_0.state().clone(), rigid_body_1.state().clone());
            } else {
                did_intersect_last_step = false;
            }
        }

        return (last_intersection.0, last_intersection.1);
    }

    fn revert_to_time_of_contact_with_static<C: CollisionSpace>(&self, collision_space: &mut C, current_intersection: Intersection, rigid_body: &mut RigidBody, static_body: &StaticBody, time_window: f32) -> (Intersection, f32) {
        // let intersection_option = collision_space.find_intersection(rigid_body, static_body);
        // debug_assert!(intersection_option.is_some(), "find_intersection returned false when there was a contact!");
        let mut last_intersection: (Intersection, f32, State) = (current_intersection, 0.0, rigid_body.state().clone());
        let mut did_intersect_last_step = true;
        let mut current_time = time_window;

        for i in (0..5) {
            let multiplier = if did_intersect_last_step {
                -1.0
            } else {
                1.0
            };

            let step = multiplier * time_window / ((2usize << i) as f32);
            current_time = current_time + step;

            self.integrator.integrate_in_place(rigid_body.state_mut(), step, self.gravity);

            if let Some(intersection) = collision_space.find_intersection(rigid_body, static_body) {
                did_intersect_last_step = true;
                last_intersection = (intersection, current_time, rigid_body.state().clone());
            } else {
                did_intersect_last_step = false;
            }
        }

        return (last_intersection.0, last_intersection.1);
    }

    fn update_rigid_body(&self, rigid_body: &mut RigidBody, change: (Vector, Vector), remaining_time: f32) {
        let v = rigid_body.velocity();
        let w = rigid_body.angular_velocity();
        rigid_body.set_velocity_with_vector(v + change.0);
        rigid_body.set_angular_velocity_with_vector(w + change.1);

        self.integrator.integrate_in_place(rigid_body.state_mut(), remaining_time, self.gravity);
    }
}

impl Dynamics for SimpleDynamics {
    fn update<C: CollisionSpace>(&mut self, collision_space: &mut C, time_step: f32) {
        for mut body in collision_space.bodies_iter_mut() {
            self.integrator.integrate_in_place(body.state_mut(), time_step, self.gravity);
        }

        if let Some(contacts) = collision_space.find_contacts() {
            println!("CONTACTS FOUND ({})", contacts.len());

            for contact in contacts.iter().take(1) {
                println!("HANDLING CONTACT {} FACING {}", contact.center, contact.normal);

                match contact.pair {
                    ContactPair::RigidRigid(ref cell_0, ref cell_1) => {
                        let rigid_body_0 = &mut cell_0.borrow_mut();
                        let rigid_body_1 = &mut cell_1.borrow_mut();
                        let current_intersection = Intersection::new(contact.center, contact.normal);

                        let (intersection, remaining_time) = self.revert_to_time_of_contact(collision_space, current_intersection, rigid_body_0, rigid_body_1, time_step);
                        let changes = self.solve_for_contact(rigid_body_0, rigid_body_1, intersection.point(), intersection.normal());

                        self.update_rigid_body(rigid_body_0, changes.0, remaining_time);
                        self.update_rigid_body(rigid_body_1, changes.1, remaining_time);
                    },

                    ContactPair::RigidStatic(ref cell_0, ref cell_1) => {
                        let rigid_body = &mut cell_0.borrow_mut();
                        let static_body = &mut cell_1.borrow();
                        let current_intersection = Intersection::new(contact.center, contact.normal);

                        let (intersection, remaining_time) = self.revert_to_time_of_contact_with_static(collision_space, current_intersection, rigid_body, static_body, time_step);
                        let change = self.solve_for_contact_with_static(rigid_body, static_body, intersection.point(), intersection.normal());

                        self.update_rigid_body(rigid_body, change, remaining_time);
                    },
                }
            }
        }
    }

    fn gravity(&self) -> Vector {
        self.gravity
    }

    fn set_gravity(&mut self, gravity: Vector) {
        self.gravity = gravity;
    }
}
