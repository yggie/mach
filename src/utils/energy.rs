use Scalar;
use maths::DotProduct;
use entities::RigidBody;

/// Computes the total kinetic energy for the `RigidBody`.
pub fn kinetic_energy_for(body: &RigidBody) -> Scalar {
    translational_kinetic_energy_for(body) +
        rotational_kinetic_energy_for(body)
}

/// Computes the translational component of the kinetic energy for the
/// `RigidBody`.
pub fn translational_kinetic_energy_for(body: &RigidBody) -> Scalar {
    body.mass() * body.velocity().squared_length() / 2.0
}

/// Computes the rotational component of the kinetic energy for the `RigidBody`.
pub fn rotational_kinetic_energy_for(body: &RigidBody) -> Scalar {
    (body.inertia() * body.angular_velocity()).dot(*body.angular_velocity()) / 2.0
}
