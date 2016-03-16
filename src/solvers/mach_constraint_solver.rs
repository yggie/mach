extern crate rand;

use std::mem;

use self::rand::Rng;

use Scalar;
use maths::{lcp_solvers, Integrator, LCP, LCPSolver, Matrix, Vect};
use maths::integrators::SemiImplicitEuler;
use solvers::{ConstraintSolver, ImpulseSolver};
use entities::{BodyType, BodyTypeMut, RigidBody};
use detection::ContactEvent;

pub struct MachConstraintSolver;

impl MachConstraintSolver {
    pub fn new() -> MachConstraintSolver {
        MachConstraintSolver
    }

    fn formuate_lcp(time_step: Scalar, contacts: &Vec<ContactEvent>) -> (LCP, Vec<Vect>) {
        let number_of_contacts = contacts.len();
        let size = number_of_contacts * 2;
        let mut problem = LCP::new(size);
        let mut friction_directions: Vec<Vect> = Vec::new();

        for (i, contact_event) in contacts.iter().enumerate() {
            // TODO handle more than one contact point
            let contact_center = contact_event.point(0);
            let contact_normal = contact_event.normal();
            let body_0 = contact_event.bodies().0.borrow();
            let body_1 = contact_event.bodies().1.borrow();

            let (mu, mass_inverse, inertia_inverse, rel_vel, contact_offset) = match (body_0.downcast(), body_1.downcast()) {
                (BodyType::Rigid(rigid_body_0), BodyType::Rigid(rigid_body_1)) => {
                    let contact_offset_0 = contact_center - rigid_body_0.translation();
                    let contact_offset_1 = contact_center - rigid_body_1.translation();

                    let mu = rigid_body_0.friction_coefficient() * rigid_body_1.friction_coefficient();
                    let mass_inverse = (rigid_body_0.mass_inverse(), rigid_body_1.mass_inverse());
                    let inertia_inverse = (rigid_body_0.inertia_inverse(), rigid_body_1.inertia_inverse());

                    let rel_vel = rigid_body_0.velocity() - rigid_body_1.velocity() +
                        contact_offset_0.cross(rigid_body_0.angular_velocity().clone()) -
                        contact_offset_1.cross(rigid_body_1.angular_velocity().clone());

                    (mu, mass_inverse, inertia_inverse, rel_vel, (contact_offset_0, contact_offset_1))
                },

                (BodyType::Rigid(rigid_body), BodyType::Static(static_body)) => {
                    let contact_offset_0 = contact_center - rigid_body.translation();
                    let contact_offset_1 = contact_center - static_body.translation();

                    let mu = rigid_body.friction_coefficient() * static_body.friction_coefficient();
                    let mass_inverse = (rigid_body.mass_inverse(), 0.0);
                    let inertia_inverse = (rigid_body.inertia_inverse(), Matrix::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));

                    let rel_vel = rigid_body.velocity() +
                        contact_offset_0.cross(rigid_body.angular_velocity().clone());

                    (mu, mass_inverse, inertia_inverse, rel_vel, (contact_offset_0, contact_offset_1))
                },

                _otherwise => panic!("unhandled body type combination!"),
            };

            mem::drop(body_0);
            mem::drop(body_1);

            let impulse = ImpulseSolver::compute_impulse_for_event(contact_event);
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

            let generalized_mass_inverse_norm = generalized_mass_inverse(contact_normal.clone());

            // FRICTION
            let mut perpendicular_direction = rel_vel.cross(contact_normal.clone()).normalize();
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
            let friction_direction = -contact_normal.cross(perpendicular_direction).normalize();
            let generalized_mass_inverse_fric = generalized_mass_inverse(friction_direction);

            *problem.matrix_mut(friction_offset, friction_offset) = friction_direction.dot(generalized_mass_inverse_fric);
            *problem.matrix_mut(impulse_offset, friction_offset) = contact_normal.dot(generalized_mass_inverse_fric);
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
            let impulse_diagonal = contact_normal.dot(generalized_mass_inverse_norm);
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

    fn apply_lcp_solution(problem: LCP, friction_directions: Vec<Vect>, time_step: Scalar, contact_events: &Vec<ContactEvent>) {
        let number_of_contacts = contact_events.len();
        for (i, contact_event) in contact_events.iter().enumerate() {
            let impulse_offset = number_of_contacts * i;
            let friction_offset = impulse_offset + 1;

            let friction_direction = &friction_directions[i];

            let contact_normal = contact_event.normal();
            let contact_center = contact_event.point(0);
            let penetration_depth = contact_event.penetration_depth(0);
            let mut body_0 = contact_event.bodies().0.borrow_mut();
            let mut body_1 = contact_event.bodies().1.borrow_mut();

            match (body_0.downcast_mut(), body_1.downcast_mut()) {
                (BodyTypeMut::Rigid(rigid_body_0), BodyTypeMut::Rigid(rigid_body_1)) => {
                    let velocity_change = (contact_normal * problem.solution(impulse_offset)
                        + friction_direction * problem.solution(friction_offset))
                        / time_step;

                    let mass = [rigid_body_0.mass(), rigid_body_1.mass()];
                    let inertia_inverse = [rigid_body_0.inertia().inverse(), rigid_body_1.inertia().inverse()];
                    // relative vector from position to contact center
                    let to_contact_center = [
                        contact_center - rigid_body_0.translation(),
                        contact_center - rigid_body_1.translation(),
                    ];

                    let angular_velocity_change_0 = inertia_inverse[0]*to_contact_center[0].cross( velocity_change);
                    let angular_velocity_change_1 = inertia_inverse[1]*to_contact_center[1].cross(-velocity_change);

                    let correction = -0.5 * penetration_depth * contact_normal;
                    let change_0 = (velocity_change / mass[0], angular_velocity_change_0);
                    MachConstraintSolver::update_rigid_body(rigid_body_0, change_0, time_step, correction);

                    let change_1 = (-velocity_change / mass[1], angular_velocity_change_1);
                    MachConstraintSolver::update_rigid_body(rigid_body_1, change_1, 0.0, -correction);
                },

                (BodyTypeMut::Rigid(rigid_body), BodyTypeMut::Static(_static_body)) => {
                    let velocity_change = (contact_normal * problem.solution(impulse_offset)
                        + friction_direction * problem.solution(friction_offset))
                        / time_step;

                    let to_contact_center = contact_center - rigid_body.translation();

                    let angular_velocity_change = rigid_body.inertia().inverse()*to_contact_center.cross(velocity_change);

                    let correction = -0.5 * penetration_depth * contact_normal;
                    let change = (velocity_change / rigid_body.mass(), angular_velocity_change);
                    MachConstraintSolver::update_rigid_body(rigid_body, change, 0.0, correction);
                },

                _otherwise => panic!("unhandled body combination"),
            }
        }
    }

    fn update_rigid_body(rigid_body: &mut RigidBody, change: (Vect, Vect), remaining_time: Scalar, correction: Vect) {
        let v = rigid_body.velocity().clone();
        let w = rigid_body.angular_velocity().clone();
        *rigid_body.velocity_mut() = v + change.0;
        *rigid_body.angular_velocity_mut() = w + change.1;

        let position = rigid_body.translation().clone();
        *rigid_body.translation_mut() = position + correction;

        // TODO missing gravity!
        SemiImplicitEuler.integrate_in_place(&mut rigid_body.as_integratable_mut(), remaining_time, Vect::zero());
    }
}

impl ConstraintSolver for MachConstraintSolver {
    fn solve_with_contacts(&mut self, time_step: Scalar, contact_events: &Vec<ContactEvent>) {
        let (mut problem, friction_directions) = MachConstraintSolver::formuate_lcp(time_step, contact_events);

        lcp_solvers::GaussSeidel.solve_in_place(&mut problem);

        MachConstraintSolver::apply_lcp_solution(problem, friction_directions, time_step, contact_events);
    }
}
