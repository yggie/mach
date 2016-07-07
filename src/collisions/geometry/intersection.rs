pub trait Intersection<T> {
    type Output;

    fn intersection(&self, other: &T) -> Option<Self::Output>;
}
