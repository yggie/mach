use collisions::{ BroadPhase, BruteForce };
use solvers::force::naive_solver;
use math::{ Vector, Matrix, Transform };
use properties::Rigid;
use shapes::Sphere;
use core::Database;

#[test]
fn non_rotating_sphere_sphere_collision() {
    let database = &mut Database::new();
    let broadphase = &mut BruteForce::new();
    let mut contacts = Vec::new();

    let s = Sphere::new(1.0);
    let p = Rigid::new(1.0);

    let t_0 = Transform::new(Matrix::new_rotation(1.3, Vector::new(1.0, 1.0, 0.0)), Vector::new(2.0, 0.0, 0.0));
    let t_1 = Transform::new_rotation(Matrix::new_rotation(-2.1, Vector::new(-0.1, 0.5, 0.3)));

    let v_0 = Transform::new_translation(Vector::new(-1.0, 0.0, 0.0));
    let v_1 = Transform::new_translation(Vector::new(-0.3, 0.0, 0.0));

    let b_0 = database.create_body(s, p, t_0, v_0);
    let b_1 = database.create_body(s, p, t_1, v_1);

    broadphase.reindex(database);
    broadphase.each_contact(database, |contact| contacts.push(contact));

    naive_solver(database, &contacts);

    match database.find(b_0) {
        Some(b) => {
            assert_eq!(b.impulse(), Vector::new(-0.7, 0.0, 0.0))
        },
        None => fail!("Could not find Body with UID {}", b_0),
    }

    match database.find(b_1) {
        Some(b) => {
            assert_eq!(b.impulse(), Vector::new(0.7, 0.0, 0.0))
        },
        None => fail!("Could not find Body with UID {}", b_1),
    }
}
