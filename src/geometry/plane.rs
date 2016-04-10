use {Scalar, TOLERANCE};
use maths::Vect;
use geometry::PlaneLocation;

#[derive(Clone)]
pub struct Plane {
    normal: Vect,
    reference_point: Vect,
}

impl Plane {
    pub fn from_point(point: &Vect, normal: &Vect) -> Plane {
        Plane {
            normal: normal.normalize(),
            reference_point: point.clone(),
        }
    }

    pub fn from_reference(vertices: (Vect, Vect, Vect), reference_point: Vect) -> Plane {
        let edge_01 = vertices.1 - vertices.0;
        let edge_12 = vertices.2 - vertices.1;
        let guess = edge_01.cross(edge_12).normalize();

        let reference_offset = reference_point - vertices.0;
        let normal = if guess.dot(reference_offset) > 0.0 {
            -guess
        } else {
            guess
        };

        return Plane {
            normal: normal,
            reference_point: vertices.0,
        };
    }

    #[inline]
    pub fn reversed(self) -> Plane {
        Plane {
            normal: -self.normal,
            .. self
        }
    }

    #[inline]
    pub fn offset_for_origin(&self) -> Scalar {
        -Vect::dot(&self.normal, self.reference_point)
    }

    #[inline]
    pub fn offset_for(&self, point: &Vect) -> Scalar {
        Vect::dot(&self.normal, point - self.reference_point)
    }

    pub fn location_of(&self, point: &Vect) -> PlaneLocation {
        match self.offset_for(point) {
            x if x > TOLERANCE => PlaneLocation::Above(x),
            x if -x > TOLERANCE => PlaneLocation::Below(x),
            x => PlaneLocation::OnPlane(x),
        }
    }

    #[inline]
    pub fn normal(&self) -> &Vect {
        &self.normal
    }

    #[inline]
    pub fn reference(&self) -> &Vect {
        &self.reference_point
    }
}
