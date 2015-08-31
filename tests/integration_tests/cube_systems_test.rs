use support::Simulation;

use mach::core::{ State, Transform };
use mach::maths::Vector;
use mach::shapes::Cube;
use mach::materials::Rigid;
use mach::dynamics::SimpleDynamics;
use mach::collisions::SimpleCollisionSpace;

#[test]
fn colliding_two_cubes() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            let shape = Cube::new(1.0, 1.0, 1.0);

            let state_0 =  State::new_stationary()
                .with_position(0.0,  3.0, 0.0)
                .with_velocity(0.0, -1.0, 0.0);
            world.create_body(shape.clone(), Rigid::new(1.0), state_0);

            let state_1 =  State::new_stationary()
                .with_position(0.0, -3.0, 0.0)
                .with_axis_angle(Vector::new(1.0, 1.0, 0.0), 1.0)
                .with_velocity(0.0,  1.0, 0.0);
            world.create_body(shape.clone(), Rigid::new(2.0), state_1);
        })
        .execute_multiple_steps(200, 0.1)
        .assert_compliance();
}

#[test]
fn dropping_a_cube_on_a_platform() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            // world.set_gravity(Vector::new(0.0, -4.0, 0.0));
            world.set_gravity(Vector::new(0.0, 0.0, -0.5));

            let material = Rigid::new(1.0);

            let state_0 = State::new_stationary()
                .with_position(0.0, 0.0,  3.0)
                .with_velocity(0.0, 0.0, -1.0)
                .with_angular_velocity(0.3, 0.4, 0.5);
            world.create_body(Cube::new(1.0, 1.0, 1.0), material.clone(), state_0);

            let transform_1 = Transform::new_identity();
            // TODO investigate why this causes a crash
            // world.create_static_body(Cube::new(100.0, 0.1, 100.0), material.clone(), transform_1);
            world.create_static_body(Cube::new(10.0, 10.0, 0.1), material.clone(), transform_1);
        })
        .execute_multiple_steps(200, 0.1)
        .assert_compliance();
}
