// TODO this file needs to be split into meaningful tests once the engine matures

use std::rc::Rc;

use support::Simulation;

use mach::{EntityDesc, World};
use mach::maths::Vector;
use mach::dynamics::SimpleDynamics;
use mach::collisions::SimpleCollisionSpace;

#[test]
fn colliding_two_cubes() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            let entity_desc = EntityDesc::default().as_cube(1.0);

            world.create_body(
                &entity_desc.clone()
                    .with_density(1.0)
                    .with_pos(0.0,  3.0, 0.0)
                    .with_vel(0.0, -1.0, 0.0)
            );

            world.create_body(
                &entity_desc.clone()
                    .with_density(2.0)
                    .with_pos(0.0, -3.0, 0.0)
                    .with_axis_angle(Vector::new(1.0, 1.0, 0.0), 1.0)
                    .with_vel(0.0,  1.0, 0.0)
            );
        })
        .execute_multiple_steps(100, 0.1)
        .assert_compliance();
}

#[test]
fn dropping_a_cube_on_a_platform() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            world.set_gravity(Vector::new(0.0, 0.0, -0.5));

            let entity_desc = EntityDesc::default()
                .with_density(1.0)
                .with_restitution_coefficient(1.0);

            world.create_body(
                &entity_desc.clone()
                    .as_cube(1.0)
                    .with_pos(0.0, 0.0,  3.0)
                    .with_vel(0.0, 0.0, -1.0)
                    .with_ang_vel(0.3, 0.4, 0.5)
            );

            world.create_static_body(&entity_desc.clone().as_cuboid(10.0, 10.0, 0.1));
        })
        .execute_multiple_steps(200, 0.1)
        .assert_compliance();
}

#[test]
fn dropping_a_sphere_on_a_platform() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            world.set_gravity(Vector::new(0.0, 0.0, -0.5));

            let entity_desc = EntityDesc::default().with_density(1.0);

            world.create_body(
                &entity_desc.as_sphere(0.5)
                    .with_pos(0.0, 0.0,  3.0)
                    .with_vel(0.0, 0.0, -1.0)
                    .with_ang_vel(0.3, 0.4, 0.5)
            );

            world.create_static_body(
                &EntityDesc::default()
                    .as_cuboid(10.0, 10.0, 0.1)
                    .with_density(1.0)
            );
        })
        .execute_multiple_steps(100, 0.1)
        .assert_compliance();
}

#[test]
fn dropping_a_stuff_on_a_sinkhole() {
    Simulation::<SimpleCollisionSpace, SimpleDynamics>::new_default()
        .configure(|world| {
            world.set_gravity(Vector::new(0.0, 0.0, -0.5));

            let entity_desc = EntityDesc::default().with_mass(1.0);

            world.create_body(
                &entity_desc.clone()
                    .as_cube(1.0)
                    .with_pos(0.0, 0.0,  3.0)
                    .with_vel(2.0, 1.0, -1.0)
                    .with_ang_vel(0.3, 0.4, 0.5)
            );

            world.create_body(
                &entity_desc.clone()
                    .as_sphere(0.5)
                    .with_pos(0.0, 2.0,  3.0)
                    .with_vel(1.0, 0.0, -1.0)
                    .with_ang_vel(0.5, 0.3, 0.4)
            );

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

            world.create_static_body(
                &EntityDesc::default()
                    .as_triangle_mesh(vertices.clone(), elements_0)
            );

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

            world.create_static_body(
                &EntityDesc::default()
                    .as_triangle_mesh(vertices.clone(), elements_1)
            );

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

            world.create_static_body(
                &EntityDesc::default()
                    .as_triangle_mesh(vertices.clone(), elements_2)
            );
        })
        .execute_multiple_steps(300, 0.1)
        .assert_compliance();
}
