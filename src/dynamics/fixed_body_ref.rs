use Scalar;
use maths::Vec3D;
use dynamics::FixedBodyData;
use collisions::{BodyData, Narrowphase};

pub struct FixedBodyRef<'a, N, T>(&'a BodyData<N>, &'a FixedBodyData<T>) where T: 'static, N: Narrowphase;
pub struct FixedBodyRefMut<'a, N, T>(&'a mut BodyData<N>, &'a mut FixedBodyData<T>) where T: 'static, N: Narrowphase;

impl<'a, N, T> FixedBodyRef<'a, N, T> where N: Narrowphase {
    pub fn new(body_data: &'a BodyData<N>, extra_data: &'a FixedBodyData<T>) -> FixedBodyRef<'a, N, T> {
        FixedBodyRef(body_data, extra_data)
    }

    #[inline(always)]
    pub fn translation(&self) -> &Vec3D {
        self.0.translation()
    }

    #[inline(always)]
    pub fn friction_coefficient(&self) -> Scalar {
        self.1.friction_coefficient()
    }

    #[inline(always)]
    pub fn restitution_coefficient(&self) -> Scalar {
        self.1.restitution_coefficient()
    }
}

impl<'a, N, T> FixedBodyRefMut<'a, N, T> where N: Narrowphase {
    pub fn new(body_data: &'a mut BodyData<N>, extra_data: &'a mut FixedBodyData<T>) -> FixedBodyRefMut<'a, N, T> {
        FixedBodyRefMut(body_data, extra_data)
    }
}
