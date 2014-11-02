use mithril::shapes::Sphere;
use mithril::properties::Rigid;
use mithril::solvers::naive_solver;
use mithril::core::{ State, World };
use mithril::collisions::BruteForce;
use mithril::integrators::euler_integration;

#[test]
fn trapped_spheres() {
    let mut world = World::new(BruteForce::new(), naive_solver, euler_integration);
    let s = Sphere::new(1.0);
    let p = Rigid::new(1.0);
    let num_bodies = 10u;

    for _ in range(0u, num_bodies) {
        world.create_body(s, p, State::new_stationary());
    }
    assert_eq!(world.num_bodies(), num_bodies);

    for _ in range(0u, 1000) {
        world.update(0.2);
    }
}
