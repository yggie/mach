use mithril::collisions::{ Space, BruteForce, ContactGraph };
use mithril::properties::Rigid;
use mithril::math::Transform;
use mithril::shapes::Sphere;
use mithril::bodies::Body;

use std::rc::Rc;

fn populate(space: &mut Space) {
    let s = Sphere::new(1.0);
    let p = Rigid::new(1.0);
    let num_bodies = 10u;

    for _ in range(0u, num_bodies) {
        let t = Transform::new_identity();
        let dt = Transform::new_identity();
        let b = Rc::new(Body::new(box s, box p, t, dt));

        space.add(&b);
    }

    assert_eq!(space.size(), num_bodies);
}

#[test]
fn trapped_spheres() {
    let space = &mut BruteForce::new();
    let graph = &mut ContactGraph::new();
    populate(space);

    space.each_contact(|contact| graph.add(contact));
    graph.solve();
}
