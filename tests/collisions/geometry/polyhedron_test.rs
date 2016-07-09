use maths::Vec3D;
use collisions::geometry::{Polyhedron, PolyhedronError};

#[test]
fn it_returns_an_error_with_too_few_points() {
    let vertices = vec!(
        Vec3D::new(0.0,  1.0,  1.0),
        Vec3D::new(0.0, -1.0,  1.0),
        Vec3D::new(0.0,  1.0, -1.0),
    );

    match Polyhedron::convex_hull(&vertices) {
        Err(PolyhedronError::NotEnoughPoints) => (),

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

    match Polyhedron::convex_hull(&vertices) {
        Err(PolyhedronError::CoplanarPoints) => (),

        Ok(polyhedron) =>
            panic!("expected a coplanar error but got {:?}", polyhedron),

        Err(other_error) =>
            panic!("expected a coplanar error but got {:?} instead", other_error),
    }
}
