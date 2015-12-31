use Scalar;
use entities::RigidBody;

/// Computes the total kinetic energy for the `RigidBody`.
pub fn kinetic_energy_for(body: &RigidBody) -> Scalar {
    translational_kinetic_energy_for(body) +
        rotational_kinetic_energy_for(body)
}

/// Computes the translational component of the kinetic energy for the
/// `RigidBody`.
pub fn translational_kinetic_energy_for(body: &RigidBody) -> Scalar {
    body.mass() * body.vel().length_sq() / 2.0
}

/// Computes the rotational component of the kinetic energy for the `RigidBody`.
pub fn rotational_kinetic_energy_for(body: &RigidBody) -> Scalar {
    (body.inertia() * body.ang_vel()).dot(*body.ang_vel()) / 2.0
}
