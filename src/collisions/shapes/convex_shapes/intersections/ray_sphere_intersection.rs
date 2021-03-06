use maths::Vec3D;
use collisions::shapes::{Intersection, Point, Ray};
use collisions::shapes::convex_shapes::Sphere;

impl Intersection<Sphere> for Ray {
    type Output = [Point; 2];

    fn intersection(&self, sphere: &Sphere) -> Option<Self::Output> {
        Some([Point::from(Vec3D::zero()), Point::from(Vec3D::zero())])
    }
}

impl Intersection<Ray> for Sphere {
    type Output = [Point; 2];

    fn intersection(&self, ray: &Ray) -> Option<Self::Output> {
        ray.intersection(self)
    }
}
