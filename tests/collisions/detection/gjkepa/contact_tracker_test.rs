extern crate quickcheck;

use maths::{Approximations, Transform, UnitQuat, Vec3D};
use utils::is_coplanar;
use algorithms::{Execute, PanicOnIteration};
use collisions::CollisionData;
use collisions::geometry::convex_shapes::Cuboid;
use collisions::detection::gjkepa::{ContactTracker, GJK, GJKSimplex};

#[test]
fn it_can_be_instantiated_with_intersecting_bodies() {
    fn property(rot: UnitQuat) {
        let control = CollisionData::new(Box::new(Cuboid::cube(1.0)), Transform::identity());
        let data = CollisionData::new(
            Box::new(Cuboid::cube(1.0)),
            Transform {
                translation: Vec3D::zero(),
                rotation: rot.into(),
            }
        );

        let mut contact_tracker = ContactTracker::new(&control, &data);

        assert_valid_simplex(&contact_tracker);

        if let None = find_origin(&mut contact_tracker, &control, &data) {
            panic!("Expected the simplex not to contain the origin, but it did");
        }

        assert_valid_simplex(&contact_tracker);
    }

    quickcheck::quickcheck(property as fn(UnitQuat));
}

#[test]
fn it_can_be_instantiated_with_non_intersecting_bodies() {
    fn property(rot: UnitQuat) {
        let control = CollisionData::new(Box::new(Cuboid::cube(1.0)), Transform::identity());
        let data = CollisionData::new(
            Box::new(Cuboid::cube(1.0)),
            Transform {
                translation: Vec3D::new(4.0, 4.0, 4.0),
                rotation: rot.into(),
            }
        );

        let mut contact_tracker = ContactTracker::new(&control, &data);

        assert_valid_simplex(&contact_tracker);

        if let Some(_simplex) = find_origin(&mut contact_tracker, &control, &data) {
            panic!("Expected the simplex not to contain the origin, but it did");
        }

        assert_valid_simplex(&contact_tracker);
    }

    quickcheck::quickcheck(property as fn(UnitQuat));
}

fn find_origin<'a>(tracker: &'a mut ContactTracker, data_0: &'a CollisionData, data_1: &'a CollisionData) -> Option<&'a GJKSimplex> {
    GJK::using_simplex(tracker.simplex_mut(), data_0, data_1)
        .panic_on_iteration(1000, "looking for origin (in tests)")
        .execute()
}

fn assert_valid_simplex(tracker: &ContactTracker) {
    let simplex = tracker.simplex();

    for (out_of_plane_index, plane) in simplex.separating_planes_with_index_of_out_of_plane_point_iter() {
        let vertex = *simplex.vertex(out_of_plane_index);
        let projection = plane.normal_projection_of(vertex);

        match projection {
            x if x.is_strictly_positive() =>
                panic!(format!("{:?} is degenerate, a separating plane is pointing in the wrong direction (projection = {})", tracker, projection)),

            x if x.is_strictly_negative() => (),

            _otherwise =>
                panic!(format!("{:?} is degenerate, all points are on the same plane (projection = {})", tracker, projection)),
        }
    }

    if is_coplanar(simplex.vertices()) {
        panic!(format!("{:?} is degenerate, all vertices are coplanar", simplex));
    }
}
