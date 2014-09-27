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

    for i in range(0u, num_bodies) {
        let t = Transform::identity();
        let b = Rc::new(Body::new(box s, box p, t));

        space.add(&b);
    }

    assert_eq!(space.size(), num_bodies);
}

#[test]
fn trapped_spheres() {
    let mut space = BruteForce::new();
    let mut graph = ContactGraph::new();
    populate(&mut space);

    space.each_contact(|contact| {
        graph.add(contact);
    });

    graph.solve();
}
