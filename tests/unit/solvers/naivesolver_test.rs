use math::Vector;
use shapes::Sphere;
use properties::Rigid;
use solvers::naive_solver;
use core::{ Database, State };
use collisions::{ BroadPhase, BruteForce };

#[test]
fn non_rotating_sphere_sphere_collision() {
    let database = &mut Database::new();
    let broadphase = &mut BruteForce::new();
    let mut contacts = Vec::new();

    let s = Sphere::new(1.0);
    let p = Rigid::new(1.0);

    let state_1 = State::new_with_rotation(1.3, 1.0, 1.0, 0.0)
            .with_position(2.0, 0.0, 0.0)
            .with_velocity(-1.0, 0.0, 0.0);
    let state_2 = State::new_with_rotation(-2.1, -0.1, 0.5, 0.3)
            .with_velocity(-0.3, 0.0, 0.0);

    let uids = [
        database.create_body(s, p, state_1),
        database.create_body(s, p, state_2),
    ];

    broadphase.reindex(database);
    broadphase.each_contact(database, |contact| contacts.push(contact));

    naive_solver(database, &contacts);

    match database.find(uids[0]) {
        Some(body) => {
            assert_eq!(body.impulse(), Vector::new(-0.7, 0.0, 0.0))
        },
        None => panic!("Could not find Body with UID {}", uids[0]),
    }

    match database.find(uids[1]) {
        Some(body) => {
            assert_eq!(body.impulse(), Vector::new(0.7, 0.0, 0.0))
        },
        None => panic!("Could not find Body with UID {}", uids[1]),
    }
}
