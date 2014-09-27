use collisions::BruteForce;
use collisions::tests::helpers::assert_behaves_like_a_space;

#[test]
fn it_behaves_like_a_space() {
    let mut b = BruteForce::new();
    assert_behaves_like_a_space(&mut b);
}
