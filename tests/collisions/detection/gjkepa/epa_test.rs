extern crate quickcheck;

use std::collections::HashMap;

use Scalar;
use maths::{Approximations, Quat, Transform, UnitQuat, Vec3D};
use algorithms::{Execute, PanicOnIteration};
use collisions::CollisionData;
use collisions::geometry::ConvexPolyhedron;
use collisions::geometry::shapes::Cuboid;
use collisions::detection::gjkepa::{ContactTracker, EPA, GJK, GJKSimplex};

// TODO fix this failing scenario
// #[test]
// fn failing_test_test() {
//     // TODO FIXME FIXME FIXME
//     temp(Quat::new(0.99956876, -0.015006613, -0.00038310335, 0.025238236).normalize());
//     fn temp(rot: UnitQuat) -> quickcheck::TestResult {
//         let control = CollisionData::new(Box::new(Cuboid::cube(1.0)), Transform::identity());
//         let data = CollisionData::new(
//             Box::new(Cuboid::cube(1.0)),
//             Transform {
//                 translation: Vec3D::zero(),
//                 rotation: rot.into(),
//             }
//         );
//
//         let mut contact_tracker = ContactTracker::new(&control, &data);
//         let simplex = find_origin(&mut contact_tracker, &control, &data)
//             .expect("Expected simplex to contain origin but it did not");
//
//         let epa_polyhedron = EPA::new(simplex, &control, &data)
//             .panic_on_iteration(1000, "EPA failed to converge after 1000 iterations (in test)")
//             .execute();
//         let polyhedron = epa_polyhedron.polyhedron();
//
//         // assert_vertices_are_subset_of_input(&polyhedron, &vertices)
//         quickcheck_expect!(has_enough_faces(&polyhedron));
//         quickcheck_expect!(edges_are_only_connected_to_two_faces(&polyhedron));
//         quickcheck_expect!(all_points_are_on_the_convex_hull(&polyhedron));
//         quickcheck_expect!(all_faces_are_pointing_outwards(&polyhedron));
//
//         quickcheck::TestResult::passed()
//     }
// }

quickcheck! {
    fn it_generates_valid_convex_polyhedron(rot: UnitQuat) -> quickcheck::TestResult {
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
        quickcheck_expect!(has_enough_faces(&polyhedron));
        quickcheck_expect!(edges_are_only_connected_to_two_faces(&polyhedron));
        quickcheck_expect!(all_points_are_on_the_convex_hull(&polyhedron));
        quickcheck_expect!(all_faces_are_pointing_outwards(&polyhedron));

        quickcheck::TestResult::passed()
    }
}

fn find_origin<'a>(tracker: &'a mut ContactTracker, data_0: &'a CollisionData, data_1: &'a CollisionData) -> Option<&'a GJKSimplex> {
    GJK::using_simplex(tracker.simplex_mut(), data_0, data_1)
        .panic_on_iteration(1000, "looking for origin (in tests)")
        .execute()
}

fn has_enough_faces(polyhedron: &ConvexPolyhedron) -> quickcheck::TestResult {
    let face_count = polyhedron.faces_iter().count();
    let vertex_count = polyhedron.vertices_iter().count();

    let expected_face_count = 4 + (vertex_count - 4) * 2;

    quickcheck_assert!(face_count == expected_face_count, format!("expected polyhedron to have {} faces, but got {} instead", expected_face_count, face_count));

    quickcheck::TestResult::passed()
}

fn edges_are_only_connected_to_two_faces(polyhedron: &ConvexPolyhedron) -> quickcheck::TestResult {
    let total_edge_counts = polyhedron.triangulation_iter()
        .flat_map(|triangulation| {
            vec!(
                sort_pair(triangulation[0], triangulation[1]),
                sort_pair(triangulation[1], triangulation[2]),
                sort_pair(triangulation[2], triangulation[0]),
            ).into_iter()
        })
        .fold(<HashMap<(usize, usize), usize>>::new(), |mut edge_counts, edge| {
            let count = edge_counts.get(&edge).cloned().unwrap_or(0usize);

            edge_counts.insert(edge, count + 1);

            return edge_counts;
        });

    let outliers: Vec<((usize, usize), usize)> = total_edge_counts.into_iter()
        .filter(|&(_edge, count)| count != 2)
        .collect();

    if outliers.len() == 0 {
        quickcheck::TestResult::passed()
    } else {
        quickcheck::TestResult::error(format!("expected all edges to be connected to two faces, but found the following outliers instead: {:?}", outliers))
    }
}

fn all_points_are_on_the_convex_hull(polyhedron: &ConvexPolyhedron) -> quickcheck::TestResult {
    use maths::Approximations;

    let mut outside_points_count = 0;
    for face in polyhedron.faces_iter() {
        for vertex in polyhedron.vertices_iter() {
            if face.normal_projection_of(*vertex).is_strictly_positive() {
                outside_points_count += 1;
            }
        }
    }

    quickcheck_assert!(outside_points_count == 0, format!("expected polyhedron to have 0 points outside the separating planes, but found {} instead", outside_points_count));

    quickcheck::TestResult::passed()
}

fn all_faces_are_pointing_outwards(polyhedron: &ConvexPolyhedron) -> quickcheck::TestResult {
    let mid_point = polyhedron.vertices().iter()
        .fold(Vec3D::zero(), |total, vertex| {
            total + vertex
        }) / polyhedron.vertices().len() as Scalar;

    for face in polyhedron.faces_iter() {
        quickcheck_assert!(
            face.normal_projection_of(mid_point).is_strictly_negative(),
            format!("The ConvexPolyhedron has a face ({:?}) facing the wrong way!", face.normal()),
        );
    }

    quickcheck::TestResult::passed()
}

fn sort_pair(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
