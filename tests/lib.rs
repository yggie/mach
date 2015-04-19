extern crate mithril;

use mithril::math::Vector;

#[test]
fn simple_collision_test() {
    let collisions = mithril::collisions::SimpleCollisions::new();
    let dynamics = mithril::dynamics::SimpleDynamics::new();
    let mut world = mithril::World::new(
        mithril::utils::log::CollisionsLogger::new(collisions),
        mithril::utils::log::DynamicsLogger::new(dynamics),
    );

    let base_shape = mithril::shapes::Cube::new(1.0, 1.0, 1.0);
    let base_material = mithril::materials::Rigid::new(1.0);

    let state_0 =  mithril::core::State::new_stationary()
        .with_position(0.0,  3.0, 0.0)
        .with_velocity(0.0, -1.0, 0.0);
    world.create_body(base_shape.clone(), base_material.clone(), state_0);

    let state_1 =  mithril::core::State::new_stationary()
        .with_position(0.0, -3.0, 0.0)
        .with_rotation(Vector::new(1.0, 1.0, 0.0), 1.0)
        .with_velocity(0.0,  1.0, 0.0);
    world.create_body(base_shape.clone(), base_material.clone(), state_1);

    for _ in (0..100) {
        world.update(0.05);
    }

    // TODO implement test violations assertions
}
