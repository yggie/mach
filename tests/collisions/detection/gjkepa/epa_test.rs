extern crate quickcheck;

use Scalar;
use maths::{Approximations, Transform, UnitQuat, Vec3D};
use algorithms::{Execute, PanicOnIteration};
use collisions::CollisionData;
use collisions::geometry::ConvexPolyhedron;
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

        // assert_vertices_are_subset_of_input(&polyhedron, &vertices)
        assert_has_enough_faces(&polyhedron);
        assert_all_points_are_on_the_convex_hull(&polyhedron);
        assert_all_faces_are_pointing_outwards(&polyhedron);
    }

    quickcheck::quickcheck(property as fn(UnitQuat));
}

fn find_origin<'a>(tracker: &'a mut ContactTracker, data_0: &'a CollisionData, data_1: &'a CollisionData) -> Option<&'a GJKSimplex> {
    GJK::using_simplex(tracker.simplex_mut(), data_0, data_1)
        .panic_on_iteration(1000, "looking for origin (in tests)")
        .execute()
}

fn assert_has_enough_faces(polyhedron: &ConvexPolyhedron) {
    let face_count = polyhedron.faces_iter().count();
    let vertex_count = polyhedron.vertices_iter().count();

    let expected_face_count = 4 + (vertex_count - 4) * 2;

    assert!(face_count == expected_face_count, format!("expected polyhedron to have {} faces, but got {} instead", expected_face_count, face_count))
}

fn assert_all_points_are_on_the_convex_hull(polyhedron: &ConvexPolyhedron) {
    use maths::Approximations;

    let mut outside_points_count = 0;
    for face in polyhedron.faces_iter() {
        for vertex in polyhedron.vertices_iter() {
            if face.normal_projection_of(*vertex).is_strictly_positive() {
                outside_points_count += 1;
            }
        }
    }

    assert!(outside_points_count == 0, format!("expected polyhedron to have 0 points outside the separating planes, but found {} instead", outside_points_count))
}

fn assert_all_faces_are_pointing_outwards(polyhedron: &ConvexPolyhedron) {
    let mid_point = polyhedron.vertices().iter()
        .fold(Vec3D::zero(), |total, vertex| {
            total + vertex
        }) / polyhedron.vertices().len() as Scalar;

    for face in polyhedron.faces_iter() {
        if !face.normal_projection_of(mid_point).is_strictly_negative() {
            panic!(format!("The ConvexPolyhedron has a face ({:?}) facing the wrong way!", face.normal()));
        }
    }
}
