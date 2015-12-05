use Scalar;

pub enum ShapeSpec {
    Sphere(Scalar),
    Cuboid {
        depth: Scalar,
        width: Scalar,
        height: Scalar,
    },
    TriangleMesh,
    Custom(&'static str),
}
