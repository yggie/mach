use dynamics::{FixedBodyData, RigidBodyData};

pub enum DynamicBodyExtension<E> {
    Rigid(Box<RigidBodyData<E>>),
    Fixed(Box<FixedBodyData<E>>),
}

impl<E> DynamicBodyExtension<E> {
    pub fn extension_data(&self) -> &E {
        match self {
            &DynamicBodyExtension::Rigid(ref rigid_body_data) =>
                rigid_body_data.extension_data(),

            &DynamicBodyExtension::Fixed(ref fixed_body_data) =>
                fixed_body_data.extension_data(),
        }
    }

    pub fn extension_data_mut(&mut self) -> &mut E {
        match self {
            &mut DynamicBodyExtension::Rigid(ref mut rigid_body_data) =>
                rigid_body_data.extension_data_mut(),

            &mut DynamicBodyExtension::Fixed(ref mut fixed_body_data) =>
                fixed_body_data.extension_data_mut(),
        }
    }
}
