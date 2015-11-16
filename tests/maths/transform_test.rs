use mach::PI;
use mach::maths::{ Transform, Quat, Vector };

#[test]
fn it_can_be_instantiated_as_the_identity_transform() {
    let transform = Transform::new_identity();

    assert_approx_eq!(transform.translation(), Vector::new_zero());
    assert_approx_eq!(transform.rotation(), Quat::new_identity());
}

#[test]
fn it_can_be_instantiated_with_translation_and_rotation() {
    let translation = Vector::new(1.0, 2.0, 3.0);
    let rotation = Quat::new_from_axis_angle(Vector::new(1.0, 0.3, 0.5), 0.3 * PI);

    let transform = Transform::new(translation, rotation);

    assert_approx_eq!(transform.translation(), translation);
    assert_approx_eq!(transform.rotation(), rotation);
}

#[test]
fn it_can_be_apply_itself_to_a_point() {
    let translation = Vector::new(1.0, 2.0, 3.0);
    let rotation = Quat::new_from_axis_angle(Vector::new(1.0, 0.0, 0.0), 0.5 * PI);
    let transform = Transform::new(translation, rotation);
    let point = Vector::new(0.0, 1.0, 0.0);

    assert_approx_eq!(transform.apply_to_point(point), Vector::new(1.0, 2.0, 4.0));
}

#[test]
fn it_can_be_apply_itself_to_a_direction() {
    let translation = Vector::new(1.0, 2.0, 3.0);
    let rotation = Quat::new_from_axis_angle(Vector::new(1.0, 0.0, 0.0), 0.5 * PI);
    let transform = Transform::new(translation, rotation);
    let direction = Vector::new(0.0, 1.0, 0.0);

    assert_approx_eq!(transform.apply_to_direction(direction), Vector::new(0.0, 0.0, 1.0));
}

#[test]
fn it_can_be_apply_its_inverse_to_a_direction() {
    let translation = Vector::new(1.0, 2.0, 3.0);
    let rotation = Quat::new_from_axis_angle(Vector::new(1.0, 0.0, 0.0), 0.5 * PI);
    let transform = Transform::new(translation, rotation);
    let direction = Vector::new(0.0, 1.0, 0.0);

    assert_approx_eq!(transform.apply_inverse_to_direction(direction), Vector::new(0.0, 0.0, -1.0));
}
