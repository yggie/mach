use collisions::BruteForce;
use collisions::behaviours::assert_broadphase_behaviour;

#[test]
fn it_conforms_to_broadphase_behaviour() {
    assert_broadphase_behaviour::<BruteForce>(&mut BruteForce::new());
}
