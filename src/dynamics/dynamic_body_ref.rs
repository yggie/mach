use dynamics::{DynamicBody, DynamicBodyExtension, FixedBodyRef, FixedBodyRefMut, RigidBodyRef, RigidBodyRefMut};

pub enum DynamicBodyRef<'a, T> where T: DynamicBody {
    Rigid(RigidBodyRef<'a, T>),
    Fixed(FixedBodyRef<'a, T>),
}

pub enum DynamicBodyRefMut<'a, T> where T: DynamicBody {
    Rigid(RigidBodyRefMut<'a, T>),
    Fixed(FixedBodyRefMut<'a, T>),
}

impl<'a, T> From<&'a T> for DynamicBodyRef<'a, T> where T: DynamicBody {
    fn from(body: &'a T) -> DynamicBodyRef<'a, T> {
        match body.dynamic_extension_data() {
            &DynamicBodyExtension::Rigid(ref data) => {
                DynamicBodyRef::Rigid(RigidBodyRef::new(body.data(), data))
            },

            &DynamicBodyExtension::Fixed(ref data) => {
                DynamicBodyRef::Fixed(FixedBodyRef::new(body.data(), data))
            },
        }
    }
}

impl<'a, T> From<&'a mut T> for DynamicBodyRefMut<'a, T> where T: DynamicBody {
    fn from(body: &'a mut T) -> DynamicBodyRefMut<'a, T> {
        let (body_data, dynamic_extension) = body.split_dynamic_extension_mut();

        match dynamic_extension {
            &mut DynamicBodyExtension::Rigid(ref mut data) => {
                DynamicBodyRefMut::Rigid(RigidBodyRefMut::new(body_data, data))
            },

            &mut DynamicBodyExtension::Fixed(ref mut data) => {
                DynamicBodyRefMut::Fixed(FixedBodyRefMut::new(body_data, data))
            },
        }
    }
}
