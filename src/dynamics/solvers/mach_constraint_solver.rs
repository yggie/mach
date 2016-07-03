use std::mem;

use Scalar;
use maths::{LCP, Vec3D};
use dynamics::{ConstraintSolver, Integrator, RigidBody};
use collisions::NarrowphaseData;

static NUM_COMPONENTS: usize = 2;

pub struct MachConstraintSolver;

impl MachConstraintSolver {
    pub fn new() -> MachConstraintSolver {
        MachConstraintSolver
    }

    // fn formulate_lcp(time_step: Scalar, contacts: &Vec<ContactEvent>) -> (LCP, Vec<UnitVec3D>) {
    //     let number_of_contacts = contacts.len();
    //     let size = number_of_contacts * NUM_COMPONENTS;
    //     let mut problem = LCP::new(size);
    //     let mut friction_directions: Vec<UnitVec3D> = Vec::new();
    //
    //     for (i, contact_event) in contacts.iter().enumerate() {
    //         // TODO handle more than one contact point
    //         // NOTE simple approximation of contact center
    //         let contact_center = contact_event.points().iter()
    //             .fold(Vec3D::zero(), |total, point| {
    //                 total + point
    //             }) / contact_event.points().len() as Scalar;
    //         let contact_normal = contact_event.normal();
    //         let body_0 = contact_event.bodies().0.borrow();
    //         let body_1 = contact_event.bodies().1.borrow();
    //
    //         let (mu, mass_inverse, inertia_inverse, rel_vel, contact_offset) = match (body_0.downcast(), body_1.downcast()) {
    //             (BodyRef::Rigid(rigid_body_0), BodyRef::Rigid(rigid_body_1)) => {
    //                 let contact_offset_0 = contact_center - rigid_body_0.translation();
    //                 let contact_offset_1 = contact_center - rigid_body_1.translation();
    //
    //                 let mu = rigid_body_0.friction_coefficient() * rigid_body_1.friction_coefficient();
    //                 let mass_inverse = (rigid_body_0.mass_inverse(), rigid_body_1.mass_inverse());
    //                 let inertia_inverse = (rigid_body_0.inertia_inverse(), rigid_body_1.inertia_inverse());
    //
    //                 let rel_vel = rigid_body_0.velocity() - rigid_body_1.velocity() +
    //                     contact_offset_0.cross(rigid_body_0.angular_velocity().clone()) -
    //                     contact_offset_1.cross(rigid_body_1.angular_velocity().clone());
    //
    //                 (mu, mass_inverse, inertia_inverse, rel_vel, (contact_offset_0, contact_offset_1))
    //             },
    //
    //             (BodyRef::Rigid(rigid_body), BodyRef::Static(static_body)) => {
    //                 let contact_offset_0 = contact_center - rigid_body.translation();
    //                 let contact_offset_1 = contact_center - static_body.translation();
    //
    //                 let mu = rigid_body.friction_coefficient() * static_body.friction_coefficient();
    //                 let mass_inverse = (rigid_body.mass_inverse(), 0.0);
    //                 let inertia_inverse = (rigid_body.inertia_inverse(), Matrix::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
    //
    //                 let rel_vel = rigid_body.velocity() +
    //                     contact_offset_0.cross(rigid_body.angular_velocity().clone());
    //
    //                 (mu, mass_inverse, inertia_inverse, rel_vel, (contact_offset_0, contact_offset_1))
    //             },
    //
    //             _otherwise => panic!("unhandled body type combination!"),
    //         };
    //
    //         mem::drop(body_0);
    //         mem::drop(body_1);
    //
    //         let impulse = ImpulseSolver::compute_impulse_for_event(contact_event);
    //         let impulse = if impulse < 0.0 {
    //             0.0
    //         } else {
    //             impulse
    //         };
    //         let impulse_offset = i * NUM_COMPONENTS;
    //         let friction_offset = impulse_offset + 1;
    //
    //         let generalized_mass_inverse = |vect: UnitVec3D| -> Vec3D {
    //             (mass_inverse.0 + mass_inverse.1) * vect +
    //                 contact_offset.0.cross(inertia_inverse.0 * contact_offset.0.cross(vect)) +
    //                 contact_offset.1.cross(inertia_inverse.1 * contact_offset.1.cross(vect))
    //         };
    //
    //         let generalized_mass_inverse_norm = generalized_mass_inverse(contact_normal);
    //
    //         let mut generator = UnitVec3DGenerator::new();
    //         // FRICTION
    //         let mut perpendicular_direction = rel_vel.cross(contact_normal).normalize();
    //         // ASSUMPTION: Non-finite length means that the relative velocity
    //         // and contact normal directions are aligned
    //         while !perpendicular_direction.squared_length().is_finite() {
    //             // pick any arbitrary direction to avoid the singularity when
    //             // relative velocity is aligned  with the contact normal
    //             let guess = Vec3D::from(generator.next());
    //             perpendicular_direction = rel_vel.cross(guess).normalize();
    //         };
    //         let friction_direction = -contact_normal.cross(perpendicular_direction);
    //         let generalized_mass_inverse_fric = generalized_mass_inverse(friction_direction);
    //
    //         *problem.matrix_mut(friction_offset, friction_offset) = friction_direction.dot(generalized_mass_inverse_fric);
    //         *problem.matrix_mut(impulse_offset, friction_offset) = contact_normal.dot(generalized_mass_inverse_fric);
    //         *problem.matrix_mut(friction_offset, impulse_offset) = friction_direction.dot(generalized_mass_inverse_norm);
    //
    //         *problem.unknown_mut(friction_offset) = time_step * mu * impulse;
    //
    //         *problem.bias_mut(friction_offset) = friction_direction.dot(rel_vel);
    //
    //         let max_friction_impulse = time_step * rel_vel.dot(friction_direction).abs();
    //         problem.add_value_constraint(friction_offset, Box::new(move |local_problem, value| {
    //             if value < 0.0 {
    //                 0.0
    //             } else {
    //                 let friction_limit_due_to_normal_impulse = mu * local_problem.solution(impulse_offset);
    //                 let friction_limit = if max_friction_impulse > friction_limit_due_to_normal_impulse {
    //                     friction_limit_due_to_normal_impulse
    //                 } else {
    //                     max_friction_impulse
    //                 };
    //
    //                 if value > friction_limit {
    //                     friction_limit
    //                 } else {
    //                     value
    //                 }
    //             }
    //         }));
    //
    //         friction_directions.push(friction_direction);
    //
    //         // IMPULSE
    //         let impulse_diagonal = contact_normal.dot(generalized_mass_inverse_norm);
    //         *problem.matrix_mut(impulse_offset, impulse_offset) = impulse_diagonal;
    //         *problem.unknown_mut(impulse_offset) = time_step * impulse;
    //         // TODO this satisfies the constraints when friction is zero, but
    //         // is it theoretically valid?
    //         *problem.bias_mut(impulse_offset) = problem.solution(impulse_offset) * impulse_diagonal;
    //
    //         let impulse_limit = time_step * impulse;
    //         problem.add_value_constraint(impulse_offset, Box::new(move |_problem: &LCP, value: Scalar| -> Scalar {
    //             if value < 0.0 {
    //                 0.0
    //             } else if value > impulse_limit {
    //                 impulse_limit
    //             } else {
    //                 value
    //             }
    //         }));
    //     }
    //
    //     return (problem, friction_directions);
    // }

    // fn apply_lcp_solution(problem: LCP, friction_directions: Vec<UnitVec3D>, time_step: Scalar, contact_events: &Vec<ContactEvent>) {
    //     for (i, contact_event) in contact_events.iter().enumerate() {
    //         let impulse_offset = NUM_COMPONENTS * i;
    //         let friction_offset = impulse_offset + 1;
    //
    //         let friction_direction = &friction_directions[i];
    //
    //         let contact_normal = contact_event.normal();
    //         let contact_center = contact_event.point(0);
    //         let penetration_depth = contact_event.penetration_depth(0);
    //         let mut body_0 = contact_event.bodies().0.borrow_mut();
    //         let mut body_1 = contact_event.bodies().1.borrow_mut();
    //
    //         match (body_0.downcast_mut(), body_1.downcast_mut()) {
    //             (BodyRefMut::Rigid(rigid_body_0), BodyRefMut::Rigid(rigid_body_1)) => {
    //                 let velocity_change = (contact_normal * problem.solution(impulse_offset)
    //                     + friction_direction * problem.solution(friction_offset))
    //                     / time_step;
    //
    //                 let mass = [rigid_body_0.mass(), rigid_body_1.mass()];
    //                 let inertia_inverse = [rigid_body_0.inertia().inverse(), rigid_body_1.inertia().inverse()];
    //                 // relative vector from position to contact center
    //                 let to_contact_center = [
    //                     contact_center - rigid_body_0.translation(),
    //                     contact_center - rigid_body_1.translation(),
    //                 ];
    //
    //                 let angular_velocity_change_0 = inertia_inverse[0]*to_contact_center[0].cross( velocity_change);
    //                 let angular_velocity_change_1 = inertia_inverse[1]*to_contact_center[1].cross(-velocity_change);
    //
    //                 let correction = -0.5 * penetration_depth * contact_normal;
    //                 let change_0 = (velocity_change / mass[0], angular_velocity_change_0);
    //                 MachConstraintSolver::update_rigid_body(rigid_body_0, change_0, time_step, correction);
    //
    //                 let change_1 = (-velocity_change / mass[1], angular_velocity_change_1);
    //                 MachConstraintSolver::update_rigid_body(rigid_body_1, change_1, 0.0, -correction);
    //             },
    //
    //             (BodyRefMut::Rigid(rigid_body), BodyRefMut::Static(_static_body)) => {
    //                 let velocity_change = (contact_normal * problem.solution(impulse_offset)
    //                     + friction_direction * problem.solution(friction_offset))
    //                     / time_step;
    //
    //                 let to_contact_center = contact_center - rigid_body.translation();
    //
    //                 let angular_velocity_change = rigid_body.inertia().inverse()*to_contact_center.cross(velocity_change);
    //
    //                 let correction = -0.5 * penetration_depth * contact_normal;
    //                 let change = (velocity_change / rigid_body.mass(), angular_velocity_change);
    //                 MachConstraintSolver::update_rigid_body(rigid_body, change, 0.0, correction);
    //             },
    //
    //             _otherwise => panic!("unhandled body combination"),
    //         }
    //     }
    // }

    fn update_rigid_body<I, T>(rigid_body: &mut RigidBody<T>, integrator: &I, change: (Vec3D, Vec3D), remaining_time: Scalar, correction: Vec3D) where I: Integrator, T: NarrowphaseData {
        *rigid_body.velocity_mut() += change.0;
        *rigid_body.angular_velocity_mut() += change.1;
        *rigid_body.translation_mut() += correction;

        // TODO missing gravity!
        integrator.integrate_in_place(rigid_body.as_integratable(), remaining_time, Vec3D::zero());
    }
}

// impl ConstraintSolver for MachConstraintSolver {
//     fn solve_with_contacts(&mut self, time_step: Scalar, contact_events: &Vec<ContactEvent>) {
//         let (mut problem, friction_directions) = MachConstraintSolver::formulate_lcp(time_step, contact_events);
//
//         lcp_solvers::GaussSeidel.solve_in_place(&mut problem);
//
//         MachConstraintSolver::apply_lcp_solution(problem, friction_directions, time_step, contact_events);
//     }
// }

// use Scalar;
// use maths::{CrossProduct, DotProduct, UnitVec3D, Vec3D};
// use entities::{BodyRef, RigidBody, StaticBody};
// use detection::ContactEvent;
//
// pub struct ImpulseSolver;
//
// impl ImpulseSolver {
//     pub fn compute_impulse_for_event(contact_event: &ContactEvent) -> Scalar {
//         let contact_center = contact_event.point(0);
//         let contact_normal = contact_event.normal();
//         let body_0 = contact_event.bodies().0.borrow();
//         let body_1 = contact_event.bodies().1.borrow();
//
//         match (body_0.downcast(), body_1.downcast()) {
//             (BodyRef::Rigid(rigid_body_0), BodyRef::Rigid(rigid_body_1)) => {
//                 ImpulseSolver::compute_rigid_rigid_impulse((&rigid_body_0, &rigid_body_1), contact_center, contact_normal)
//             },
//
//             (BodyRef::Rigid(rigid_body), BodyRef::Static(static_body)) => {
//                 ImpulseSolver::compute_rigid_static_impulse((&rigid_body, &static_body), contact_center, contact_normal)
//             },
//
//             _otherwise => panic!("unhandled body combination"),
//         }
//     }
//
//     fn compute_rigid_rigid_impulse(bodies: (&RigidBody, &RigidBody), center: Vec3D, normal: UnitVec3D) -> Scalar {
//         let epsilon = bodies.0.restitution_coefficient() *
//             bodies.1.restitution_coefficient();
//         let mass_inverse = (bodies.0.mass_inverse(), bodies.1.mass_inverse());
//         let inertia_inverse = (bodies.0.inertia_inverse(), bodies.1.inertia_inverse());
//         let velocities = (bodies.0.velocity(), bodies.1.velocity());
//         let angular_velocities = (bodies.0.angular_velocity(), bodies.1.angular_velocity());
//         let to_contact_center = (
//             center - bodies.0.translation(),
//             center - bodies.1.translation(),
//         );
//         let k_scaled = (
//             to_contact_center.0.cross(normal.clone()),
//             to_contact_center.1.cross(normal.clone()),
//         );
//         let velocity_due_to_rotation = (
//             angular_velocities.0.dot(k_scaled.0),
//             angular_velocities.1.dot(k_scaled.1),
//         );
//
//         let numerator = normal.dot(velocities.0 - velocities.1) + velocity_due_to_rotation.0 - velocity_due_to_rotation.1;
//         let denominator = mass_inverse.0 + mass_inverse.1 +
//             k_scaled.0.dot(inertia_inverse.0 * k_scaled.0) +
//             k_scaled.1.dot(inertia_inverse.1 * k_scaled.1);
//
//         - (1.0 + epsilon) * numerator / denominator
//     }
//
//     fn compute_rigid_static_impulse((rigid_body, static_body): (&RigidBody, &StaticBody), center: Vec3D, normal: UnitVec3D) -> Scalar {
//         let epsilon = rigid_body.restitution_coefficient() *
//             static_body.restitution_coefficient();
//         let to_contact_center = center - rigid_body.translation();
//         let k_scaled = to_contact_center.cross(normal.clone());
//         let velocity_due_to_rotation = rigid_body.angular_velocity().dot(k_scaled);
//
//         let numerator = normal.dot(rigid_body.velocity().clone()) + velocity_due_to_rotation;
//         let denominator = rigid_body.mass_inverse() + k_scaled.dot(rigid_body.inertia_inverse() * k_scaled);
//
//         - (1.0 + epsilon) * numerator / denominator
//     }
// }
