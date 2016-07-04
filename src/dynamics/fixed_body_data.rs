use Scalar;
use dynamics::MaterialData;

#[derive(Clone, Debug)]
pub struct FixedBodyData<T> {
    extra_data: T,
    material_data: MaterialData,
}

impl<T> FixedBodyData<T> {
    #[inline(always)]
    pub fn friction_coefficient(&self) -> Scalar {
        self.material_data.friction_coefficient
    }

    #[inline(always)]
    pub fn restitution_coefficient(&self) -> Scalar {
        self.material_data.restitution_coefficient
    }
}
