use collisions::shapes::convex_shapes::{Cuboid, Sphere, TriangleMesh};

pub enum ShapeRef<'a> {
    Sphere(&'a Sphere),
    Cuboid(&'a Cuboid),
    TriangleMesh(&'a TriangleMesh),
}
