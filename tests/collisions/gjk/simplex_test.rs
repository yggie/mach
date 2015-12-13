extern crate quickcheck;

use mach::{TOLERANCE, Vector};
use mach::collisions::gjk::{MinkowskiDifference, Simplex};

use support::TestVolumetricBody;

#[test]
fn it_does_not_initialize_degenerate_simplices() {
    fn valid_simplex_plane(simplex: &Simplex, plane: (usize, usize, usize), point: usize) -> bool {
        let a = simplex.vertex(plane.1) - simplex.vertex(plane.0);
        let b = simplex.vertex(plane.2) - simplex.vertex(plane.0);
        let plane_normal = Vector::cross(&a, b).normalize();

        return Vector::dot(&plane_normal, simplex.vertex(point)).abs() > TOLERANCE;
    }

    fn property(body_0: TestVolumetricBody, body_1: TestVolumetricBody) -> bool {
        let body_0 = body_0.as_volumetric_body();
        let body_1 = body_1.as_volumetric_body();

        let diff = MinkowskiDifference::new_from_bodies(body_0.as_ref(), body_1.as_ref());
        let simplex = Simplex::new(diff);

        return valid_simplex_plane(&simplex, (1, 2, 3), 0) &&
            valid_simplex_plane(&simplex, (0, 2, 3), 1) &&
            valid_simplex_plane(&simplex, (0, 1, 3), 2) &&
            valid_simplex_plane(&simplex, (0, 1, 2), 3);
    }

    quickcheck::quickcheck(property as fn(TestVolumetricBody, TestVolumetricBody) -> bool);
}
