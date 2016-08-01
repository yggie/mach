extern crate quickcheck;

use maths::{ApproxEq, Approximations, CrossProduct, DotProduct, UnitVec3D, Vec3D};
use collisions::geometry::SupportMap;

use tests::support::{One, Four, Ten, VariableSizeVec};

quickcheck! {
    fn it_always_produces_at_least_one_support_point(input_points: VariableSizeVec<Vec3D, One, Ten>, direction: UnitVec3D) -> quickcheck::TestResult {
        let points = input_points.to_vec();

        quickcheck_assert!(points.support_points_iter(Vec3D::from(direction)).count() > 0, format!("expected at least one support point in direction {:?} but found none", direction));

        return quickcheck::TestResult::passed();
    }
}

quickcheck! {
    fn it_computes_support_points_which_are_deterministic(input_points: VariableSizeVec<Vec3D, One, Ten>, input_directions: VariableSizeVec<UnitVec3D, One, Ten>) -> quickcheck::TestResult {
        let points = input_points.to_vec();
        let directions = input_directions.to_vec();

        let initial_solutions: Vec<Vec<Vec3D>> = directions.iter()
            .map(|&direction| {
                points.support_points_iter(Vec3D::from(direction))
                    .collect::<Vec<Vec3D>>()
            })
            .collect();

        for _ in 0..2 {
            for (index, &direction) in directions.iter().enumerate() {
                let solution: Vec<Vec3D> = points.support_points_iter(Vec3D::from(direction))
                    .collect();

                assert_approx_matching!(&initial_solutions[index], &solution);
            }
        }

        quickcheck::TestResult::passed()
    }
}

quickcheck! {
    fn it_returns_unique_support_points_consistent_with_the_support_points_queries(input_points: VariableSizeVec<Vec3D, One, Ten>, input_direction: UnitVec3D) -> quickcheck::TestResult {
        let points = input_points.to_vec();
        let direction = Vec3D::from(input_direction);
        let support_points: Vec<Vec3D> = points.support_points_iter(direction)
            .collect();

        if support_points.len() == 1 {
            let expected_support_point = support_points[0];

            let unique_support_point_option = points.unique_support_point(direction);
            quickcheck_assert!(
                unique_support_point_option.is_some(),
                format!("expected to find {:?} as a unique support point, but instead got nothing", expected_support_point),
            );

            let unique_support_point = unique_support_point_option.unwrap();
            quickcheck_assert!(
                unique_support_point.approx_eq(expected_support_point),
                format!("expected unique support point {:?} to equal {:?}", unique_support_point, expected_support_point),
            );
        } else {
            let option = points.unique_support_point(direction);

            quickcheck_assert!(
                option.is_none(),
                format!("expected a unique support point not to exist, but instead got {:?}", option.unwrap()),
            );
        }

        return quickcheck::TestResult::passed();
    }
}

quickcheck! {
    fn it_computes_boundary_support_points_which_are_a_strict_subset_of_the_support_points(input_points: VariableSizeVec<Vec3D, One, Ten>, input_direction: UnitVec3D) -> quickcheck::TestResult {
        let points = input_points.to_vec();
        let direction = Vec3D::from(input_direction);
        let support_points: Vec<Vec3D> = points.support_points_iter(direction)
            .collect();
        let boundary_support_points: Vec<Vec3D> = points.support_points_iter(direction)
            .collect();

        quickcheck_assert!(
            boundary_support_points.is_subset_of(&support_points),
            format!("expected {:?} to be a strict subset of {:?}, but was not", boundary_support_points, support_points),
        );

        return quickcheck::TestResult::passed();
    }
}

quickcheck! {
    fn it_computes_boundary_support_points_which_are_strictly_on_the_expected_boundary(input_points: VariableSizeVec<Vec3D, Four, Ten>, input_direction: UnitVec3D) -> quickcheck::TestResult {
        let points = input_points.to_vec();
        let direction = Vec3D::from(input_direction);
        let boundary_support_points: Vec<Vec3D> = points.support_points_iter(direction)
            .collect();

        assert_points_form_a_boundary(boundary_support_points, input_direction)
    }
}

quickcheck! {
    fn it_computes_boundary_support_points_which_are_deterministic(input_points: VariableSizeVec<Vec3D, One, Ten>, input_directions: VariableSizeVec<UnitVec3D, One, Ten>) -> quickcheck::TestResult {
        let points = input_points.to_vec();
        let directions = input_directions.to_vec();

        let initial_solutions: Vec<Vec<Vec3D>> = directions.iter()
            .map(|&direction| {
                points.boundary_support_points_iter(Vec3D::from(direction))
                    .collect::<Vec<Vec3D>>()
            })
            .collect();

        for _ in 0..2 {
            for (index, &direction) in directions.iter().enumerate() {
                let solution: Vec<Vec3D> = points.boundary_support_points_iter(Vec3D::from(direction))
                    .collect();

                assert_approx_matching!(&initial_solutions[index], &solution);
            }
        }

        quickcheck::TestResult::passed()
    }
}

fn assert_points_form_a_boundary(original_boundary_support_points: Vec<Vec3D>, direction: UnitVec3D) -> quickcheck::TestResult {
    let mut boundary_support_points = original_boundary_support_points.clone();

    let mut tracking_head = boundary_support_points.pop().unwrap();
    while !boundary_support_points.is_empty() {
        let mut marked_for_removal: Option<usize> = None;
        for (index, &point) in boundary_support_points.iter().enumerate() {
            let edge = point - tracking_head;
            let edge_normal = edge.cross(direction).normalize();
            let mut strictly_positive = true;
            let mut strictly_negative = true;

            for &repeated_point in boundary_support_points.iter() {
                let distance = edge_normal.dot(repeated_point - tracking_head);

                match distance {
                    x if x.is_strictly_positive() => {
                        strictly_negative = false;
                    },

                    x if x.is_strictly_negative() => {
                        strictly_positive = false;
                    },

                    _otherwise => (),
                }

                if !strictly_negative && !strictly_positive {
                    break;
                }
            }

            if strictly_negative || strictly_positive {
                marked_for_removal = Some(index);
                break;
            }
        }

        quickcheck_assert!(
            marked_for_removal.is_some(),
            format!("expected all boundary support points to exist strictly on the boundary but found {:?} which is not on the boundary of the set {:?}", tracking_head, boundary_support_points),
        );

        let index_to_remove = marked_for_removal.unwrap();
        tracking_head = boundary_support_points[index_to_remove];
        boundary_support_points.remove(index_to_remove);
    }

    return quickcheck::TestResult::passed();
}

trait Subset {
    fn is_subset_of(&self, other: &Self) -> bool;
}

impl Subset for Vec<Vec3D> {
    fn is_subset_of(&self, other: &Self) -> bool {
        let self_clone = self.clone();
        let mut other_clone = other.clone();

        for item in self_clone.into_iter() {
            let result = other_clone.iter().position(|other_item| {
                other_item.approx_eq(item)
            });

            if let Some(index) = result {
                other_clone.remove(index);
            } else {
                return false;
            }
        }

        return true;
    }
}
