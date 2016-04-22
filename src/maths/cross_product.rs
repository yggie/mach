pub trait CrossProduct<T> {
    type Output;

    fn cross(self, T) -> Self::Output;
}
