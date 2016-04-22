use Scalar;
use maths::Vec3D;

pub enum ShapeSpec<'a> {
    Sphere(Scalar),
    Cuboid(Scalar, Scalar, Scalar),
    TriangleMesh(&'a Vec<Vec3D>, &'a Vec<(usize, usize, usize)>),
}
