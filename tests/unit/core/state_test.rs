use core::State;
use math::{ Vector, Quaternion, PI };

#[test]
fn new_stationary_test() {
    let s = State::new_stationary();

    assert_eq!(s.position(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(s.rotation(), Quaternion::new(1.0, 0.0, 0.0, 0.0));
    assert_eq!(s.velocity(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(s.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
}

#[test]
fn new_with_position_test() {
    let s = State::new_with_position(1.0, 0.0, -1.0);

    assert_eq!(s.position(), Vector::new(1.0, 0.0, -1.0));
    assert_eq!(s.rotation(), Quaternion::new(1.0, 0.0, 0.0, 0.0));
    assert_eq!(s.velocity(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(s.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
}

#[test]
fn new_with_rotation_test() {
    let s = State::new_with_rotation(PI, 20.0, 12.0, -9.0);

    assert_eq!(s.position(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(s.rotation(), Quaternion::new(0.0, 0.80, 0.48, -0.36));
    assert_eq!(s.velocity(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(s.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
}

#[test]
fn set_position_test() {
    let mut s = State::new_stationary();
    s.set_position(-3.3, 5.5, 4.1);

    assert_eq!(s.position(), Vector::new(-3.3, 5.5, 4.1));
}

#[test]
fn set_position_with_vector_test() {
    let mut s = State::new_stationary();
    let v = Vector::new(-2.1, 7.7, -2.1);
    s.set_position_with_vector(v);

    assert_eq!(s.position(), v);
}

#[test]
fn set_rotation_test() {
    let mut s = State::new_stationary();
    s.set_rotation(0.0, 11.0, -9.0, 20.0);

    assert_eq!(s.rotation(), Quaternion::new(1.0, 0.0, 0.0, 0.0));
}

#[test]
fn with_position_test() {
    let original_state = State::new_stationary();
    let state = original_state.with_position(0.0, 9.0, 0.0);

    assert_eq!(state.position(), Vector::new(0.0, 9.0, 0.0));
}

#[test]
fn set_velocity_test() {
    let mut s = State::new_stationary();
    s.set_velocity(1.0, 0.0, -5.5);

    assert_eq!(s.velocity(), Vector::new(1.0, 0.0, -5.5));
}

#[test]
fn with_rotation_test() {
    let original_state = State::new_stationary();
    let state = original_state.with_rotation(3.0*PI, 20.0, 12.0, -9.0);

    assert_eq!(state.rotation(), Quaternion::new(0.0, -0.80, -0.48, 0.36));
}

#[test]
fn set_velocity_with_vector_test() {
    let mut s = State::new_stationary();
    let v = Vector::new(-1.1, -5.3, -0.1);
    s.set_velocity_with_vector(v);

    assert_eq!(s.velocity(), v);
}

#[test]
fn with_velocity_test() {
    let original_state = State::new_stationary();
    let state = original_state.with_velocity(1.0, -1.0, 1.0);

    assert_eq!(state.velocity(), Vector::new(1.0, -1.0, 1.0));
}

#[test]
fn set_angular_velocity_test() {
    let mut s = State::new_stationary();
    s.set_angular_velocity(3.0, 1.0, 1.5);

    assert_eq!(s.angular_velocity(), Vector::new(3.0, 1.0, 1.5));
}

#[test]
fn with_angular_velocity_test() {
    let original_state = State::new_stationary();
    let state = original_state.with_angular_velocity(1.0, 3.0, 9.0);

    assert_eq!(state.angular_velocity(), Vector::new(1.0, 3.0, 9.0));
}
