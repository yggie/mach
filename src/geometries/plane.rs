use {Scalar, TOLERANCE};
use maths::Vector;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlaneLocation {
    Above,
    Plane,
    Below,
}

#[derive(Clone)]
pub struct Plane {
    normal: Vector,
    point_on_plane: Vector,
}

impl Plane {
    pub fn from_point(point: &Vector, normal: &Vector) -> Plane {
        Plane {
            normal: normal.normalize(),
            point_on_plane: point.clone(),
        }
    }

    pub fn from_reference(vertices: (Vector, Vector, Vector), reference_point: Vector) -> Plane {
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
            point_on_plane: vertices.0,
        };
    }

    #[inline]
    pub fn offset_for_origin(&self) -> Scalar {
        -Vector::dot(&self.normal, self.point_on_plane)
    }

    #[inline]
    pub fn offset_for(&self, point: &Vector) -> Scalar {
        Vector::dot(&self.normal, point - self.point_on_plane)
    }

    pub fn location_of(&self, point: &Vector) -> PlaneLocation {
        let offset = self.offset_for(point);

        if offset > TOLERANCE {
            PlaneLocation::Above
        } else if offset < TOLERANCE {
            PlaneLocation::Below
        } else {
            PlaneLocation::Plane
        }
    }

    #[inline]
    pub fn normal(&self) -> &Vector {
        &self.normal
    }
}
