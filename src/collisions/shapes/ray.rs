#[cfg(test)]
#[path="../../../tests/collisions/shapes/ray_test.rs"]
mod tests;

use Scalar;
use maths::{DotProduct, UnitVec3D, Vec3D};
use collisions::shapes::Shape;

pub struct Ray {
    source: Vec3D,
    direction: UnitVec3D,
}

impl Shape for Ray {}

impl Ray {
    #[inline]
    pub fn new(source: Vec3D, direction: UnitVec3D) -> Ray {
        Ray {
            source: source,
            direction: direction,
        }
    }

    #[inline]
    pub fn from_points(start: Vec3D, end: Vec3D) -> Ray {
        Ray::new(start, (end - start).normalize())
    }

    #[inline]
    pub fn direction(&self) -> UnitVec3D {
        self.direction
    }

    #[inline]
    pub fn source(&self) -> &Vec3D {
        &self.source
    }

    pub fn point_along_ray_with_offset(&self, offset: Scalar) -> Vec3D {
        self.source + self.direction * offset
    }

    pub fn closest_point_to_rays(ray_0: &Ray, ray_1: &Ray) -> Vec3D {
        let w = ray_0.source() - ray_1.source();
        let a = ray_0.direction().dot(ray_0.direction());
        let b = ray_0.direction().dot(ray_1.direction());
        let c = ray_1.direction().dot(ray_1.direction());
        let d = ray_0.direction().dot(w);
        let e = ray_1.direction().dot(w);

        let denominator = a*c - b*b;
        let offset_0 = (b*e - c*d) / denominator;
        let offset_1 = (a*e - b*d) / denominator;

        let closest_point_0 = ray_0.point_along_ray_with_offset(offset_0);
        let closest_point_1 = ray_1.point_along_ray_with_offset(offset_1);

        return (closest_point_0 + closest_point_1) / 2.0;
    }
}
