use std::marker::PhantomData;

use TOLERANCE;
use maths::{CrossProduct, DotProduct, UnitVec3D, Vec3D};
use utils::UnitVec3DGenerator;
use collisions::geometry::Plane;

#[derive(Clone, Debug)]
pub struct CoordinateTransform<'a> {
    x_axis: UnitVec3D,
    y_axis: UnitVec3D,
    z_axis: UnitVec3D,
    _phantom: PhantomData<&'a Plane>,
}

impl<'a> CoordinateTransform<'a> {
    pub fn from_plane(plane: &'a Plane) -> CoordinateTransform<'a> {
        let mut generator = UnitVec3DGenerator::new();
        let plane_normal = plane.normal();
        let mut guess = generator.gen_next();

        while (1.0 - guess.dot(plane_normal).abs()) < TOLERANCE {
            guess = generator.gen_next();
        }

        let x_axis = plane_normal.cross(guess);
        let y_axis = plane_normal.cross(x_axis);

        CoordinateTransform {
            x_axis: x_axis,
            y_axis: y_axis,
            z_axis: plane_normal,
            _phantom: PhantomData,
        }
    }

    pub fn transform(&self, point: Vec3D) -> Vec3D {
        Vec3D::new(
            self.x_axis.dot(point),
            self.y_axis.dot(point),
            self.z_axis.dot(point),
        )
    }

    pub fn transform_with_inverse(&self, point: Vec3D) -> Vec3D {
        point.x * self.x_axis + point.y * self.y_axis + point.z * self.z_axis
    }
}
