extern crate quickcheck;

use mach::{TOLERANCE, Vector};
use mach::collisions::gjk::{MinkowskiDifference, Simplex};

use support::inputs;

#[test]
fn it_does_not_initialize_degenerate_simplices() {
    fn valid_simplex_plane(simplex: &Simplex, diff: &MinkowskiDifference, plane: (usize, usize, usize), point: usize) -> bool {
        let datum = diff.vertex(simplex.support_point(plane.0));
        let a = diff.vertex(simplex.support_point(plane.1)) - datum;
        let b = diff.vertex(simplex.support_point(plane.2)) - datum;
        let plane_normal = Vector::cross(&a, b).normalize();

        let point = diff.vertex(simplex.support_point(point));
        return Vector::dot(&plane_normal, point).abs() > TOLERANCE;
    }

    fn property(body_0: inputs::VolumetricBody, body_1: inputs::VolumetricBody) -> bool {
        let body_0 = body_0.to_object();
        let body_1 = body_1.to_object();

        let diff = MinkowskiDifference::new_from_bodies(body_0.as_ref(), body_1.as_ref());
        let simplex = Simplex::new(&diff);

        return valid_simplex_plane(&simplex, &diff, (1, 2, 3), 0) &&
            valid_simplex_plane(&simplex, &diff, (0, 2, 3), 1) &&
            valid_simplex_plane(&simplex, &diff, (0, 1, 3), 2) &&
            valid_simplex_plane(&simplex, &diff, (0, 1, 2), 3);
    }

    quickcheck::quickcheck(property as fn(inputs::VolumetricBody, inputs::VolumetricBody) -> bool);
}
