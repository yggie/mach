extern crate rand;

use self::rand::Rng;

use Scalar;
use maths::{lcp_solvers, LCP, LCPSolver, Matrix, Vect};
use dynamics::{Dynamics, Integrator, SemiImplicitEuler};
use dynamics::solvers::ImpulseSolver;
use entities::{RigidBody};
use detection::{Contact, ContactPair, Space};

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

    fn update_rigid_body(&self, rigid_body: &mut RigidBody, change: (Vect, Vect), remaining_time: Scalar, correction: Vect) {
        let v = rigid_body.velocity().clone();
        let w = rigid_body.angular_velocity().clone();
        *rigid_body.velocity_mut() = v + change.0;
        *rigid_body.angular_velocity_mut() = w + change.1;

        let position = rigid_body.translation().clone();
        *rigid_body.translation_mut() = position + correction;

        self.integrator.integrate(rigid_body, remaining_time, self.gravity);
    }

    fn formulate_lcp(&self, time_step: Scalar, contacts: &Vec<Contact>) -> (LCP, Vec<Vect>) {
        let number_of_contacts = contacts.len();
        let size = number_of_contacts * 2;
        let mut problem = LCP::new(size);
        let mut friction_directions: Vec<Vect> = Vec::new();

        for (i, contact) in contacts.iter().enumerate() {
            let (mu, mass_inverse, inertia_inverse, rel_vel, contact_offset) = match contact.pair {
                ContactPair::RigidRigid(ref cell_0, ref cell_1) => {
                    let rigid_body_0 = &cell_0.borrow();
                    let rigid_body_1 = &cell_1.borrow();

                    let contact_offset_0 = contact.center - rigid_body_0.translation();
                    let contact_offset_1 = contact.center - rigid_body_1.translation();

                    let mu = rigid_body_0.friction_coefficient() * rigid_body_1.friction_coefficient();
                    let mass_inverse = (rigid_body_0.mass_inverse(), rigid_body_1.mass_inverse());
                    let inertia_inverse = (rigid_body_0.inertia_inverse(), rigid_body_1.inertia_inverse());

                    let rel_vel = rigid_body_0.velocity() - rigid_body_1.velocity() +
                        contact_offset_0.cross(rigid_body_0.angular_velocity().clone()) -
                        contact_offset_1.cross(rigid_body_1.angular_velocity().clone());

                    (mu, mass_inverse, inertia_inverse, rel_vel, (contact_offset_0, contact_offset_1))
                },

                ContactPair::RigidStatic(ref cell_0, ref cell_1) => {
                    let rigid_body = &cell_0.borrow();
                    let static_body = &cell_1.borrow();

                    let contact_offset_0 = contact.center - rigid_body.translation();
                    let contact_offset_1 = contact.center - static_body.translation();

                    let mu = rigid_body.friction_coefficient() * static_body.friction_coefficient();
                    let mass_inverse = (rigid_body.mass_inverse(), 0.0);
                    let inertia_inverse = (rigid_body.inertia_inverse(), Matrix::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));

                    let rel_vel = rigid_body.velocity() +
                        contact_offset_0.cross(rigid_body.angular_velocity().clone());

                    (mu, mass_inverse, inertia_inverse, rel_vel, (contact_offset_0, contact_offset_1))
                },
            };

            let impulse = ImpulseSolver::compute_impulse_for(contact);
            let impulse = if impulse < 0.0 {
                0.0
            } else {
                impulse
            };
            let impulse_offset = number_of_contacts * i;
            let friction_offset = impulse_offset + 1;

            let generalized_mass_inverse = |vect| {
                (mass_inverse.0 + mass_inverse.1) * vect +
                    contact_offset.0.cross(inertia_inverse.0 * contact_offset.0.cross(vect)) +
                    contact_offset.1.cross(inertia_inverse.1 * contact_offset.1.cross(vect))
            };

            let generalized_mass_inverse_norm = generalized_mass_inverse(contact.normal);

            // FRICTION
            let mut perpendicular_direction = rel_vel.cross(contact.normal).normalize();
            // ASSUMPTION: Non-finite length means that the relative velocity
            // and contact normal directions are aligned
            while !perpendicular_direction.length_sq().is_finite() {
                // pick any arbitrary direction to avoid the singularity when
                // relative velocity is aligned  with the contact normal
                let mut rng = rand::thread_rng();
                let guess = Vect::new(
                    rng.gen_range(-1.0, 1.0),
                    rng.gen_range(-1.0, 1.0),
                    rng.gen_range(-1.0, 1.0),
                );
                perpendicular_direction = rel_vel.cross(guess).normalize();
            };
            let friction_direction = -contact.normal.cross(perpendicular_direction).normalize();
            let generalized_mass_inverse_fric = generalized_mass_inverse(friction_direction);

            *problem.matrix_mut(friction_offset, friction_offset) = friction_direction.dot(generalized_mass_inverse_fric);
            *problem.matrix_mut(impulse_offset, friction_offset) = contact.normal.dot(generalized_mass_inverse_fric);
            *problem.matrix_mut(friction_offset, impulse_offset) = friction_direction.dot(generalized_mass_inverse_norm);

            *problem.unknown_mut(friction_offset) = time_step * mu * impulse;

            *problem.bias_mut(friction_offset) = friction_direction.dot(rel_vel);

            let max_friction_impulse = time_step * rel_vel.dot(friction_direction).abs();
            problem.add_value_constraint(friction_offset, Box::new(move |local_problem, value| {
                if value < 0.0 {
                    0.0
                } else {
                    let friction_limit_due_to_normal_impulse = mu * local_problem.solution(impulse_offset);
                    let friction_limit = if max_friction_impulse > friction_limit_due_to_normal_impulse {
                        friction_limit_due_to_normal_impulse
                    } else {
                        max_friction_impulse
                    };

                    if value > friction_limit {
                        friction_limit
                    } else {
                        value
                    }
                }
            }));

            friction_directions.push(friction_direction);

            // IMPULSE
            let impulse_diagonal = contact.normal.dot(generalized_mass_inverse_norm);
            *problem.matrix_mut(impulse_offset, impulse_offset) = impulse_diagonal;
            *problem.unknown_mut(impulse_offset) = time_step * impulse;
            // TODO this satisfies the constraints when friction is zero, but
            // is it theoretically valid?
            *problem.bias_mut(impulse_offset) = problem.solution(impulse_offset) * impulse_diagonal;

            let impulse_limit = time_step * impulse;
            problem.add_value_constraint(impulse_offset, Box::new(move |_problem: &LCP, value: Scalar| -> Scalar {
                if value < 0.0 {
                    0.0
                } else if value > impulse_limit {
                    impulse_limit
                } else {
                    value
                }
            }));
        }

        return (problem, friction_directions);
    }

    fn apply_lcp_solution(&self, problem: LCP, friction_directions: Vec<Vect>, time_step: Scalar, contacts: &Vec<Contact>) {
        let number_of_contacts = contacts.len();
        for (i, contact) in contacts.iter().enumerate() {
            let impulse_offset = number_of_contacts * i;
            let friction_offset = impulse_offset + 1;

            let friction_direction = &friction_directions[i];

            match contact.pair {
                ContactPair::RigidRigid(ref cell_0, ref cell_1) => {
                    let rigid_body_0 = &mut cell_0.borrow_mut();
                    let rigid_body_1 = &mut cell_1.borrow_mut();

                    let velocity_change = (contact.normal * problem.solution(impulse_offset)
                        + friction_direction * problem.solution(friction_offset))
                        / time_step;

                    let mass = [rigid_body_0.mass(), rigid_body_1.mass()];
                    let inertia_inverse = [rigid_body_0.inertia().inverse(), rigid_body_1.inertia().inverse()];
                    // relative vector from position to contact center
                    let to_contact_center = [
                        contact.center - rigid_body_0.translation(),
                        contact.center - rigid_body_1.translation(),
                    ];

                    let angular_velocity_change_0 = inertia_inverse[0]*to_contact_center[0].cross( velocity_change);
                    let angular_velocity_change_1 = inertia_inverse[1]*to_contact_center[1].cross(-velocity_change);

                    let correction = -0.5 * contact.penetration_depth * contact.normal;
                    let change_0 = (velocity_change / mass[0], angular_velocity_change_0);
                    self.update_rigid_body(rigid_body_0, change_0, time_step, correction);

                    let change_1 = (-velocity_change / mass[1], angular_velocity_change_1);
                    self.update_rigid_body(rigid_body_1, change_1, 0.0, -correction);
                },

                ContactPair::RigidStatic(ref cell_0, ref _cell_1) => {
                    let rigid_body = &mut cell_0.borrow_mut();

                    let velocity_change = (contact.normal * problem.solution(impulse_offset)
                        + friction_direction * problem.solution(friction_offset))
                        / time_step;

                    let to_contact_center = contact.center - rigid_body.translation();

                    let angular_velocity_change = rigid_body.inertia().inverse()*to_contact_center.cross(velocity_change);

                    let correction = -0.5 * contact.penetration_depth * contact.normal;
                    let change = (velocity_change / rigid_body.mass(), angular_velocity_change);
                    self.update_rigid_body(rigid_body, change, 0.0, correction);
                },
            }
        }
    }
}

impl Dynamics for MachDynamics {
    fn update<S: Space>(&mut self, space: &mut S, time_step: Scalar) -> Option<Vec<Contact>> {
        for mut body in space.bodies_iter_mut() {
            self.integrator.integrate(&mut *body, time_step, self.gravity);
        }

        let contacts_option = space.find_contacts();
        if let &Some(ref contacts) = &contacts_option {
            let (mut problem, friction_directions) = self.formulate_lcp(time_step, contacts);

            lcp_solvers::GaussSeidel.solve_in_place(&mut problem);

            self.apply_lcp_solution(problem, friction_directions, time_step, contacts);
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
