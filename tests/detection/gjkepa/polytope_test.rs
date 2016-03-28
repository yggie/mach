extern crate quickcheck;

use Scalar;
use maths::Vect;
use shapes::{Cuboid, PlaneLocation};
use entities::RigidBody;

use super::Polytope;
use super::super::simplex_cache::SimplexCache;
use super::super::minkowski_difference::MinkowskiDifference;

use support::inputs;

#[test]
fn it_should_not_generate_incomplete_shells() {
    fn property(rot: inputs::UnitQuat) {
        let control = RigidBody::default()
            .with_shape(Cuboid::cube(1.0));
        let rigid_body = RigidBody::default()
            .with_shape(Cuboid::cube(1.0))
            .with_rotation(rot.into());

        let diff = MinkowskiDifference(control.form(), rigid_body.form());

        let mut simplex_cache = SimplexCache::new(&diff);

        let simplex = simplex_cache.update_to_contain_origin(diff)
            .expect("Expected simplex to contain origin but it did not");

        let polytope = Polytope::new(simplex);

        let mid_point = polytope.support_points.iter()
            .fold(Vect::zero(), |total, &(vertex, _index_pair)| {
                total + vertex
            }) / polytope.support_points.len() as Scalar;

        for &(ref surface, _vertex_indices) in polytope.surfaces.iter() {
            if surface.location_of(&mid_point) != PlaneLocation::Below {
                panic!(format!("The Polytope has a surface ({:?}) facing the wrong way!", surface.normal()));
            }
        }
    }

    quickcheck::quickcheck(property as fn(inputs::UnitQuat));
}