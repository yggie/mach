extern crate quickcheck;

use maths::{ApproxEq, Approximations, CrossProduct, DotProduct, UnitVec3D, Vec3D};
use collisions::shapes::{Direction, SupportMap};

pub fn support_map_behaviour<S>(subject: S, input: UnitVec3D) -> quickcheck::TestResult where S: SupportMap {
    let direction = Direction::from(input);

    quickcheck_expect!(always_produces_at_least_one_support_point(&subject, direction));
    quickcheck_expect!(computes_support_points_which_are_deterministic(&subject, direction));
    quickcheck_expect!(unique_support_points_are_a_strict_subset_of_the_support_points(&subject, direction));
    quickcheck_expect!(boundary_support_points_are_a_strict_subset_of_the_support_points(&subject, direction));
    quickcheck_expect!(boundary_support_points_are_strictly_on_the_expected_boundary(&subject, direction));
    quickcheck_expect!(boundary_support_points_are_deterministic(&subject, direction));

    quickcheck::TestResult::passed()
}

fn always_produces_at_least_one_support_point<S>(subject: &S, direction: Direction) -> quickcheck::TestResult where S: SupportMap {
    quickcheck_assert!(subject.support_points_iter(direction).count() > 0, format!("expected at least one support point in direction {:?} but found none", direction));

    quickcheck::TestResult::passed()
}

fn computes_support_points_which_are_deterministic<S>(subject: &S, direction: Direction) -> quickcheck::TestResult where S: SupportMap {
    let initial_solution: Vec<Vec3D> = subject.support_points_iter(direction)
        .collect();

    for _ in 0..2 {
        let solution: Vec<Vec3D> = subject.support_points_iter(direction)
            .collect();

        assert_approx_matching!(&initial_solution, &solution);
    }

    quickcheck::TestResult::passed()
}

fn unique_support_points_are_a_strict_subset_of_the_support_points<S>(subject: &S, direction: Direction) -> quickcheck::TestResult where S: SupportMap {
    let support_points: Vec<Vec3D> = subject.support_points_iter(direction)
        .collect();

    if support_points.len() == 1 {
        let expected_support_point = support_points[0];

        let unique_support_point_option = subject.unique_support_point(direction);
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
        let option = subject.unique_support_point(direction);

        quickcheck_assert!(
            option.is_none(),
            format!("expected a unique support point not to exist, but instead got {:?}", option.unwrap()),
            );
    }

    quickcheck::TestResult::passed()
}

fn boundary_support_points_are_a_strict_subset_of_the_support_points<S>(subject: &S, direction: Direction) -> quickcheck::TestResult where S: SupportMap {
    let support_points: Vec<Vec3D> = subject.support_points_iter(direction)
        .collect();
    let boundary_support_points: Vec<Vec3D> = subject.support_points_iter(direction)
        .collect();

    quickcheck_assert!(
        boundary_support_points.is_subset_of(&support_points),
        format!("expected {:?} to be a strict subset of {:?}, but was not", boundary_support_points, support_points),
        );

    quickcheck::TestResult::passed()
}

fn boundary_support_points_are_strictly_on_the_expected_boundary<S>(subject: &S, direction: Direction) -> quickcheck::TestResult where S: SupportMap {
    let mut boundary_support_points: Vec<Vec3D> = subject.support_points_iter(direction)
        .collect();

    let mut tracking_head = boundary_support_points.pop().unwrap();
    while !boundary_support_points.is_empty() {
        let mut marked_for_removal: Option<usize> = None;
        for (index, &point) in boundary_support_points.iter().enumerate() {
            let edge = point - tracking_head;
            let edge_normal = edge.cross(Vec3D::from(direction)).normalize();
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

fn boundary_support_points_are_deterministic<S>(subject: &S, direction: Direction) -> quickcheck::TestResult where S: SupportMap {
    let initial_solution: Vec<Vec3D> = subject.boundary_support_points_iter(direction)
        .collect();

    for _ in 0..2 {
        let solution: Vec<Vec3D> = subject.boundary_support_points_iter(direction)
            .collect();

        assert_approx_matching!(&initial_solution, &solution);
    }

    quickcheck::TestResult::passed()
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
