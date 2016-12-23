use maths::Vec3D;
use collisions::geometry::Geometry;

pub struct Point(Vec3D);

impl Geometry for Point {}

impl From<Vec3D> for Point {
    fn from(vec: Vec3D) -> Point {
        Point(vec)
    }
}

impl From<Point> for Vec3D {
    fn from(point: Point) -> Vec3D {
        point.0
    }
}
