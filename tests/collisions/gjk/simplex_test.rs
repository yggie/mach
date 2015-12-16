extern crate quickcheck;

use mach::{TOLERANCE, Vector};
use mach::collisions::gjk::{MinkowskiDifference, Simplex};

use support::inputs;

fn valid_simplex_plane(simplex: &Simplex, diff: &MinkowskiDifference, plane: (usize, usize, usize), point: usize) -> bool {
    let datum = diff.vertex(&simplex.support_points()[plane.0]);
    let a = diff.vertex(&simplex.support_points()[plane.1]) - datum;
    let b = diff.vertex(&simplex.support_points()[plane.2]) - datum;
    let plane_normal = Vector::cross(&a, b).normalize();

    let point = diff.vertex(&simplex.support_points()[point]);
    return Vector::dot(&plane_normal, point).abs() > TOLERANCE;
}

fn is_simplex_valid(simplex: &Simplex, diff: &MinkowskiDifference) -> bool {
    // TODO replace everything with panics once this issue is resolved:
    // https://github.com/BurntSushi/quickcheck/issues/91
    valid_simplex_plane(simplex, diff, (1, 2, 3), 0) &&
        valid_simplex_plane(simplex, diff, (0, 2, 3), 1) &&
        valid_simplex_plane(simplex, diff, (0, 1, 3), 2) &&
        valid_simplex_plane(simplex, diff, (0, 1, 2), 3)
}

#[test]
fn it_does_not_initialize_degenerate_simplices() {
    fn property(body_0: inputs::VolumetricBody, body_1: inputs::VolumetricBody) -> bool {
        let body_0 = body_0.to_value();
        let body_1 = body_1.to_value();

        let diff = MinkowskiDifference::new_from_bodies(body_0.as_ref(), body_1.as_ref());
        let simplex = Simplex::new(&diff);

        return is_simplex_valid(&simplex, &diff);
    }

    quickcheck::quickcheck(property as fn(inputs::VolumetricBody, inputs::VolumetricBody) -> bool);
}

#[test]
fn it_does_not_create_degenerate_simplices_when_reshaping_to_contain_origin() {
    fn property(body_0: inputs::VolumetricBody, body_1: inputs::VolumetricBody) -> bool {
        let body_0 = body_0.to_value();
        let body_1 = body_1.to_value();

        let diff = MinkowskiDifference::new_from_bodies(body_0.as_ref(), body_1.as_ref());
        let mut simplex = Simplex::new(&diff);

        simplex.reshape_to_contain_origin(&diff);
        return is_simplex_valid(&simplex, &diff);
    }

    quickcheck::quickcheck(property as fn(inputs::VolumetricBody, inputs::VolumetricBody) -> bool);
}
