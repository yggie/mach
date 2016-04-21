use PI;
use maths::{Transform, UnitQuat, Vect};

#[test]
fn it_can_be_instantiated_as_the_identity_transform() {
    let transform = Transform::identity();

    assert_approx_eq!(transform.translation(), Vect::zero());
    assert_approx_eq!(transform.rotation(), UnitQuat::identity());
}

#[test]
fn it_can_be_instantiated_with_translation_and_rotation() {
    let translation = Vect::new(1.0, 2.0, 3.0);
    let rotation = UnitQuat::from_axis_angle(Vect::new(1.0, 0.3, 0.5), 0.3 * PI);

    let transform = Transform::new(translation, rotation);

    assert_approx_eq!(transform.translation(), translation);
    assert_approx_eq!(transform.rotation(), rotation);
}

#[test]
fn it_can_be_apply_itself_to_a_point() {
    let translation = Vect::new(1.0, 2.0, 3.0);
    let rotation = UnitQuat::from_axis_angle(Vect::new(1.0, 0.0, 0.0), 0.5 * PI);
    let transform = Transform::new(translation, rotation);
    let point = Vect::new(0.0, 1.0, 0.0);

    assert_approx_eq!(transform.apply_to_point(point), Vect::new(1.0, 2.0, 4.0));
}

#[test]
fn it_can_be_apply_itself_to_a_direction() {
    let translation = Vect::new(1.0, 2.0, 3.0);
    let rotation = UnitQuat::from_axis_angle(Vect::new(1.0, 0.0, 0.0), 0.5 * PI);
    let transform = Transform::new(translation, rotation);
    let direction = Vect::new(0.0, 1.0, 0.0);

    assert_approx_eq!(transform.apply_to_direction(direction), Vect::new(0.0, 0.0, 1.0));
}

#[test]
fn it_can_be_apply_its_inverse_to_a_direction() {
    let translation = Vect::new(1.0, 2.0, 3.0);
    let rotation = UnitQuat::from_axis_angle(Vect::new(1.0, 0.0, 0.0), 0.5 * PI);
    let transform = Transform::new(translation, rotation);
    let direction = Vect::new(0.0, 1.0, 0.0);

    assert_approx_eq!(transform.apply_inverse_to_direction(direction), Vect::new(0.0, 0.0, -1.0));
}
