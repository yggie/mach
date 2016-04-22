extern crate quickcheck;

use super::MinkowskiDifference;

use Scalar;
use maths::{ApproxEq, DotProduct, UnitQuat};
use shapes::Cuboid;
use support::inputs;
use entities::RigidBody;

#[test]
fn it_always_returns_at_least_one_support_point_at_an_offset_from_the_origin() {
    fn property(rotation: UnitQuat, direction: inputs::UnitVect) {
        let control = RigidBody::default()
            .with_shape(Cuboid::cube(1.0));
        let rigid_body = RigidBody::default()
            .with_shape(Cuboid::cube(1.0))
            .with_rotation(rotation.into());

        let diff = MinkowskiDifference(control.form(), rigid_body.form());

        let direction = direction.into();
        let index_pairs = diff.support_index_pairs(&direction);

        assert!(index_pairs.len() > 0, "Expected the Minkowski Difference to always return at least one support point, but got none");

        let distances: Vec<Scalar> = index_pairs.iter()
            .map(|point| diff.vertex(point).dot(direction))
            .collect();

        let first_distance = distances[0];
        if distances.iter().any(|&distance| !ApproxEq::approx_eq(first_distance, distance)) {
            panic!(format!("Expected all points to share the same distance along the support direction from the origin, but some differed: {:?}", distances));
        }
    }

    quickcheck::quickcheck(property as fn(UnitQuat, inputs::UnitVect));
}
