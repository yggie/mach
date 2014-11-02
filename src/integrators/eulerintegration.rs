use core::Body;

#[cfg(test)]
#[path="../../tests/unit/integrators/eulerintegration_test.rs"]
mod tests;

/// Applies Euler integration on the `Body` with the specified time step.
pub fn euler_integration(body: &mut Body, time_step: f32) {
    // TODO rotation component
    // TODO deal with temporaries
    let v = body.velocity();
    let p = body.position();
    let i = body.impulse;
    body.set_velocity_with_vector(v + i * time_step);
    let v2 = body.velocity();
    body.set_position_with_vector(p + v2 * time_step);
}
