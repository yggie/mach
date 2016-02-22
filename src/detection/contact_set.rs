use Scalar;
use maths::Vect;
use shapes::Plane;

#[derive(Clone)]
pub struct ContactSet {
    plane: Plane,
    // at most, there will be 4 points (FACE-FACE), is there anything we can do
    // to optimise for this use case?
    points: Vec<Vect>,
}

impl ContactSet {
    pub fn new(plane: Plane, points: Vec<Vect>) -> ContactSet {
        ContactSet {
            plane: plane,
            points: points,
        }
    }

    #[inline]
    pub fn point(&self, index: usize) -> &Vect {
        &self.points[index]
    }

    #[inline]
    pub fn surface_normal(&self) -> &Vect {
        self.plane.normal()
    }

    pub fn penetration_depth(&self, index: usize) -> Scalar {
        self.plane.offset_for(self.point(index))
    }
}
