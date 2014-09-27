use collisions::BruteForce;
use collisions::tests::helpers::assert_behaves_like_broadphase;

#[test]
fn it_behaves_like_broadphase() {
    let mut b = BruteForce::new();
    assert_behaves_like_broadphase(&mut b);
}
