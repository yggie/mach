extern crate quickcheck;

use maths::UnitVec3D;
use collisions::shapes::convex_shapes::{ConvexShape, Sphere};
use collisions::shapes::behaviour::support_map_behaviour;

quickcheck! {
    fn it_behaves_like_a_support_map(sphere: Sphere, direction: UnitVec3D) -> quickcheck::TestResult {
        quickcheck_expect!(support_map_behaviour(Box::new(sphere) as Box<ConvexShape>, direction));

        quickcheck::TestResult::passed()
    }
}
