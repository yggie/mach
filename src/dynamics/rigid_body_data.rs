use Scalar;
use maths::{Motion, Vec3D};
use dynamics::{MaterialData, RigidBodyDef};

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

    pub fn new(def: &RigidBodyDef, extra: T) -> RigidBodyData<T> {
        RigidBodyData {
            mass: def.mass,
            motion: Motion {
                velocity: def.velocity,
                angular_velocity: def.angular_velocity,
            },
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

    #[inline(always)]
    pub fn mass(&self) -> Scalar {
        self.mass
    }

    #[inline(always)]
    pub fn mass_inverse(&self) -> Scalar {
        1.0 / self.mass
    }
}
