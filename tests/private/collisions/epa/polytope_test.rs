extern crate quickcheck;

use super::Polytope;

use {Scalar, TOLERANCE};
use maths::Vector;
use collisions::gjk::{MinkowskiDifference, Simplex};

use support::{inputs, EntityBuilder};

#[test]
fn it_should_not_generate_incomplete_shells() {
    fn property(rot: inputs::UnitQuat) {
        let control_body = EntityBuilder::new_cube(1.0).build_region();
        let body = EntityBuilder::new_cube(1.0)
            .with_rotation(rot)
            .build_region();

        let diff = MinkowskiDifference::new(
            control_body.as_ref(),
            body.as_ref(),
        );

        let mut simplex = Simplex::new(&diff);
        let simplex_with_origin = simplex.reshape_to_contain_origin(&diff)
            .expect("Expected simplex to contain origin but it did not");

        let polytope = Polytope::new(&simplex_with_origin);

        let mid_point = polytope.support_points.iter()
            .fold(Vector::new_zero(), |total, support_point| {
                total + diff.vertex(&support_point)
            }) / polytope.support_points.len() as Scalar;

        for surface in polytope.surfaces.iter() {
            let point_on_surface = diff.vertex(&polytope.support_points[surface.indices.0]);
            let mid_point_surface_offset = (mid_point - point_on_surface).dot(surface.normal);

            if mid_point_surface_offset > TOLERANCE {
                panic!(format!("The Polytope has a surface ({:?}) facing the wrong way!", surface.normal));
            }
        }
    }

    quickcheck::quickcheck(property as fn(inputs::UnitQuat));
}
