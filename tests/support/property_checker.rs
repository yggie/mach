pub trait PropertyCheck<T> {
    fn assert(&self, instance: &T) -> Result<(), String>;
}
