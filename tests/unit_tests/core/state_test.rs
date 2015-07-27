use mach::core::State;
use mach::maths::{ Vector, Quaternion, PI };

#[test]
fn instantiating_as_stationary() {
    let s = State::new_stationary();

    assert_eq!(s.position(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(s.rotation(), Quaternion::new(1.0, 0.0, 0.0, 0.0));
    assert_eq!(s.velocity(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(s.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
}

#[test]
fn instantiating_with_position() {
    let s = State::new_with_position(1.0, 0.0, -1.0);

    assert_eq!(s.position(), Vector::new(1.0, 0.0, -1.0));
    assert_eq!(s.rotation(), Quaternion::new(1.0, 0.0, 0.0, 0.0));
    assert_eq!(s.velocity(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(s.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
}

#[test]
fn instantiating_with_rotation() {
    let s = State::new_with_rotation(Vector::new(20.0, 12.0, -9.0), PI);

    assert_eq!(s.position(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(s.rotation(), Quaternion::new(0.0, 0.80, 0.48, -0.36));
    assert_eq!(s.velocity(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(s.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
}

#[test]
fn setting_the_position_with_scalar_components() {
    let mut s = State::new_stationary();
    s.set_position(-3.3, 5.5, 4.1);

    assert_eq!(s.position(), Vector::new(-3.3, 5.5, 4.1));
}

#[test]
fn setting_the_position_with_a_vector() {
    let mut s = State::new_stationary();
    let v = Vector::new(-2.1, 7.7, -2.1);
    s.set_position_with_vector(v);

    assert_eq!(s.position(), v);
}

#[test]
fn chaining_with_position() {
    let original_state = State::new_stationary();
    let state = original_state.with_position(0.0, 9.0, 0.0);

    assert_eq!(state.position(), Vector::new(0.0, 9.0, 0.0));
}

#[test]
fn setting_the_velocity_with_scalar_components() {
    let mut s = State::new_stationary();
    s.set_velocity(1.0, 0.0, -5.5);

    assert_eq!(s.velocity(), Vector::new(1.0, 0.0, -5.5));
}

#[test]
fn setting_velocity_with_a_vector() {
    let mut s = State::new_stationary();
    let v = Vector::new(-1.1, -5.3, -0.1);
    s.set_velocity_with_vector(v);

    assert_eq!(s.velocity(), v);
}

#[test]
fn chaining_with_velocity() {
    let original_state = State::new_stationary();
    let state = original_state.with_velocity(1.0, -1.0, 1.0);

    assert_eq!(state.velocity(), Vector::new(1.0, -1.0, 1.0));
}

#[test]
fn setting_the_rotation_with_scalar_components() {
    let mut s = State::new_stationary();
    s.set_rotation(Vector::new(11.0, -9.0, 20.0), 0.0);

    assert_eq!(s.rotation(), Quaternion::new(1.0, 0.0, 0.0, 0.0));
}

#[test]
fn chaining_with_rotation() {
    let original_state = State::new_stationary();
    let state = original_state.with_rotation(Vector::new(20.0, 12.0, -9.0), 3.0*PI);

    assert_eq!(state.rotation(), Quaternion::new(0.0, -0.80, -0.48, 0.36));
}

#[test]
fn setting_the_angular_velocity_with_scalar_components() {
    let mut s = State::new_stationary();
    s.set_angular_velocity(3.0, 1.0, 1.5);

    assert_eq!(s.angular_velocity(), Vector::new(3.0, 1.0, 1.5));
}

#[test]
fn chaining_with_angular_velocity() {
    let original_state = State::new_stationary();
    let state = original_state.with_angular_velocity(1.0, 3.0, 9.0);

    assert_eq!(state.angular_velocity(), Vector::new(1.0, 3.0, 9.0));
}

#[test]
fn transforming_a_point_with_only_translation() {
    let state = State::new_with_position(1.0, 2.0, 3.0);

    let v = state.transform_point(Vector::new(4.0, 5.0, 6.0));

    assert_eq!((v[0], v[1], v[2]), (5.0, 7.0, 9.0));
}

#[test]
fn transforming_a_point_with_only_rotation() {
    let state = State::new_with_rotation(Vector::new(1.0, 1.0, 1.0), PI/2.0);

    let v = state.transform_point(Vector::new(4.0, 5.0, 6.0));

    assert!(v.distance_to(Vector::new(5.577350269189626, 3.845299461620749, 5.577350269189626)) < 0.001);
}

#[test]
fn transforming_a_point_with_both_translation_and_rotation() {
    let state = State::new_with_rotation(Vector::new(1.0, 2.0, -1.0), PI/3.0)
        .with_position(1.0, -1.0, 2.0);

    let v = state.transform_point(Vector::new(3.0, 2.0, 1.0));

    assert!(v.distance_to(Vector::new(4.4142135623730954, -0.41421356237309503, 0.5857864376269053)) < 0.001);
}

#[test]
fn transforming_a_direction_with_only_translation() {
    let state = State::new_with_position(7.0, 8.0, 9.0);

    let v = state.transform_direction(Vector::new(1.0, 2.0, 3.0));

    assert_eq!((v[0], v[1], v[2]), (1.0, 2.0, 3.0));
}

#[test]
fn transforming_a_direction_with_only_rotation() {
    let state = State::new_with_rotation(Vector::new(1.0, 1.0, 1.0), PI/2.0);

    let v = state.transform_direction(Vector::new(4.0, 5.0, 6.0));

    assert!(v.distance_to(Vector::new(5.577350269189626, 3.845299461620749, 5.577350269189626)) < 0.001);
}

#[test]
fn transforming_a_direction_with_translation_and_rotation() {
    let state = State::new_with_rotation(Vector::new(1.0, 1.0, 1.0), PI/2.0)
        .with_position(1.0, -1.0, 2.0);

    let v = state.transform_direction(Vector::new(4.0, 5.0, 6.0));

    assert!(v.distance_to(Vector::new(5.577350269189626, 3.845299461620749, 5.577350269189626)) < 0.001);
}
