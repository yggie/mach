use support::Simulation;

use mithril::core::{ State, Transform };
use mithril::maths::Vector;
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
        .execute_multiple_steps(100, 0.2)
        .assert_compliance();
}

#[test]
fn dropping_a_cube_on_a_platform() {
    Simulation::<SimpleCollisions, SimpleDynamics>::new_default()
        .configure(|world| {
            // world.set_gravity(Vector::new(0.0, -4.0, 0.0));
            world.set_gravity(Vector::new(0.0, -0.5, 0.0));

            let material = Rigid::new(1.0);

            let state_0 = State::new_stationary()
                .with_position(0.0,  3.0, 0.0)
                .with_velocity(0.0, -1.0, 0.0)
                .with_angular_velocity(0.3, 0.4, 0.5);
            world.create_body(Cube::new(1.0, 1.0, 1.0), material.clone(), state_0);

            let transform_1 = Transform::new_identity();
            // TODO investigate why this causes a crash
            // world.create_static_body(Cube::new(100.0, 0.1, 100.0), material.clone(), transform_1);
            world.create_static_body(Cube::new(10.0, 0.1, 10.0), material.clone(), transform_1);
        })
        .execute_multiple_steps(100, 0.2)
        .assert_compliance();
}
