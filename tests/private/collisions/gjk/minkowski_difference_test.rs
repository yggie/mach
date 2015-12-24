extern crate quickcheck;

use super::MinkowskiDifference;

use Scalar;
use maths::ApproxEq;
use support::{inputs, EntityBuilder};

#[test]
fn it_always_returns_at_least_one_support_point_at_an_offset_from_the_origin() {
    fn property(rotation: inputs::UnitQuat, direction: inputs::UnitVect) {
        let control = EntityBuilder::new_cube(1.0).build_region();
        let body = EntityBuilder::new_cube(1.0)
            .with_rotation(rotation)
            .build_region();

        let diff = MinkowskiDifference::new(
            control.as_ref(),
            body.as_ref(),
        );

        let direction = direction.to_value();
        let support_points = diff.support_points(&direction);

        assert!(support_points.len() > 0, "Expected the Minkowski Difference to always return at least one support point, but got none");

        let distances: Vec<Scalar> = support_points.iter()
            .map(|point| diff.vertex(point).dot(direction))
            .collect();

        let first_distance = distances[0];
        if distances.iter().any(|&distance| !ApproxEq::approx_eq(first_distance, distance)) {
            panic!(format!("Expected all points to share the same distance along the support direction from the origin, but some differed: {:?}", distances));
        }
    }

    quickcheck::quickcheck(property as fn(inputs::UnitQuat, inputs::UnitVect));
}
