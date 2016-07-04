use Scalar;
use maths::{Motion, Vec3D};
use dynamics::MaterialData;

#[derive(Clone, Debug)]
pub struct RigidBodyData<T> {
    mass: Scalar,
    motion: Motion,
    extra_data: T,
    material_data: MaterialData,
}

impl<T> RigidBodyData<T> {
    include_motion_helpers! {
        struct_signature: RigidBodyData<T>,
        struct_name: RigidBodyData,
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
    pub fn mass(&self) -> Scalar {
        self.mass
    }

    #[inline(always)]
    pub fn mass_inverse(&self) -> Scalar {
        1.0 / self.mass
    }
}
