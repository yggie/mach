use collisions::shapes::Shape;

pub trait Intersection<T: Shape> {
    type Output;

    fn intersection(&self, other: &T) -> Option<Self::Output>;

    fn fast_intersection(&self, other: &T) -> bool {
        self.intersection(other).is_some()
    }
}
