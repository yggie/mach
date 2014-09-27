use mithril::collisions::{ BroadPhase, BruteForce, ContactGraph };
use mithril::properties::Rigid;
use mithril::math::Transform;
use mithril::shapes::Sphere;
use mithril::bodies::Body;

use std::rc::Rc;

fn populate(broadphase: &mut BroadPhase) {
    let s = Sphere::new(1.0);
    let p = Rigid::new(1.0);
    let num_bodies = 10u;

    for i in range(0u, num_bodies) {
        let t = Transform::identity();
        let b = Rc::new(Body::new(box s, box p, t));

        broadphase.add(&b);
    }

    assert_eq!(broadphase.count(), num_bodies);
}

#[test]
fn trapped_spheres() {
    let mut broadphase = BruteForce::new();
    let mut contacts = ContactGraph::new();
    populate(&mut broadphase);

    for bodies in broadphase.partitions().iter() {
        let num_bodies = bodies.len();

        for i in range(0u, num_bodies) {
            for j in range(i + 1, num_bodies) {
                contacts.add_pair(&*bodies[i], &*bodies[j]);
            }
        }

        contacts.solve();
    }
}
