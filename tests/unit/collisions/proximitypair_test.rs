use math::Vector;
use shapes::Sphere;
use properties::Rigid;
use core::{ Body, State };
use collisions::ProximityPair;

#[test]
fn new_test() {
    let s = Sphere::new(5.0);
    let p = Rigid::new(3.0);
    let a = Body::new(box s, box p, State::new_stationary());
    let b = Body::new(box s, box p, State::new_stationary());

    ProximityPair::new(&a, &b);
}

#[test]
fn sphere_sphere_contact_test() {
    let s = Sphere::new(2.5);
    let p = Rigid::new(3.0);

    let state_1 = State::new_with_rotation(-1.3, 0.1, 0.0, 0.8);
    let state_2 = State::new_with_rotation(2.1, 0.5, 0.5, 1.0)
        .with_position(4.0, 3.0, 0.0);

    let a = &Body::new_with_id(10u, box s, box p, state_1);
    let b = &Body::new_with_id(101u, box s, box p, state_2);

    let mut pair = ProximityPair::new(a, b);
    match pair.compute_contact(a, b) {
        None => panic!("should be in contact"),
        Some(contact) => {
            assert_eq!(contact.point, Vector::new(2.0, 1.5, 0.0));
        },
    }
}

#[test]
fn sphere_sphere_no_contact_test() {
    let s = Sphere::new(2.5);
    let p = Rigid::new(3.0);

    let a = &Body::new_with_id(11u, box s, box p, State::new_with_position(-0.05, -0.05, 0.00));
    let b = &Body::new_with_id(1u, box s, box p, State::new_with_position(5.0, 0.0, 0.0));

    let mut pair = ProximityPair::new(a, b);
    match pair.compute_contact(a, b) {
        None => (),
        Some(_) => panic!("should not be in contact"),
    }
}
