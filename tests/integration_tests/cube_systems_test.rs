// TODO this file needs to be split into meaningful tests once the engine matures

use std::rc::Rc;

use support::Simulation;

use mach::World;
use mach::maths::{ State, Transform, Vector };
use mach::shapes::{ Cuboid, Sphere, TriangleMesh };
use mach::entities::Material;
use mach::dynamics::SimpleDynamics;
use mach::collisions::SimpleCollisionSpace;

#[test]
fn colliding_two_cubes() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            let shape = Cuboid::new_cube(1.0);

            let state_0 =  State::new_stationary()
                .with_pos(0.0,  3.0, 0.0)
                .with_vel(0.0, -1.0, 0.0);
            world.create_body(shape.clone(), &Material::new_with_density(1.0), state_0);

            let state_1 =  State::new_stationary()
                .with_pos(0.0, -3.0, 0.0)
                .with_axis_angle(Vector::new(1.0, 1.0, 0.0), 1.0)
                .with_vel(0.0,  1.0, 0.0);
            world.create_body(shape.clone(), &Material::new_with_density(2.0), state_1);
        })
        .execute_multiple_steps(100, 0.1)
        .assert_compliance();
}

#[test]
fn dropping_a_cube_on_a_platform() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            world.set_gravity(Vector::new(0.0, 0.0, -0.5));

            let material = Material::new_with_density(1.0)
                .with_coefficient_of_restitution(1.0);

            let state_0 = State::new_stationary()
                .with_pos(0.0, 0.0,  3.0)
                .with_vel(0.0, 0.0, -1.0)
                .with_ang_vel(0.3, 0.4, 0.5);
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
                .with_pos(0.0, 0.0,  3.0)
                .with_vel(0.0, 0.0, -1.0)
                .with_ang_vel(0.3, 0.4, 0.5);
            world.create_body(Sphere::new(0.5), &material, state_0);

            let transform_1 = Transform::new_identity();
            world.create_static_body(Cuboid::new(10.0, 10.0, 0.1), &material, transform_1);
        })
        .execute_multiple_steps(100, 0.1)
        .assert_compliance();
}

#[test]
fn dropping_a_stuff_on_a_sinkhole() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            world.set_gravity(Vector::new(0.0, 0.0, -0.5));

            let material = Material::new_with_mass(1.0);

            let state_0 = State::new_stationary()
                .with_pos(0.0, 0.0,  3.0)
                .with_vel(2.0, 1.0, -1.0)
                .with_ang_vel(0.3, 0.4, 0.5);
            world.create_body(Cuboid::new_cube(1.0), &material, state_0);

            let state_0 = State::new_stationary()
                .with_pos(0.0, 2.0,  3.0)
                .with_vel(1.0, 0.0, -1.0)
                .with_ang_vel(0.5, 0.3, 0.4);
            world.create_body(Sphere::new(0.5), &material, state_0);

            let vertices = Rc::new(vec!(
                Vector::new(  0.0,   0.0,   0.0),
                Vector::new( 10.0,  10.0,  -1.0),
                Vector::new( 10.0, -10.0,  -2.0),
                Vector::new(-10.0,  10.0,  -3.0),
                Vector::new(-10.0, -10.0,  -1.0),
                Vector::new( 30.0,  30.0,   4.0),
                Vector::new( 30.0,  30.0,   0.0),
                Vector::new( 30.0, -30.0,   3.0),
                Vector::new( 30.0, -30.0,   0.0),
                Vector::new(-30.0,  30.0,   5.0),
                Vector::new(-30.0,  30.0,   0.0),
                Vector::new(-30.0, -30.0,   4.0),
                Vector::new(-30.0, -30.0,   0.0),
            ));

            let elements_0 = vec!(
                (0, 2, 1),
                (0, 4, 2),
                (0, 3, 4),
                (0, 1, 3),
                (1, 2, 3),
                (3, 2, 4),
             );

            let mesh = TriangleMesh::new(vertices.clone(), elements_0);
            let transform_1 = Transform::new_identity();
            world.create_static_body(mesh, &material, transform_1);

            let elements_1 = vec!(
                (1, 2, 5),
                (5, 2, 7),
                (6, 1, 5),
                (7, 6, 5),
                (7, 8, 6),
                (2, 1, 6),
                (2, 6, 8),
                (7, 2, 8),
            );

            let mesh = TriangleMesh::new(vertices.clone(), elements_1);
            let transform_1 = Transform::new_identity();
            world.create_static_body(mesh, &material, transform_1);

            let elements_2 = vec!(
                (9, 3, 1),
                (9, 1, 5),
                (5, 1, 6),
                (5, 6, 9),
                (9, 6, 10),
                (10, 1, 3),
                (10, 6, 1),
                (9, 10, 3),
            );

            let mesh = TriangleMesh::new(vertices, elements_2);
            let transform_1 = Transform::new_identity();
            world.create_static_body(mesh, &material, transform_1);
        })
        .execute_multiple_steps(300, 0.1)
        .assert_compliance();
}
