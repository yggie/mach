use Scalar;
use maths::Vect;

pub enum ShapeSpec<'a> {
    Sphere(Scalar),
    Cuboid(Scalar, Scalar, Scalar),
    TriangleMesh(&'a Vec<Vect>, &'a Vec<(usize, usize, usize)>),
}
