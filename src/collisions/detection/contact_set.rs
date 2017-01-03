use Scalar;
use maths::{UnitVec3D, Vec3D};
use collisions::shapes::Plane;

#[derive(Clone, Debug)]
pub struct ContactSet {
    plane: Plane,
    // at most, there will be 4 points (FACE-FACE), is there anything we can do
    // to optimise for this use case?
    points: Vec<Vec3D>,
}

impl ContactSet {
    pub fn new(plane: Plane, points: Vec<Vec3D>) -> ContactSet {
        ContactSet {
            plane: plane,
            points: points,
        }
    }

    #[inline(always)]
    pub fn point(&self, index: usize) -> Vec3D {
        self.points[index]
    }

    #[inline(always)]
    pub fn points(&self) -> &Vec<Vec3D> {
        &self.points
    }

    #[inline(always)]
    pub fn normal(&self) -> UnitVec3D {
        self.plane.normal()
    }

    pub fn penetration_depth(&self, index: usize) -> Scalar {
        self.plane.normal_projection_of(self.point(index))
    }
}
