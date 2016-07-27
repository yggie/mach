extern crate quickcheck;

use TOLERANCE;
use maths::{DotProduct, Transform, UnitQuat, UnitVec3D, Vec3D};
use collisions::CollisionData;
use collisions::geometry::shapes::Cuboid;
use collisions::detection::gjkepa::MinkowskiDifference;

// TODO should this go under a behaviour for SupportMap?
#[test]
fn it_always_generates_a_valid_support_point() {
    fn property(rotation: UnitQuat, random_direction: UnitVec3D) {
        let control = CollisionData::new(
            Box::new(Cuboid::cube(1.0)),
            Transform::identity(),
        );
        let data = CollisionData::new(
            Box::new(Cuboid::cube(1.0)),
            Transform {
                translation: Vec3D::zero(),
                rotation: rotation.into(),
            },
        );

        let diff = MinkowskiDifference(&control, &data);
        let direction = Vec3D::from(random_direction);

        let max_control = control.vertices_iter()
            .max_by_key(|vertex| (vertex.dot(direction) / TOLERANCE) as i32)
            .unwrap();

        let max_neg_data = data.vertices_iter()
            .max_by_key(|vertex| (-vertex.dot(direction) / TOLERANCE) as i32)
            .unwrap();

        let support_point = diff.support_point(direction);
        let support_point_distance = support_point.dot(direction);
        let expected_support_point_distance =(max_control - max_neg_data).dot(direction);

        assert_approx_eq!(support_point_distance, expected_support_point_distance);
    }

    quickcheck::quickcheck(property as fn(UnitQuat, UnitVec3D));
}
