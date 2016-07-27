use Scalar;
use maths::{CrossProduct, DotProduct, UnitVec3D, Vec3D};
use collisions::geometry::Plane;

#[derive(Clone)]
pub struct Face<'a> {
    indices: [usize; 3],
    vertices: &'a [Vec3D],
}

impl<'a> Face<'a> {
    pub fn counter_clockwise_from(vertices: &'a [Vec3D], indices: [usize; 3]) -> Face<'a> {
        Face {
            indices: indices,
            vertices: vertices,
        }
    }

    #[inline(always)]
    pub fn normal(&self) -> UnitVec3D {
        let vertex_0 = self.vertex(0);
        let vertex_1 = self.vertex(1);
        let vertex_2 = self.vertex(2);

        (vertex_2 - vertex_1).cross(vertex_0 - vertex_1).normalize()
    }

    #[inline(always)]
    pub fn vertex(&self, index: usize) -> Vec3D {
        self.vertices[self.indices[index]]
    }

    pub fn indices(&self) -> [usize; 3] {
        self.indices.clone()
    }

    pub fn normal_projection_of(&self, point: Vec3D) -> Scalar {
        self.normal().dot(point - self.vertices[self.indices[0]])
    }

    pub fn normal_projection_of_origin(&self) -> Scalar {
        -self.normal().dot(self.vertices[self.indices[0]])
    }
}

impl<'a> From<Face<'a>> for Plane {
    fn from(face: Face<'a>) -> Plane {
        Plane::new(face.vertices[0], face.normal())
    }
}
