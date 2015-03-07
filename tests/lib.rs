extern crate mithril;

#[test]
fn falling_cubes_test() {
    let collisions = mithril::collisions::SimpleCollisions::new();
    let dynamics = mithril::dynamics::SimpleDynamics::new();
    let mut world = mithril::World::new(collisions, dynamics);

    for _ in (0..1000) {
        world.update(0.05);
    }
}
