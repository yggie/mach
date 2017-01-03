use maths::{Approximations, CrossProduct, Vec3D};
use collisions::shapes::Plane;

pub fn is_coplanar(points: &[Vec3D]) -> bool {
    let mut remaining_points = points.iter();
    let first = remaining_points.next().unwrap();
    let second = remaining_points.next().unwrap();
    let third = remaining_points.next().unwrap();

    let normal = (second - first).cross(third - first).normalize();
    let plane = Plane::new(first.clone(), normal);

    return remaining_points.all(|&point| {
        plane.normal_projection_of(point).is_approximately_zero()
    });
}
