use tests::support::{Action, PropertyCheck};

pub trait Property<T> {
    type Action: Action<T>;

    fn property_check_for(&self, instance: &T, action: &Self::Action) -> Option<Box<PropertyCheck<T>>>;
}
