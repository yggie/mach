use Scalar;
use dynamics::{FixedBodyDef, MaterialData};

#[derive(Clone, Debug)]
pub struct FixedBodyData<T> {
    extra_data: T,
    material_data: MaterialData,
}

impl<T> FixedBodyData<T> {
    pub fn new(def: &FixedBodyDef, extra: T) -> FixedBodyData<T> {
        FixedBodyData {
            extra_data: extra,
            material_data: MaterialData {
                friction_coefficient: def.friction_coefficient,
                restitution_coefficient: def.restitution_coefficient,
            },
        }
    }

    #[inline(always)]
    pub fn friction_coefficient(&self) -> Scalar {
        self.material_data.friction_coefficient
    }

    #[inline(always)]
    pub fn restitution_coefficient(&self) -> Scalar {
        self.material_data.restitution_coefficient
    }
}
