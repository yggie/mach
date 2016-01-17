use {Scalar, TOLERANCE};
use maths::Vect;
use dynamics::{Dynamics, Integrator, SemiImplicitEuler};
use dynamics::solvers::ImpulseSolver;
use entities::{RigidBody, StaticBody};
use detection::{Contact, ContactPair, Space, Intersection};

/// Contains the simplest implementation for a time marching scheme.
pub struct MachDynamics {
    gravity: Vect,
    integrator: SemiImplicitEuler,
}

impl MachDynamics {
    /// Instantiates a new `MachDynamics` object.
    pub fn new() -> MachDynamics {
        MachDynamics {
            gravity: Vect::zero(),
            integrator: SemiImplicitEuler,
        }
    }

    #[allow(non_snake_case)]
    fn solve_for_contact(&mut self, contact: &Contact) -> ((Vect, Vect), (Vect, Vect)) {
        let impulse = ImpulseSolver::compute_impulse_for(contact);

        let M;
        let Jinv;
        let to_contact_center;

        match &contact.pair {
            &ContactPair::RigidRigid(ref cell_0, ref cell_1) => {
                let rigid_body_0 = &cell_0.borrow();
                let rigid_body_1 = &cell_1.borrow();
                M = [rigid_body_0.mass(), rigid_body_1.mass()];
                Jinv = [rigid_body_0.inertia().inverse(), rigid_body_1.inertia().inverse()];
                // relative vector from position to contact center
                to_contact_center = [
                    contact.center - rigid_body_0.translation(),
                    contact.center - rigid_body_1.translation(),
                ];
            },

            _otherwise => panic!("Should never have come here"),
        }

        let impulse = if impulse > TOLERANCE {
            println!("[WARNING] NON-SEPARATING IMPULSE! = {}", impulse);
            0.0
        } else {
            impulse
        };

        let velocity_change = contact.normal * impulse;
        let angular_velocity_change_0 = Jinv[0]*to_contact_center[0].cross( velocity_change);
        let angular_velocity_change_1 = Jinv[1]*to_contact_center[1].cross(-velocity_change);

        return ((velocity_change / M[0], angular_velocity_change_0), (-velocity_change / M[1], angular_velocity_change_1));
    }

    #[allow(non_snake_case)]
    fn solve_for_contact_with_static(&mut self, contact: &Contact) -> (Vect, Vect) {
        let impulse = ImpulseSolver::compute_impulse_for(contact);

        let impulse = if impulse > TOLERANCE {
            println!("[WARNING] NON-SEPARATING IMPULSE! = {}", impulse);
            0.0
        } else {
            impulse
        };

        let m;
        let Jinv;
        let to_contact_center;

        match &contact.pair {
            &ContactPair::RigidStatic(ref cell_0, ref _cell_1) => {
                let rigid_body = &cell_0.borrow();
                m = rigid_body.mass();
                to_contact_center = contact.center - rigid_body.translation();
                Jinv = rigid_body.inertia().inverse();
            },

            _otherwise => panic!("Should never have come here"),
        }

        let velocity_change = contact.normal * impulse;
        let angular_velocity_change = Jinv*to_contact_center.cross(velocity_change);

        return (velocity_change / m, angular_velocity_change);
    }

    fn revert_to_time_of_contact<S: Space>(&self, space: &mut S, current_intersection: Intersection, rigid_body_0: &mut RigidBody, rigid_body_1: &mut RigidBody, time_window: Scalar) -> (Intersection, Scalar) {
        let mut last_intersection: (Intersection, Scalar) = (current_intersection, 0.0);
        let mut did_intersect_last_step = true;
        let mut current_time = time_window;

        for i in 0..5 {
            let multiplier = if did_intersect_last_step {
                -1.0
            } else {
                1.0
            };

            let step = multiplier * time_window / ((2usize << i) as Scalar);
            current_time = current_time + step;

            self.integrator.integrate(rigid_body_0, step, self.gravity);
            self.integrator.integrate(rigid_body_1, step, self.gravity);

            if let Some(intersection) = space.find_intersection(rigid_body_0.form(), rigid_body_1.form()) {
                did_intersect_last_step = true;
                last_intersection = (intersection, current_time);
            } else {
                did_intersect_last_step = false;
            }
        }

        return (last_intersection.0, last_intersection.1);
    }

    fn revert_to_time_of_contact_with_static<S: Space>(&self, space: &mut S, current_intersection: Intersection, rigid_body: &mut RigidBody, static_body: &StaticBody, time_window: Scalar) -> (Intersection, Scalar) {
        let mut last_intersection: (Intersection, Scalar) = (current_intersection, 0.0);
        let mut did_intersect_last_step = true;
        let mut current_time = time_window;

        for i in 0..5 {
            let multiplier = if did_intersect_last_step {
                -1.0
            } else {
                1.0
            };

            let step = multiplier * time_window / ((2usize << i) as Scalar);
            current_time = current_time + step;

            self.integrator.integrate(rigid_body, step, self.gravity);

            if let Some(intersection) = space.find_intersection(rigid_body.form(), static_body.form()) {
                did_intersect_last_step = true;
                last_intersection = (intersection, current_time);
            } else {
                did_intersect_last_step = false;
            }
        }

        return (last_intersection.0, last_intersection.1);
    }

    fn update_rigid_body(&self, rigid_body: &mut RigidBody, change: (Vect, Vect), remaining_time: Scalar, correction: Vect) {
        let v = rigid_body.velocity().clone();
        let w = rigid_body.angular_velocity().clone();
        *rigid_body.velocity_mut() = v + change.0;
        *rigid_body.angular_velocity_mut() = w + change.1;

        let position = rigid_body.translation().clone();
        *rigid_body.translation_mut() = position + correction;

        self.integrator.integrate(rigid_body, remaining_time, self.gravity);
    }
}

impl Dynamics for MachDynamics {
    fn update<S: Space>(&mut self, space: &mut S, time_step: Scalar) -> Option<Vec<Contact>> {
        for mut body in space.bodies_iter_mut() {
            self.integrator.integrate(&mut *body, time_step, self.gravity);
        }

        let contacts_option = space.find_contacts();
        if let &Some(ref contacts) = &contacts_option {
            for contact in contacts.iter() {
                match contact.pair {
                    ContactPair::RigidRigid(ref cell_0, ref cell_1) => {
                        let changes = self.solve_for_contact(contact);

                        let rigid_body_0 = &mut cell_0.borrow_mut();
                        let rigid_body_1 = &mut cell_1.borrow_mut();
                        let current_intersection = Intersection::new(contact.center, contact.normal, contact.penetration_depth);

                        let (_intersection, remaining_time) = self.revert_to_time_of_contact(space, current_intersection, rigid_body_0, rigid_body_1, time_step);

                        let correction = 0.5 * contact.penetration_depth * contact.normal;
                        self.update_rigid_body(rigid_body_0, changes.0, remaining_time,  correction);
                        self.update_rigid_body(rigid_body_1, changes.1, remaining_time, -correction);
                    },

                    ContactPair::RigidStatic(ref cell_0, ref cell_1) => {
                        let change = self.solve_for_contact_with_static(contact);

                        let rigid_body = &mut cell_0.borrow_mut();
                        let static_body = &cell_1.borrow();
                        let current_intersection = Intersection::new(contact.center, contact.normal, contact.penetration_depth);

                        let (intersection, remaining_time) = self.revert_to_time_of_contact_with_static(space, current_intersection, rigid_body, static_body, time_step);

                        let correction = 0.5 * contact.penetration_depth * contact.normal;
                        self.update_rigid_body(rigid_body, change, remaining_time, correction);
                    },
                }
            }
        }

        return contacts_option;
    }

    fn gravity(&self) -> Vect {
        self.gravity
    }

    fn set_gravity(&mut self, gravity: Vect) {
        self.gravity = gravity;
    }
}
