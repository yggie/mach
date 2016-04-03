use maths::Vect;
use geometry::Line;

#[test]
fn it_can_compute_the_closest_point_to_another_line() {
    let line_0 = Line::from_points(Vect::new(-1.0, 0.0, 0.0), Vect::new(1.0, 0.0, 0.0));
    let line_1 = Line::from_points(Vect::new(0.0, -1.0, 0.1), Vect::new(0.0, 1.0, 0.1));

    assert_approx_eq!(line_0.closest_point_to_line(&line_1), Vect::new(0.0, 0.0, 0.05));
}
