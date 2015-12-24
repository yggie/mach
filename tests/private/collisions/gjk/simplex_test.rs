extern crate quickcheck;

use {TOLERANCE, Vector};
use collisions::gjk::{MinkowskiDifference, Simplex};

fn assert_valid_simplex_surfaces(simplex: &Simplex, diff: &MinkowskiDifference) {
    let centroid = simplex.centroid(diff);
    let support_points = simplex.support_points();
    for surface in simplex.surfaces_iter(diff) {
        let vertex = diff.vertex(&support_points[surface.indices.0]);
        let distance_from_centroid = (vertex - centroid).dot(surface.normal);

        if distance_from_centroid < -TOLERANCE {
            panic!(format!("{:?} is degenerate, surfaces do not point outwards", simplex));
        }
    }
}

fn assert_valid_simplex_plane(simplex: &Simplex, diff: &MinkowskiDifference, plane: (usize, usize, usize), outside_plane: usize) {
    let datum = diff.vertex(&simplex.support_points()[plane.0]);
    let a = diff.vertex(&simplex.support_points()[plane.1]) - datum;
    let b = diff.vertex(&simplex.support_points()[plane.2]) - datum;
    let plane_normal = Vector::cross(&a, b).normalize();

    let point = diff.vertex(&simplex.support_points()[outside_plane]);

    if Vector::dot(&plane_normal, point - datum).abs() < TOLERANCE {
        panic!(format!("{:?} is degenerate, all points are on the same plane", simplex));
    }
}

pub fn assert_valid_simplex(simplex: &Simplex, diff: &MinkowskiDifference) {
    assert_valid_simplex_plane(simplex, diff, (1, 2, 3), 0);
    assert_valid_simplex_plane(simplex, diff, (0, 2, 3), 1);
    assert_valid_simplex_plane(simplex, diff, (0, 1, 3), 2);
    assert_valid_simplex_plane(simplex, diff, (0, 1, 2), 3);
    assert_valid_simplex_surfaces(simplex, diff);
}

#[cfg(test)]
mod without_intersections {
    extern crate quickcheck;

    use collisions::gjk::{MinkowskiDifference, Simplex};
    use support::{inputs, EntityBuilder};

    #[test]
    fn it_can_handle_arbitrary_rotations() {
        fn property(rot: inputs::UnitQuat) {
            let control_body = EntityBuilder::new_cube(1.0).build_region();
            let body = EntityBuilder::new_cube(1.0)
                .with_translation(4.0, 4.0, 4.0)
                .with_rotation(rot)
                .build_region();

            let diff = MinkowskiDifference::new(
                control_body.as_ref(),
                body.as_ref(),
            );
            let mut simplex = Simplex::new(&diff);

            super::assert_valid_simplex(&simplex, &diff);

            if let Some(_result) = simplex.reshape_to_contain_origin(&diff) {
                panic!("Expected the simplex not to contain the origin, but it did");
            }

            super::assert_valid_simplex(&simplex, &diff);
        }

        quickcheck::quickcheck(property as fn(inputs::UnitQuat));
    }
}

#[cfg(test)]
mod with_intersections {
    extern crate quickcheck;

    use collisions::gjk::{MinkowskiDifference, Simplex};
    use support::{inputs, EntityBuilder};

    #[test]
    fn it_can_handle_arbitrary_rotations() {
        fn property(rot: inputs::UnitQuat) {
            let control_body = EntityBuilder::new_cube(1.0).build_region();
            let body = EntityBuilder::new_cube(1.0)
                .with_rotation(rot)
                .build_region();

            let diff = MinkowskiDifference::new(
                control_body.as_ref(),
                body.as_ref(),
            );
            let mut simplex = Simplex::new(&diff);

            super::assert_valid_simplex(&simplex, &diff);

            simplex.reshape_to_contain_origin(&diff)
                .expect("Expected the simplex to contain the origin, but it did not");

            super::assert_valid_simplex(&simplex, &diff);
        }

        quickcheck::quickcheck(property as fn(inputs::UnitQuat));
    }
}
