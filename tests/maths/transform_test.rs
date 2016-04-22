use PI;
use maths::{Transform, UnitQuat, Vec3D};

#[test]
fn it_can_be_instantiated_as_the_identity_transform() {
    let transform = Transform::identity();

    assert_approx_eq!(transform.translation(), Vec3D::zero());
    assert_approx_eq!(transform.rotation(), UnitQuat::identity());
}

#[test]
fn it_can_be_instantiated_with_translation_and_rotation() {
    let translation = Vec3D::new(1.0, 2.0, 3.0);
    let rotation = UnitQuat::from_axis_angle(Vec3D::new(1.0, 0.3, 0.5).normalize(), 0.3 * PI);

    let transform = Transform::new(translation, rotation);

    assert_approx_eq!(transform.translation(), translation);
    assert_approx_eq!(transform.rotation(), rotation);
}

#[test]
fn it_can_be_apply_itself_to_a_point() {
    let translation = Vec3D::new(1.0, 2.0, 3.0);
    let rotation = UnitQuat::from_axis_angle(Vec3D::new(1.0, 0.0, 0.0).normalize(), 0.5 * PI);
    let transform = Transform::new(translation, rotation);
    let point = Vec3D::new(0.0, 1.0, 0.0);

    assert_approx_eq!(transform.apply_to_point(point), Vec3D::new(1.0, 2.0, 4.0));
}

#[test]
fn it_can_be_apply_itself_to_a_direction() {
    let translation = Vec3D::new(1.0, 2.0, 3.0);
    let rotation = UnitQuat::from_axis_angle(Vec3D::new(1.0, 0.0, 0.0).normalize(), 0.5 * PI);
    let transform = Transform::new(translation, rotation);
    let direction = Vec3D::new(0.0, 1.0, 0.0);

    assert_approx_eq!(transform.apply_to_direction(direction), Vec3D::new(0.0, 0.0, 1.0));
}

#[test]
fn it_can_be_apply_its_inverse_to_a_direction() {
    let translation = Vec3D::new(1.0, 2.0, 3.0);
    let rotation = UnitQuat::from_axis_angle(Vec3D::new(1.0, 0.0, 0.0).normalize(), 0.5 * PI);
    let transform = Transform::new(translation, rotation);
    let direction = Vec3D::new(0.0, 1.0, 0.0);

    assert_approx_eq!(transform.apply_inverse_to_direction(direction), Vec3D::new(0.0, 0.0, -1.0));
}
