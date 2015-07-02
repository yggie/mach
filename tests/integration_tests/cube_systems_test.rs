use support::Simulation;

use mithril::core::State;
use mithril::math::Vector;
use mithril::shapes::Cube;
use mithril::materials::Rigid;
use mithril::dynamics::SimpleDynamics;
use mithril::collisions::SimpleCollisions;

#[test]
fn colliding_two_cubes() {
    Simulation::<SimpleCollisions, SimpleDynamics>::new_default()
        .configure(|world| {
            let shape = Cube::new(1.0, 1.0, 1.0);
            let material = Rigid::new(1.0);

            let state_0 =  State::new_stationary()
                .with_position(0.0,  3.0, 0.0)
                .with_velocity(0.0, -1.0, 0.0);
            world.create_body(shape.clone(), material.clone(), state_0);

            let state_1 =  State::new_stationary()
                .with_position(0.0, -3.0, 0.0)
                .with_rotation(Vector::new(1.0, 1.0, 0.0), 1.0)
                .with_velocity(0.0,  1.0, 0.0);
            world.create_body(shape.clone(), material.clone(), state_1);
        })
        .execute_multiple_steps(100, 0.5)
        .assert_compliance();
}
