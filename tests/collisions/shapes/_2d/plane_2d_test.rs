use maths::_2d::Vec2D;
use collisions::shapes::_2d::Plane2D;

#[test]
fn it_can_project_2d_vectors_along_the_plane_normal() {
    let plane = Plane2D::new(Vec2D::new(0.0, 0.0), Vec2D::new(1.0, 0.0).normalize());

    assert_approx_eq!(plane.normal_projection_of(&Vec2D::new(0.1, 0.0)), 0.1);
}
