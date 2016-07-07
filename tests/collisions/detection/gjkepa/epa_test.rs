extern crate quickcheck;

use Scalar;
use maths::{Approximations, Transform, UnitQuat, Vec3D};
use algorithms::{Execute, PanicOnIteration};
use collisions::CollisionData;
use collisions::geometry::shapes::Cuboid;
use collisions::detection::gjkepa::{ContactTracker, EPA, GJK, GJKSimplex};

#[test]
fn it_should_not_generate_incomplete_shells() {
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
        let simplex = find_origin(&mut contact_tracker, &control, &data)
            .expect("Expected simplex to contain origin but it did not");

        let epa_polyhedron = EPA::new(simplex, &control, &data)
            .panic_on_iteration(1000, "EPA failed to converge after 1000 iterations (in test)")
            .execute();
        let polyhedron = epa_polyhedron.polyhedron();

        let mid_point = polyhedron.vertices().iter()
            .fold(Vec3D::zero(), |total, vertex| {
                total + vertex
            }) / polyhedron.vertices().len() as Scalar;

        for face in polyhedron.faces_iter() {
            if !face.normal_projection_of(mid_point).is_strictly_negative() {
                panic!(format!("The Polytope has a face ({:?}) facing the wrong way!", face.normal()));
            }
        }
    }

    quickcheck::quickcheck(property as fn(UnitQuat));
}

fn find_origin<'a>(tracker: &'a mut ContactTracker, data_0: &'a CollisionData, data_1: &'a CollisionData) -> Option<&'a GJKSimplex> {
    GJK::using_simplex(tracker.simplex_mut(), data_0, data_1)
        .panic_on_iteration(1000, "looking for origin (in tests)")
        .execute()
}
