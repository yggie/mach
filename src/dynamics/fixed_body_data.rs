use Scalar;
use dynamics::{FixedBodyDef, MaterialData};

#[derive(Clone, Debug)]
pub struct FixedBodyData<E> {
    extension_data: E,
    material_data: MaterialData,
}

impl<E> FixedBodyData<E> {
    pub fn new(def: &FixedBodyDef, extension: E) -> FixedBodyData<E> {
        FixedBodyData {
            extension_data: extension,
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

    #[inline(always)]
    pub fn extension_data(&self) -> &E {
        &self.extension_data
    }

    #[inline(always)]
    pub fn extension_data_mut(&mut self) -> &mut E {
        &mut self.extension_data
    }
}
