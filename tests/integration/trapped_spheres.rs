use mithril::collisions::{ BroadPhase, BruteForce };
use mithril::solvers::force::naive_solver;
use mithril::properties::Rigid;
use mithril::math::Transform;
use mithril::core::Database;
use mithril::shapes::Sphere;

fn seed(database: &mut Database) {
    let s = Sphere::new(1.0);
    let p = Rigid::new(1.0);
    let num_bodies = 10u;

    for _ in range(0u, num_bodies) {
        let t = Transform::new_identity();
        let dt = Transform::new_identity();
        database.create_body(s, p, t, dt);
    }

    assert_eq!(database.size(), num_bodies);
}

#[test]
fn trapped_spheres() {
    let database = &mut Database::new();
    let broadphase = &mut BruteForce::new();
    let mut contacts = Vec::new();
    seed(database);

    broadphase.reindex(database);
    broadphase.each_contact(database, |contact| contacts.push(contact));

    naive_solver(database, &contacts);
}
