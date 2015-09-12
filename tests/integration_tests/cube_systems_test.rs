use support::Simulation;

use mach::maths::{ State, Transform, Vector };
use mach::shapes::{ Cuboid, Sphere };
use mach::entities::Material;
use mach::dynamics::SimpleDynamics;
use mach::collisions::SimpleCollisionSpace;

#[test]
fn colliding_two_cubes() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            let shape = Cuboid::new_cube(1.0);

            let state_0 =  State::new_stationary()
                .with_position(0.0,  3.0, 0.0)
                .with_velocity(0.0, -1.0, 0.0);
            world.create_body(shape.clone(), &Material::new_with_density(1.0), state_0);

            let state_1 =  State::new_stationary()
                .with_position(0.0, -3.0, 0.0)
                .with_axis_angle(Vector::new(1.0, 1.0, 0.0), 1.0)
                .with_velocity(0.0,  1.0, 0.0);
            world.create_body(shape.clone(), &Material::new_with_density(2.0), state_1);
        })
        .execute_multiple_steps(200, 0.1)
        .assert_compliance();
}

#[test]
fn dropping_a_cube_on_a_platform() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            world.set_gravity(Vector::new(0.0, 0.0, -0.5));

            let material = Material::new_with_density(1.0);

            let state_0 = State::new_stationary()
                .with_position(0.0, 0.0,  3.0)
                .with_velocity(0.0, 0.0, -1.0)
                .with_angular_velocity(0.3, 0.4, 0.5);
            world.create_body(Cuboid::new_cube(1.0), &material, state_0);

            let transform_1 = Transform::new_identity();
            world.create_static_body(Cuboid::new(10.0, 10.0, 0.1), &material, transform_1);
        })
        .execute_multiple_steps(200, 0.1)
        .assert_compliance();
}

#[test]
fn dropping_a_sphere_on_a_platform() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            world.set_gravity(Vector::new(0.0, 0.0, -0.5));

            let material = Material::new_with_density(1.0);

            let state_0 = State::new_stationary()
                .with_position(0.0, 0.0,  3.0)
                .with_velocity(0.0, 0.0, -1.0)
                .with_angular_velocity(0.3, 0.4, 0.5);
            world.create_body(Sphere::new(1.0), &material, state_0);

            let transform_1 = Transform::new_identity();
            world.create_static_body(Cuboid::new(10.0, 10.0, 0.1), &material, transform_1);
        })
        .execute_multiple_steps(200, 0.1)
        .assert_compliance();
}
