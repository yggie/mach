use PI;
use maths::{Transform, UnitQuat, Vec3D};
use collisions::geometry::Direction;

#[test]
fn it_can_be_transformed_by_a_transform() {
    let translation = Vec3D::new(1.0, 2.0, 3.0);
    let rotation = UnitQuat::from_axis_angle(Vec3D::new(1.0, 0.0, 0.0).normalize(), 0.5 * PI);
    let transform = Transform::new(translation, rotation);
    let direction = Direction::from(Vec3D::new(0.0, 1.0, 0.0));

    let new_direction = direction.transform_with(&transform);
    assert_approx_eq!(Vec3D::from(new_direction), Vec3D::new(0.0, 0.0, 1.0));
}

#[test]
fn it_can_be_transformed_by_the_inverse_of_a_transform() {
    let translation = Vec3D::new(1.0, 2.0, 3.0);
    let rotation = UnitQuat::from_axis_angle(Vec3D::new(1.0, 0.0, 0.0).normalize(), 0.5 * PI);
    let transform = Transform::new(translation, rotation);
    let direction = Direction::from(Vec3D::new(0.0, 1.0, 0.0));

    let new_direction = direction.transform_with_inverse_of(&transform);
    assert_approx_eq!(Vec3D::from(new_direction), Vec3D::new(0.0, 0.0, -1.0));
}
