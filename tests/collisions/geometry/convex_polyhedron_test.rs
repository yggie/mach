extern crate quickcheck;

use maths::Vec3D;
use collisions::geometry::{ConvexPolyhedron, ConvexPolyhedronError};

#[test]
fn it_returns_an_error_with_too_few_points() {
    let vertices = vec!(
        Vec3D::new(0.0,  1.0,  1.0),
        Vec3D::new(0.0, -1.0,  1.0),
        Vec3D::new(0.0,  1.0, -1.0),
    );

    match ConvexPolyhedron::from_vertices(&vertices) {
        Err(ConvexPolyhedronError::NotEnoughPoints) => (),

        Ok(polyhedron) =>
            panic!("expected an insufficient points error but got {:?}", polyhedron),

        Err(other_error) =>
            panic!("expected an insufficient points error but got {:?} instead", other_error),
    }
}

#[test]
fn it_returns_an_error_with_coplanar_points() {
    let vertices = vec!(
        Vec3D::new(0.0,  1.0,  1.0),
        Vec3D::new(0.0, -1.0,  1.0),
        Vec3D::new(0.0,  1.0, -1.0),
        Vec3D::new(0.0, -1.0, -1.0),
    );

    match ConvexPolyhedron::from_vertices(&vertices) {
        Err(ConvexPolyhedronError::CoplanarPoints) => (),

        Ok(polyhedron) =>
            panic!("expected a coplanar error but got {:?}", polyhedron),

        Err(other_error) =>
            panic!("expected a coplanar error but got {:?} instead", other_error),
    }
}
