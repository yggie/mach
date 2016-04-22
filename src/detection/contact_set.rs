use Scalar;
use maths::Vec3D;
use geometry::Plane;

#[derive(Clone)]
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

    #[inline]
    pub fn point(&self, index: usize) -> &Vec3D {
        &self.points[index]
    }

    #[inline]
    pub fn points(&self) -> &Vec<Vec3D> {
        &self.points
    }

    #[inline]
    pub fn normal(&self) -> &Vec3D {
        self.plane.normal()
    }

    pub fn penetration_depth(&self, index: usize) -> Scalar {
        self.plane.offset_for(self.point(index))
    }
}
