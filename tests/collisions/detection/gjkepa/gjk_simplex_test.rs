use maths::Vec3D;
use collisions::detection::gjkepa::{GJKSimplex, GJKSimplexError};

#[test]
fn it_returns_an_error_when_the_vertices_are_coplanar() {
    let result = GJKSimplex::from_vertices(
        Vec3D::new(0.0,  1.0,  1.0),
        Vec3D::new(0.0, -1.0,  1.0),
        Vec3D::new(0.0,  1.0, -1.0),
        Vec3D::new(0.0, -1.0, -1.0),
    );

    match result {
        Err(GJKSimplexError::CoplanarPoints) => (),

        Ok(simplex) =>
            panic!("expected a coplanar points error but instead got {:?}", simplex),

        // Err(other_error) =>
        //     panic!("expected a coplanar points error but instead got {:?}", other_error),
    }
}
