extern crate quickcheck;

use Scalar;
use collisions::geometry::Intersection;
use collisions::geometry::_2d::Ray2D;

use tests::support::{PositiveScalar, Radians};

#[test]
fn it_correctly_identifies_intersecting_rays() {
    fn property(ray: Ray2D, offset: PositiveScalar, rotation: Radians) {
        let offset: Scalar = offset.into();
        let intersection = ray.source() + ray.direction() * offset;
        let new_ray_direction = ray.direction().rotate_by(rotation.into());
        let new_ray_source = intersection - new_ray_direction * offset;
        let new_ray = Ray2D::new(new_ray_source, new_ray_direction);

        let result = ray.intersection(&new_ray)
            .expect(format!("expected {:?} to intersect with {:?}, but did not", ray, new_ray).as_str());

        assert_approx_eq!(result.0, &intersection);
    }

    quickcheck::quickcheck(property as fn(Ray2D, PositiveScalar, Radians));
}

#[test]
fn it_correctly_identifies_non_intersecting_rays() {
    fn property(ray: Ray2D, offset: PositiveScalar) {
        let offset: Scalar = offset.into();
        let new_ray_source = ray.source() + ray.direction().rotate_90() * offset;
        let new_ray = Ray2D::new(new_ray_source, ray.direction().clone());

        match ray.intersection(&new_ray) {
            Some(intersection) => {
                panic!(format!("expected no intersection between {:?} and {:?} but got {:?}", ray, new_ray, intersection))
            },

            _otherwise => (),
        }
    }

    quickcheck::quickcheck(property as fn(Ray2D, PositiveScalar));
}
