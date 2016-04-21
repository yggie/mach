use maths::_2d::Vec2D;
use geometry::_2d::Plane2D;

#[test]
fn it_can_project_2d_vectors_along_the_plane_normal() {
    let plane = Plane2D::new(Vec2D::new(0.0, 0.0), Vec2D::new(1.0, 0.0).normalize());

    assert_approx_eq!(plane.project_along_normal(&Vec2D::new(0.1, 0.0)), 0.1);
}

#[test]
fn it_recognises_points_above_the_plane() {
    let plane = Plane2D::new(Vec2D::new(0.0, 0.0), Vec2D::new(1.0, 0.0).normalize());

    assert!(plane.projection_of(&Vec2D::new(0.1, 0.0)).is_above_plane());
}

#[test]
fn it_recognises_points_below_the_plane() {
    let plane = Plane2D::new(Vec2D::new(0.0, 0.0), Vec2D::new(1.0, 0.0).normalize());

    assert!(plane.projection_of(&Vec2D::new(-0.1, 0.0)).is_below_plane());
}

#[test]
fn it_recognises_points_on_the_plane() {
    let plane = Plane2D::new(Vec2D::new(0.0, 0.0), Vec2D::new(1.0, 0.0).normalize());

    assert!(plane.projection_of(&Vec2D::new(0.0, 0.0)).is_on_plane());
}
