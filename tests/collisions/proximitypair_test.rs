use bodies::Body;
use shapes::Sphere;
use properties::Rigid;
use collisions::ProximityPair;
use math::{ Vector, Matrix, Transform };

use std::rc::Rc;

fn build_body<'a>(shape: Box<Sphere>, property: Box<Rigid>, transform: Transform) -> Rc<Body<'a>> {
    Rc::new(Body::new(shape, property, transform))
}

#[test]
fn new_test() {
    let s = Sphere::new(5.0);
    let p = Rigid::new(3.0);
    let t = Transform::new_identity();
    let a = Rc::new(Body::new(box s, box p, t));
    let b = Rc::new(Body::new(box s, box p, t));

    ProximityPair::new(a, b);
}

#[test]
fn sphere_sphere_contact_test() {
    let s = Sphere::new(2.5);
    let p = Rigid::new(3.0);

    let a = build_body(box s, box p, Transform::new(
            Matrix::new_rotation(-1.3, Vector::new(0.1, 0.0, 0.8)),
            Vector::new_zero()
            ));
    let b = build_body(box s, box p, Transform::new(
            Matrix::new_rotation(2.1, Vector::new(0.5, 0.5, 1.0)),
            Vector::new(4.0, 3.0, 0.0)
            ));

    let pair = ProximityPair::new(a, b);
    let mut did_contact = false;

    assert!(pair.in_contact());
    pair.if_contact(|contact| {
        if did_contact {
            fail!("contact callback called more than once!");
        }
        did_contact = true;
        assert_eq!(contact.point, Vector::new(2.0, 1.5, 0.0));
    });
}

#[test]
fn sphere_sphere_no_contact_test() {
    let s = Sphere::new(2.5);
    let p = Rigid::new(3.0);

    let a = build_body(box s, box p, Transform::new_translation(Vector::new(-0.05, -0.05, 0.0)));
    let b = build_body(box s, box p, Transform::new_translation(Vector::new(5.0, 0.0, 0.0)));

    let pair = ProximityPair::new(a, b);
    assert!(!pair.in_contact());
    pair.if_contact(|_| {
        fail!("contact callback called when it should not have!");
    });
}
