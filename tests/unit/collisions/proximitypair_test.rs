use core::{ Body, UID };
use shapes::Sphere;
use properties::Rigid;
use collisions::ProximityPair;
use math::{ Vector, Matrix, Transform };

fn build_body(id: UID, shape: Box<Sphere>, property: Box<Rigid>, transform: Transform) -> Body {
    Body::new_with_id(id, shape, property, transform, Transform::new_identity())
}

#[test]
fn new_test() {
    let s = Sphere::new(5.0);
    let p = Rigid::new(3.0);
    let t = Transform::new_identity();
    let a = Body::new(box s, box p, t, t);
    let b = Body::new(box s, box p, t, t);

    ProximityPair::new(&a, &b);
}

#[test]
fn sphere_sphere_contact_test() {
    let s = Sphere::new(2.5);
    let p = Rigid::new(3.0);

    let t1 = Transform::new(
        Matrix::new_rotation(-1.3, Vector::new(0.1, 0.0, 0.8)),
        Vector::new_zero()
    );
    let t2 = Transform::new(
        Matrix::new_rotation(2.1, Vector::new(0.5, 0.5, 1.0)),
        Vector::new(4.0, 3.0, 0.0)
    );

    let a = &build_body(10u, box s, box p, t1);
    let b = &build_body(101u, box s, box p, t2);

    let mut pair = ProximityPair::new(a, b);
    match pair.compute_contact(a, b) {
        None => fail!("should be in contact"),
        Some(contact) => {
            assert_eq!(contact.point, Vector::new(2.0, 1.5, 0.0));
        },
    }
}

#[test]
fn sphere_sphere_no_contact_test() {
    let s = Sphere::new(2.5);
    let p = Rigid::new(3.0);

    let a = &build_body(11u, box s, box p, Transform::new_translation(Vector::new(-0.05, -0.05, 0.0)));
    let b = &build_body(1u, box s, box p, Transform::new_translation(Vector::new(5.0, 0.0, 0.0)));

    let mut pair = ProximityPair::new(a, b);
    match pair.compute_contact(a, b) {
        None => (),
        Some(_) => fail!("should not be in contact"),
    }
}
