use Scalar;
use maths::Vector;

pub enum ShapeSpec<'a> {
    Sphere(Scalar),
    Cuboid {
        depth: Scalar,
        width: Scalar,
        height: Scalar,
    },
    TriangleMesh(&'a Vec<Vector>, &'a Vec<(usize, usize, usize)>),
    Custom(&'a str),
}
