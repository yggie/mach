use Scalar;

pub trait DotProduct<T> {
    fn dot(&self, other: &T) -> Scalar;
}
