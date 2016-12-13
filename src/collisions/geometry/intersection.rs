use collisions::geometry::Geometry;

pub trait Intersection<T: Geometry> {
    type Output: Geometry;

    fn intersection(&self, other: &T) -> Option<Self::Output>;
}
