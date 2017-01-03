use maths::Vec3D;
use collisions::shapes::Shape;

pub struct Point(Vec3D);

impl Shape for Point {}

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
