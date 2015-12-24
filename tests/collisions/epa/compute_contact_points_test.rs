extern crate quickcheck;

use mach::collisions::epa;
use mach::collisions::gjk::{MinkowskiDifference, Simplex};

use support::EntityBuilder;
use support::inputs;

#[test]
fn it_always_returns_valid_contact_points() {
    fn property(rot: inputs::UnitQuat) {
        let control_body = EntityBuilder::new_cube(1.0).build_region();
        let body = EntityBuilder::new_cube(1.0)
            .with_rotation(rot)
            .build_region();

        let diff = MinkowskiDifference::new(
            control_body.as_ref(),
            body.as_ref(),
        );

        // TODO not all cases are covered yet
        let _intersection = Simplex::new(&diff).reshape_to_contain_origin(&diff)
            .map(|simplex_with_origin| epa::compute_contact_points(simplex_with_origin))
            .expect("Expected simplex to contain origin but it did not");
    }

    quickcheck::quickcheck(property as fn(inputs::UnitQuat))
}
