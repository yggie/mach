use maths::Vec3D;
use collisions::shapes::Ray;

#[test]
fn it_can_compute_the_closest_point_to_another_ray() {
    let line_0 = Ray::from_points(Vec3D::new(-1.0, 0.0, 0.0), Vec3D::new(1.0, 0.0, 0.0));
    let line_1 = Ray::from_points(Vec3D::new(0.0, -1.0, 0.1), Vec3D::new(0.0, 1.0, 0.1));

    assert_approx_eq!(Ray::closest_point_to_rays(&line_0, &line_1), Vec3D::new(0.0, 0.0, 0.05));
}
