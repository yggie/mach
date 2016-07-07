use Scalar;
use maths::Vec3D;
use dynamics::{DynamicBody, FixedBodyData};
use collisions::BodyData;

pub struct FixedBodyRef<'a, T>(&'a BodyData<T::Narrowphase>, &'a FixedBodyData<<T as DynamicBody>::Extension>) where T: DynamicBody;
pub struct FixedBodyRefMut<'a, T>(&'a mut BodyData<T::Narrowphase>, &'a mut FixedBodyData<<T as DynamicBody>::Extension>) where T: DynamicBody;

impl<'a, T> FixedBodyRef<'a, T> where T: DynamicBody {
    pub fn new(body_data: &'a BodyData<T::Narrowphase>, fixed_body_data: &'a FixedBodyData<<T as DynamicBody>::Extension>) -> FixedBodyRef<'a, T> {
        FixedBodyRef(body_data, fixed_body_data)
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

impl<'a, T> FixedBodyRefMut<'a, T> where T: DynamicBody {
    pub fn new(body_data: &'a mut BodyData<T::Narrowphase>, fixed_body_data: &'a mut FixedBodyData<<T as DynamicBody>::Extension>) -> FixedBodyRefMut<'a, T> {
        FixedBodyRefMut(body_data, fixed_body_data)
    }
}
