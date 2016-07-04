use collisions::Narrowphase;
use dynamics::{DynamicBody, DynamicBodyType, FixedBodyRef, FixedBodyRefMut, RigidBodyRef, RigidBodyRefMut};

pub enum DynamicBodyRef<'a, N, T> where T: 'static, N: Narrowphase {
    Rigid(RigidBodyRef<'a, N, T>),
    Fixed(FixedBodyRef<'a, N, T>),
}

pub enum DynamicBodyRefMut<'a, N, T> where T: 'static, N: Narrowphase {
    Rigid(RigidBodyRefMut<'a, N, T>),
    Fixed(FixedBodyRefMut<'a, N, T>),
}

impl<'a, N, T> From<&'a DynamicBody<N, T>> for DynamicBodyRef<'a, N, T> where N: Narrowphase {
    fn from(body: &'a DynamicBody<N, T>) -> DynamicBodyRef<'a, N, T> {
        match body.extra_data() {
            &DynamicBodyType::Rigid(ref data) => {
                DynamicBodyRef::Rigid(RigidBodyRef::new(body.data(), data))
            },

            &DynamicBodyType::Fixed(ref data) => {
                DynamicBodyRef::Fixed(FixedBodyRef::new(body.data(), data))
            },
        }
    }
}

impl<'a, N, T> From<&'a mut DynamicBody<N, T>> for DynamicBodyRefMut<'a, N, T> where N: Narrowphase {
    fn from(body: &'a mut DynamicBody<N, T>) -> DynamicBodyRefMut<'a, N, T> {
        let (body_data, extra) = body.split_data_mut();

        match extra {
            &mut DynamicBodyType::Rigid(ref mut data) => {
                DynamicBodyRefMut::Rigid(RigidBodyRefMut::new(body_data, data))
            },

            &mut DynamicBodyType::Fixed(ref mut data) => {
                DynamicBodyRefMut::Fixed(FixedBodyRefMut::new(body_data, data))
            },
        }
    }
}
