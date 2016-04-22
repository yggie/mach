extern crate quickcheck;

use super::MinkowskiDifference;

use Scalar;
use maths::{ApproxEq, DotProduct, UnitQuat, UnitVec3D};
use shapes::Cuboid;
use entities::RigidBody;

#[test]
fn it_always_returns_at_least_one_support_point_at_an_offset_from_the_origin() {
    fn property(rotation: UnitQuat, random_direction: UnitVec3D) {
        let control = RigidBody::default()
            .with_shape(Cuboid::cube(1.0));
        let rigid_body = RigidBody::default()
            .with_shape(Cuboid::cube(1.0))
            .with_rotation(rotation.into());

        let diff = MinkowskiDifference(control.form(), rigid_body.form());

        let index_pairs = diff.support_index_pairs(random_direction.into());

        assert!(index_pairs.len() > 0, "Expected the Minkowski Difference to always return at least one support point, but got none");

        let distances: Vec<Scalar> = index_pairs.iter()
            .map(|point| diff.vertex(point).dot(random_direction))
            .collect();

        let first_distance = distances[0];
        if distances.iter().any(|&distance| !ApproxEq::approx_eq(first_distance, distance)) {
            panic!(format!("Expected all points to share the same distance along the support direction from the origin, but some differed: {:?}", distances));
        }
    }

    quickcheck::quickcheck(property as fn(UnitQuat, UnitVec3D));
}
