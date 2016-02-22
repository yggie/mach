use Scalar;
use maths::Vect;
use entities::{BodyType, RigidBody, StaticBody};
use detection::{Contact, ContactEvent, ContactPair};

pub struct ImpulseSolver;

impl ImpulseSolver {
    pub fn compute_impulse_for_event(contact_event: &ContactEvent) -> Scalar {
        let contact_center = contact_event.contact_set.point(0);
        let contact_normal = contact_event.contact_set.surface_normal();
        let body_0 = contact_event.bodies.0.borrow();
        let body_1 = contact_event.bodies.1.borrow();

        match (body_0.downcast(), body_1.downcast()) {
            (BodyType::Rigid(rigid_body_0), BodyType::Rigid(rigid_body_1)) => {
                ImpulseSolver::compute_rigid_rigid_impulse((&rigid_body_0, &rigid_body_1), &contact_center, &contact_normal)
            },

            (BodyType::Rigid(rigid_body), BodyType::Static(static_body)) => {
                ImpulseSolver::compute_rigid_static_impulse((&rigid_body, &static_body), &contact_center, &contact_normal)
            },

            _otherwise => panic!("unhandled body combination"),
        }
    }

    pub fn compute_impulse_for(contact: &Contact) -> Scalar {
        match contact.pair {
            ContactPair::RigidRigid(ref cell_0, ref cell_1) => {
                let rigid_body_0 = &cell_0.borrow();
                let rigid_body_1 = &cell_1.borrow();

                ImpulseSolver::compute_rigid_rigid_impulse((&rigid_body_0, &rigid_body_1), &contact.center, &contact.normal)
            },

            ContactPair::RigidStatic(ref cell_0, ref cell_1) => {
                let rigid_body = &cell_0.borrow();
                let static_body = &cell_1.borrow();

                ImpulseSolver::compute_rigid_static_impulse((&rigid_body, &static_body), &contact.center, &contact.normal)
            },
        }
    }

    fn compute_rigid_rigid_impulse(bodies: (&RigidBody, &RigidBody), center: &Vect, normal: &Vect) -> Scalar {
        let epsilon = bodies.0.coefficient_of_restitution() *
            bodies.1.coefficient_of_restitution();
        let mass_inverse = (bodies.0.mass_inverse(), bodies.1.mass_inverse());
        let inertia_inverse = (bodies.0.inertia_inverse(), bodies.1.inertia_inverse());
        let velocities = (bodies.0.velocity(), bodies.1.velocity());
        let angular_velocities = (bodies.0.angular_velocity(), bodies.1.angular_velocity());
        let to_contact_center = (
            center - bodies.0.translation(),
            center - bodies.1.translation(),
        );
        let k_scaled = (
            to_contact_center.0.cross(normal.clone()),
            to_contact_center.1.cross(normal.clone()),
        );
        let velocity_due_to_rotation = (
            angular_velocities.0.dot(k_scaled.0),
            angular_velocities.1.dot(k_scaled.1),
        );

        let numerator = normal.dot(velocities.0 - velocities.1) + velocity_due_to_rotation.0 - velocity_due_to_rotation.1;
        let denominator = mass_inverse.0 + mass_inverse.1 +
            k_scaled.0.dot(inertia_inverse.0 * k_scaled.0) +
            k_scaled.1.dot(inertia_inverse.1 * k_scaled.1);

        - (1.0 + epsilon) * numerator / denominator
    }

    fn compute_rigid_static_impulse((rigid_body, static_body): (&RigidBody, &StaticBody), center: &Vect, normal: &Vect) -> Scalar {
        let epsilon = rigid_body.coefficient_of_restitution() *
            static_body.coefficient_of_restitution();
        let to_contact_center = center - rigid_body.translation();
        let k_scaled = to_contact_center.cross(normal.clone());
        let velocity_due_to_rotation = rigid_body.angular_velocity().dot(k_scaled);

        let numerator = normal.dot(rigid_body.velocity().clone()) + velocity_due_to_rotation;
        let denominator = rigid_body.mass_inverse() + k_scaled.dot(rigid_body.inertia_inverse() * k_scaled);

        - (1.0 + epsilon) * numerator / denominator
    }
}
