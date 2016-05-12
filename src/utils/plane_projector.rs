use std::marker::PhantomData;

use TOLERANCE;
use maths::{CrossProduct, DotProduct, UnitVec3D, Vec3D};
use maths::_2d::Vec2D;
use utils::UnitVec3DGenerator;
use geometry::Plane;

pub struct PlaneProjector<'a> {
    x_axis: UnitVec3D,
    y_axis: UnitVec3D,
    _phantom: PhantomData<&'a Plane>,
}

impl<'a> PlaneProjector<'a> {
    pub fn new(plane: &'a Plane) -> PlaneProjector<'a> {
        let mut generator = UnitVec3DGenerator::new();
        let plane_normal = plane.normal();
        let mut guess = generator.next();

        while (1.0 - guess.dot(plane_normal).abs()) < TOLERANCE {
            guess = generator.next();
        }

        let x_axis = plane_normal.cross(guess);
        let y_axis = plane_normal.cross(x_axis);

        PlaneProjector {
            x_axis: x_axis,
            y_axis: y_axis,
            _phantom: PhantomData,
        }
    }

    pub fn project(&self, point: Vec3D) -> Vec2D {
        Vec2D::new(
            self.x_axis.dot(point),
            self.y_axis.dot(point),
        )
    }

    pub fn unproject(&self, point: Vec2D) -> Vec3D {
        point.x * self.x_axis + point.y * self.y_axis
    }
}
