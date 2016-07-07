use maths::_2d::Vec2D;
use collisions::geometry::_2d::Line2D;

#[test]
fn it_recognises_projected_points_beyond_the_end_of_the_line() {
    let line = Line2D::new(Vec2D::new(0.0, 0.0), Vec2D::new(0.0, 1.0));

    assert!(line.projection_of(&Vec2D::new(0.0, 1.1)).is_after_line());
}

#[test]
fn it_recognises_projected_points_before_the_start_of_the_line() {
    let line = Line2D::new(Vec2D::new(0.0, 0.0), Vec2D::new(0.0, 1.0));

    assert!(line.projection_of(&Vec2D::new(0.0, -0.1)).is_before_line());
}

#[test]
fn it_recognises_projected_points_on_the_line() {
    let line = Line2D::new(Vec2D::new(0.0, 0.0), Vec2D::new(0.0, 1.0));

    assert!(line.projection_of(&Vec2D::new(0.0, 0.1)).is_on_line());
}
