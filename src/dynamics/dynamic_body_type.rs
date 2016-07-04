use dynamics::{FixedBodyData, RigidBodyData};

pub enum DynamicBodyType<T> {
    Rigid(Box<RigidBodyData<T>>),
    Fixed(Box<FixedBodyData<T>>),
}
