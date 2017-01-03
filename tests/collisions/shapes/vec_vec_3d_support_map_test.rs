extern crate quickcheck;

use maths::{UnitVec3D, Vec3D};
use collisions::shapes::behaviour::support_map_behaviour;

use tests::support::{One, Ten, VariableSizeVec};

quickcheck! {
    fn it_behaves_like_a_support_map(input_points: VariableSizeVec<Vec3D, One, Ten>, direction: UnitVec3D) -> quickcheck::TestResult {
        let points = input_points.to_vec();

        quickcheck_expect!(support_map_behaviour(points, direction));

        quickcheck::TestResult::passed()
    }
}
