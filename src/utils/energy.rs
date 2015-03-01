use core::{ Body, Handle };

/// Computes the total kinetic energy for the `Body`.
pub fn kinetic_energy_for<H: Handle>(body: &Body<H>) -> f32 {
    translational_kinetic_energy_for(body) +
        rotational_kinetic_energy_for(body)
}

/// Computes the translational component of the kinetic energy for the `Body`.
pub fn translational_kinetic_energy_for<H: Handle>(body: &Body<H>) -> f32 {
    body.mass() * body.velocity().length_sq() / 2.0
}

/// Computes the rotational component of the kinetic energy for the `Body`.
pub fn rotational_kinetic_energy_for<H: Handle>(body: &Body<H>) -> f32 {
    (body.inertia() * body.angular_velocity()).dot(body.angular_velocity()) / 2.0
}
