use utils::{ CollisionsMonitor, DynamicsMonitor };

use mithril::World;
use mithril::core::State;
use mithril::math::Vector;
use mithril::shapes::Cube;
use mithril::materials::Rigid;
use mithril::dynamics::SimpleDynamics;
use mithril::collisions::SimpleCollisions;

#[test]
fn colliding_two_cubes() {
    let collisions = SimpleCollisions::new();
    let dynamics = SimpleDynamics::new();
    let mut world = World::new(
        CollisionsMonitor::new(collisions),
        DynamicsMonitor::new(dynamics),
    );

    let base_shape = Cube::new(1.0, 1.0, 1.0);
    let base_material = Rigid::new(1.0);

    let state_0 =  State::new_stationary()
        .with_position(0.0,  3.0, 0.0)
        .with_velocity(0.0, -1.0, 0.0);
    world.create_body(base_shape.clone(), base_material.clone(), state_0);

    let state_1 =  State::new_stationary()
        .with_position(0.0, -3.0, 0.0)
        .with_rotation(Vector::new(1.0, 1.0, 0.0), 1.0)
        .with_velocity(0.0,  1.0, 0.0);
    world.create_body(base_shape.clone(), base_material.clone(), state_1);

    for _ in (0..100) {
        world.update(0.05);
    }

    // TODO implement test violations assertions
}
