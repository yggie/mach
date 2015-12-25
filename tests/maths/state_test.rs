use mach::PI;
use mach::maths::{State, Vect, Quat};

#[test]
fn it_can_be_instantiated_as_stationary() {
    let s = State::new_stationary();

    assert_eq!(s.pos(), Vect::new(0.0, 0.0, 0.0));
    assert_eq!(s.rot(), Quat::new(1.0, 0.0, 0.0, 0.0));
    assert_eq!(s.vel(), Vect::new(0.0, 0.0, 0.0));
    assert_eq!(s.ang_vel(), Vect::new(0.0, 0.0, 0.0));
}

#[test]
fn it_can_be_instantiated_with_a_position() {
    let s = State::new_with_pos(1.0, 0.0, -1.0);

    assert_eq!(s.pos(), Vect::new(1.0, 0.0, -1.0));
    assert_eq!(s.rot(), Quat::new(1.0, 0.0, 0.0, 0.0));
    assert_eq!(s.vel(), Vect::new(0.0, 0.0, 0.0));
    assert_eq!(s.ang_vel(), Vect::new(0.0, 0.0, 0.0));
}

#[test]
fn it_can_be_instantiated_with_axis_angle_rotation() {
    let s = State::new_with_axis_angle(Vect::new(20.0, 12.0, -9.0), PI);

    assert_eq!(s.pos(), Vect::new(0.0, 0.0, 0.0));
    assert_eq!(s.rot(), Quat::new(0.0, 0.80, 0.48, -0.36));
    assert_eq!(s.vel(), Vect::new(0.0, 0.0, 0.0));
    assert_eq!(s.ang_vel(), Vect::new(0.0, 0.0, 0.0));
}

#[test]
fn it_can_set_the_position_with_scalars() {
    let mut s = State::new_stationary();

    s.set_pos(&(-3.3, 5.5, 4.1));

    assert_eq!(s.pos(), Vect::new(-3.3, 5.5, 4.1));
}

#[test]
fn it_can_set_the_position_with_a_vector() {
    let mut s = State::new_stationary();
    let v = Vect::new(-2.1, 7.7, -2.1);

    s.set_pos(&v);

    assert_eq!(s.pos(), v);
}

#[test]
fn chaining_with_position() {
    let original_state = State::new_stationary();
    let state = original_state.with_pos(0.0, 9.0, 0.0);

    assert_eq!(state.pos(), Vect::new(0.0, 9.0, 0.0));
}

#[test]
fn setting_the_velocity_with_scalar_components() {
    let mut s = State::new_stationary();

    s.set_vel(&(1.0, 0.0, -5.5));

    assert_eq!(s.vel(), Vect::new(1.0, 0.0, -5.5));
}

#[test]
fn setting_velocity_with_a_vector() {
    let mut s = State::new_stationary();
    let v = Vect::new(-1.1, -5.3, -0.1);

    s.set_vel(&v);

    assert_eq!(s.vel(), v);
}

#[test]
fn chaining_with_velocity() {
    let original_state = State::new_stationary();
    let state = original_state.with_vel(1.0, -1.0, 1.0);

    assert_eq!(state.vel(), Vect::new(1.0, -1.0, 1.0));
}

#[test]
fn setting_the_rotation_with_axis_angle() {
    let mut s = State::new_stationary();
    s.set_axis_angle(Vect::new(11.0, -9.0, 20.0), 0.0);

    assert_eq!(s.rot(), Quat::new(1.0, 0.0, 0.0, 0.0));
}

#[test]
fn chaining_with_axis_angle() {
    let original_state = State::new_stationary();
    let state = original_state.with_axis_angle(Vect::new(20.0, 12.0, -9.0), 3.0*PI);

    assert_eq!(state.rot(), Quat::new(0.0, -0.80, -0.48, 0.36));
}

#[test]
fn setting_the_angular_velocity_with_scalar_components() {
    let mut s = State::new_stationary();

    s.set_ang_vel(&(3.0, 1.0, 1.5));

    assert_eq!(s.ang_vel(), Vect::new(3.0, 1.0, 1.5));
}

#[test]
fn chaining_with_angular_velocity() {
    let original_state = State::new_stationary();
    let state = original_state.with_ang_vel(1.0, 3.0, 9.0);

    assert_eq!(state.ang_vel(), Vect::new(1.0, 3.0, 9.0));
}

#[test]
fn transforming_a_point_with_only_translation() {
    let state = State::new_with_pos(1.0, 2.0, 3.0);

    let v = state.transform_point(Vect::new(4.0, 5.0, 6.0));

    assert_eq!((v.x, v.y, v.z), (5.0, 7.0, 9.0));
}

#[test]
fn transforming_a_point_with_only_rotation() {
    let state = State::new_with_axis_angle(Vect::new(1.0, 1.0, 1.0), PI/2.0);

    let v = state.transform_point(Vect::new(4.0, 5.0, 6.0));

    assert!(v.distance_to(Vect::new(5.577350269189626, 3.845299461620749, 5.577350269189626)) < 0.001);
}

#[test]
fn transforming_a_point_with_both_translation_and_rotation() {
    let state = State::new_with_axis_angle(Vect::new(1.0, 2.0, -1.0), PI/3.0)
        .with_pos(1.0, -1.0, 2.0);

    let v = state.transform_point(Vect::new(3.0, 2.0, 1.0));

    assert!(v.distance_to(Vect::new(4.4142135623730954, -0.41421356237309503, 0.5857864376269053)) < 0.001);
}

#[test]
fn transforming_a_direction_with_only_translation() {
    let state = State::new_with_pos(7.0, 8.0, 9.0);

    let v = state.transform_direction(Vect::new(1.0, 2.0, 3.0));

    assert_eq!((v.x, v.y, v.z), (1.0, 2.0, 3.0));
}

#[test]
fn transforming_a_direction_with_only_rotation() {
    let state = State::new_with_axis_angle(Vect::new(1.0, 1.0, 1.0), PI/2.0);

    let v = state.transform_direction(Vect::new(4.0, 5.0, 6.0));

    assert!(v.distance_to(Vect::new(5.577350269189626, 3.845299461620749, 5.577350269189626)) < 0.001);
}

#[test]
fn transforming_a_direction_with_translation_and_rotation() {
    let state = State::new_with_axis_angle(Vect::new(1.0, 1.0, 1.0), PI/2.0)
        .with_pos(1.0, -1.0, 2.0);

    let v = state.transform_direction(Vect::new(4.0, 5.0, 6.0));

    assert!(v.distance_to(Vect::new(5.577350269189626, 3.845299461620749, 5.577350269189626)) < 0.001);
}
