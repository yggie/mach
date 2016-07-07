use Scalar;
use maths::{Motion, Vec3D};
use dynamics::{MaterialData, RigidBodyDef};

#[derive(Clone, Debug)]
pub struct RigidBodyData<E> {
    mass: Scalar,
    motion: Motion,
    material_data: MaterialData,
    extension_data: E,
}

impl<E> RigidBodyData<E> {
    include_motion_helpers! {
        struct_signature: RigidBodyData<E>,
        struct_name: RigidBodyData,
    }

    pub fn new(def: &RigidBodyDef, extension: E) -> RigidBodyData<E> {
        RigidBodyData {
            mass: def.mass,
            motion: Motion {
                velocity: def.velocity,
                angular_velocity: def.angular_velocity,
            },
            material_data: MaterialData {
                friction_coefficient: def.friction_coefficient,
                restitution_coefficient: def.restitution_coefficient,
            },
            extension_data: extension,
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
    pub fn mass(&self) -> Scalar {
        self.mass
    }

    #[inline(always)]
    pub fn mass_inverse(&self) -> Scalar {
        1.0 / self.mass
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
